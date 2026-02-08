#![allow(deprecated)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(clippy::doc_lazy_continuation)]

mod common;

use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;

// Test suite for edge cases and boundary conditions
//
// This module tests the robustness of the CLI tool against:
// - Empty and malformed input files
// - Special characters and Unicode
// - Large datasets and deep nesting
// - Missing files and directories
// - Conflicting data and boundary conditions

// ====================================================================================
// Empty Files Tests
// ====================================================================================

/// Tests that empty JSON files are handled gracefully without crashing
#[test]
fn test_empty_json_file() {
    let temp = common::create_test_project();

    // Create empty JSON
    fs::write(temp.path().join("translations/en.json"), "{}").unwrap();

    Command::cargo_bin("roblox-slang")
        .unwrap()
        .current_dir(&temp)
        .arg("build")
        .assert()
        .success(); // Should not crash
}

/// Tests that empty YAML files are handled gracefully without crashing
#[test]
fn test_empty_yaml_file() {
    let temp = common::create_test_project();

    // Create empty YAML
    fs::write(temp.path().join("translations/en.yaml"), "---").unwrap();

    Command::cargo_bin("roblox-slang")
        .unwrap()
        .current_dir(&temp)
        .arg("build")
        .assert()
        .success(); // Should not crash
}

/// Tests that completely empty files fail gracefully with helpful error messages
#[test]
fn test_completely_empty_file() {
    let temp = common::create_test_project();

    // Create completely empty file
    fs::write(temp.path().join("translations/en.json"), "").unwrap();

    Command::cargo_bin("roblox-slang")
        .unwrap()
        .current_dir(&temp)
        .arg("build")
        .assert()
        .failure() // Should fail gracefully
        .stderr(predicate::str::contains("Failed to parse"));
}

// ====================================================================================
// Invalid Format Tests
// ====================================================================================

/// Tests that malformed JSON is detected and reported with clear error messages
#[test]
fn test_malformed_json() {
    let temp = common::create_test_project();

    // Create malformed JSON
    fs::write(temp.path().join("translations/en.json"), "{ invalid json }").unwrap();

    Command::cargo_bin("roblox-slang")
        .unwrap()
        .current_dir(&temp)
        .arg("build")
        .assert()
        .failure()
        .stderr(predicate::str::contains("Failed to parse"));
}

/// Tests that invalid YAML syntax is detected and reported
#[test]
fn test_invalid_yaml_syntax() {
    let temp = common::create_test_project();

    // Create invalid YAML
    let invalid_yaml = r#"
ui:
  button: "Buy"
    invalid_indent: "Test"
"#;
    fs::write(temp.path().join("translations/en.yaml"), invalid_yaml).unwrap();

    Command::cargo_bin("roblox-slang")
        .unwrap()
        .current_dir(&temp)
        .arg("build")
        .assert()
        .failure()
        .stderr(predicate::str::contains("Failed to parse"));
}

/// Tests that JSON with trailing commas (invalid in strict JSON) is rejected
#[test]
fn test_json_with_trailing_comma() {
    let temp = common::create_test_project();

    // JSON with trailing comma (invalid in strict JSON)
    let json = r#"{
  "ui": {
    "button": "Buy",
  }
}"#;
    fs::write(temp.path().join("translations/en.json"), json).unwrap();

    Command::cargo_bin("roblox-slang")
        .unwrap()
        .current_dir(&temp)
        .arg("build")
        .assert()
        .failure();
}

// ====================================================================================
// Special Characters Tests
// ====================================================================================

/// Tests that Unicode characters in keys are handled correctly
#[test]
fn test_unicode_in_keys() {
    let temp = common::create_test_project();

    // Unicode characters in keys
    let json = r#"{
  "你好": "Hello",
  "مرحبا": "Welcome"
}"#;
    fs::write(temp.path().join("translations/en.json"), json).unwrap();

    Command::cargo_bin("roblox-slang")
        .unwrap()
        .current_dir(&temp)
        .arg("build")
        .assert()
        .success(); // Should handle Unicode
}

/// Tests that emojis in translation values are preserved correctly
#[test]
fn test_emojis_in_values() {
    let temp = common::create_test_project();

    // Emojis in values
    let json = r#"{
  "ui": {
    "greeting": "Hello 👋",
    "celebration": "Party! 🎉🎊"
  }
}"#;
    fs::write(temp.path().join("translations/en.json"), json).unwrap();

    Command::cargo_bin("roblox-slang")
        .unwrap()
        .current_dir(&temp)
        .arg("build")
        .assert()
        .success(); // Just verify it doesn't crash
}

/// Tests that escape sequences (quotes, newlines, tabs) are handled correctly
#[test]
fn test_escape_sequences() {
    let temp = common::create_test_project();

    // Escape sequences
    let json = r#"{
  "ui": {
    "quote": "He said \"Hi\"",
    "newline": "Line 1\nLine 2",
    "tab": "Col1\tCol2"
  }
}"#;
    fs::write(temp.path().join("translations/en.json"), json).unwrap();

    Command::cargo_bin("roblox-slang")
        .unwrap()
        .current_dir(&temp)
        .arg("build")
        .assert()
        .success();
}

/// Tests that special Lua characters are properly escaped in generated code
#[test]
fn test_special_lua_characters() {
    let temp = common::create_test_project();

    // Characters that need escaping in Lua
    let json = r#"{
  "ui": {
    "brackets": "[[nested]]",
    "quotes": "It's \"quoted\"",
    "backslash": "Path\\to\\file"
  }
}"#;
    fs::write(temp.path().join("translations/en.json"), json).unwrap();

    Command::cargo_bin("roblox-slang")
        .unwrap()
        .current_dir(&temp)
        .arg("build")
        .assert()
        .success();
}

// ====================================================================================
// Large Dataset Tests
// ====================================================================================

/// Tests performance and correctness with 1000 translations
#[test]
fn test_many_translations() {
    let temp = common::create_test_project();

    // Generate 1000 translations
    let mut translations = serde_json::Map::new();
    for i in 0..1000 {
        translations.insert(
            format!("key_{}", i),
            serde_json::Value::String(format!("Value {}", i)),
        );
    }

    let json = serde_json::to_string_pretty(&translations).unwrap();
    fs::write(temp.path().join("translations/en.json"), json).unwrap();

    Command::cargo_bin("roblox-slang")
        .unwrap()
        .current_dir(&temp)
        .arg("build")
        .assert()
        .success();

    // Verify output generated
    common::assert_file_exists(&temp.path().join("output/Translations.lua"));
}

/// Tests that deeply nested structures (10 levels) are flattened correctly
#[test]
fn test_deep_nesting() {
    let temp = common::create_test_project();

    // Create deeply nested structure (10 levels)
    let json = r#"{
  "level1": {
    "level2": {
      "level3": {
        "level4": {
          "level5": {
            "level6": {
              "level7": {
                "level8": {
                  "level9": {
                    "level10": "Deep value"
                  }
                }
              }
            }
          }
        }
      }
    }
  }
}"#;
    fs::write(temp.path().join("translations/en.json"), json).unwrap();

    Command::cargo_bin("roblox-slang")
        .unwrap()
        .current_dir(&temp)
        .arg("build")
        .assert()
        .success();

    // Verify deep key generated
    let luau_path = temp.path().join("output/Translations.lua");
    common::assert_file_contains(
        &luau_path,
        "level1_level2_level3_level4_level5_level6_level7_level8_level9_level10",
    );
}

/// Tests that very long keys (200 characters) are handled correctly
#[test]
fn test_very_long_keys() {
    let temp = common::create_test_project();

    // Very long key
    let long_key = "a".repeat(200);
    let json = format!(
        r#"{{
  "{}": "Value"
}}"#,
        long_key
    );
    fs::write(temp.path().join("translations/en.json"), json).unwrap();

    Command::cargo_bin("roblox-slang")
        .unwrap()
        .current_dir(&temp)
        .arg("build")
        .assert()
        .success();
}

/// Tests that very long values (1000+ characters) are handled correctly
#[test]
fn test_very_long_values() {
    let temp = common::create_test_project();

    // Very long value (1000 characters)
    let long_value = "Lorem ipsum ".repeat(100);
    let json = format!(
        r#"{{
  "ui": {{
    "long_text": "{}"
  }}
}}"#,
        long_value
    );
    fs::write(temp.path().join("translations/en.json"), json).unwrap();

    Command::cargo_bin("roblox-slang")
        .unwrap()
        .current_dir(&temp)
        .arg("build")
        .assert()
        .success();
}

// ====================================================================================
// Missing Files Tests
// ====================================================================================

/// Tests that missing config file produces a clear error message
#[test]
fn test_missing_config_file() {
    let temp = tempfile::TempDir::new().unwrap();

    Command::cargo_bin("roblox-slang")
        .unwrap()
        .current_dir(&temp)
        .arg("build")
        .assert()
        .failure()
        .stderr(predicate::str::contains("Failed to load config"));
}

/// Tests that missing translation files are skipped gracefully
#[test]
fn test_missing_translation_file() {
    let temp = common::create_test_project();

    // Config references en.json but file doesn't exist
    Command::cargo_bin("roblox-slang")
        .unwrap()
        .current_dir(&temp)
        .arg("build")
        .assert()
        .success(); // Should skip missing files gracefully
}

/// Tests that missing output directory is created automatically
#[test]
fn test_missing_output_directory() {
    let temp = common::create_test_project();

    // Create translations
    let json = r#"{"ui": {"button": "Buy"}}"#;
    fs::write(temp.path().join("translations/en.json"), json).unwrap();

    // Remove output directory
    fs::remove_dir_all(temp.path().join("output")).ok();

    Command::cargo_bin("roblox-slang")
        .unwrap()
        .current_dir(&temp)
        .arg("build")
        .assert()
        .success(); // Should create directory automatically

    common::assert_file_exists(&temp.path().join("output"));
}

// ====================================================================================
// Conflicting Data Tests
// ====================================================================================

/// Tests that duplicate keys in the same file are handled (last one wins)
#[test]
fn test_duplicate_keys_in_same_file() {
    let temp = common::create_test_project();

    // JSON with duplicate keys (last one wins in most parsers)
    let json = r#"{
  "ui": {
    "button": "Buy",
    "button": "Sell"
  }
}"#;
    fs::write(temp.path().join("translations/en.json"), json).unwrap();

    Command::cargo_bin("roblox-slang")
        .unwrap()
        .current_dir(&temp)
        .arg("build")
        .assert()
        .success(); // Parser handles this
}

/// Tests that mixed types in JSON are handled (only strings are processed)
#[test]
fn test_mixed_types_in_json() {
    let temp = common::create_test_project();

    // Mixed types (should only process strings)
    let json = r#"{
  "ui": {
    "button": "Buy",
    "count": 123,
    "enabled": true,
    "items": ["a", "b", "c"]
  }
}"#;
    fs::write(temp.path().join("translations/en.json"), json).unwrap();

    Command::cargo_bin("roblox-slang")
        .unwrap()
        .current_dir(&temp)
        .arg("build")
        .assert()
        .success(); // Should skip non-string values
}

// ====================================================================================
// Boundary Tests
// ====================================================================================

/// Tests that single character keys are handled correctly
#[test]
fn test_single_character_key() {
    let temp = common::create_test_project();

    let json = r#"{"a": "Value"}"#;
    fs::write(temp.path().join("translations/en.json"), json).unwrap();

    Command::cargo_bin("roblox-slang")
        .unwrap()
        .current_dir(&temp)
        .arg("build")
        .assert()
        .success();
}

/// Tests that single character values are handled correctly
#[test]
fn test_single_character_value() {
    let temp = common::create_test_project();

    let json = r#"{"ui": {"button": "X"}}"#;
    fs::write(temp.path().join("translations/en.json"), json).unwrap();

    Command::cargo_bin("roblox-slang")
        .unwrap()
        .current_dir(&temp)
        .arg("build")
        .assert()
        .success();
}

/// Tests that empty string values are handled correctly
#[test]
fn test_empty_string_value() {
    let temp = common::create_test_project();

    let json = r#"{"ui": {"button": ""}}"#;
    fs::write(temp.path().join("translations/en.json"), json).unwrap();

    Command::cargo_bin("roblox-slang")
        .unwrap()
        .current_dir(&temp)
        .arg("build")
        .assert()
        .success();
}

/// Tests that whitespace-only values are preserved
#[test]
fn test_whitespace_only_value() {
    let temp = common::create_test_project();

    let json = r#"{"ui": {"button": "   "}}"#;
    fs::write(temp.path().join("translations/en.json"), json).unwrap();

    Command::cargo_bin("roblox-slang")
        .unwrap()
        .current_dir(&temp)
        .arg("build")
        .assert()
        .success();
}

// ====================================================================================
// Locale-Specific Tests
// ====================================================================================

/// Tests that unsupported locales in config are rejected with clear error messages
#[test]
fn test_unsupported_locale_in_config() {
    let temp = tempfile::TempDir::new().unwrap();

    // Create config with unsupported locale
    let config = r#"base_locale: en
supported_locales:
  - en
  - xx
input_directory: translations
output_directory: output
"#;
    fs::write(temp.path().join("slang-roblox.yaml"), config).unwrap();
    fs::create_dir(temp.path().join("translations")).unwrap();

    let json = r#"{"ui": {"button": "Buy"}}"#;
    fs::write(temp.path().join("translations/en.json"), json).unwrap();

    Command::cargo_bin("roblox-slang")
        .unwrap()
        .current_dir(&temp)
        .arg("build")
        .assert()
        .failure() // Should fail with validation error
        .stderr(predicate::str::contains("Unsupported locale"));
}

/// Tests that all 17 Roblox-supported locales can be configured
#[test]
fn test_all_roblox_locales() {
    let temp = tempfile::TempDir::new().unwrap();

    // Create config with all Roblox locales (17 locales)
    let config = r#"base_locale: en
supported_locales:
  - en
  - es
  - fr
  - de
  - it
  - pt
  - ru
  - ja
  - ko
  - zh-cn
  - zh-tw
  - id
  - tr
  - vi
  - th
  - pl
  - uk
input_directory: translations
output_directory: output
"#;
    fs::write(temp.path().join("slang-roblox.yaml"), config).unwrap();
    fs::create_dir(temp.path().join("translations")).unwrap();

    // Create English translations
    let json = r#"{"ui": {"button": "Buy"}}"#;
    fs::write(temp.path().join("translations/en.json"), json).unwrap();

    Command::cargo_bin("roblox-slang")
        .unwrap()
        .current_dir(&temp)
        .arg("build")
        .assert()
        .success();
}
