/// Roblox supported locales
/// Based on: <https://create.roblox.com/docs/production/localization/language-codes>
/// Roblox supported locale information
#[derive(Debug, Clone, PartialEq)]
pub struct LocaleInfo {
    pub code: &'static str,
    pub name: &'static str,
    pub native_name: &'static str,
}

/// Get all Roblox supported locales
pub fn get_roblox_locales() -> Vec<LocaleInfo> {
    vec![
        LocaleInfo {
            code: "en",
            name: "English",
            native_name: "English",
        },
        LocaleInfo {
            code: "es",
            name: "Spanish",
            native_name: "Español",
        },
        LocaleInfo {
            code: "fr",
            name: "French",
            native_name: "Français",
        },
        LocaleInfo {
            code: "de",
            name: "German",
            native_name: "Deutsch",
        },
        LocaleInfo {
            code: "pt",
            name: "Portuguese",
            native_name: "Português",
        },
        LocaleInfo {
            code: "id",
            name: "Indonesian",
            native_name: "Bahasa Indonesia",
        },
        LocaleInfo {
            code: "it",
            name: "Italian",
            native_name: "Italiano",
        },
        LocaleInfo {
            code: "ja",
            name: "Japanese",
            native_name: "日本語",
        },
        LocaleInfo {
            code: "ko",
            name: "Korean",
            native_name: "한국어",
        },
        LocaleInfo {
            code: "ru",
            name: "Russian",
            native_name: "Русский",
        },
        LocaleInfo {
            code: "th",
            name: "Thai",
            native_name: "ไทย",
        },
        LocaleInfo {
            code: "tr",
            name: "Turkish",
            native_name: "Türkçe",
        },
        LocaleInfo {
            code: "vi",
            name: "Vietnamese",
            native_name: "Tiếng Việt",
        },
        LocaleInfo {
            code: "pl",
            name: "Polish",
            native_name: "Polski",
        },
        LocaleInfo {
            code: "zh-cn",
            name: "Chinese (Simplified)",
            native_name: "简体中文",
        },
        LocaleInfo {
            code: "zh-tw",
            name: "Chinese (Traditional)",
            native_name: "繁體中文",
        },
        LocaleInfo {
            code: "uk",
            name: "Ukrainian",
            native_name: "Українська",
        },
    ]
}

/// Check if a locale is supported by Roblox
pub fn is_roblox_locale(code: &str) -> bool {
    get_roblox_locales()
        .iter()
        .any(|locale| locale.code == code)
}

/// Get all supported locale codes
pub fn get_supported_locale_codes() -> Vec<&'static str> {
    get_roblox_locales()
        .iter()
        .map(|locale| locale.code)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_roblox_locales() {
        let locales = get_roblox_locales();
        assert_eq!(locales.len(), 17);
    }

    #[test]
    fn test_is_roblox_locale() {
        assert!(is_roblox_locale("en"));
        assert!(is_roblox_locale("id"));
        assert!(is_roblox_locale("zh-cn"));
        assert!(!is_roblox_locale("xx"));
    }

    #[test]
    fn test_get_supported_locale_codes() {
        let codes = get_supported_locale_codes();
        assert_eq!(codes.len(), 17);
        assert!(codes.contains(&"en"));
        assert!(codes.contains(&"id"));
    }
}

/// Get country code to locale mapping
/// Based on Roblox's GetCountryRegionForPlayerAsync() return values
pub fn get_country_locale_map() -> Vec<(&'static str, &'static str)> {
    vec![
        // English-speaking countries
        ("US", "en"),
        ("GB", "en"),
        ("CA", "en"),
        ("AU", "en"),
        ("NZ", "en"),
        ("IE", "en"),
        ("ZA", "en"),
        ("SG", "en"),
        ("PH", "en"),
        // Spanish-speaking countries
        ("ES", "es"),
        ("MX", "es"),
        ("AR", "es"),
        ("CO", "es"),
        ("CL", "es"),
        ("PE", "es"),
        ("VE", "es"),
        ("EC", "es"),
        ("GT", "es"),
        ("CU", "es"),
        ("BO", "es"),
        ("DO", "es"),
        ("HN", "es"),
        ("PY", "es"),
        ("SV", "es"),
        ("NI", "es"),
        ("CR", "es"),
        ("PA", "es"),
        ("UY", "es"),
        // French-speaking countries
        ("FR", "fr"),
        ("BE", "fr"),
        ("CH", "fr"),
        ("LU", "fr"),
        ("MC", "fr"),
        // German-speaking countries
        ("DE", "de"),
        ("AT", "de"),
        ("LI", "de"),
        // Portuguese-speaking countries
        ("PT", "pt"),
        ("BR", "pt"),
        ("AO", "pt"),
        ("MZ", "pt"),
        // Indonesian
        ("ID", "id"),
        // Italian
        ("IT", "it"),
        ("SM", "it"),
        ("VA", "it"),
        // Japanese
        ("JP", "ja"),
        // Korean
        ("KR", "ko"),
        // Russian
        ("RU", "ru"),
        ("BY", "ru"),
        ("KZ", "ru"),
        // Thai
        ("TH", "th"),
        // Turkish
        ("TR", "tr"),
        // Vietnamese
        ("VN", "vi"),
        // Polish
        ("PL", "pl"),
        // Chinese (Simplified)
        ("CN", "zh-cn"),
        ("SG", "zh-cn"),
        // Chinese (Traditional)
        ("TW", "zh-tw"),
        ("HK", "zh-tw"),
        ("MO", "zh-tw"),
        // Ukrainian
        ("UA", "uk"),
    ]
}

#[cfg(test)]
mod tests_locale_detection {
    use super::*;

    #[test]
    fn test_get_locale_for_country() {
        let map = get_country_locale_map();

        // Test some common mappings
        assert!(map
            .iter()
            .any(|(code, locale)| *code == "US" && *locale == "en"));
        assert!(map
            .iter()
            .any(|(code, locale)| *code == "ID" && *locale == "id"));
        assert!(map
            .iter()
            .any(|(code, locale)| *code == "ES" && *locale == "es"));
        assert!(map
            .iter()
            .any(|(code, locale)| *code == "JP" && *locale == "ja"));
        assert!(map
            .iter()
            .any(|(code, locale)| *code == "CN" && *locale == "zh-cn"));
        assert!(map
            .iter()
            .any(|(code, locale)| *code == "TW" && *locale == "zh-tw"));
    }

    #[test]
    fn test_country_locale_map_coverage() {
        let map = get_country_locale_map();

        // Should have mappings for all major countries
        assert!(map.len() > 50);

        // Check all Roblox locales are covered
        let locales: Vec<&str> = map.iter().map(|(_, locale)| *locale).collect();
        assert!(locales.contains(&"en"));
        assert!(locales.contains(&"es"));
        assert!(locales.contains(&"fr"));
        assert!(locales.contains(&"de"));
        assert!(locales.contains(&"pt"));
        assert!(locales.contains(&"id"));
        assert!(locales.contains(&"it"));
        assert!(locales.contains(&"ja"));
        assert!(locales.contains(&"ko"));
        assert!(locales.contains(&"ru"));
        assert!(locales.contains(&"th"));
        assert!(locales.contains(&"tr"));
        assert!(locales.contains(&"vi"));
        assert!(locales.contains(&"pl"));
        assert!(locales.contains(&"zh-cn"));
        assert!(locales.contains(&"zh-tw"));
        assert!(locales.contains(&"uk"));
    }
}
