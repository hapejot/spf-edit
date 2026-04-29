//! Variable pool: shared and panel-local variable storage.
//!
//! In ISPF, variables live in a shared pool accessible to all panels.
//! System variables (Z-prefix) are pre-populated; panel variables are
//! set by )INIT assignments and user input, and read back in )PROC.
//!
//! Per-panel **profiles** persist field values across sessions. When a
//! panel is displayed, its profile variables are loaded into the local
//! pool before )INIT runs (so )INIT defaults only apply to variables
//! that have no saved value). When the panel exits, all non-command
//! field values are saved back to the profile.

use std::collections::HashMap;
use std::io;
use std::path::{Path, PathBuf};

use tracing::{debug, info, warn};

/// The variable pool for panel display.
pub struct VarPool {
    /// Shared variables (persist across panel displays).
    shared: HashMap<String, String>,
    /// Panel-local variables (cleared on each DISPLAY).
    local: HashMap<String, String>,
    /// Per-panel profile variables (persist to disk across sessions).
    /// Outer key = panel ID (uppercase), inner map = variable name → value.
    profiles: HashMap<String, HashMap<String, String>>,
}

impl VarPool {
    pub fn new() -> Self {
        let mut shared = HashMap::new();

        // System variables
        shared.insert("ZPRODTSK".into(), "SPF-Edit".into());
        shared.insert("ZSHRTVER".into(), env!("CARGO_PKG_VERSION").into());
        shared.insert("ZOS".into(), std::env::consts::OS.into());
        shared.insert("ZUSER".into(), whoami().into());
        shared.insert("ZDATE".into(), today_date());
        shared.insert("ZTIME".into(), "".into()); // updated at display time
        shared.insert(
            "ZENVIR".into(),
            format!("Rust/{}", env!("CARGO_PKG_VERSION")),
        );

        VarPool {
            shared,
            local: HashMap::new(),
            profiles: HashMap::new(),
        }
    }

    /// Get a variable value (local takes precedence over shared).
    pub fn get(&self, name: &str) -> Option<&str> {
        let key = name.to_uppercase();
        self.local
            .get(&key)
            .or_else(|| self.shared.get(&key))
            .map(|s| s.as_str())
    }

    /// Set a variable (goes into local pool).
    pub fn set(&mut self, name: &str, value: &str) {
        self.local.insert(name.to_uppercase(), value.to_string());
    }

    /// Set a shared (persistent) variable.
    pub fn set_shared(&mut self, name: &str, value: &str) {
        self.shared.insert(name.to_uppercase(), value.to_string());
    }

    /// Clear local variables (called before each panel display).
    pub fn clear_local(&mut self) {
        self.local.clear();
    }

    /// Resolve a string containing &VAR references.
    /// Replaces `&VAR` with the variable's value, or empty string if unset.
    pub fn resolve(&self, text: &str) -> String {
        let mut result = String::with_capacity(text.len());
        let mut chars = text.chars().peekable();

        while let Some(ch) = chars.next() {
            if ch == '&' {
                // Collect variable name (alphanumeric + underscore)
                let mut var_name = String::new();
                while let Some(&c) = chars.peek() {
                    if c.is_alphanumeric() || c == '_' {
                        var_name.push(c);
                        chars.next();
                    } else {
                        break;
                    }
                }
                if var_name.is_empty() {
                    result.push('&');
                } else {
                    result.push_str(self.get(&var_name).unwrap_or(""));
                }
            } else {
                result.push(ch);
            }
        }

        result
    }

    pub fn dump(&self) {
        debug!("Variable pool dump:");
        self.local
            .iter()
            .for_each(|(k, v)| debug!("local: {k}={v}"));
        self.shared
            .iter()
            .for_each(|(k, v)| debug!("shared: {k}={v}"));
    }

    // ─── Per-panel profile persistence ──────────────────────────────────

    /// Load saved profile variables for a panel into the local pool.
    /// Called before )INIT so that saved values act as defaults.
    pub fn load_profile(&mut self, panel_id: &str) {
        let key = panel_id.to_uppercase();
        if let Some(profile) = self.profiles.get(&key) {
            debug!(panel = %key, count = profile.len(), "loading profile vars");
            for (var, val) in profile {
                // Only set if not already present in local (preserves any
                // values that were pre-set before the panel display).
                if !self.local.contains_key(var) {
                    self.local.insert(var.clone(), val.clone());
                }
            }
        }
    }

    /// Save field variable values into the profile for a panel.
    /// `fields` is a list of (variable_name, value) pairs from the panel's
    /// non-command input fields.
    pub fn save_profile(&mut self, panel_id: &str, fields: &[(String, String)]) {
        let key = panel_id.to_uppercase();
        let profile = self.profiles.entry(key.clone()).or_default();
        for (var, val) in fields {
            let var_key = var.to_uppercase();
            debug!(panel = %key, var = %var_key, val = %val, "saving profile var");
            profile.insert(var_key, val.clone());
        }
    }

    /// Load all profiles from a JSON file on disk.
    pub fn load_profiles_from_file(&mut self, path: &Path) {
        match std::fs::read_to_string(path) {
            Ok(contents) => {
                match serde_json::from_str::<HashMap<String, HashMap<String, String>>>(&contents) {
                    Ok(loaded) => {
                        info!(path = %path.display(), panels = loaded.len(), "loaded profiles");
                        self.profiles = loaded;
                    }
                    Err(e) => {
                        warn!(path = %path.display(), error = %e, "failed to parse profiles file");
                    }
                }
            }
            Err(e) if e.kind() == io::ErrorKind::NotFound => {
                debug!(path = %path.display(), "no profiles file yet");
            }
            Err(e) => {
                warn!(path = %path.display(), error = %e, "failed to read profiles file");
            }
        }
    }

    /// Save all profiles to a JSON file on disk.
    pub fn save_profiles_to_file(&self, path: &Path) -> io::Result<()> {
        if self.profiles.is_empty() {
            return Ok(());
        }
        // Ensure parent directory exists
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let json = serde_json::to_string_pretty(&self.profiles)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        std::fs::write(path, json)?;
        info!(path = %path.display(), panels = self.profiles.len(), "saved profiles");
        Ok(())
    }

    /// Return the default profiles file path for this platform.
    pub fn default_profiles_path() -> PathBuf {
        if cfg!(windows) {
            if let Ok(appdata) = std::env::var("APPDATA") {
                return PathBuf::from(appdata)
                    .join("spf-edit")
                    .join("profiles.json");
            }
        }
        // Unix / fallback
        if let Ok(home) = std::env::var("HOME") {
            return PathBuf::from(home)
                .join(".config")
                .join("spf-edit")
                .join("profiles.json");
        }
        // Last resort
        PathBuf::from("profiles.json")
    }
}

fn whoami() -> String {
    std::env::var("USERNAME")
        .or_else(|_| std::env::var("USER"))
        .unwrap_or_else(|_| "user".into())
}

fn today_date() -> String {
    // Simple date without external crate — use system time
    let now = std::time::SystemTime::now();
    let since_epoch = now
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default();
    let days = since_epoch.as_secs() / 86400;
    // Rough year/month/day calculation — sufficient for display
    let year = 1970 + (days / 365);
    format!("{year}/01/01") // Placeholder — real date formatting needs a crate
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resolve_simple() {
        let mut pool = VarPool::new();
        pool.set("NAME", "World");
        assert_eq!(pool.resolve("Hello &NAME!"), "Hello World!");
    }

    #[test]
    fn test_resolve_no_var() {
        let pool = VarPool::new();
        assert_eq!(pool.resolve("No variables here"), "No variables here");
    }

    #[test]
    fn test_resolve_unknown_var() {
        let pool = VarPool::new();
        assert_eq!(pool.resolve("Hello &NOBODY"), "Hello ");
    }

    #[test]
    fn test_local_over_shared() {
        let mut pool = VarPool::new();
        pool.set_shared("X", "shared");
        pool.set("X", "local");
        assert_eq!(pool.get("X"), Some("local"));
    }

    #[test]
    fn test_clear_local() {
        let mut pool = VarPool::new();
        pool.set("X", "local");
        pool.clear_local();
        assert_eq!(pool.get("X"), None);
    }
}
