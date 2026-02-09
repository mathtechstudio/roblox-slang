use crate::parser::Translation;
use crate::utils::plurals;
use anyhow::Result;
use std::collections::HashSet;

/// Generate Luau type definitions (.d.luau)
pub fn generate_type_definitions(
    translations: &[Translation],
    base_locale: &str,
) -> Result<String> {
    let mut code = String::new();

    // Professional header with documentation
    code.push_str("--[[\n");
    code.push_str("    Roblox Slang - Type Definitions\n");
    code.push_str("    \n");
    code.push_str("    This file provides type definitions for Luau LSP autocomplete.\n");
    code.push_str("    DO NOT MODIFY BY HAND - Your changes will be overwritten!\n");
    code.push_str("    \n");
    code.push_str("    Generated from translation files in your project.\n");
    code.push_str("    To update type definitions, edit your JSON/YAML files and run:\n");
    code.push_str("        roblox-slang build\n");
    code.push_str("    \n");
    code.push_str("    Usage:\n");
    code.push_str("    Place this file alongside your Translations.lua module.\n");
    code.push_str("    Your IDE/LSP will automatically provide autocomplete and type checking.\n");
    code.push_str("    \n");
    code.push_str("    Learn more: https://github.com/mathtechstudio/roblox-slang\n");
    code.push_str("--]]\n\n");

    // Get base locale translations
    let base_translations: Vec<_> = translations
        .iter()
        .filter(|t| t.locale == base_locale)
        .collect();

    if base_translations.is_empty() {
        return Ok(code + "export type Translations = {}\n");
    }

    // Build namespace type structure
    code.push_str("export type Translations = {\n");
    code.push_str("    new: (locale: string?) -> TranslationsInstance,\n");
    code.push_str("}\n\n");

    // Build instance type
    code.push_str("export type TranslationsInstance = {\n");

    // Add internal fields
    code.push_str("    _locale: string,\n");
    code.push_str("    _translator: any,\n");
    code.push_str("    _localeChangedCallbacks: {any},\n\n");

    // Add methods
    code.push_str("    setLocale: (self: TranslationsInstance, locale: string) -> (),\n");
    code.push_str("    getLocale: (self: TranslationsInstance) -> string,\n");
    code.push_str("    onLocaleChanged: (self: TranslationsInstance, callback: (newLocale: string, oldLocale: string) -> ()) -> (),\n");
    code.push_str("    getAsset: (self: TranslationsInstance, assetKey: string) -> string,\n\n");

    // Separate plural and non-plural translations
    let mut plural_base_keys: HashSet<String> = HashSet::new();
    let mut regular_translations = Vec::new();

    for translation in &base_translations {
        if plurals::is_plural_key(&translation.key) {
            let base_key = plurals::extract_base_key(&translation.key);
            plural_base_keys.insert(base_key);
        } else {
            regular_translations.push(*translation);
        }
    }

    // Sort for deterministic output
    regular_translations.sort_by(|a, b| a.key.cmp(&b.key));

    // Add flat methods for regular translations
    for translation in &regular_translations {
        let method_name = translation.key.replace(".", "_");
        let params = super::luau::extract_parameters(&translation.value);

        if params.is_empty() {
            code.push_str(&format!(
                "    {}: (self: TranslationsInstance) -> string,\n",
                method_name
            ));
        } else {
            code.push_str(&format!(
                "    {}: (self: TranslationsInstance, params: {{}}) -> string,\n",
                method_name
            ));
        }
    }

    // Add flat methods for plural translations
    let mut plural_keys_sorted: Vec<_> = plural_base_keys.iter().collect();
    plural_keys_sorted.sort();
    
    for base_key in &plural_keys_sorted {
        let method_name = base_key.replace(".", "_");
        code.push_str(&format!(
            "    {}: (self: TranslationsInstance, count: number, params: {{}}?) -> string,\n",
            method_name
        ));
    }

    code.push('\n');

    // Add namespace structure
    let namespaces = build_namespace_tree(&regular_translations, &plural_base_keys);
    generate_namespace_types(
        &mut code,
        &namespaces,
        &regular_translations,
        &plural_base_keys,
    );

    code.push_str("}\n");

    Ok(code)
}

/// Build namespace tree from translations
fn build_namespace_tree(
    translations: &[&Translation],
    plural_base_keys: &HashSet<String>,
) -> HashSet<String> {
    let mut namespaces = HashSet::new();

    // Add namespaces from regular translations
    for translation in translations {
        let parts: Vec<&str> = translation.key.split('.').collect();
        for i in 0..parts.len() - 1 {
            let namespace = parts[0..=i].join(".");
            namespaces.insert(namespace);
        }
    }

    // Add namespaces from plural base keys
    for base_key in plural_base_keys {
        let parts: Vec<&str> = base_key.split('.').collect();
        for i in 0..parts.len() - 1 {
            let namespace = parts[0..=i].join(".");
            namespaces.insert(namespace);
        }
    }

    namespaces
}

/// Generate namespace type definitions
fn generate_namespace_types(
    code: &mut String,
    namespaces: &HashSet<String>,
    translations: &[&Translation],
    plural_base_keys: &HashSet<String>,
) {
    let mut sorted_namespaces: Vec<_> = namespaces.iter().collect();
    sorted_namespaces.sort();

    // Generate namespace fields
    for namespace in &sorted_namespaces {
        let parts: Vec<&str> = namespace.split('.').collect();
        let last_part = parts[parts.len() - 1];

        // Check if this is a top-level namespace
        if parts.len() == 1 {
            code.push_str(&format!("    {}: {{\n", last_part));

            // Add methods for this namespace (direct children only)
            for translation in translations {
                if translation.key.starts_with(&format!("{}.", namespace)) {
                    let key_parts: Vec<&str> = translation.key.split('.').collect();

                    // Only add if this is a direct child
                    if key_parts.len() == 2 {
                        let method = key_parts[1];
                        let params = super::luau::extract_parameters(&translation.value);

                        if params.is_empty() {
                            code.push_str(&format!(
                                "        {}: (self: TranslationsInstance) -> string,\n",
                                method
                            ));
                        } else {
                            code.push_str(&format!(
                                "        {}: (self: TranslationsInstance, params: {{}}) -> string,\n",
                                method
                            ));
                        }
                    }
                }
            }

            // Add nested namespaces
            for nested in sorted_namespaces.iter() {
                if nested.starts_with(&format!("{}.", namespace)) {
                    let nested_parts: Vec<&str> = nested.split('.').collect();
                    if nested_parts.len() == 2 {
                        let nested_name = nested_parts[1];
                        code.push_str(&format!("        {}: {{\n", nested_name));

                        // Add methods for nested namespace
                        for translation in translations {
                            if translation.key.starts_with(&format!("{}.", nested)) {
                                let key_parts: Vec<&str> = translation.key.split('.').collect();

                                if key_parts.len() == 3 {
                                    let method = key_parts[2];
                                    let params =
                                        super::luau::extract_parameters(&translation.value);

                                    if params.is_empty() {
                                        code.push_str(&format!(
                                            "            {}: (self: TranslationsInstance) -> string,\n",
                                            method
                                        ));
                                    } else {
                                        code.push_str(&format!(
                                            "            {}: (self: TranslationsInstance, params: {{}}) -> string,\n",
                                            method
                                        ));
                                    }
                                }
                            }
                        }

                        // Add plural methods for nested namespace
                        for base_key in plural_base_keys {
                            if base_key.starts_with(&format!("{}.", nested)) {
                                let key_parts: Vec<&str> = base_key.split('.').collect();

                                if key_parts.len() == 3 {
                                    let method = key_parts[2];
                                    code.push_str(&format!(
                                        "            {}: (self: TranslationsInstance, count: number, params: {{}}?) -> string,\n",
                                        method
                                    ));
                                }
                            }
                        }

                        code.push_str("        },\n");
                    }
                }
            }

            code.push_str("    },\n");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_namespace_tree() {
        let translations = [
            Translation {
                key: "ui.buttons.buy".to_string(),
                value: "Buy".to_string(),
                locale: "en".to_string(),
                context: None,
            },
            Translation {
                key: "ui.labels.welcome".to_string(),
                value: "Welcome".to_string(),
                locale: "en".to_string(),
                context: None,
            },
        ];

        let refs: Vec<_> = translations.iter().collect();
        let plural_base_keys = HashSet::new();
        let namespaces = build_namespace_tree(&refs, &plural_base_keys);

        assert!(namespaces.contains("ui"));
        assert!(namespaces.contains("ui.buttons"));
        assert!(namespaces.contains("ui.labels"));
    }

    #[test]
    fn test_generate_type_definitions_with_plurals() {
        let translations = vec![
            Translation {
                key: "ui.messages.items(one)".to_string(),
                value: "{count} item".to_string(),
                locale: "en".to_string(),
                context: None,
            },
            Translation {
                key: "ui.messages.items(other)".to_string(),
                value: "{count} items".to_string(),
                locale: "en".to_string(),
                context: None,
            },
        ];

        let code = generate_type_definitions(&translations, "en").unwrap();

        // Should have flat method with count parameter
        assert!(code.contains(
            "ui_messages_items: (self: TranslationsInstance, count: number, params: {}?) -> string"
        ));

        // Should have namespace method with count parameter
        assert!(code
            .contains("items: (self: TranslationsInstance, count: number, params: {}?) -> string"));

        // Should NOT have invalid syntax like items(one) or items(other)
        assert!(!code.contains("items(one)"));
        assert!(!code.contains("items(other)"));
    }
}
