//! Panel manager: stack-based panel navigation.
//!
//! Implements the ISPF model where DISPLAY PANEL(name) pushes a panel
//! onto the stack, and UP/F3 pops back to the previous panel.

use std::io::{self, Write};
use std::path::{Path, PathBuf};

use crate::engine::{PanelEngine, PanelResult};
use crate::loader::PanelLoader;
use crate::vars::VarPool;
use tracing::{debug, info, warn};

/// Manages panel display with a navigation stack.
pub struct PanelManager {
    loader: PanelLoader,
    vars: VarPool,
    profiles_path: PathBuf,
}

impl PanelManager {
    /// Create a new panel manager pointing at the given panels directory.
    pub fn new(panels_dir: &Path) -> io::Result<Self> {
        let loader = PanelLoader::new(panels_dir)?;
        let mut vars = VarPool::new();
        let profiles_path = VarPool::default_profiles_path();
        vars.load_profiles_from_file(&profiles_path);
        Ok(PanelManager { loader, vars, profiles_path })
    }

    /// Get a reference to the variable pool (for reading variables after display).
    pub fn vars(&self) -> &VarPool {
        &self.vars
    }

    /// Get a mutable reference to the variable pool (for pre-setting variables).
    pub fn vars_mut(&mut self) -> &mut VarPool {
        &mut self.vars
    }

    /// Check if a panel exists.
    pub fn has_panel(&self, panel_id: &str) -> bool {
        self.loader.has_panel(panel_id)
    }

    /// Display a panel and handle navigation.
    /// This is the main entry point — it runs a modal loop until the user
    /// returns (F3/UP) or quits (Ctrl+Q).
    ///
    /// Returns `Ok(true)` if the user quit (Ctrl+Q), `Ok(false)` if they returned normally.
    pub fn display<W: Write>(&mut self, stdout: &mut W, panel_id: &str) -> io::Result<bool> {
        info!(panel_id = %panel_id, "display requested");
        let mut stack: Vec<String> = vec![panel_id.to_uppercase()];

        while let Some(current_id) = stack.last().cloned() {
            // Load the panel
            let panel = self.loader.get(&current_id)?.clone();

            // Run the engine
            let result = PanelEngine::run(stdout, &panel, &mut self.vars)?;
            debug!(?result, panel = %current_id, "panel engine returned");
            self.vars.dump();
            match result {
                PanelResult::Up => {
                    stack.pop();
                    // If stack is empty, we're done
                    if stack.is_empty() {
                        self.save_profiles();
                        return Ok(false);
                    }
                }
                PanelResult::Navigate(target) => {
                    if self.loader.has_panel(&target) {
                        debug!(target = %target, "pushing panel onto stack");
                        stack.push(target.to_uppercase());
                    } else {
                        warn!(target = %target, "navigation target panel not found");
                    }
                }
                PanelResult::NavigateList(targets) => {
                    // Display each panel in sequence
                    for target in &targets {
                        if self.loader.has_panel(target) {
                            let quit = self.display(stdout, target)?;
                            if quit {
                                return Ok(true);
                            }
                        }
                    }
                    // After the list, stay on current panel
                }
                PanelResult::Ctc(command) => {
                    info!(command = %command, "CTC — returning to caller");
                    self.vars.set("ZCTC", &command);
                    stack.pop();
                    if stack.is_empty() {
                        self.save_profiles();
                        return Ok(false);
                    }
                }
                PanelResult::Quit => {
                    self.save_profiles();
                    return Ok(true);
                }
            }
        }

        self.save_profiles();
        Ok(false)
    }

    /// Persist profiles to disk (best-effort).
    fn save_profiles(&self) {
        if let Err(e) = self.vars.save_profiles_to_file(&self.profiles_path) {
            warn!(error = %e, "failed to save panel profiles");
        }
    }
}
