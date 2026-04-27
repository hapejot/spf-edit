//! Variable pool: shared and panel-local variable storage.
//!
//! In ISPF, variables live in a shared pool accessible to all panels.
//! System variables (Z-prefix) are pre-populated; panel variables are
//! set by )INIT assignments and user input, and read back in )PROC.

use std::collections::HashMap;

/// The variable pool for panel display.
pub struct VarPool {
    /// Shared variables (persist across panel displays).
    shared: HashMap<String, String>,
    /// Panel-local variables (cleared on each DISPLAY).
    local: HashMap<String, String>,
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
        shared.insert("ZENVIR".into(), format!("Rust/{}", env!("CARGO_PKG_VERSION")));

        VarPool {
            shared,
            local: HashMap::new(),
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
