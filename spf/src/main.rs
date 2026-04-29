//! SPF: ISPF-style launcher for spf-edit.
//!
//! Displays a primary option menu panel and launches spf-edit
//! for editing/browsing, or navigates to settings/utility panels.
//!
//! Usage:
//!   spf [--panels <dir>]

use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::Command;

use crossterm::{
    execute,
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
};

use panel_runtime::PanelManager;
use tracing::{debug, info, warn};

fn main() {
    // Initialise tracing — writes to spf.log in the current directory.
    // Control verbosity via RUST_LOG env var (default: debug).
    let file_appender = tracing_appender::rolling::never(".", "spf.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("debug")),
        )
        .with_writer(non_blocking)
        .with_ansi(false)
        .init();

    info!("spf launcher starting");

    let panels_dir = find_panels_dir();

    if !panels_dir.is_dir() {
        eprintln!(
            "Error: panels directory not found at '{}'",
            panels_dir.display()
        );
        eprintln!("Run from the spf-edit workspace root, or pass --panels <dir>");
        std::process::exit(1);
    }

    if let Err(e) = run(&panels_dir) {
        // Make sure terminal is cleaned up before printing error
        let _ = terminal::disable_raw_mode();
        let _ = execute!(io::stdout(), LeaveAlternateScreen);
        warn!(error = %e, "spf exiting with error");
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}

fn run(panels_dir: &Path) -> io::Result<()> {
    let mut manager = PanelManager::new(panels_dir)?;

    if !manager.has_panel("SPFMAIN") {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "SPFMAIN panel not found in panels directory",
        ));
    }

    // Setup terminal
    terminal::enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;

    // Install panic hook for terminal cleanup
    let original_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        let _ = terminal::disable_raw_mode();
        let _ = execute!(io::stdout(), LeaveAlternateScreen);
        original_hook(info);
    }));

    // Main loop
    let result = main_loop(&mut stdout, &mut manager);

    // Cleanup terminal
    execute!(stdout, LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;

    result
}

fn main_loop<W: Write>(stdout: &mut W, manager: &mut PanelManager) -> io::Result<()> {
    loop {
        // Clear CTC before each display
        manager.vars_mut().set("ZCTC", "");

        // Display the main menu
        debug!("displaying SPFMAIN");
        let quit = manager.display(stdout, "SPFMAIN")?;
        if quit {
            break; // Ctrl+Q
        }

        // Check what action was requested
        let ctc = manager.vars().get("ZCTC").map(|s| s.to_string());
        let filename = manager
            .vars()
            .get("ZFILE")
            .map(|s| s.trim().to_string())
            .unwrap_or_default();

        match ctc.as_deref() {
            Some("EDIT") => {
                if filename.is_empty() {
                    debug!("EDIT requested but no filename");
                    continue;
                }
                info!(filename = %filename, "launching spf-edit (edit)");
                launch_spf_edit(stdout, &filename, false)?;
            }
            Some("BROWSE") => {
                if filename.is_empty() {
                    debug!("BROWSE requested but no filename");
                    continue;
                }
                info!(filename = %filename, "launching spf-edit (browse)");
                launch_spf_edit(stdout, &filename, true)?;
            }
            Some("") | None => {
                info!("user exited (UP/X)");
                break;
            }
            Some(other) => {
                warn!(ctc = %other, "unknown CTC command, ignoring");
            }
        }
    }

    Ok(())
}

/// Launch spf-edit to edit or browse a file.
/// Temporarily leaves raw mode / alternate screen so spf-edit can take over.
fn launch_spf_edit<W: Write>(stdout: &mut W, filename: &str, browse: bool) -> io::Result<()> {
    // Leave raw mode + alternate screen
    execute!(stdout, LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;

    // Find the spf-edit executable
    let exe = find_spf_edit();

    // Build and run the command
    let mut cmd = Command::new(&exe);
    cmd.arg(filename);
    if browse {
        cmd.arg("--browse");
    }

    debug!(exe = %exe.display(), filename, browse, "spawning spf-edit");
    let status = cmd.status();

    // Re-enter raw mode + alternate screen (even if spf-edit failed)
    terminal::enable_raw_mode()?;
    execute!(stdout, EnterAlternateScreen)?;

    // Report launch errors (not spf-edit's own exit code)
    if let Err(e) = status {
        if e.kind() == io::ErrorKind::NotFound {
            warn!(exe = %exe.display(), error = %e, "spf-edit executable not found");
            eprintln!("Could not find spf-edit executable: {e}");
        }
    }

    Ok(())
}

/// Find the panels directory.
fn find_panels_dir() -> PathBuf {
    // Check command-line args for --panels
    let args: Vec<String> = std::env::args().collect();
    for i in 0..args.len() {
        if args[i] == "--panels" {
            if let Some(dir) = args.get(i + 1) {
                return PathBuf::from(dir);
            }
        }
    }

    // Check current directory
    let cwd_panels = PathBuf::from("panels");
    if cwd_panels.is_dir() {
        return cwd_panels;
    }

    // Check next to the executable
    if let Ok(exe_path) = std::env::current_exe() {
        if let Some(exe_dir) = exe_path.parent() {
            let exe_panels = exe_dir.join("panels");
            if exe_panels.is_dir() {
                return exe_panels;
            }
        }
    }

    // Default: relative path (will fail with a clear error message)
    PathBuf::from("panels")
}

/// Find the spf-edit executable.
fn find_spf_edit() -> PathBuf {
    // Check next to our own executable first
    if let Ok(exe_path) = std::env::current_exe() {
        if let Some(exe_dir) = exe_path.parent() {
            let candidate = exe_dir.join(spf_edit_name());
            if candidate.exists() {
                return candidate;
            }
        }
    }

    // Fall back to PATH lookup (just use the name, let OS find it)
    PathBuf::from(spf_edit_name())
}

fn spf_edit_name() -> &'static str {
    if cfg!(windows) {
        "spf-edit.exe"
    } else {
        "spf-edit"
    }
}
