use super::Config;

impl Default for Config {
    fn default() -> Self {
        Self {
            base_locale: "en".to_string(),
            supported_locales: vec!["en".to_string()],
            input_directory: "translations".to_string(),
            output_directory: "output".to_string(),
            namespace: None,
            overrides: None,
            analytics: None,
            cloud: None,
        }
    }
}
