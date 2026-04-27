//! Panel runtime: load, display, and interact with JSON panel definitions.
//!
//! This module implements the ISPF panel display model:
//!
//! 1. **Load** — Read JSON panel files from the `panels/` directory.
//! 2. **Variables** — Maintain a shared variable pool (system + panel-local).
//! 3. **Render** — Draw the panel to the terminal using crossterm.
//! 4. **Engine** — Modal event loop: field input, validation, navigation.
//! 5. **Manager** — Panel stack for nested panel display (push/pop/UP).
//!
//! ## ISPF Panel Lifecycle
//!
//! ```text
//! DISPLAY(panel_id)
//!   → load panel JSON
//!   → run )INIT (set .CURSOR, .HELP, ZVARS, assignments)
//!   → render panel
//!   → input loop (user types in fields, presses Enter or PF key)
//!   → collect field values into variable pool
//!   → run )PROC (VER validation, TRANS navigation)
//!   → if validation fails: show error, run )REINIT, re-render
//!   → if TRANS navigates: push new panel, recurse
//!   → if UP/F3: pop panel, return to caller
//! ```

pub mod loader;
pub mod vars;
pub mod renderer;
pub mod engine;
pub mod manager;

pub use loader::PanelLoader;
pub use vars::VarPool;
pub use renderer::PanelRenderer;
pub use engine::PanelEngine;
pub use manager::PanelManager;
