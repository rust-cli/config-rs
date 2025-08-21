use std::ffi::OsString;

use config::{Config, Environment, Map};
use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
struct AppConfig {
    debug: bool,
    port: u16,
    secret_key: Option<String>, // This should NOT be overridable via env
}

/// Example of safe environment variable filtering
///
/// This demonstrates how to safely filter environment variables
/// without risking Unicode panics or race conditions.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Simulate setting some environment variables
    std::env::set_var("MYAPP_DEBUG", "true");
    std::env::set_var("MYAPP_PORT", "8080");
    std::env::set_var("MYAPP_SECRET_KEY", "should_be_ignored");

    // Safe filtering using os_source - no Unicode panics, no race conditions
    let filtered_env = safe_filter_env_vars("MYAPP_")?;

    let config = Config::builder()
        .set_default("debug", false)?
        .set_default("port", 3000)?
        .set_default("secret_key", "hardcoded_secret")?
        .add_source(
            Environment::with_prefix("MYAPP")
                .separator("_")
                .source_os(Some(filtered_env))
                .try_parsing(true),
        )
        .build()?;

    let app_config: AppConfig = config.try_deserialize()?;

    println!("Config: {app_config:#?}");

    // Verify that sensitive keys were blocked
    assert_eq!(app_config.debug, true); // ✅ Overridden by env
    assert_eq!(app_config.port, 8080); // ✅ Overridden by env
    assert_eq!(app_config.secret_key, Some("hardcoded_secret".to_owned())); // ✅ NOT overridden

    println!("✅ Safe environment filtering working correctly!");

    Ok(())
}

/// Safely filter environment variables without Unicode panics or race conditions
///
/// This approach:
/// 1. Uses `env::vars_os()` to avoid Unicode panics
/// 2. Captures a single atomic snapshot to avoid race conditions
/// 3. Allows safe filtering based on key names
fn safe_filter_env_vars(
    prefix: &str,
) -> Result<Map<OsString, OsString>, Box<dyn std::error::Error>> {
    let mut filtered_env = Map::new();

    // Single atomic snapshot - no race condition
    for (key, value) in std::env::vars_os() {
        // Safe Unicode conversion - no panic
        if let Some(key_str) = key.to_str() {
            if key_str.starts_with(prefix) {
                // Extract leaf key for sensitivity check
                let stripped = key_str.strip_prefix(prefix).unwrap_or(key_str);
                let leaf_key = stripped.split("__").last().unwrap_or(stripped);

                // Block sensitive environment variable overrides
                if !is_sensitive_leaf_key(leaf_key) {
                    filtered_env.insert(key, value);
                }
                // Sensitive keys are silently ignored
            }
        }
        // Non-Unicode keys are silently ignored (no panic)
    }

    Ok(filtered_env)
}

/// Check if a key should be considered sensitive
fn is_sensitive_leaf_key(leaf_key: &str) -> bool {
    let lower = leaf_key.to_ascii_lowercase();
    lower.ends_with("password")
        || lower.ends_with("secret")
        || lower.ends_with("token")
        || lower.ends_with("cookie")
        || lower.ends_with("private_key")
}
