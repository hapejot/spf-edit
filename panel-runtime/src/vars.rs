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

use chrono::Local;
use tracing::{debug, info, warn};

/// A PF key binding: a label shown in the function-key bar plus the
/// command that gets submitted (as if typed on the command line and
/// followed by Enter) when the key is pressed.
#[derive(Debug, Clone)]
pub struct PfKeyDef {
    pub label: String,
    pub command: String,
}

/// The variable pool for panel display.
pub struct VarPool {
    /// Shared variables (persist across panel displays).
    shared: HashMap<String, String>,
    /// Panel-local variables (cleared on each DISPLAY).
    local: HashMap<String, String>,
    /// Per-panel profile variables (persist to disk across sessions).
    /// Outer key = panel ID (uppercase), inner map = variable name → value.
    profiles: HashMap<String, HashMap<String, String>>,
    /// Default PF key bindings (F1..F24). Per-panel overrides live in the
    /// panel JSON itself.
    pf_keys: std::collections::BTreeMap<u8, PfKeyDef>,
}

impl VarPool {
    pub fn new() -> Self {
        let mut shared = HashMap::new();

        // System variables
        shared.insert("ZPRODTSK".into(), "SPF-Edit".into());
        shared.insert(
            "ZSHRTVER".into(),
            format!("V {}", env!("CARGO_PKG_VERSION")),
        );
        shared.insert("ZVERSION".into(), env!("CARGO_PKG_VERSION").into());
        shared.insert("ZOS".into(), os_name().into());
        shared.insert("ZUSER".into(), whoami().to_uppercase());
        shared.insert(
            "ZENVIR".into(),
            format!("Rust/{}", env!("CARGO_PKG_VERSION")),
        );

        let mut pool = VarPool {
            shared,
            local: HashMap::new(),
            profiles: HashMap::new(),
            pf_keys: std::collections::BTreeMap::new(),
        };
        pool.init_default_pf_keys();
        pool.refresh_clock();
        pool
    }

    /// Install the SPF/PC default PF key set (F1..F12).
    fn init_default_pf_keys(&mut self) {
        let defaults: &[(u8, &str, &str)] = &[
            (1, "HELP", "HELP"),
            (2, "SPLIT", "SPLIT"),
            (3, "END", "END"),
            (4, "RETURN", "RETURN"),
            (5, "RFIND", "RFIND"),
            (6, "RCHANGE", "RCHANGE"),
            (7, "UP", "UP"),
            (8, "DOWN", "DOWN"),
            (9, "SWAP", "SWAP"),
            (10, "LEFT", "LEFT"),
            (11, "RIGHT", "RIGHT"),
            (12, "RETRIEVE", "RETRIEVE"),
        ];
        for (n, label, cmd) in defaults {
            self.pf_keys.insert(
                *n,
                PfKeyDef {
                    label: (*label).to_string(),
                    command: (*cmd).to_string(),
                },
            );
        }
    }

    /// Look up the default binding for PF key `n` (1..=24).
    pub fn pf_key(&self, n: u8) -> Option<&PfKeyDef> {
        self.pf_keys.get(&n)
    }

    /// Iterate the default PF key bindings in ascending key order.
    pub fn pf_keys(&self) -> impl Iterator<Item = (u8, &PfKeyDef)> {
        self.pf_keys.iter().map(|(k, v)| (*k, v))
    }

    /// Set or replace a default PF key binding.
    pub fn set_pf_key(&mut self, n: u8, label: &str, command: &str) {
        self.pf_keys.insert(
            n,
            PfKeyDef {
                label: label.to_string(),
                command: command.to_string(),
            },
        );
    }

    /// Look up a value in the saved profile for `panel_id`, without
    /// loading the profile into the local pool.
    #[allow(dead_code)]
    pub fn profile_get(&self, panel_id: &str, var: &str) -> Option<&str> {
        self.profiles
            .get(&panel_id.to_uppercase())
            .and_then(|p| p.get(&var.to_uppercase()))
            .map(|s| s.as_str())
    }

    /// Update PF key bindings from `ZPFnnCMD` / `ZPFnnLBL` variables in
    /// the current local pool. Used after the SPFKEYS configuration panel
    /// returns so the user's edits take effect immediately.
    pub fn apply_pf_keys_from_local(&mut self) {
        for n in 1u8..=24 {
            let cmd_key = format!("ZPF{:02}CMD", n);
            let lbl_key = format!("ZPF{:02}LBL", n);
            let cmd = self
                .local
                .get(&cmd_key)
                .or_else(|| self.shared.get(&cmd_key))
                .cloned();
            let label = self
                .local
                .get(&lbl_key)
                .or_else(|| self.shared.get(&lbl_key))
                .cloned();
            if let Some(cmd) = cmd {
                let cmd = cmd.trim().to_string();
                if cmd.is_empty() {
                    self.pf_keys.remove(&n);
                } else {
                    let label = label
                        .map(|s| s.trim().to_string())
                        .filter(|s| !s.is_empty())
                        .unwrap_or_else(|| cmd.to_uppercase());
                    self.pf_keys.insert(n, PfKeyDef { label, command: cmd });
                }
            }
        }
    }

    /// Update PF key bindings from a saved profile (typically the
    /// SPFKEYS panel's profile). Called once at startup so user edits
    /// persist across sessions.
    pub fn apply_pf_keys_from_profile(&mut self, panel_id: &str) {
        let key = panel_id.to_uppercase();
        let entries: Vec<(u8, Option<String>, Option<String>)> = (1u8..=24)
            .filter_map(|n| {
                let profile = self.profiles.get(&key)?;
                let cmd = profile.get(&format!("ZPF{:02}CMD", n)).cloned();
                let lbl = profile.get(&format!("ZPF{:02}LBL", n)).cloned();
                if cmd.is_some() || lbl.is_some() {
                    Some((n, cmd, lbl))
                } else {
                    None
                }
            })
            .collect();
        for (n, cmd, lbl) in entries {
            if let Some(cmd) = cmd {
                let cmd = cmd.trim().to_string();
                if cmd.is_empty() {
                    self.pf_keys.remove(&n);
                    continue;
                }
                let label = lbl
                    .map(|s| s.trim().to_string())
                    .filter(|s| !s.is_empty())
                    .unwrap_or_else(|| cmd.to_uppercase());
                self.pf_keys.insert(n, PfKeyDef { label, command: cmd });
            }
        }
    }

    /// Populate local variables `ZPFnnCMD`/`ZPFnnLBL` from current PF
    /// key bindings. Used by the SPFKEYS panel's )INIT-style setup so
    /// the user sees the present values when editing.
    pub fn populate_pf_key_locals(&mut self) {
        for n in 1u8..=24 {
            let (cmd, lbl) = self
                .pf_keys
                .get(&n)
                .map(|d| (d.command.clone(), d.label.clone()))
                .unwrap_or_default();
            self.local.insert(format!("ZPF{:02}CMD", n), cmd);
            self.local.insert(format!("ZPF{:02}LBL", n), lbl);
        }
    }

    /// Refresh time-of-day system variables (ZDATE, ZTIME).
    /// Should be called immediately before each panel display.
    pub fn refresh_clock(&mut self) {
        let now = Local::now();
        self.shared
            .insert("ZDATE".into(), now.format("%y/%m/%d").to_string());
        self.shared
            .insert("ZTIME".into(), now.format("%H:%M").to_string());
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

fn os_name() -> &'static str {
    if cfg!(target_os = "windows") {
        "WINDOWS"
    } else if cfg!(target_os = "macos") {
        "MACOS"
    } else if cfg!(target_os = "linux") {
        "LINUX"
    } else {
        "UNKNOWN"
    }
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
