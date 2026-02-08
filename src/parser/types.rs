use std::collections::HashMap;

/// A single translation entry
#[derive(Debug, Clone, PartialEq)]
pub struct Translation {
    /// Dot-separated key path (e.g., "ui.buttons.buy")
    pub key: String,

    /// Translated text
    pub value: String,

    /// Locale identifier (e.g., "en", "id", "es")
    pub locale: String,

    /// Optional context for disambiguation
    pub context: Option<String>,
}

/// Map of translation keys to values
#[allow(dead_code)]
pub type TranslationMap = HashMap<String, String>;
