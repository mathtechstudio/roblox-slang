use crate::config;
use crate::roblox::{AuthConfig, RobloxCloudClient, SyncOrchestrator};
use anyhow::{Context, Result};
use colored::Colorize;
use std::path::Path;

/// Download translations from Roblox Cloud
pub async fn download(table_id: Option<String>, dry_run: bool) -> Result<()> {
    // Load configuration
    let config_path = Path::new("slang-roblox.yaml");
    let config = config::load_config(config_path).context("Failed to load configuration")?;

    // Get table_id from CLI or config
    let table_id = table_id
        .or_else(|| config.cloud.as_ref().and_then(|c| c.table_id.clone()))
        .context("Table ID not provided. Specify via --table-id or set cloud.table_id in config")?;

    // Load authentication
    let auth = AuthConfig::load(&config).context("Failed to load authentication")?;

    // Create client
    let client =
        RobloxCloudClient::new(auth.api_key).context("Failed to create Roblox Cloud client")?;

    // Create orchestrator
    let orchestrator = SyncOrchestrator::new(client, config);

    // Download
    if dry_run {
        println!("{} Dry-run mode: No files will be written", "ℹ".cyan());
    }

    println!("{} Downloading translations from cloud...", "→".blue());

    let stats = orchestrator
        .download(&table_id, dry_run)
        .await
        .context("Download failed")?;

    // Display statistics
    println!("\n{} Download complete!", "✓".green().bold());
    println!("  Entries downloaded: {}", stats.entries_downloaded);
    println!("  Locales created: {}", stats.locales_created);
    println!("  Locales updated: {}", stats.locales_updated);
    println!("  Duration: {:.2}s", stats.duration.as_secs_f64());

    if dry_run {
        println!(
            "\n{} This was a dry-run. No files were written.",
            "ℹ".cyan()
        );
    }

    Ok(())
}
