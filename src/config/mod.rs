//! Configuration management
//!
//! This module handles loading, validating, and creating configuration files
//! for roblox-slang projects.

mod defaults;
mod schema;

pub use schema::*;

use anyhow::{bail, Result};
use std::path::Path;

/// Load configuration from a YAML file
pub fn load_config(path: &Path) -> Result<Config> {
    // Check if config file exists
    if !path.exists() {
        bail!(
            "Configuration file not found: {}\n\
             \n\
             Hint: Run `slang-roblox init` to create a default configuration.\n\
             Or create the file manually with:\n\
             \n\
             base_locale: en\n\
             supported_locales:\n\
               - en\n\
             input_directory: translations\n\
             output_directory: output",
            path.display()
        );
    }

    // Read config file
    let content = std::fs::read_to_string(path).map_err(|e| {
        anyhow::anyhow!(
            "Failed to read configuration file: {}\n\
             Error: {}\n\
             \n\
             Hint: Check file permissions and ensure the file is readable.",
            path.display(),
            e
        )
    })?;

    // Check if file is empty
    if content.trim().is_empty() {
        bail!(
            "Configuration file is empty: {}\n\
             \n\
             Hint: Run `slang-roblox init` to create a default configuration.",
            path.display()
        );
    }

    // Parse YAML config
    let config: Config = serde_yaml::from_str(&content).map_err(|e| {
        let location = e.location();
        let (line, column) = if let Some(loc) = location {
            (loc.line(), loc.column())
        } else {
            (0, 0)
        };

        // Try to extract the problematic line
        let lines: Vec<&str> = content.lines().collect();
        let context_line = if line > 0 && line <= lines.len() {
            lines[line - 1]
        } else {
            ""
        };

        anyhow::anyhow!(
            "Failed to parse configuration file: {}\n\
             Error at line {}, column {}: {}\n\
             \n\
             Problematic line:\n\
             {}\n\
             {}^\n\
             \n\
             Common configuration errors:\n\
             - Incorrect indentation (use 2 spaces)\n\
             - Missing colon after key\n\
             - Wrong field names (check spelling)\n\
             \n\
             Hint: Compare with the example in the documentation or run `slang-roblox init`.",
            path.display(),
            line,
            column,
            e,
            context_line,
            " ".repeat(column.saturating_sub(1))
        )
    })?;

    // Validate config
    config.validate().map_err(|e| {
        anyhow::anyhow!(
            "{}\n\
             \n\
             Configuration file: {}",
            e,
            path.display()
        )
    })?;

    Ok(config)
}

/// Create a default configuration file
pub fn create_default_config(path: &Path) -> Result<()> {
    let yaml = r#"# Roblox Slang Configuration
# Documentation: https://github.com/mathtechstudio/roblox-slang

# Base locale (fallback when translation is missing)
base_locale: en

# Supported locales for your game
supported_locales:
  - en

# Directory containing translation files (JSON/YAML)
input_directory: translations

# Directory for generated Luau code
output_directory: output

# Optional: Namespace for generated module (null = no namespace)
namespace: null

# Optional: Translation overrides
# overrides:
#   enabled: true
#   file: overrides.yaml

# Optional: Analytics for tracking missing translations
# analytics:
#   enabled: true
#   track_missing: true
#   track_usage: true

# Optional: Roblox Cloud integration for syncing translations
# Enables upload, download, and bidirectional sync with Roblox Cloud Localization Tables
# cloud:
#   # Localization table ID (UUID format)
#   # Get from: Creator Dashboard > Localization > Table Settings
#   table_id: "your-table-id-here"
#   
#   # Game/Universe ID (numeric)
#   # Get from: Creator Dashboard > Game Settings > Basic Info
#   game_id: "your-game-id-here"
#   
#   # API Key (RECOMMENDED: Use environment variable instead)
#   # Set via: export ROBLOX_CLOUD_API_KEY=your_key_here
#   # Get from: https://create.roblox.com/credentials
#   # api_key: "your-api-key-here"
#   
#   # Default merge strategy for sync command
#   # Options:
#   #   - merge: Upload local-only, download cloud-only, prefer cloud for conflicts (recommended)
#   #   - overwrite: Replace all cloud translations with local (destructive)
#   #   - skip-conflicts: Only sync non-conflicting entries
#   strategy: merge
"#;

    std::fs::write(path, yaml).map_err(|e| {
        anyhow::anyhow!(
            "Failed to write configuration file: {}\n\
             Error: {}\n\
             \n\
             Hint: Check directory permissions and ensure you have write access.",
            path.display(),
            e
        )
    })?;

    Ok(())
}

/// Create a default overrides file
pub fn create_default_overrides(path: &Path) -> Result<()> {
    let yaml = r#"# Translation Overrides
# Override specific translations per locale without modifying source files
# Useful for A/B testing, seasonal events, or quick fixes

# Example overrides:
# en:
#   ui.buttons.buy: "Purchase Now!"
#   ui.messages.greeting: "Hey there, {name}!"
#
# es:
#   ui.buttons.buy: "¡Comprar Ahora!"
#
# id:
#   ui.buttons.buy: "Beli Sekarang!"
"#;

    std::fs::write(path, yaml).map_err(|e| {
        anyhow::anyhow!(
            "Failed to write overrides file: {}\n\
             Error: {}\n\
             \n\
             Hint: Check directory permissions and ensure you have write access.",
            path.display(),
            e
        )
    })?;

    Ok(())
}
