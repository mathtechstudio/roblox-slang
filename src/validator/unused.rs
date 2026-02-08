use anyhow::Result;
use std::fs;
use std::path::Path;

/// Detect unused translation keys by scanning source files
pub fn detect_unused_keys(translation_keys: &[String], source_dir: &Path) -> Result<Vec<String>> {
    if !source_dir.exists() {
        return Ok(Vec::new());
    }

    // Read all Lua files
    let lua_files = find_lua_files(source_dir)?;

    // Read all file contents
    let mut all_content = String::new();
    for file in lua_files {
        if let Ok(content) = fs::read_to_string(&file) {
            all_content.push_str(&content);
            all_content.push('\n');
        }
    }

    // Check which keys are NOT found in any file
    let mut unused = Vec::new();
    for key in translation_keys {
        // Check if key appears in any form:
        // - As string literal: "ui.buttons.buy"
        // - As method call: t.ui.buttons.buy()
        // - As nested access: t.ui.buttons.buy

        let key_variations = [
            format!("\"{}\"", key), // String literal
            format!("'{}'", key),   // Single quote string
            key.to_string(),        // Direct key
        ];

        let found = key_variations
            .iter()
            .any(|variation| all_content.contains(variation));

        if !found {
            unused.push(key.clone());
        }
    }

    Ok(unused)
}

/// Recursively find all .lua and .luau files
fn find_lua_files(dir: &Path) -> Result<Vec<std::path::PathBuf>> {
    let mut lua_files = Vec::new();

    if dir.is_file() {
        if let Some(ext) = dir.extension() {
            if ext == "lua" || ext == "luau" {
                lua_files.push(dir.to_path_buf());
            }
        }
        return Ok(lua_files);
    }

    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                lua_files.extend(find_lua_files(&path)?);
            } else if let Some(ext) = path.extension() {
                if ext == "lua" || ext == "luau" {
                    lua_files.push(path);
                }
            }
        }
    }

    Ok(lua_files)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::TempDir;

    #[test]
    fn test_detect_unused_keys() {
        let temp_dir = TempDir::new().unwrap();

        // Create a test Lua file
        let lua_file = temp_dir.path().join("test.lua");
        let mut file = fs::File::create(&lua_file).unwrap();
        writeln!(file, "local text = t.ui.buttons.buy()").unwrap();
        writeln!(file, "print(\"ui.labels.welcome\")").unwrap();

        let keys = vec![
            "ui.buttons.buy".to_string(),
            "ui.labels.welcome".to_string(),
            "ui.unused.key".to_string(),
        ];

        let unused = detect_unused_keys(&keys, temp_dir.path()).unwrap();

        assert_eq!(unused.len(), 1);
        assert_eq!(unused[0], "ui.unused.key");
    }

    #[test]
    fn test_find_lua_files() {
        let temp_dir = TempDir::new().unwrap();

        // Create test files
        fs::File::create(temp_dir.path().join("test1.lua")).unwrap();
        fs::File::create(temp_dir.path().join("test2.luau")).unwrap();
        fs::File::create(temp_dir.path().join("test.txt")).unwrap();

        let lua_files = find_lua_files(temp_dir.path()).unwrap();

        assert_eq!(lua_files.len(), 2);
    }
}
