use super::client::RobloxCloudClient;
use super::merge::{MergeEngine, MergeStrategy};
use super::types::{DownloadStats, LocalizationEntry, SyncStats, UploadStats};
use crate::config::Config;
use crate::parser::{self, Translation};
use anyhow::{Context, Result};
use std::collections::HashMap;
use std::path::Path;
use std::time::Instant;

/// Orchestrates synchronization between local translation files and Roblox Cloud
pub struct SyncOrchestrator {
    /// HTTP client for Roblox Cloud API
    client: RobloxCloudClient,
    /// Configuration
    config: Config,
}

impl SyncOrchestrator {
    /// Create a new sync orchestrator
    pub fn new(client: RobloxCloudClient, config: Config) -> Self {
        Self { client, config }
    }

    /// Upload local translations to Roblox Cloud
    pub async fn upload(&self, table_id: &str, dry_run: bool) -> Result<UploadStats> {
        let start = Instant::now();

        // Read all local translation files
        let translations = self.read_local_translations()?;

        let entries_count = translations.len();
        let locales: std::collections::HashSet<_> =
            translations.iter().map(|t| t.locale.clone()).collect();
        let locales_count = locales.len();

        if !dry_run {
            // Convert to LocalizationEntry format
            let entries = self.translations_to_entries(&translations);

            // Upload to cloud
            self.client
                .update_table_entries(table_id, &entries)
                .await
                .context("Failed to upload translations")?;
        }

        let duration = start.elapsed();

        Ok(UploadStats {
            entries_uploaded: entries_count,
            locales_processed: locales_count,
            duration,
        })
    }

    /// Download translations from Roblox Cloud
    pub async fn download(&self, table_id: &str, dry_run: bool) -> Result<DownloadStats> {
        let start = Instant::now();

        // Download from cloud
        let entries = self
            .client
            .get_table_entries(table_id)
            .await
            .context("Failed to download translations")?;

        let entries_count = entries.len();

        // Convert to Translation format
        let translations = self.entries_to_translations(&entries);

        // Group by locale
        let mut by_locale: HashMap<String, Vec<Translation>> = HashMap::new();
        for translation in translations {
            by_locale
                .entry(translation.locale.clone())
                .or_insert_with(Vec::new)
                .push(translation);
        }

        let mut locales_created = 0;
        let mut locales_updated = 0;

        if !dry_run {
            // Write translation files for each locale
            for (locale, locale_translations) in &by_locale {
                let file_path = Path::new(&self.config.input_directory)
                    .join(format!("{}.json", locale));

                let existed = file_path.exists();

                self.write_translation_file(&file_path, locale_translations)?;

                if existed {
                    locales_updated += 1;
                } else {
                    locales_created += 1;
                }
            }
        } else {
            // In dry-run, count what would be created/updated
            for locale in by_locale.keys() {
                let file_path = Path::new(&self.config.input_directory)
                    .join(format!("{}.json", locale));

                if file_path.exists() {
                    locales_updated += 1;
                } else {
                    locales_created += 1;
                }
            }
        }

        let duration = start.elapsed();

        Ok(DownloadStats {
            entries_downloaded: entries_count,
            locales_created,
            locales_updated,
            duration,
        })
    }

    /// Synchronize translations between local and cloud with merge strategy
    pub async fn sync(
        &self,
        table_id: &str,
        strategy: MergeStrategy,
        dry_run: bool,
    ) -> Result<SyncStats> {
        let start = Instant::now();

        // Get local translations
        let local_translations = self.read_local_translations()?;
        let local_map = self.translations_to_map(&local_translations);

        // Get cloud translations
        let cloud_entries = self
            .client
            .get_table_entries(table_id)
            .await
            .context("Failed to download translations")?;
        let cloud_translations = self.entries_to_translations(&cloud_entries);
        let cloud_map = self.translations_to_map(&cloud_translations);

        // Compute diff
        let diff = MergeEngine::compute_diff(&local_map, &cloud_map);

        // Apply strategy
        let merge_result = MergeEngine::apply_strategy(&diff, strategy, &local_map);

        let mut entries_added = 0;
        let mut entries_updated = 0;
        let entries_deleted = 0;

        if !dry_run {
            // Upload to_upload entries
            if !merge_result.to_upload.is_empty() {
                let upload_translations: Vec<Translation> = merge_result
                    .to_upload
                    .iter()
                    .map(|(key, locale, value)| Translation {
                        key: key.clone(),
                        locale: locale.clone(),
                        value: value.clone(),
                        context: None,
                    })
                    .collect();

                let entries = self.translations_to_entries(&upload_translations);
                self.client
                    .update_table_entries(table_id, &entries)
                    .await
                    .context("Failed to upload translations")?;

                entries_added = merge_result.to_upload.len();
            }

            // Download to_download entries
            if !merge_result.to_download.is_empty() {
                let download_translations: Vec<Translation> = merge_result
                    .to_download
                    .iter()
                    .map(|(key, locale, value)| Translation {
                        key: key.clone(),
                        locale: locale.clone(),
                        value: value.clone(),
                        context: None,
                    })
                    .collect();

                // Group by locale and write files
                let mut by_locale: HashMap<String, Vec<Translation>> = HashMap::new();
                for translation in download_translations {
                    by_locale
                        .entry(translation.locale.clone())
                        .or_insert_with(Vec::new)
                        .push(translation);
                }

                for (locale, locale_translations) in &by_locale {
                    let file_path = Path::new(&self.config.input_directory)
                        .join(format!("{}.json", locale));

                    self.write_translation_file(&file_path, locale_translations)?;
                }

                entries_updated = merge_result.to_download.len();
            }

            // Write conflicts file if any
            if !merge_result.conflicts.is_empty() {
                self.write_conflicts_file(&merge_result.conflicts)?;
            }
        } else {
            // In dry-run, just count
            entries_added = merge_result.to_upload.len();
            entries_updated = merge_result.to_download.len();
        }

        let duration = start.elapsed();

        Ok(SyncStats {
            entries_added,
            entries_updated,
            entries_deleted,
            conflicts_skipped: merge_result.conflicts.len(),
            duration,
        })
    }

    // Helper methods

    /// Read all local translation files
    fn read_local_translations(&self) -> Result<Vec<Translation>> {
        let mut all_translations = Vec::new();

        for locale in &self.config.supported_locales {
            let file_path = Path::new(&self.config.input_directory)
                .join(format!("{}.json", locale));

            if !file_path.exists() {
                continue;
            }

            let translations = parser::parse_json_file(&file_path, locale)
                .context(format!("Failed to parse {}", file_path.display()))?;

            all_translations.extend(translations);
        }

        Ok(all_translations)
    }

    /// Convert translations to LocalizationEntry format
    fn translations_to_entries(&self, translations: &[Translation]) -> Vec<LocalizationEntry> {
        use super::types::{EntryMetadata, Identifier, Translation as ApiTranslation};

        // Group by key
        let mut by_key: HashMap<String, Vec<&Translation>> = HashMap::new();
        for translation in translations {
            by_key
                .entry(translation.key.clone())
                .or_insert_with(Vec::new)
                .push(translation);
        }

        // Convert to LocalizationEntry
        by_key
            .into_iter()
            .map(|(key, translations)| {
                let source = translations
                    .iter()
                    .find(|t| t.locale == self.config.base_locale)
                    .map(|t| t.value.clone())
                    .unwrap_or_else(|| translations[0].value.clone());

                LocalizationEntry {
                    identifier: Identifier {
                        key: key.clone(),
                        context: translations[0].context.clone(),
                        source,
                    },
                    metadata: EntryMetadata {
                        example: None,
                        entry_type: Some("manual".to_string()),
                    },
                    translations: translations
                        .iter()
                        .map(|t| ApiTranslation {
                            locale: t.locale.clone(),
                            translation_text: t.value.clone(),
                        })
                        .collect(),
                }
            })
            .collect()
    }

    /// Convert LocalizationEntry to Translation format
    fn entries_to_translations(&self, entries: &[LocalizationEntry]) -> Vec<Translation> {
        let mut translations = Vec::new();

        for entry in entries {
            for api_translation in &entry.translations {
                translations.push(Translation {
                    key: entry.identifier.key.clone(),
                    locale: api_translation.locale.clone(),
                    value: api_translation.translation_text.clone(),
                    context: entry.identifier.context.clone(),
                });
            }
        }

        translations
    }

    /// Convert translations to HashMap for merge engine
    fn translations_to_map(
        &self,
        translations: &[Translation],
    ) -> HashMap<(String, String), String> {
        translations
            .iter()
            .map(|t| ((t.key.clone(), t.locale.clone()), t.value.clone()))
            .collect()
    }

    /// Write translation file in nested JSON format
    fn write_translation_file(&self, path: &Path, translations: &[Translation]) -> Result<()> {
        use crate::utils::flatten::unflatten_translations;
        use std::fs;

        // Convert flat translations to nested structure
        let nested = unflatten_translations(translations);

        // Write to file
        let json = serde_json::to_string_pretty(&nested)
            .context("Failed to serialize translations")?;

        fs::write(path, json).context(format!("Failed to write {}", path.display()))?;

        Ok(())
    }

    /// Write conflicts to YAML file
    fn write_conflicts_file(&self, conflicts: &[super::merge::Conflict]) -> Result<()> {
        use std::fs;

        // Group conflicts by locale
        let mut by_locale: HashMap<String, Vec<&super::merge::Conflict>> = HashMap::new();
        for conflict in conflicts {
            by_locale
                .entry(conflict.locale.clone())
                .or_insert_with(Vec::new)
                .push(conflict);
        }

        // Create YAML structure
        let mut yaml_content = String::from("# Translation Conflicts\n");
        yaml_content.push_str("# Resolve these conflicts manually\n\n");

        for (locale, locale_conflicts) in by_locale {
            yaml_content.push_str(&format!("{}:\n", locale));
            for conflict in locale_conflicts {
                yaml_content.push_str(&format!("  {}:\n", conflict.key));
                yaml_content.push_str(&format!("    local: \"{}\"\n", conflict.local_value));
                yaml_content.push_str(&format!("    cloud: \"{}\"\n", conflict.cloud_value));
            }
            yaml_content.push('\n');
        }

        let conflicts_path = Path::new(&self.config.output_directory).join("conflicts.yaml");

        fs::write(&conflicts_path, yaml_content)
            .context(format!("Failed to write {}", conflicts_path.display()))?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sync_orchestrator_new() {
        let client = RobloxCloudClient::new("test_key".to_string()).unwrap();
        let config = Config::default();
        let _orchestrator = SyncOrchestrator::new(client, config);
    }

    #[test]
    fn test_translations_to_map() {
        let client = RobloxCloudClient::new("test_key".to_string()).unwrap();
        let config = Config::default();
        let orchestrator = SyncOrchestrator::new(client, config);

        let translations = vec![
            Translation {
                key: "ui.button".to_string(),
                locale: "en".to_string(),
                value: "Buy".to_string(),
                context: None,
            },
            Translation {
                key: "ui.button".to_string(),
                locale: "id".to_string(),
                value: "Beli".to_string(),
                context: None,
            },
        ];

        let map = orchestrator.translations_to_map(&translations);

        assert_eq!(map.len(), 2);
        assert_eq!(
            map.get(&("ui.button".to_string(), "en".to_string())),
            Some(&"Buy".to_string())
        );
        assert_eq!(
            map.get(&("ui.button".to_string(), "id".to_string())),
            Some(&"Beli".to_string())
        );
    }
}
