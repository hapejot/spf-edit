//! Panel loader: reads JSON panel files from the panels/ directory.
//!
//! Panels are organized by category subdirectory (dialog/, menu/, help/, etc.)
//! with a manifest.json at the root listing all panels.

use panel_model::Panel;
use std::collections::HashMap;
use std::io;
use std::path::{Path, PathBuf};
use tracing::{debug, info, warn};

/// Loads and caches panel definitions from the panels/ directory.
pub struct PanelLoader {
    panels_dir: PathBuf,
    cache: HashMap<String, Panel>,
    /// Maps panel ID (uppercase) → relative JSON path
    index: HashMap<String, PathBuf>,
}

impl PanelLoader {
    /// Create a new loader pointing at the given panels directory.
    /// Scans the directory to build an index of available panels.
    pub fn new(panels_dir: &Path) -> io::Result<Self> {
        info!(dir = %panels_dir.display(), "scanning panels directory");
        let mut index = HashMap::new();

        // Scan all subdirectories for .json files (skip manifest.json)
        if panels_dir.is_dir() {
            for entry in std::fs::read_dir(panels_dir)? {
                let entry = entry?;
                let path = entry.path();
                if path.is_dir() {
                    // Category subdirectory
                    for sub_entry in std::fs::read_dir(&path)? {
                        let sub_entry = sub_entry?;
                        let sub_path = sub_entry.path();
                        if let Some(ext) = sub_path.extension() {
                            if ext == "json" {
                                if let Some(stem) = sub_path.file_stem() {
                                    let id = stem.to_string_lossy().to_uppercase();
                                    let rel = sub_path.strip_prefix(panels_dir)
                                        .unwrap_or(&sub_path)
                                        .to_path_buf();
                                    index.insert(id, rel);
                                }
                            }
                        }
                    }
                }
            }
        }

        info!(count = index.len(), "indexed panels");

        Ok(PanelLoader {
            panels_dir: panels_dir.to_path_buf(),
            cache: HashMap::new(),
            index,
        })
    }

    /// Get a panel by ID, loading from disk if not cached.
    pub fn get(&mut self, panel_id: &str) -> io::Result<&Panel> {
        let key = panel_id.to_uppercase();

        if !self.cache.contains_key(&key) {
            debug!(panel_id = %key, "loading panel from disk");
            let panel = self.load(&key)?;
            self.cache.insert(key.clone(), panel);
        } else {
            debug!(panel_id = %key, "panel found in cache");
        }

        Ok(self.cache.get(&key).unwrap())
    }

    /// Load a panel from its JSON file.
    fn load(&self, panel_id: &str) -> io::Result<Panel> {
        let rel_path = self.index.get(panel_id).ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::NotFound,
                format!("Panel not found: {panel_id}"),
            )
        })?;

        let full_path = self.panels_dir.join(rel_path);
        debug!(path = %full_path.display(), panel_id, "reading panel JSON");
        let json_str = std::fs::read_to_string(&full_path)?;
        let panel: Panel = serde_json::from_str(&json_str).map_err(|e| {
            warn!(path = %full_path.display(), error = %e, "failed to parse panel JSON");
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Failed to parse {}: {e}", full_path.display()),
            )
        })?;

        Ok(panel)
    }

    /// Check whether a panel exists.
    pub fn has_panel(&self, panel_id: &str) -> bool {
        self.index.contains_key(&panel_id.to_uppercase())
    }

    /// Number of indexed panels.
    pub fn panel_count(&self) -> usize {
        self.index.len()
    }
}
