// Type definitions for Roblox Cloud API and cloud sync operations

use serde::{Deserialize, Serialize};
use std::time::Duration;
use thiserror::Error;

/// Error types for cloud sync operations
#[derive(Debug, Error)]
pub enum CloudSyncError {
    #[error("Authentication failed: {0}")]
    AuthenticationError(String),

    #[error("Rate limit exceeded. Retrying in {retry_after}s (attempt {attempt}/3)")]
    RateLimitError { retry_after: u64, attempt: u32 },

    #[error("Server error: {status} - {message}")]
    ServerError { status: u16, message: String },

    #[error("Network error: {0}")]
    #[allow(dead_code)]
    NetworkError(String),

    #[error("Validation failed in {file}:{line} - {reason}")]
    #[allow(dead_code)]
    ValidationError {
        file: String,
        line: usize,
        reason: String,
    },

    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("API error: {0}")]
    ApiError(String),
}

/// Response from get table entries API
#[derive(Debug, Deserialize)]
pub struct GetTableEntriesResponse {
    #[serde(alias = "entries", alias = "data")]
    pub entries: Vec<LocalizationEntry>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "nextPageCursor")]
    #[allow(dead_code)]
    pub next_cursor: Option<String>,
}

/// Request body for updating table entries
#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub struct UpdateTableRequest {
    /// List of entries to update
    pub entries: Vec<LocalizationEntry>,
}

/// Response from list tables API
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct ListTablesResponse {
    pub data: Vec<TableInfo>,
}

/// Table information
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct TableInfo {
    pub id: String,
    pub name: Option<String>,
    #[serde(rename = "ownerType")]
    pub owner_type: Option<String>,
    #[serde(rename = "ownerId")]
    pub owner_id: Option<i64>,
    #[serde(rename = "assetId")]
    pub asset_id: Option<i64>,
}

/// Localization entry in Roblox Cloud API format
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocalizationEntry {
    /// Translation key identifier
    pub identifier: Identifier,

    /// Entry metadata (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<EntryMetadata>,

    /// Translations for each locale
    pub translations: Vec<Translation>,
}

/// Translation key identifier
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Identifier {
    /// The key path (e.g., "ui.buttons.buy")
    pub key: String,

    /// Context for disambiguation (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<String>,

    /// Source text (for ATC compatibility)
    pub source: String,
}

/// Entry metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntryMetadata {
    /// Example usage context
    #[serde(skip_serializing_if = "Option::is_none")]
    pub example: Option<String>,

    /// Entry type (manual, automatic, etc.)
    #[serde(rename = "entryType", skip_serializing_if = "Option::is_none")]
    pub entry_type: Option<String>,
}

/// Translation for a specific locale
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Translation {
    /// Locale code (e.g., "en", "es", "pt-BR")
    pub locale: String,

    /// Translated text
    #[serde(rename = "translationText")]
    pub translation_text: String,
}

/// Cloud configuration section
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct CloudConfig {
    /// Default table ID for cloud operations
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_id: Option<String>,

    /// Default game ID for cloud operations
    #[serde(skip_serializing_if = "Option::is_none")]
    pub game_id: Option<String>,

    /// API key (prefer environment variable)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_key: Option<String>,

    /// Default merge strategy for sync operations
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strategy: Option<String>,
}

/// Statistics for upload operation
#[derive(Debug)]
pub struct UploadStats {
    pub entries_uploaded: usize,
    pub locales_processed: usize,
    pub duration: Duration,
}

/// Statistics for download operation
#[derive(Debug)]
pub struct DownloadStats {
    pub entries_downloaded: usize,
    pub locales_created: usize,
    pub locales_updated: usize,
    pub duration: Duration,
}

/// Statistics for sync operation
#[derive(Debug)]
pub struct SyncStats {
    pub entries_added: usize,
    pub entries_updated: usize,
    pub entries_deleted: usize,
    pub conflicts_skipped: usize,
    pub duration: Duration,
}
