use anyhow::{bail, Context, Result};
use colored::Colorize;
use std::path::Path;

use crate::migrator::{KeyTransform, MigrationFormat};

/// Migrate translations from another format to Slang format
pub fn migrate(
    format: &str,
    input_path: &Path,
    output_path: &Path,
    transform: Option<&str>,
) -> Result<()> {
    println!("{} Migrating translations...", "→".blue());

    // Parse format
    let migration_format = match format.to_lowercase().as_str() {
        "custom-json" | "custom" | "json" => MigrationFormat::CustomJson,
        "gettext" | "po" => MigrationFormat::Gettext,
        _ => bail!(
            "Unsupported format: {}. Supported: custom-json, gettext",
            format
        ),
    };

    // Parse transform
    let key_transform = match transform {
        Some("snake-to-camel") => KeyTransform::SnakeToCamel,
        Some("upper-to-lower") => KeyTransform::UpperToLower,
        Some("dot-to-nested") => KeyTransform::DotToNested,
        Some("none") | None => KeyTransform::None,
        Some(t) => bail!("Unsupported transform: {}. Supported: snake-to-camel, upper-to-lower, dot-to-nested, none", t),
    };

    // Check input file exists
    if !input_path.exists() {
        bail!("Input file not found: {}", input_path.display());
    }

    println!("{} Format: {:?}", "→".blue(), migration_format);
    println!("{} Transform: {:?}", "→".blue(), key_transform);
    println!("{} Input: {}", "→".blue(), input_path.display());
    println!("{} Output: {}", "→".blue(), output_path.display());

    // Perform migration
    crate::migrator::migrate(migration_format, input_path, output_path, key_transform)
        .context("Migration failed")?;

    println!("{} Migration completed successfully!", "✓".green());
    println!();
    println!("Next steps:");
    println!("  1. Review the output file: {}", output_path.display());
    println!("  2. Copy to your translations directory");
    println!("  3. Run: roblox-slang build");

    Ok(())
}
