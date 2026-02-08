//! Migration tools
//!
//! This module provides tools for migrating translations from other formats
//! (custom JSON, gettext) to roblox-slang format.

pub mod custom_json;
pub mod gettext;
pub mod key_transform;

use anyhow::Result;
use std::path::Path;

/// Migration format
#[derive(Debug, Clone, Copy)]
pub enum MigrationFormat {
    CustomJson,
    Gettext,
}

/// Key transformation strategy
#[derive(Debug, Clone, Copy)]
pub enum KeyTransform {
    SnakeToCamel,
    UpperToLower,
    DotToNested,
    None,
}

/// Migrate translations from another format to Slang format
pub fn migrate(
    format: MigrationFormat,
    input_path: &Path,
    output_path: &Path,
    transform: KeyTransform,
) -> Result<()> {
    match format {
        MigrationFormat::CustomJson => {
            custom_json::migrate_custom_json(input_path, output_path, transform)
        }
        MigrationFormat::Gettext => gettext::migrate_gettext(input_path, output_path, transform),
    }
}
