use crate::utils::locales;
use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};

/// Main configuration structure for Roblox Slang
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Config {
    /// Base locale (e.g., "en")
    pub base_locale: String,

    /// List of supported locales (e.g., ["en", "id", "es"])
    pub supported_locales: Vec<String>,

    /// Input directory containing translation files
    #[serde(default = "default_input_directory")]
    pub input_directory: String,

    /// Output directory for generated files
    #[serde(default = "default_output_directory")]
    pub output_directory: String,

    /// Optional namespace prefix for generated code
    #[serde(default)]
    pub namespace: Option<String>,

    /// Override configuration
    #[serde(default)]
    pub overrides: Option<OverrideConfig>,

    /// Analytics configuration
    #[serde(default)]
    pub analytics: Option<AnalyticsConfig>,
}

/// Override configuration
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OverrideConfig {
    /// Enable override system
    #[serde(default)]
    pub enabled: bool,

    /// Path to override file (relative to project root)
    #[serde(default = "default_override_file")]
    pub file: String,
}

/// Analytics configuration
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AnalyticsConfig {
    /// Enable analytics tracking
    #[serde(default)]
    pub enabled: bool,

    /// Track missing translations
    #[serde(default = "default_true")]
    pub track_missing: bool,

    /// Track translation usage
    #[serde(default)]
    pub track_usage: bool,

    /// Optional custom callback module path
    #[serde(default)]
    pub callback: Option<String>,
}

impl Config {
    /// Validate configuration values
    pub fn validate(&self) -> Result<()> {
        // Validate base_locale
        if self.base_locale.is_empty() {
            bail!(
                "Configuration error: base_locale cannot be empty\n\
                 \n\
                 Expected format: base_locale: en\n\
                 \n\
                 Hint: The base_locale is your primary language (fallback).\n\
                 Common values: en, es, pt, de, fr, ja, ko, zh-cn, zh-tw"
            );
        }

        // Validate supported_locales
        if self.supported_locales.is_empty() {
            bail!(
                "Configuration error: supported_locales cannot be empty\n\
                 \n\
                 Expected format:\n\
                 supported_locales:\n\
                   - en\n\
                   - id\n\
                   - es\n\
                 \n\
                 Hint: List all languages your game will support."
            );
        }

        // Validate base_locale is in supported_locales
        if !self.supported_locales.contains(&self.base_locale) {
            bail!(
                "Configuration error: base_locale '{}' must be included in supported_locales\n\
                 \n\
                 Current supported_locales: [{}]\n\
                 \n\
                 Fix: Add '{}' to your supported_locales list:\n\
                 supported_locales:\n\
                   - {}\n\
                 {}",
                self.base_locale,
                self.supported_locales.join(", "),
                self.base_locale,
                self.base_locale,
                self.supported_locales
                    .iter()
                    .map(|l| format!("  - {}", l))
                    .collect::<Vec<_>>()
                    .join("\n")
            );
        }

        // Validate that all locales are supported by Roblox
        let mut unsupported = Vec::new();
        for locale in &self.supported_locales {
            if !locales::is_roblox_locale(locale) {
                unsupported.push(locale.clone());
            }
        }

        if !unsupported.is_empty() {
            let supported = locales::get_supported_locale_codes();
            bail!(
                "Configuration error: Unsupported locale(s): {}\n\
                 \n\
                 Roblox supports these 17 locales:\n\
                 {}\n\
                 \n\
                 Common mistakes:\n\
                 - Using uppercase (use 'en' not 'EN')\n\
                 - Using wrong format (use 'zh-cn' not 'zh_CN')\n\
                 - Using unsupported locales\n\
                 \n\
                 Hint: Check https://create.roblox.com/docs/production/localization for details.",
                unsupported.join(", "),
                supported
                    .iter()
                    .map(|l| format!("  • {}", l))
                    .collect::<Vec<_>>()
                    .join("\n")
            );
        }

        // Validate input_directory
        if self.input_directory.is_empty() {
            bail!(
                "Configuration error: input_directory cannot be empty\n\
                 \n\
                 Expected format: input_directory: translations\n\
                 \n\
                 Hint: This is where your JSON/YAML translation files are located."
            );
        }

        // Validate output_directory
        if self.output_directory.is_empty() {
            bail!(
                "Configuration error: output_directory cannot be empty\n\
                 \n\
                 Expected format: output_directory: output\n\
                 \n\
                 Hint: This is where generated Luau code will be placed."
            );
        }

        // Validate input and output are different
        if self.input_directory == self.output_directory {
            bail!(
                "Configuration error: input_directory and output_directory cannot be the same\n\
                 \n\
                 Current value: '{}'\n\
                 \n\
                 Hint: Use different directories to avoid overwriting source files.\n\
                 Example:\n\
                   input_directory: translations\n\
                   output_directory: output",
                self.input_directory
            );
        }

        Ok(())
    }
}

fn default_input_directory() -> String {
    "translations".to_string()
}

fn default_output_directory() -> String {
    "output".to_string()
}

fn default_override_file() -> String {
    "overrides.yaml".to_string()
}

fn default_true() -> bool {
    true
}
