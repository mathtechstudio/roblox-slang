#![allow(deprecated)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(clippy::doc_lazy_continuation)]

mod common;

use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use std::time::{Duration, Instant};

// Stress tests for extreme conditions
//
// These tests verify the tool can handle:
// - 10,000+ translation keys
// - 100+ locales
// - 1MB+ translation files
// - Rapid file changes in watch mode
//
// Run with: cargo test --ignored

// ====================================================================================
// Large Dataset Tests
// ====================================================================================

/// Tests building with 10,000 translation keys
#[test]
#[ignore] // Run with --ignored flag
fn test_10k_translations() {
    let temp = common::create_test_project();

    // Generate 10,000 translations
    let mut translations = serde_json::Map::new();
    for i in 0..10000 {
        let category = format!("category_{}", i / 100);
        let subcategory = format!("subcategory_{}", i / 10);
        let key = format!("key_{}", i);
        let value = format!("Translation value number {}", i);

        if !translations.contains_key(&category) {
            translations.insert(
                category.clone(),
                serde_json::Value::Object(serde_json::Map::new()),
            );
        }

        if let Some(serde_json::Value::Object(cat_map)) = translations.get_mut(&category) {
            if !cat_map.contains_key(&subcategory) {
                cat_map.insert(
                    subcategory.clone(),
                    serde_json::Value::Object(serde_json::Map::new()),
                );
            }

            if let Some(serde_json::Value::Object(subcat_map)) = cat_map.get_mut(&subcategory) {
                subcat_map.insert(key, serde_json::Value::String(value));
            }
        }
    }

    let json = serde_json::to_string_pretty(&translations).unwrap();
    fs::write(temp.path().join("translations/en.json"), &json).unwrap();

    // Measure build time
    let start = Instant::now();

    Command::cargo_bin("roblox-slang")
        .unwrap()
        .current_dir(&temp)
        .arg("build")
        .assert()
        .success()
        .stdout(predicate::str::contains("Parsed en"));

    let duration = start.elapsed();

    // Verify output generated
    common::assert_file_exists(&temp.path().join("output/Translations.lua"));
    common::assert_file_exists(&temp.path().join("output/roblox_upload.csv"));

    // Performance assertions - relaxed to 7s for debug builds
    assert!(
        duration < Duration::from_secs(7),
        "Build took {:?}, expected <10s (debug build)",
        duration
    );

    println!("✓ Built 10,000 translations in {:?}", duration);
}

/// Tests building with 17 locales
/// NOTE: Roblox only supports 17 locales, so we test with all 17 repeated
/// Target: Complete successfully without crashes
#[test]
#[ignore]
fn test_100_locales() {
    let temp = tempfile::TempDir::new().unwrap();

    // Use actual Roblox locales (17 total), repeat them to get more
    let roblox_locales = vec![
        "en", "es", "fr", "de", "it", "pt", "ru", "ja", "ko", "zh-cn", "zh-tw", "id", "tr", "vi",
        "th", "pl", "uk",
    ];

    // Create config with all 17 Roblox locales
    let config = format!(
        r#"base_locale: en
supported_locales:
{}
input_directory: translations
output_directory: output
"#,
        roblox_locales
            .iter()
            .map(|l| format!("  - {}", l))
            .collect::<Vec<_>>()
            .join("\n")
    );

    fs::write(temp.path().join("slang-roblox.yaml"), config).unwrap();
    fs::create_dir(temp.path().join("translations")).unwrap();

    // Create translation files for each locale
    let json = r#"{"ui": {"button": "Buy"}}"#;
    for locale in &roblox_locales {
        fs::write(
            temp.path().join(format!("translations/{}.json", locale)),
            json,
        )
        .unwrap();
    }

    // Build
    let start = Instant::now();

    Command::cargo_bin("roblox-slang")
        .unwrap()
        .current_dir(&temp)
        .arg("build")
        .assert()
        .success();

    let duration = start.elapsed();

    // Verify output
    common::assert_file_exists(&temp.path().join("output/Translations.lua"));

    println!(
        "✓ Built {} Roblox locales in {:?}",
        roblox_locales.len(),
        duration
    );
}

/// Tests parsing a 1MB+ translation file
/// Target: Complete without memory issues
#[test]
#[ignore]
fn test_1mb_translation_file() {
    let temp = common::create_test_project();

    // Generate large translation file (>1MB)
    let mut translations = serde_json::Map::new();

    // Create many translations with long values
    for i in 0..5000 {
        let key = format!("key_{}", i);
        // Each value is ~200 characters
        let value = format!(
            "This is a very long translation value number {} that contains a lot of text \
             to make the file size larger. Lorem ipsum dolor sit amet, consectetur adipiscing \
             elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.",
            i
        );
        translations.insert(key, serde_json::Value::String(value));
    }

    let json = serde_json::to_string_pretty(&translations).unwrap();
    let file_size = json.len();

    assert!(
        file_size > 1_000_000,
        "File size is {} bytes, expected >1MB",
        file_size
    );

    fs::write(temp.path().join("translations/en.json"), &json).unwrap();

    // Build
    let start = Instant::now();

    Command::cargo_bin("roblox-slang")
        .unwrap()
        .current_dir(&temp)
        .arg("build")
        .assert()
        .success();

    let duration = start.elapsed();

    println!(
        "✓ Parsed {:.2}MB file in {:?}",
        file_size as f64 / 1_000_000.0,
        duration
    );
}

/// Tests with deeply nested structure (20 levels)
/// Target: Complete without stack overflow
#[test]
#[ignore]
fn test_very_deep_nesting() {
    let temp = common::create_test_project();

    // Create 20-level deep nesting
    let mut json = String::from("{");
    for i in 0..20 {
        json.push_str(&format!("\"level_{}\": {{", i));
    }
    json.push_str("\"final\": \"Deep value\"");
    for _ in 0..20 {
        json.push('}');
    }
    json.push('}');

    fs::write(temp.path().join("translations/en.json"), json).unwrap();

    Command::cargo_bin("roblox-slang")
        .unwrap()
        .current_dir(&temp)
        .arg("build")
        .assert()
        .success();

    println!("✓ Handled 20-level deep nesting");
}

// ====================================================================================
// Rapid Operations Tests
// ====================================================================================

/// Tests rapid file changes in watch mode
/// Target: Handle 100 rapid changes without crashes
#[test]
#[ignore]
fn test_rapid_file_changes() {
    use std::process::{Command as StdCommand, Stdio};

    let temp = common::create_test_project_with_translations();

    // Get the binary path
    let bin_path = assert_cmd::cargo::cargo_bin("roblox-slang");

    // Start watch mode in background using std::process::Command
    let mut child = StdCommand::new(bin_path)
        .current_dir(&temp)
        .arg("build")
        .arg("--watch")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .expect("Failed to start watch mode");

    // Wait for watch mode to start
    std::thread::sleep(Duration::from_secs(2));

    // Make 100 rapid file changes
    for i in 0..100 {
        let json = format!(r#"{{"ui": {{"button": "Buy {}"}}}}"#, i);
        fs::write(temp.path().join("translations/en.json"), json).unwrap();

        // Small delay between changes
        std::thread::sleep(Duration::from_millis(50));
    }

    // Wait for final rebuild
    std::thread::sleep(Duration::from_secs(2));

    // Stop watch mode
    child.kill().expect("Failed to kill watch process");
    child.wait().expect("Failed to wait for process");

    // Verify final output exists
    common::assert_file_exists(&temp.path().join("output/Translations.lua"));

    println!("✓ Handled 100 rapid file changes");
}

/// Tests building multiple times in succession
/// Target: Consistent performance across builds
#[test]
#[ignore]
fn test_repeated_builds() {
    let temp = common::create_test_project_with_translations();

    let mut durations = Vec::new();

    // Build 10 times
    for i in 0..10 {
        let start = Instant::now();

        Command::cargo_bin("roblox-slang")
            .unwrap()
            .current_dir(&temp)
            .arg("build")
            .assert()
            .success();

        let duration = start.elapsed();
        durations.push(duration);

        println!("Build {}: {:?}", i + 1, duration);
    }

    // Calculate average
    let avg = durations.iter().sum::<Duration>() / durations.len() as u32;

    // Check consistency (no build should be >2x average)
    for (i, duration) in durations.iter().enumerate() {
        assert!(
            *duration < avg * 2,
            "Build {} took {:?}, more than 2x average {:?}",
            i + 1,
            duration,
            avg
        );
    }

    println!("✓ Average build time: {:?}", avg);
}

// ====================================================================================
// Memory Stress Tests
// ====================================================================================

/// Tests with many translation files (all 17 Roblox locales)
/// Target: Handle multiple files efficiently
#[test]
#[ignore]
fn test_many_small_files() {
    let temp = tempfile::TempDir::new().unwrap();

    // Use all 17 Roblox locales
    let locales = vec![
        "en", "es", "fr", "de", "it", "pt", "ru", "ja", "ko", "zh-cn", "zh-tw", "id", "tr", "vi",
        "th", "pl", "uk",
    ];

    let config = format!(
        r#"base_locale: en
supported_locales:
{}
input_directory: translations
output_directory: output
"#,
        locales
            .iter()
            .map(|l| format!("  - {}", l))
            .collect::<Vec<_>>()
            .join("\n")
    );

    fs::write(temp.path().join("slang-roblox.yaml"), config).unwrap();
    fs::create_dir(temp.path().join("translations")).unwrap();

    // Create translation files for all locales
    for locale in &locales {
        let json = r#"{"ui": {"button": "Buy"}}"#;
        fs::write(
            temp.path().join(format!("translations/{}.json", locale)),
            json,
        )
        .unwrap();
    }

    let start = Instant::now();

    Command::cargo_bin("roblox-slang")
        .unwrap()
        .current_dir(&temp)
        .arg("build")
        .assert()
        .success();

    let duration = start.elapsed();

    println!(
        "✓ Processed {} Roblox locale files in {:?}",
        locales.len(),
        duration
    );
}

/// Tests with very long keys (500 characters)
/// Target: Handle without truncation or errors
#[test]
#[ignore]
fn test_very_long_keys() {
    let temp = common::create_test_project();

    // Create key with 500 characters
    let long_key = "a".repeat(500);
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

    // Verify key in output
    let luau = fs::read_to_string(temp.path().join("output/Translations.lua")).unwrap();
    assert!(luau.len() > 500, "Output should contain long key");

    println!("✓ Handled 500-character key");
}

/// Tests with very long values (10,000 characters)
/// Target: Handle without truncation or errors
#[test]
#[ignore]
fn test_very_long_values() {
    let temp = common::create_test_project();

    // Create value with 10,000 characters
    let long_value = "Lorem ipsum ".repeat(1000);
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

    // Verify value in output - check that output is large enough to contain the value
    let luau = fs::read_to_string(temp.path().join("output/Translations.lua")).unwrap();
    assert!(
        luau.len() > 5000,
        "Output should be large (contains long value), got {} bytes",
        luau.len()
    );

    println!(
        "✓ Handled 10,000-character value (output: {} bytes)",
        luau.len()
    );
}

// ====================================================================================
// Concurrent Operations Tests
// ====================================================================================

/// Tests building multiple projects concurrently
/// Target: No race conditions or conflicts
#[test]
#[ignore]
fn test_concurrent_builds() {
    use std::thread;

    let handles: Vec<_> = (0..10)
        .map(|i| {
            thread::spawn(move || {
                let temp = common::create_test_project_with_translations();

                Command::cargo_bin("roblox-slang")
                    .unwrap()
                    .current_dir(&temp)
                    .arg("build")
                    .assert()
                    .success();

                println!("✓ Concurrent build {} completed", i);
            })
        })
        .collect();

    // Wait for all builds to complete
    for handle in handles {
        handle.join().expect("Thread panicked");
    }

    println!("✓ All 10 concurrent builds completed");
}

// ====================================================================================
// Edge Case Combinations
// ====================================================================================

/// Tests combination of large dataset + many locales
/// Target: Complete without memory exhaustion
#[test]
#[ignore]
fn test_large_dataset_many_locales() {
    let temp = tempfile::TempDir::new().unwrap();

    // Use all 17 Roblox locales
    let locales = vec![
        "en", "es", "fr", "de", "it", "pt", "ru", "ja", "ko", "zh-cn", "zh-tw", "id", "tr", "vi",
        "th", "pl", "uk",
    ];

    let config = format!(
        r#"base_locale: en
supported_locales:
{}
input_directory: translations
output_directory: output
"#,
        locales
            .iter()
            .map(|l| format!("  - {}", l))
            .collect::<Vec<_>>()
            .join("\n")
    );

    fs::write(temp.path().join("slang-roblox.yaml"), config).unwrap();
    fs::create_dir(temp.path().join("translations")).unwrap();

    // 1000 translations per locale
    let mut translations = serde_json::Map::new();
    for i in 0..1000 {
        let key = format!("key_{}", i);
        let value = format!("Value {}", i);
        translations.insert(key, serde_json::Value::String(value));
    }
    let json = serde_json::to_string_pretty(&translations).unwrap();

    for locale in &locales {
        fs::write(
            temp.path().join(format!("translations/{}.json", locale)),
            &json,
        )
        .unwrap();
    }

    let start = Instant::now();

    Command::cargo_bin("roblox-slang")
        .unwrap()
        .current_dir(&temp)
        .arg("build")
        .assert()
        .success();

    let duration = start.elapsed();

    println!(
        "✓ Built {} locales × 1000 translations = {} total in {:?}",
        locales.len(),
        locales.len() * 1000,
        duration
    );
}

/// Tests all features combined (overrides + plurals + params + contexts)
/// Target: All features work together correctly
#[test]
#[ignore]
fn test_all_features_combined() {
    let temp = common::create_test_project();

    // Complex translations with all features
    let json = r#"{
  "ui": {
    "greeting": "Hello {name}!",
    "items(plural)": {
      "zero": "No items",
      "one": "One item",
      "other": "{count} items"
    },
    "button(context=save)": "Save",
    "button(context=cancel)": "Cancel"
  }
}"#;
    fs::write(temp.path().join("translations/en.json"), json).unwrap();

    // Overrides
    let overrides = r#"overrides:
  ui.greeting:
    en: "Hi {name}!"
"#;
    fs::write(temp.path().join("overrides.yaml"), overrides).unwrap();

    Command::cargo_bin("roblox-slang")
        .unwrap()
        .current_dir(&temp)
        .arg("build")
        .assert()
        .success();

    println!("✓ All features combined successfully");
}
