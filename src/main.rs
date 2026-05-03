//! SPF-Edit: A terminal-based SPF/ISPF-style line editor.
//!
//! ## Module Overview
//!
//! - `types`      — Shared enums, color constants, layout constants.
//! - `line`       — `Line` struct (data, flags, line type, prefix command state).
//! - `line_store`  — `LineStore` trait + `VecLineStore` (abstraction over line storage).
//! - `file_io`    — Read/write files (text + fixed-record), tab expansion.
//! - `buffer`     — `FileBuffer`: owns lines + file metadata, provides editing operations.
//! - `prefix`     — Parse prefix-area commands (I, D, R, C, M, CC, DD, etc.).
//! - `line_cmd`   — Collect, validate, and execute prefix commands against the buffer.
//! - `command`    — Parse and execute primary commands (FIND, SAVE, END, etc.).
//! - `screen`     — TUI rendering: title bar, command line, data area, prefix area.
//! - `input`      — Map crossterm key events to `EditorAction` values.
//! - `editor`     — Main coordinator: owns buffer + screen + input, runs the event loop.
//!
//! ## Data Flow
//!
//!  main → Editor::run()
//!       ↓ event::read()        — blocks for input
//!       ↓ InputHandler          — key → EditorAction
//!       ↓ Editor::handle_action — dispatches action
//!           • Typing → data edits applied directly to buffer
//!           • Enter  → flush edits → execute line cmds → execute primary cmd → redraw
//!       ↓ Screen::draw_full     — redraws the terminal
//!
//! ## Known Issues / TODOs (search for "TODO" / "FIXME" / "BUG")
//!
//! See inline comments throughout each module.

mod buffer;
mod command;
mod editor;
mod file_io;
mod input;
mod line;
mod line_cmd;
mod line_store;
mod panel;
mod prefix;
mod screen;
mod types;

use std::io;
use std::path::Path;
use std::process;

use simplelog::{ConfigBuilder, LevelFilter, WriteLogger};
use tracing::{error, info};
use tracing_subscriber::EnvFilter;

use crossterm::{
    cursor, execute,
    event::{KeyboardEnhancementFlags, PopKeyboardEnhancementFlags, PushKeyboardEnhancementFlags},
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen, supports_keyboard_enhancement},
};

use crate::buffer::FileBuffer;
use crate::editor::Editor;
use crate::types::RecordFormat;

struct TracingContext {
    guard: tracing_appender::non_blocking::WorkerGuard,
}

/// Initialise file-based tracing.  The trace file is placed next to the
/// edited file (or in cwd) as `spf-edit.trace`.  Set `--trace` to enable,
/// or `SPF_TRACE=1` env var.  Without it, no file is created and all log
/// macros are no-ops (the `log` crate short-circuits at the global level).
fn init_trace() -> TracingContext {
    // let config = ConfigBuilder::new()
    //     .set_time_format_rfc3339()
    //     .set_target_level(LevelFilter::Off) // skip crate-name prefix
    //     .build();
    // let path = "spf-edit.trace";
    // match std::fs::File::create(path) {
    //     Ok(file) => {
    //         let _ = WriteLogger::init(LevelFilter::Trace, config, file);
    //     }
    //     Err(e) => {
    //         eprintln!("Warning: cannot create trace file {path}: {e}");
    //     }
    // }

    let file_appender = tracing_appender::rolling::hourly("/tmp", "spf-edit.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
    tracing_subscriber::fmt()
        .with_writer(non_blocking)
        .with_level(true)
        .with_env_filter(EnvFilter::from_default_env())
        .init();
    info!("starting");
    TracingContext { guard: _guard }
}

fn main() {
    // Parse command-line arguments
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: spf-edit <filename> [--lrecl N] [--browse] [--trace]");
        process::exit(1);
    }

    let filename = &args[1];
    let mut record_format = RecordFormat::Variable;
    let mut browse_mode = false;
    let mut trace_mode = false;

    let mut i = 2;
    while i < args.len() {
        match args[i].as_str() {
            "--lrecl" => {
                i += 1;
                if i >= args.len() {
                    eprintln!("--lrecl requires a numeric argument");
                    process::exit(1);
                }
                match args[i].parse::<usize>() {
                    Ok(n) if n > 0 => record_format = RecordFormat::Fixed(n),
                    _ => {
                        eprintln!("Invalid record length: {}", args[i]);
                        process::exit(1);
                    }
                }
            }
            "--browse" => {
                browse_mode = true;
            }
            "--trace" => {
                trace_mode = true;
            }
            other => {
                eprintln!("Unknown option: {other}");
                process::exit(1);
            }
        }
        i += 1;
    }

    // Enable tracing if requested (also via SPF_TRACE=1 env var)
    trace_mode = trace_mode
        || std::env::var("SPF_TRACE")
            .map(|v| v == "1")
            .unwrap_or(false);
    let _tracing_context = if trace_mode { Some(init_trace()) } else { None };
    info!("spf-edit starting: file={filename:?} format={record_format:?} browse={browse_mode}");

    // Set up panic hook to restore terminal
    let original_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        error!("PANIC: {panic_info}");
        // Best effort terminal cleanup
        let _ = terminal::disable_raw_mode();
        let _ = execute!(io::stdout(), LeaveAlternateScreen, cursor::Show);
        original_hook(panic_info);
    }));

    // Run the editor
    if let Err(e) = run_editor(filename, record_format, browse_mode) {
        error!("Editor exited with error: {e}");
        eprintln!("Error: {e}");
        process::exit(1);
    }
    info!("spf-edit exiting normally");
}

/// Set up terminal, run the editor event loop, then restore terminal on exit.
///
/// Terminal setup: raw mode + alternate screen.  Cleanup runs even on error.
fn run_editor(filename: &str, record_format: RecordFormat, browse_mode: bool) -> io::Result<()> {
    let path = Path::new(filename);

    // Open or create the file buffer
    let buffer = if path.exists() {
        FileBuffer::open(path, record_format, browse_mode)?
    } else {
        if browse_mode {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                format!("File not found: {filename}"),
            ));
        }
        FileBuffer::new_empty(path, record_format)
    };

    // Set up terminal
    terminal::enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, cursor::Show)?;
    // Best-effort: enable keyboard enhancement so we can distinguish
    // Numpad Enter (submit) from regular Enter (newline).
    let kbd_enhanced = supports_keyboard_enhancement().unwrap_or(false);
    if kbd_enhanced {
        let _ = execute!(
            stdout,
            PushKeyboardEnhancementFlags(
                KeyboardEnhancementFlags::DISAMBIGUATE_ESCAPE_CODES
            )
        );
    }

    // Create and run editor
    let mut editor = Editor::new(buffer)?;
    let result = editor.run(&mut stdout);

    // Restore terminal — must happen even on error
    if kbd_enhanced {
        let _ = execute!(stdout, PopKeyboardEnhancementFlags);
    }
    execute!(stdout, LeaveAlternateScreen, cursor::Show)?;
    terminal::disable_raw_mode()?;

    result
}
