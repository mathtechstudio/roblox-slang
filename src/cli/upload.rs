use crate::config;
use crate::roblox::{AuthConfig, RobloxCloudClient, SyncOrchestrator};
use anyhow::{Context, Result};
use colored::Colorize;
use std::path::Path;

/// Upload local translations to Roblox Cloud
pub async fn upload(table_id: Option<String>, dry_run: bool, skip_validation: bool) -> Result<()> {
    // Load configuration
    let config_path = Path::new("slang-roblox.yaml");
    let config = config::load_config(config_path).context("Failed to load configuration")?;

    // Get table_id from CLI or config
    let table_id = table_id
        .or_else(|| config.cloud.as_ref().and_then(|c| c.table_id.clone()))
        .context("Table ID not provided. Specify via --table-id or set cloud.table_id in config")?;

    // Run validation unless skipped
    if !skip_validation {
        println!("{} Running pre-upload validation...", "→".blue());

        // Use existing validator - just validate config
        if let Err(e) = config.validate() {
            eprintln!("{} Validation failed: {}", "✗".red(), e);
            eprintln!(
                "\n{} Use --skip-validation to bypass validation",
                "Hint:".yellow()
            );
            return Err(e);
        }

        println!("{} Validation passed", "✓".green());
    }

    // Load authentication
    let auth = AuthConfig::load(&config).context("Failed to load authentication")?;

    // Create client
    let client =
        RobloxCloudClient::new(auth.api_key).context("Failed to create Roblox Cloud client")?;

    // Save locales before moving config
    let locales_str = config.supported_locales.join(", ");

    // Create orchestrator
    let orchestrator = SyncOrchestrator::new(client, config);

    // Upload
    if dry_run {
        println!("{} Dry-run mode: No changes will be made", "ℹ".cyan());
    }

    println!("{} Uploading translations to cloud...", "→".blue());
    println!("  Table ID: {}", table_id);
    println!("  Locales: {}", locales_str);

    let stats = orchestrator
        .upload(&table_id, dry_run)
        .await
        .context("Upload failed")?;

    // Display statistics
    println!("\n{} Upload complete!", "✓".green().bold());
    println!("  Entries uploaded: {}", stats.entries_uploaded);
    println!("  Locales processed: {}", stats.locales_processed);
    println!("  Duration: {:.2}s", stats.duration.as_secs_f64());

    if dry_run {
        println!("\n{} This was a dry-run. No changes were made.", "ℹ".cyan());
    } else {
        println!(
            "\n{} Translations successfully uploaded to Roblox Cloud",
            "✓".green()
        );
    }

    Ok(())
}
