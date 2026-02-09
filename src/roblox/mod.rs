//! Roblox Cloud API integration module
//!
//! This module provides functionality for syncing translations with Roblox Cloud
//! Localization Tables via the Open Cloud API.
//!
//! # Overview
//!
//! The cloud sync feature enables bidirectional synchronization between local
//! translation files and Roblox Cloud Localization Tables. This allows you to:
//!
//! - Upload local translations to Roblox Cloud
//! - Download translations from Roblox Cloud
//! - Synchronize with merge strategies (overwrite, merge, skip-conflicts)
//! - Handle rate limiting and retries automatically
//! - Resolve conflicts between local and cloud versions
//!
//! # Authentication
//!
//! Authentication requires a Roblox Cloud API key. Set it via:
//!
//! 1. Environment variable (recommended):
//!    ```bash
//!    export ROBLOX_CLOUD_API_KEY=your_api_key_here
//!    ```
//!
//! 2. Configuration file (slang-roblox.yaml):
//!    ```yaml
//!    cloud:
//!      api_key: your_api_key_here
//!    ```
//!
//! Get your API key from: <https://create.roblox.com/credentials>
//!
//! # Example Usage
//!
//! ```no_run
//! use roblox_slang::roblox::{AuthConfig, RobloxCloudClient, SyncOrchestrator};
//! use roblox_slang::config::Config;
//!
//! # async fn example() -> anyhow::Result<()> {
//! // Load configuration
//! let config = Config::default();
//!
//! // Load authentication
//! let auth = AuthConfig::load(&config)?;
//!
//! // Create client
//! let client = RobloxCloudClient::new(auth.api_key)?;
//!
//! // Create orchestrator
//! let orchestrator = SyncOrchestrator::new(client, config);
//!
//! // Upload translations
//! let stats = orchestrator.upload("table_id_here", false).await?;
//! println!("Uploaded {} entries", stats.entries_uploaded);
//! # Ok(())
//! # }
//! ```

pub mod auth;
pub mod client;
pub mod merge;
pub mod rate_limit;
pub mod sync;
pub mod types;

// Re-export commonly used types
pub use auth::AuthConfig;
pub use client::RobloxCloudClient;
pub use merge::MergeStrategy;
pub use sync::SyncOrchestrator;
