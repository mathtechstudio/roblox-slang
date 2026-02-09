//! # Roblox Slang
//!
//! Type-safe internationalization (i18n) for Roblox experiences.
//!
//! Roblox Slang is a CLI tool and library that generates type-safe Luau code
//! from translation files, enabling autocomplete and compile-time validation
//! for game localization.
//!
//! ## Features
//!
//! - **Type-safe translations**: Generate Luau code with full autocomplete support
//! - **Multiple input formats**: JSON, YAML, and CSV
//! - **Pluralization**: CLDR-compliant plural rules for all languages
//! - **String interpolation**: Type-safe parameter substitution
//! - **Roblox Cloud integration**: Upload/download translations to Roblox Cloud
//! - **Watch mode**: Auto-rebuild on file changes
//! - **Validation**: Detect missing keys, conflicts, and coverage issues
//!
//! ## Quick Start
//!
//! ```bash
//! # Initialize a new project
//! roblox-slang init
//!
//! # Build translations
//! roblox-slang build
//!
//! # Watch for changes
//! roblox-slang build --watch
//! ```
//!
//! ## Library Usage
//!
//! ```no_run
//! use roblox_slang::{Config, parser, generator, config};
//! use std::path::Path;
//!
//! # fn main() -> anyhow::Result<()> {
//! // Load configuration
//! let cfg = config::load_config(Path::new("slang-roblox.yaml"))?;
//!
//! // Parse translations
//! let translations = parser::json::parse_json_file(
//!     Path::new("translations/en.json"),
//!     "en"
//! )?;
//!
//! // Generate Luau code
//! let luau_code = generator::luau::generate_luau(&translations, &cfg.base_locale)?;
//! # Ok(())
//! # }
//! ```
//!
//! ## Modules
//!
//! - [`cli`]: Command-line interface implementation
//! - [`config`]: Configuration file parsing and validation
//! - [`generator`]: Luau code and CSV generation
//! - [`migrator`]: Migration from other i18n formats
//! - [`parser`]: Translation file parsing (JSON, YAML, CSV)
//! - [`utils`]: Utility functions (flattening, validation, etc.)
//! - [`validator`]: Translation validation and analysis

pub mod cli;
pub mod config;
pub mod generator;
pub mod migrator;
pub mod parser;
pub mod roblox;
pub mod utils;
pub mod validator;

// Re-export commonly used types
pub use config::Config;
pub use parser::types::Translation;
