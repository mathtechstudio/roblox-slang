// Roblox Cloud API integration module
//
// This module provides functionality for syncing translations with Roblox Cloud
// Localization Tables via the Open Cloud API.

pub mod auth;
pub mod client;
pub mod merge;
pub mod rate_limit;
pub mod sync;
pub mod types;

// Re-export commonly used types
pub use auth::AuthConfig;
pub use client::RobloxCloudClient;
pub use merge::{Conflict, Diff, MergeEngine, MergeResult, MergeStrategy};
pub use rate_limit::RateLimiter;
pub use sync::SyncOrchestrator;
pub use types::{
    CloudConfig, CloudSyncError, DownloadStats, LocalizationEntry, SyncStats, UploadStats,
};
