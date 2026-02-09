use std::collections::HashMap;

/// Merge strategy for resolving conflicts between local and cloud translations
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MergeStrategy {
    /// Overwrite cloud with all local translations
    Overwrite,
    /// Merge local and cloud, preferring cloud for conflicts
    Merge,
    /// Skip conflicts, only sync non-conflicting entries
    SkipConflicts,
}

/// Difference between local and cloud translations
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Diff {
    /// Entries only in local (new translations to upload)
    pub added_local: Vec<(String, String, String)>, // (key, locale, value)
    /// Entries only in cloud (new translations to download)
    pub added_cloud: Vec<(String, String, String)>, // (key, locale, value)
    /// Entries in both with different values (conflicts)
    pub modified_both: Vec<(String, String, String, String)>, // (key, locale, local_value, cloud_value)
    /// Entries deleted from local (present in cloud but not local)
    pub deleted_local: Vec<(String, String)>, // (key, locale)
}

/// Conflict between local and cloud translation
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Conflict {
    /// Translation key
    pub key: String,
    /// Locale code
    pub locale: String,
    /// Local translation value
    pub local_value: String,
    /// Cloud translation value
    pub cloud_value: String,
}

/// Result of applying merge strategy
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MergeResult {
    /// Entries to upload to cloud
    pub to_upload: Vec<(String, String, String)>, // (key, locale, value)
    /// Entries to download from cloud
    pub to_download: Vec<(String, String, String)>, // (key, locale, value)
    /// Conflicts that were skipped
    pub conflicts: Vec<Conflict>,
}

/// Merge engine for computing diffs and applying merge strategies
pub struct MergeEngine;

impl MergeEngine {
    /// Compute difference between local and cloud translations
    ///
    /// # Arguments
    ///
    /// * `local` - Local translations as HashMap<(key, locale), value>
    /// * `cloud` - Cloud translations as HashMap<(key, locale), value>
    ///
    /// # Returns
    ///
    /// Diff struct with added_local, added_cloud, modified_both, deleted_local
    ///
    /// # Examples
    ///
    /// ```
    /// use std::collections::HashMap;
    /// use roblox_slang::roblox::merge::MergeEngine;
    ///
    /// let mut local = HashMap::new();
    /// local.insert(("ui.button".to_string(), "en".to_string()), "Buy".to_string());
    ///
    /// let mut cloud = HashMap::new();
    /// cloud.insert(("ui.button".to_string(), "en".to_string()), "Purchase".to_string());
    ///
    /// let diff = MergeEngine::compute_diff(&local, &cloud);
    /// assert_eq!(diff.modified_both.len(), 1);
    /// ```
    pub fn compute_diff(
        local: &HashMap<(String, String), String>,
        cloud: &HashMap<(String, String), String>,
    ) -> Diff {
        let mut added_local = Vec::new();
        let mut added_cloud = Vec::new();
        let mut modified_both = Vec::new();
        let mut deleted_local = Vec::new();

        // Find entries only in local or modified
        for ((key, locale), local_value) in local {
            if let Some(cloud_value) = cloud.get(&(key.clone(), locale.clone())) {
                // Entry exists in both
                if local_value != cloud_value {
                    // Values differ - conflict
                    modified_both.push((
                        key.clone(),
                        locale.clone(),
                        local_value.clone(),
                        cloud_value.clone(),
                    ));
                }
            } else {
                // Entry only in local
                added_local.push((key.clone(), locale.clone(), local_value.clone()));
            }
        }

        // Find entries only in cloud
        for ((key, locale), cloud_value) in cloud {
            if !local.contains_key(&(key.clone(), locale.clone())) {
                added_cloud.push((key.clone(), locale.clone(), cloud_value.clone()));
            }
        }

        // Deleted local = entries in cloud but not in local (same as added_cloud for our purposes)
        // We track this separately for clarity
        for ((key, locale), _) in cloud {
            if !local.contains_key(&(key.clone(), locale.clone())) {
                deleted_local.push((key.clone(), locale.clone()));
            }
        }

        Diff {
            added_local,
            added_cloud,
            modified_both,
            deleted_local,
        }
    }

    /// Apply merge strategy to diff
    ///
    /// # Arguments
    ///
    /// * `diff` - Computed diff between local and cloud
    /// * `strategy` - Merge strategy to apply
    /// * `local` - Local translations (needed for Overwrite strategy)
    ///
    /// # Returns
    ///
    /// MergeResult with to_upload, to_download, and conflicts
    ///
    /// # Examples
    ///
    /// ```
    /// use std::collections::HashMap;
    /// use roblox_slang::roblox::merge::{MergeEngine, MergeStrategy};
    ///
    /// let mut local = HashMap::new();
    /// local.insert(("ui.button".to_string(), "en".to_string()), "Buy".to_string());
    ///
    /// let cloud = HashMap::new();
    ///
    /// let diff = MergeEngine::compute_diff(&local, &cloud);
    /// let result = MergeEngine::apply_strategy(&diff, MergeStrategy::Overwrite, &local);
    ///
    /// assert_eq!(result.to_upload.len(), 1);
    /// assert_eq!(result.to_download.len(), 0);
    /// assert_eq!(result.conflicts.len(), 0);
    /// ```
    pub fn apply_strategy(
        diff: &Diff,
        strategy: MergeStrategy,
        local: &HashMap<(String, String), String>,
    ) -> MergeResult {
        match strategy {
            MergeStrategy::Overwrite => Self::apply_overwrite(local),
            MergeStrategy::Merge => Self::apply_merge(diff),
            MergeStrategy::SkipConflicts => Self::apply_skip_conflicts(diff),
        }
    }

    /// Apply Overwrite strategy: upload all local translations
    fn apply_overwrite(local: &HashMap<(String, String), String>) -> MergeResult {
        let to_upload: Vec<(String, String, String)> = local
            .iter()
            .map(|((key, locale), value)| (key.clone(), locale.clone(), value.clone()))
            .collect();

        MergeResult {
            to_upload,
            to_download: Vec::new(),
            conflicts: Vec::new(),
        }
    }

    /// Apply Merge strategy: merge local and cloud, prefer cloud for conflicts
    fn apply_merge(diff: &Diff) -> MergeResult {
        let to_upload = diff.added_local.clone();
        let mut to_download = diff.added_cloud.clone();

        // For conflicts, prefer cloud (download cloud values)
        for (key, locale, _local_value, cloud_value) in &diff.modified_both {
            to_download.push((key.clone(), locale.clone(), cloud_value.clone()));
        }

        MergeResult {
            to_upload,
            to_download,
            conflicts: Vec::new(),
        }
    }

    /// Apply SkipConflicts strategy: only sync non-conflicting entries
    fn apply_skip_conflicts(diff: &Diff) -> MergeResult {
        let to_upload = diff.added_local.clone();
        let to_download = diff.added_cloud.clone();

        // Convert modified_both to conflicts
        let conflicts: Vec<Conflict> = diff
            .modified_both
            .iter()
            .map(|(key, locale, local_value, cloud_value)| Conflict {
                key: key.clone(),
                locale: locale.clone(),
                local_value: local_value.clone(),
                cloud_value: cloud_value.clone(),
            })
            .collect();

        MergeResult {
            to_upload,
            to_download,
            conflicts,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_local() -> HashMap<(String, String), String> {
        let mut local = HashMap::new();
        local.insert(
            ("ui.button.buy".to_string(), "en".to_string()),
            "Buy".to_string(),
        );
        local.insert(
            ("ui.button.sell".to_string(), "en".to_string()),
            "Sell".to_string(),
        );
        local.insert(
            ("ui.button.buy".to_string(), "id".to_string()),
            "Beli".to_string(),
        );
        local
    }

    fn create_test_cloud() -> HashMap<(String, String), String> {
        let mut cloud = HashMap::new();
        cloud.insert(
            ("ui.button.buy".to_string(), "en".to_string()),
            "Purchase".to_string(), // Conflict
        );
        cloud.insert(
            ("ui.button.cancel".to_string(), "en".to_string()),
            "Cancel".to_string(), // Only in cloud
        );
        cloud
    }

    #[test]
    fn test_compute_diff() {
        let local = create_test_local();
        let cloud = create_test_cloud();

        let diff = MergeEngine::compute_diff(&local, &cloud);

        // added_local: ui.button.sell (en), ui.button.buy (id)
        assert_eq!(diff.added_local.len(), 2);
        assert!(diff.added_local.contains(&(
            "ui.button.sell".to_string(),
            "en".to_string(),
            "Sell".to_string()
        )));
        assert!(diff.added_local.contains(&(
            "ui.button.buy".to_string(),
            "id".to_string(),
            "Beli".to_string()
        )));

        // added_cloud: ui.button.cancel (en)
        assert_eq!(diff.added_cloud.len(), 1);
        assert!(diff.added_cloud.contains(&(
            "ui.button.cancel".to_string(),
            "en".to_string(),
            "Cancel".to_string()
        )));

        // modified_both: ui.button.buy (en) - "Buy" vs "Purchase"
        assert_eq!(diff.modified_both.len(), 1);
        assert_eq!(diff.modified_both[0].0, "ui.button.buy");
        assert_eq!(diff.modified_both[0].1, "en");
        assert_eq!(diff.modified_both[0].2, "Buy");
        assert_eq!(diff.modified_both[0].3, "Purchase");

        // deleted_local: same as added_cloud
        assert_eq!(diff.deleted_local.len(), 1);
    }

    #[test]
    fn test_overwrite_strategy() {
        let local = create_test_local();
        let cloud = create_test_cloud();

        let diff = MergeEngine::compute_diff(&local, &cloud);
        let result = MergeEngine::apply_strategy(&diff, MergeStrategy::Overwrite, &local);

        // Should upload all local entries
        assert_eq!(result.to_upload.len(), 3);
        assert_eq!(result.to_download.len(), 0);
        assert_eq!(result.conflicts.len(), 0);
    }

    #[test]
    fn test_merge_strategy() {
        let local = create_test_local();
        let cloud = create_test_cloud();

        let diff = MergeEngine::compute_diff(&local, &cloud);
        let result = MergeEngine::apply_strategy(&diff, MergeStrategy::Merge, &local);

        // Should upload added_local (2 entries)
        assert_eq!(result.to_upload.len(), 2);

        // Should download added_cloud (1) + conflicts preferring cloud (1) = 2
        assert_eq!(result.to_download.len(), 2);

        // No conflicts in result (resolved by preferring cloud)
        assert_eq!(result.conflicts.len(), 0);
    }

    #[test]
    fn test_skip_conflicts_strategy() {
        let local = create_test_local();
        let cloud = create_test_cloud();

        let diff = MergeEngine::compute_diff(&local, &cloud);
        let result = MergeEngine::apply_strategy(&diff, MergeStrategy::SkipConflicts, &local);

        // Should upload added_local (2 entries)
        assert_eq!(result.to_upload.len(), 2);

        // Should download added_cloud (1 entry)
        assert_eq!(result.to_download.len(), 1);

        // Should have 1 conflict (ui.button.buy en)
        assert_eq!(result.conflicts.len(), 1);
        assert_eq!(result.conflicts[0].key, "ui.button.buy");
        assert_eq!(result.conflicts[0].locale, "en");
        assert_eq!(result.conflicts[0].local_value, "Buy");
        assert_eq!(result.conflicts[0].cloud_value, "Purchase");
    }

    #[test]
    fn test_empty_diff() {
        let local = HashMap::new();
        let cloud = HashMap::new();

        let diff = MergeEngine::compute_diff(&local, &cloud);

        assert_eq!(diff.added_local.len(), 0);
        assert_eq!(diff.added_cloud.len(), 0);
        assert_eq!(diff.modified_both.len(), 0);
        assert_eq!(diff.deleted_local.len(), 0);
    }

    #[test]
    fn test_identical_translations() {
        let mut local = HashMap::new();
        local.insert(
            ("ui.button".to_string(), "en".to_string()),
            "Buy".to_string(),
        );

        let cloud = local.clone();

        let diff = MergeEngine::compute_diff(&local, &cloud);

        // No differences
        assert_eq!(diff.added_local.len(), 0);
        assert_eq!(diff.added_cloud.len(), 0);
        assert_eq!(diff.modified_both.len(), 0);
        assert_eq!(diff.deleted_local.len(), 0);
    }
}
