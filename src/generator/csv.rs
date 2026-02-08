use crate::parser::Translation;
use anyhow::Result;
use std::collections::HashMap;

/// Generate CSV file for Roblox Cloud Localization Table
pub fn generate_csv(
    translations: &[Translation],
    base_locale: &str,
    locales: &[String],
) -> Result<String> {
    let mut csv = String::new();

    // Header row
    csv.push_str("Source,Context,Key");
    for locale in locales {
        csv.push(',');
        csv.push_str(locale);
    }
    csv.push('\n');

    // Group translations by key
    let mut translation_map: HashMap<String, (Option<String>, HashMap<String, String>)> =
        HashMap::new();

    for translation in translations {
        let entry = translation_map
            .entry(translation.key.clone())
            .or_insert_with(|| (translation.context.clone(), HashMap::new()));

        entry
            .1
            .insert(translation.locale.clone(), translation.value.clone());
    }

    // Get all unique keys (sorted for consistency)
    let mut keys: Vec<_> = translation_map.keys().cloned().collect();
    keys.sort();

    // Generate rows
    for key in keys {
        let (context, locale_values) = translation_map.get(&key).unwrap();

        // Source column (base locale value)
        let source = locale_values
            .get(base_locale)
            .map(|s| escape_csv_value(s))
            .unwrap_or_else(|| String::from("\"\""));

        csv.push_str(&source);
        csv.push(',');

        // Context column (for disambiguation)
        let context_str = context
            .as_ref()
            .map(|c| escape_csv_value(c))
            .unwrap_or_else(|| String::from("\"\""));
        csv.push_str(&context_str);
        csv.push(',');

        // Key column
        csv.push_str(&escape_csv_value(&key));

        // Locale columns
        for locale in locales {
            csv.push(',');
            let value = locale_values
                .get(locale)
                .map(|s| escape_csv_value(s))
                .unwrap_or_else(|| String::from("\"\""));
            csv.push_str(&value);
        }

        csv.push('\n');
    }

    Ok(csv)
}

/// Escape CSV value (wrap in quotes and escape internal quotes)
fn escape_csv_value(value: &str) -> String {
    // Check if value needs escaping
    let needs_escape =
        value.contains('"') || value.contains(',') || value.contains('\n') || value.contains('\r');

    if needs_escape || !value.is_empty() {
        // Escape internal quotes by doubling them
        let escaped = value.replace('"', "\"\"");
        format!("\"{}\"", escaped)
    } else {
        String::from("\"\"")
    }
}

/// Parse CSV file (for import/migration)
pub fn parse_csv(content: &str) -> Result<Vec<Translation>> {
    let mut translations = Vec::new();
    let mut lines = content.lines();

    // Parse header
    let header = lines
        .next()
        .ok_or_else(|| anyhow::anyhow!("CSV file is empty"))?;
    let headers = parse_csv_line(header);

    if headers.len() < 3 {
        anyhow::bail!("Invalid CSV header: expected at least Source,Context,Key columns");
    }

    // Extract locale columns (skip Source, Context, Key)
    let locales: Vec<String> = headers[3..].to_vec();

    // Parse data rows
    for line in lines {
        if line.trim().is_empty() {
            continue;
        }

        let values = parse_csv_line(line);

        if values.len() < 3 {
            log::warn!("Skipping invalid CSV row: {}", line);
            continue;
        }

        let key = values[2].clone();
        let context = if values[1].is_empty() {
            None
        } else {
            Some(values[1].clone())
        };

        // Create translation for each locale
        for (i, locale) in locales.iter().enumerate() {
            let value_index = 3 + i;
            if value_index < values.len() {
                let value = &values[value_index];
                if !value.is_empty() {
                    translations.push(Translation {
                        key: key.clone(),
                        value: value.clone(),
                        locale: locale.clone(),
                        context: context.clone(),
                    });
                }
            }
        }
    }

    Ok(translations)
}

/// Parse a single CSV line (handles quoted values)
fn parse_csv_line(line: &str) -> Vec<String> {
    let mut values = Vec::new();
    let mut current = String::new();
    let mut in_quotes = false;
    let mut chars = line.chars().peekable();

    while let Some(ch) = chars.next() {
        match ch {
            '"' => {
                if in_quotes {
                    // Check for escaped quote
                    if chars.peek() == Some(&'"') {
                        current.push('"');
                        chars.next();
                    } else {
                        in_quotes = false;
                    }
                } else {
                    in_quotes = true;
                }
            }
            ',' => {
                if in_quotes {
                    current.push(',');
                } else {
                    values.push(current.trim().to_string());
                    current.clear();
                }
            }
            _ => {
                current.push(ch);
            }
        }
    }

    // Add last value
    values.push(current.trim().to_string());

    values
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_escape_csv_value() {
        assert_eq!(escape_csv_value("Hello"), "\"Hello\"");
        assert_eq!(escape_csv_value("Hello, World"), "\"Hello, World\"");
        assert_eq!(escape_csv_value("Say \"Hi\""), "\"Say \"\"Hi\"\"\"");
    }

    #[test]
    fn test_generate_csv() {
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

        let csv = generate_csv(&translations, "en", &["en".to_string(), "id".to_string()]).unwrap();

        assert!(csv.contains("Source,Context,Key,en,id"));
        assert!(csv.contains("\"Buy\""));
        assert!(csv.contains("\"Beli\""));
    }

    #[test]
    fn test_parse_csv() {
        let csv_content = r#"Source,Context,Key,en,id
"Buy","","ui.button","Buy","Beli"
"Sell","","ui.sell","Sell","Jual"
"#;

        let translations = parse_csv(csv_content).unwrap();

        assert_eq!(translations.len(), 4); // 2 keys × 2 locales
        assert!(translations
            .iter()
            .any(|t| t.key == "ui.button" && t.locale == "en" && t.value == "Buy"));
        assert!(translations
            .iter()
            .any(|t| t.key == "ui.button" && t.locale == "id" && t.value == "Beli"));
    }

    #[test]
    fn test_parse_csv_line() {
        let line = "\"Hello\",\"World\",\"Test\"";
        let values = parse_csv_line(line);
        assert_eq!(values, vec!["Hello", "World", "Test"]);
    }

    #[test]
    fn test_parse_csv_line_with_quotes() {
        let line = "\"Say \"\"Hi\"\"\",\"World\"";
        let values = parse_csv_line(line);
        assert_eq!(values[0], "Say \"Hi\"");
    }
}
