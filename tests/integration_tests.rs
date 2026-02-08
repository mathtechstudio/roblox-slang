#![allow(deprecated)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(clippy::doc_lazy_continuation)]

mod common;

use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;

// Integration test suite for end-to-end CLI workflows
//
// This module tests complete user workflows including:
// - Project initialization
// - Building translations
// - Validation commands
// - Import and migration
// - Error handling

/// Tests the init command creates all necessary project files
#[test]
fn test_init_command() {
    let temp = tempfile::TempDir::new().unwrap();

    Command::cargo_bin("roblox-slang")
        .unwrap()
        .current_dir(&temp)
        .arg("init")
        .assert()
        .success()
        .stdout(predicate::str::contains("initialized"));

    // Check files created
    common::assert_file_exists(&temp.path().join("slang-roblox.yaml"));
    common::assert_file_exists(&temp.path().join("translations"));
}

/// Tests basic build command with multiple locales
#[test]
fn test_build_command_basic() {
    let temp = common::create_test_project_with_translations();

    Command::cargo_bin("roblox-slang")
        .unwrap()
        .current_dir(&temp)
        .arg("build")
        .assert()
        .success()
        .stdout(predicate::str::contains("Building translations"))
        .stdout(predicate::str::contains("Parsed en"))
        .stdout(predicate::str::contains("Parsed id"));

    // Check output files created
    common::assert_file_exists(&temp.path().join("output/Translations.lua"));
    common::assert_file_exists(&temp.path().join("output/types/Translations.d.luau"));
    common::assert_file_exists(&temp.path().join("output/roblox_upload.csv"));
}

/// Tests that generated Luau code contains expected functions
#[test]
fn test_build_generates_correct_luau() {
    let temp = common::create_test_project_with_translations();

    Command::cargo_bin("roblox-slang")
        .unwrap()
        .current_dir(&temp)
        .arg("build")
        .assert()
        .success();

    let luau_path = temp.path().join("output/Translations.lua");

    // Check generated Luau contains expected functions
    common::assert_file_contains(&luau_path, "function Translations.new");
    common::assert_file_contains(&luau_path, "function Translations:ui_buttons_buy");
    common::assert_file_contains(&luau_path, "function Translations:ui_buttons_sell");
    common::assert_file_contains(&luau_path, "function Translations:ui_labels_welcome");
}

/// Tests that type definitions are generated correctly
#[test]
fn test_build_generates_type_definitions() {
    let temp = common::create_test_project_with_translations();

    Command::cargo_bin("roblox-slang")
        .unwrap()
        .current_dir(&temp)
        .arg("build")
        .assert()
        .success();

    let types_path = temp.path().join("output/types/Translations.d.luau");

    // Check type definitions exist
    common::assert_file_contains(&types_path, "export type Translations");
    common::assert_file_contains(&types_path, "ui_buttons_buy");
}

/// Tests validate command with all checks enabled
#[test]
fn test_validate_command_all() {
    let temp = common::create_test_project_with_translations();

    Command::cargo_bin("roblox-slang")
        .unwrap()
        .current_dir(&temp)
        .arg("validate")
        .arg("--all")
        .assert()
        .success()
        .stdout(predicate::str::contains("Validating translations"))
        .stdout(predicate::str::contains(
            "Checking for missing translations",
        ))
        .stdout(predicate::str::contains("Checking for conflicts"))
        .stdout(predicate::str::contains("Translation Coverage Report"));
}

/// Tests that missing keys are detected correctly
#[test]
fn test_validate_missing_keys() {
    let temp = common::create_test_project();

    // Create English with more keys
    let en_json = r#"{
  "ui": {
    "button": "Buy",
    "label": "Welcome"
  }
}"#;
    fs::write(temp.path().join("translations/en.json"), en_json).unwrap();

    // Create Indonesian with fewer keys
    let id_json = r#"{
  "ui": {
    "button": "Beli"
  }
}"#;
    fs::write(temp.path().join("translations/id.json"), id_json).unwrap();

    Command::cargo_bin("roblox-slang")
        .unwrap()
        .current_dir(&temp)
        .arg("validate")
        .arg("--missing")
        .assert()
        .success()
        .stdout(predicate::str::contains("Missing in 'id'"))
        .stdout(predicate::str::contains("ui.label"));
}

/// Tests that coverage report is generated correctly
#[test]
fn test_validate_coverage() {
    let temp = common::create_test_project_with_translations();

    Command::cargo_bin("roblox-slang")
        .unwrap()
        .current_dir(&temp)
        .arg("validate")
        .arg("--coverage")
        .assert()
        .success()
        .stdout(predicate::str::contains("Translation Coverage Report"))
        .stdout(predicate::str::contains("Locale"))
        .stdout(predicate::str::contains("Coverage"))
        .stdout(predicate::str::contains("100.0%"));
}

/// Tests CSV import functionality
#[test]
fn test_import_csv() {
    let temp = common::create_test_project();

    // Create a CSV file
    let csv_content = r#"Source,Context,Key,en,id
"Buy","","ui.button.buy","Buy","Beli"
"Sell","","ui.button.sell","Sell","Jual"
"#;
    fs::write(temp.path().join("import.csv"), csv_content).unwrap();

    Command::cargo_bin("roblox-slang")
        .unwrap()
        .current_dir(&temp)
        .arg("import")
        .arg("import.csv")
        .assert()
        .success()
        .stdout(predicate::str::contains("Importing translations"));

    // Check translation files created
    common::assert_file_exists(&temp.path().join("translations/en.json"));
    common::assert_file_exists(&temp.path().join("translations/id.json"));
}

/// Tests migration from custom JSON format
#[test]
fn test_migrate_custom_json() {
    let temp = common::create_test_project();

    // Create custom format JSON
    let custom_json = r#"{
  "translations": {
    "en": {
      "ui_button": "Buy"
    }
  }
}"#;
    fs::write(temp.path().join("custom.json"), custom_json).unwrap();

    Command::cargo_bin("roblox-slang")
        .unwrap()
        .current_dir(&temp)
        .arg("migrate")
        .arg("--from")
        .arg("custom-json")
        .arg("--input")
        .arg("custom.json")
        .arg("--output")
        .arg("migrated.json")
        .assert()
        .success()
        .stdout(predicate::str::contains("Migration completed"));

    // Check output file created
    common::assert_file_exists(&temp.path().join("migrated.json"));
}

/// Tests migration from gettext .po format
#[test]
fn test_migrate_gettext() {
    let temp = common::create_test_project();

    // Create .po file
    let po_content = r#"msgid "ui.button"
msgstr "Buy"
"#;
    fs::write(temp.path().join("translations.po"), po_content).unwrap();

    Command::cargo_bin("roblox-slang")
        .unwrap()
        .current_dir(&temp)
        .arg("migrate")
        .arg("--from")
        .arg("gettext")
        .arg("--input")
        .arg("translations.po")
        .arg("--output")
        .arg("migrated.json")
        .assert()
        .success()
        .stdout(predicate::str::contains("Migration completed"));

    // Check output file created
    common::assert_file_exists(&temp.path().join("migrated.json"));
}

/// Tests complete end-to-end workflow from init to validate
#[test]
fn test_complete_workflow() {
    let temp = tempfile::TempDir::new().unwrap();

    // Step 1: Initialize
    Command::cargo_bin("roblox-slang")
        .unwrap()
        .current_dir(&temp)
        .arg("init")
        .assert()
        .success();

    // Step 2: Add translations
    let en_json = r#"{"ui": {"button": "Buy"}}"#;
    fs::write(temp.path().join("translations/en.json"), en_json).unwrap();

    let id_json = r#"{"ui": {"button": "Beli"}}"#;
    fs::write(temp.path().join("translations/id.json"), id_json).unwrap();

    // Step 3: Build
    Command::cargo_bin("roblox-slang")
        .unwrap()
        .current_dir(&temp)
        .arg("build")
        .assert()
        .success();

    // Step 4: Validate
    Command::cargo_bin("roblox-slang")
        .unwrap()
        .current_dir(&temp)
        .arg("validate")
        .arg("--all")
        .assert()
        .success();

    // Verify all output files exist
    common::assert_file_exists(&temp.path().join("output/Translations.lua"));
    common::assert_file_exists(&temp.path().join("output/types/Translations.d.luau"));
    common::assert_file_exists(&temp.path().join("output/roblox_upload.csv"));
}

/// Tests error handling when config file is missing
#[test]
fn test_error_handling_missing_config() {
    let temp = tempfile::TempDir::new().unwrap();

    Command::cargo_bin("roblox-slang")
        .unwrap()
        .current_dir(&temp)
        .arg("build")
        .assert()
        .failure()
        .stderr(predicate::str::contains("Failed to load config"));
}

/// Tests error handling when JSON is invalid
#[test]
fn test_error_handling_invalid_json() {
    let temp = common::create_test_project();

    // Create invalid JSON
    fs::write(temp.path().join("translations/en.json"), "{ invalid json }").unwrap();

    Command::cargo_bin("roblox-slang")
        .unwrap()
        .current_dir(&temp)
        .arg("build")
        .assert()
        .failure()
        .stderr(predicate::str::contains("Failed to parse"));
}
