/// CLDR Plural Rules Implementation
/// Based on Unicode CLDR: <https://cldr.unicode.org/index/cldr-spec/plural-rules>
/// Detect if a translation key is a plural form
pub fn is_plural_key(key: &str) -> bool {
    key.ends_with("(zero)")
        || key.ends_with("(one)")
        || key.ends_with("(two)")
        || key.ends_with("(few)")
        || key.ends_with("(many)")
        || key.ends_with("(other)")
}

/// Extract base key from plural key
/// Example: "items(one)" -> "items"
pub fn extract_base_key(key: &str) -> String {
    if let Some(pos) = key.rfind('(') {
        key[..pos].to_string()
    } else {
        key.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_plural_key() {
        assert!(is_plural_key("items(one)"));
        assert!(is_plural_key("items(other)"));
        assert!(!is_plural_key("items"));
    }

    #[test]
    fn test_extract_base_key() {
        assert_eq!(extract_base_key("items(one)"), "items");
        assert_eq!(
            extract_base_key("ui.messages.count(other)"),
            "ui.messages.count"
        );
    }
}
