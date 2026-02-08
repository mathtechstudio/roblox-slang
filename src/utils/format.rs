/// Format specifier types
/// Based on Flutter Slang format specifiers
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum FormatSpecifier {
    /// Integer formatting: {count:int}
    Int,
    /// Fixed decimal: {price:fixed(2)}
    Fixed(usize),
    /// Number formatting: {value:num}
    Num,
    /// DateTime formatting: {date:datetime}
    DateTime,
    /// Nested translation: {label:translate}
    Translate,
    /// No specifier (default string)
    None,
}

/// Parse format specifier from parameter
/// Example: "count:int" -> ("count", FormatSpecifier::Int)
pub fn parse_format_specifier(param: &str) -> (String, FormatSpecifier) {
    if let Some(colon_pos) = param.find(':') {
        let name = param[..colon_pos].trim().to_string();
        let spec = param[colon_pos + 1..].trim();

        let format = match spec {
            "int" => FormatSpecifier::Int,
            "num" => FormatSpecifier::Num,
            "datetime" => FormatSpecifier::DateTime,
            "translate" => FormatSpecifier::Translate,
            s if s.starts_with("fixed(") && s.ends_with(')') => {
                // Parse fixed(n)
                let digits_str = &s[6..s.len() - 1];
                let digits = digits_str.parse::<usize>().unwrap_or(2);
                FormatSpecifier::Fixed(digits)
            }
            _ => FormatSpecifier::None,
        };

        (name, format)
    } else {
        (param.trim().to_string(), FormatSpecifier::None)
    }
}

/// Extract all parameters with their format specifiers from a translation string
pub fn extract_parameters_with_format(text: &str) -> HashMap<String, FormatSpecifier> {
    let mut params = HashMap::new();
    let mut in_param = false;
    let mut current_param = String::new();

    for ch in text.chars() {
        match ch {
            '{' => {
                in_param = true;
                current_param.clear();
            }
            '}' => {
                if in_param && !current_param.is_empty() {
                    let (name, format) = parse_format_specifier(&current_param);
                    params.insert(name, format);
                }
                in_param = false;
            }
            _ => {
                if in_param {
                    current_param.push(ch);
                }
            }
        }
    }

    params
}

/// Generate Luau code for format specifier
pub fn generate_format_code(param_name: &str, specifier: &FormatSpecifier) -> String {
    match specifier {
        FormatSpecifier::Int => {
            format!(
                "params.{} = math.floor(tonumber(params.{}) or 0)",
                param_name, param_name
            )
        }
        FormatSpecifier::Fixed(digits) => {
            format!(
                "params.{} = string.format(\"%.{}f\", tonumber(params.{}) or 0)",
                param_name, digits, param_name
            )
        }
        FormatSpecifier::Num => {
            format!(
                "params.{} = tostring(params.{} or 0)",
                param_name, param_name
            )
        }
        FormatSpecifier::DateTime => {
            // Roblox DateTime formatting
            format!(
                "if typeof(params.{}) == \"DateTime\" then\n        params.{} = params.{}:FormatLocalTime(\"L LT\", \"en-us\")\n    end",
                param_name, param_name, param_name
            )
        }
        FormatSpecifier::Translate => {
            // Nested translation lookup
            format!(
                "if type(params.{}) == \"string\" then\n        params.{} = self._translator:FormatByKey(params.{})\n    end",
                param_name, param_name, param_name
            )
        }
        FormatSpecifier::None => String::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_format_specifier() {
        let (name, spec) = parse_format_specifier("count:int");
        assert_eq!(name, "count");
        assert_eq!(spec, FormatSpecifier::Int);
    }

    #[test]
    fn test_parse_fixed_specifier() {
        let (name, spec) = parse_format_specifier("price:fixed(2)");
        assert_eq!(name, "price");
        assert_eq!(spec, FormatSpecifier::Fixed(2));
    }

    #[test]
    fn test_parse_no_specifier() {
        let (name, spec) = parse_format_specifier("name");
        assert_eq!(name, "name");
        assert_eq!(spec, FormatSpecifier::None);
    }

    #[test]
    fn test_extract_parameters_with_format() {
        let text = "Price: {price:fixed(2)}, Count: {count:int}";
        let params = extract_parameters_with_format(text);

        assert_eq!(params.get("price"), Some(&FormatSpecifier::Fixed(2)));
        assert_eq!(params.get("count"), Some(&FormatSpecifier::Int));
    }
}
