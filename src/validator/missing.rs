use crate::parser::Translation;
use std::collections::{HashMap, HashSet};

/// Detect missing keys in non-base locales
pub fn detect_missing_keys(
    translations: &[Translation],
    base_locale: &str,
    supported_locales: &[String],
) -> HashMap<String, Vec<String>> {
    // Get all keys from base locale
    let base_keys: HashSet<String> = translations
        .iter()
        .filter(|t| t.locale == base_locale)
        .map(|t| t.key.clone())
        .collect();

    // Check each non-base locale for missing keys
    let mut missing_by_locale: HashMap<String, Vec<String>> = HashMap::new();

    for locale in supported_locales {
        if locale == base_locale {
            continue;
        }

        // Get keys for this locale
        let locale_keys: HashSet<String> = translations
            .iter()
            .filter(|t| t.locale == locale.as_str())
            .map(|t| t.key.clone())
            .collect();

        // Find missing keys
        let missing: Vec<String> = base_keys.difference(&locale_keys).cloned().collect();

        if !missing.is_empty() {
            missing_by_locale.insert(locale.clone(), missing);
        }
    }

    missing_by_locale
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_missing_keys() {
        let translations = vec![
            Translation {
                key: "ui.button".to_string(),
                value: "Buy".to_string(),
                locale: "en".to_string(),
                context: None,
            },
            Translation {
                key: "ui.label".to_string(),
                value: "Welcome".to_string(),
                locale: "en".to_string(),
                context: None,
            },
            Translation {
                key: "ui.button".to_string(),
                value: "Beli".to_string(),
                locale: "id".to_string(),
                context: None,
            },
            // ui.label missing in id
        ];

        let supported_locales = vec!["en".to_string(), "id".to_string()];
        let missing = detect_missing_keys(&translations, "en", &supported_locales);

        assert_eq!(missing.len(), 1);
        assert!(missing.contains_key("id"));
        assert_eq!(missing["id"], vec!["ui.label"]);
    }

    #[test]
    fn test_no_missing_keys() {
        let translations = vec![
            Translation {
                key: "ui.button".to_string(),
                value: "Buy".to_string(),
                locale: "en".to_string(),
                context: None,
            },
            Translation {
                key: "ui.button".to_string(),
                value: "Beli".to_string(),
                locale: "id".to_string(),
                context: None,
            },
        ];

        let supported_locales = vec!["en".to_string(), "id".to_string()];
        let missing = detect_missing_keys(&translations, "en", &supported_locales);

        assert_eq!(missing.len(), 0);
    }
}
