//! HTTP client for Roblox Cloud API
//!
//! Provides low-level HTTP operations for interacting with Roblox Cloud
//! Localization Tables API.

use crate::roblox::types::{CloudSyncError, LocalizationEntry};
use anyhow::{Context, Result};
use reqwest::header::CONTENT_TYPE;
use serde::Deserialize;
use std::time::Duration;

/// HTTP client for Roblox Cloud Localization Tables API
///
/// Handles authentication, request formation, and error handling for
/// all API operations.
///
/// # Example
///
/// ```no_run
/// use roblox_slang::roblox::RobloxCloudClient;
///
/// # async fn example() -> anyhow::Result<()> {
/// let client = RobloxCloudClient::new("api_key".to_string())?;
/// let entries = client.get_table_entries("table_id").await?;
/// println!("Fetched {} entries", entries.len());
/// # Ok(())
/// # }
/// ```
pub struct RobloxCloudClient {
    client: reqwest::Client,
    api_key: String,
    base_url: String,
}

/// Response from update table entries API
#[derive(Debug, Deserialize)]
pub struct UpdateResponse {
    pub success: bool,
}

/// Table metadata response
#[derive(Debug, Deserialize)]
pub struct TableMetadata {
    pub id: String,
    pub name: Option<String>,
}

impl RobloxCloudClient {
    /// Create a new client with API key
    ///
    /// Initializes an HTTP client with:
    /// - 30 second timeout
    /// - User agent: "roblox-slang/1.0.0"
    /// - Base URL: <https://apis.roblox.com>
    ///
    /// # Arguments
    ///
    /// * `api_key` - Roblox Cloud API key
    ///
    /// # Errors
    ///
    /// Returns error if HTTP client cannot be created.
    pub fn new(api_key: String) -> Result<Self> {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(30))
            .user_agent("roblox-slang/1.0.0")
            .build()
            .context("Failed to create HTTP client")?;

        Ok(Self {
            client,
            api_key,
            base_url: "https://apis.roblox.com".to_string(),
        })
    }

    /// Get localization table entries
    ///
    /// Fetches all translation entries from the specified localization table.
    ///
    /// # Arguments
    ///
    /// * `table_id` - Roblox localization table ID
    ///
    /// # Returns
    ///
    /// Vector of localization entries containing translations for all locales.
    ///
    /// # Errors
    ///
    /// Returns error if:
    /// - API key is invalid (401)
    /// - Insufficient permissions (403)
    /// - Rate limit exceeded (429)
    /// - Server error (5xx)
    /// - Network error
    pub async fn get_table_entries(&self, table_id: &str) -> Result<Vec<LocalizationEntry>> {
        let url = format!(
            "{}/localization-table/v1/tables/{}/entries",
            self.base_url, table_id
        );

        let response = self
            .client
            .get(&url)
            .header("x-api-key", &self.api_key)
            .send()
            .await
            .context("Failed to send GET request")?;

        // Handle HTTP errors
        if !response.status().is_success() {
            return self.handle_error_response(response).await;
        }

        let entries: Vec<LocalizationEntry> = response
            .json()
            .await
            .context("Failed to parse response JSON")?;

        Ok(entries)
    }

    /// Update localization table entries
    pub async fn update_table_entries(
        &self,
        table_id: &str,
        entries: &[LocalizationEntry],
    ) -> Result<UpdateResponse> {
        let url = format!(
            "{}/localization-table/v1/tables/{}",
            self.base_url, table_id
        );

        let response = self
            .client
            .patch(&url)
            .header("x-api-key", &self.api_key)
            .header(CONTENT_TYPE, "application/json")
            .json(entries)
            .send()
            .await
            .context("Failed to send PATCH request")?;

        // Handle HTTP errors
        if !response.status().is_success() {
            return self.handle_error_response(response).await;
        }

        let update_response: UpdateResponse = response
            .json()
            .await
            .context("Failed to parse response JSON")?;

        Ok(update_response)
    }

    /// Get table metadata
    pub async fn get_table_metadata(&self, table_id: &str) -> Result<TableMetadata> {
        let url = format!(
            "{}/localization-table/v1/tables/{}",
            self.base_url, table_id
        );

        let response = self
            .client
            .get(&url)
            .header("x-api-key", &self.api_key)
            .send()
            .await
            .context("Failed to send GET request")?;

        // Handle HTTP errors
        if !response.status().is_success() {
            return self.handle_error_response(response).await;
        }

        let metadata: TableMetadata = response
            .json()
            .await
            .context("Failed to parse response JSON")?;

        Ok(metadata)
    }

    /// Handle error responses from API
    async fn handle_error_response<T>(&self, response: reqwest::Response) -> Result<T> {
        let status = response.status();
        let status_code = status.as_u16();

        // Try to get error message from response body
        let error_body = response
            .text()
            .await
            .unwrap_or_else(|_| "Unknown error".to_string());

        match status_code {
            401 => Err(CloudSyncError::AuthenticationError(format!(
                "Invalid or expired API key. Please check your credentials.\n\
                 \n\
                 Response: {}",
                error_body
            ))
            .into()),
            403 => Err(CloudSyncError::AuthenticationError(format!(
                "Insufficient permissions for this operation.\n\
                 \n\
                 Make sure your API key has access to this table.\n\
                 Response: {}",
                error_body
            ))
            .into()),
            429 => {
                // Extract retry-after header if present
                let retry_after = 1; // Default to 1 second
                Err(CloudSyncError::RateLimitError {
                    retry_after,
                    attempt: 1,
                }
                .into())
            }
            500..=599 => Err(CloudSyncError::ServerError {
                status: status_code,
                message: format!(
                    "Roblox server error. Please try again later.\n\
                     \n\
                     Response: {}",
                    error_body
                ),
            }
            .into()),
            _ => Err(CloudSyncError::ApiError(format!(
                "API request failed with status {}: {}",
                status_code, error_body
            ))
            .into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_creation() {
        let client = RobloxCloudClient::new("test_api_key".to_string());
        assert!(client.is_ok());
    }

    #[test]
    fn test_client_has_correct_base_url() {
        let client = RobloxCloudClient::new("test_api_key".to_string()).unwrap();
        assert_eq!(client.base_url, "https://apis.roblox.com");
    }
}
