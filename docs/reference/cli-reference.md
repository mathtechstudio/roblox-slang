# CLI Reference

Complete reference for all Roblox Slang CLI commands.

## Global Options

Available for all commands:

```bash
roblox-slang [OPTIONS] <COMMAND>
```

**Options:**

- `-h, --help` - Show help information
- `-V, --version` - Show version information
- `-v, --verbose` - Enable verbose output
- `-q, --quiet` - Suppress non-error output

**Examples:**

```bash
roblox-slang --version
roblox-slang --help
roblox-slang build --verbose
```

## Commands

### `init`

Initialize a new Roblox Slang project.

**Usage:**

```bash
roblox-slang init [OPTIONS]
```

**Options:**

- `--with-overrides` - Create overrides.yaml template
- `--force` - Overwrite existing files

**Examples:**

```bash
# Basic initialization
roblox-slang init

# With overrides template
roblox-slang init --with-overrides

# Force overwrite
roblox-slang init --force
```

**Creates:**

- `slang-roblox.yaml` - Configuration file with comments
- `translations/` - Empty directory for translation files
- `overrides.yaml` - (if --with-overrides) Override template

---

### `build`

Build translations and generate Luau code.

**Usage:**

```bash
roblox-slang build [OPTIONS]
```

**Options:**

- `-w, --watch` - Watch mode (auto-rebuild on changes)
- `-c, --config <PATH>` - Custom config file path (default: slang-roblox.yaml)
- `--no-types` - Skip type definitions generation
- `--no-csv` - Skip CSV generation

**Examples:**

```bash
# One-time build
roblox-slang build

# Watch mode
roblox-slang build --watch

# Custom config
roblox-slang build --config custom-config.yaml

# Skip type definitions
roblox-slang build --no-types
```

**Generates:**

- `output/Translations.lua` - Main module
- `output/types/Translations.d.luau` - Type definitions (unless --no-types)
- `output/roblox_upload.csv` - CSV for Roblox Cloud (unless --no-csv)

**Exit Codes:**

- `0` - Success
- `1` - Build error (syntax, validation, etc.)
- `2` - Configuration error

---

### `validate`

Validate translations for errors and inconsistencies.

**Usage:**

```bash
roblox-slang validate [OPTIONS]
```

**Options:**

- `-c, --config <PATH>` - Custom config file path
- `--strict` - Enable strict validation (warnings as errors)
- `--fix` - Auto-fix issues where possible

**Examples:**

```bash
# Basic validation
roblox-slang validate

# Strict mode
roblox-slang validate --strict

# Auto-fix issues
roblox-slang validate --fix
```

**Checks:**

- Missing translations (keys in base locale but not in others)
- Unused keys (defined but never used)
- Duplicate keys
- Invalid parameter syntax
- Plural form completeness
- Format specifier validity

**Output:**

```yaml
✓ Validated en.json (173 keys)
✓ Validated es.json (173 keys)
✗ Validated id.json (170 keys)

Issues found:
  Missing translations in id:
    - ui.buttons.new
    - ui.buttons.delete
    - shop.messages.purchaseSuccess

  Unused keys:
    - old.deprecated.key (defined in en, es, id)

Summary: 3 missing, 1 unused, 0 conflicts
```

**Exit Codes:**

- `0` - No issues found
- `1` - Issues found (warnings)
- `2` - Critical issues found (errors)

---

### `migrate`

Import translations from other formats.

**Usage:**

```bash
roblox-slang migrate <FORMAT> <INPUT> [OPTIONS]
```

**Formats:**

- `csv` - Roblox CSV format
- `json` - Custom JSON format
- `gettext` - GNU gettext (.po files)

**Options:**

- `-o, --output <DIR>` - Output directory (default: translations/)
- `--base-locale <LOCALE>` - Base locale (default: en)
- `--transform <STRATEGY>` - Key transformation strategy

**Examples:**

```bash
# Import from Roblox CSV
roblox-slang migrate csv existing_table.csv

# Import from gettext
roblox-slang migrate gettext translations.po

# Custom output directory
roblox-slang migrate csv input.csv --output locales/

# Key transformation
roblox-slang migrate json old.json --transform snake_to_dot
```

**Key Transformation Strategies:**

- `none` - Keep keys as-is
- `snake_to_dot` - Convert `ui_button_buy` → `ui.button.buy`
- `camel_to_dot` - Convert `uiButtonBuy` → `ui.button.buy`
- `flat_to_nested` - Convert flat keys to nested structure

**Output:**

```yaml
→ Importing from CSV...
✓ Parsed 173 keys
✓ Detected locales: en, es, id
✓ Transformed keys (snake_to_dot)
✓ Generated translations/en.json
✓ Generated translations/es.json
✓ Generated translations/id.json

Migration complete! Run 'roblox-slang build' to generate code.
```

---

### `watch`

Watch translation files and auto-rebuild on changes.

**Usage:**

```bash
roblox-slang watch [OPTIONS]
```

**Options:**

- `-c, --config <PATH>` - Custom config file path
- `--debounce <MS>` - Debounce delay in milliseconds (default: 300)

**Examples:**

```bash
# Start watch mode
roblox-slang watch

# Custom debounce
roblox-slang watch --debounce 500
```

**Note:** This is equivalent to `roblox-slang build --watch`.

**Output:**

```bash
→ Watching for changes...
  Config: slang-roblox.yaml
  Input: translations/
  Output: output/

✓ Initial build complete (173 keys)

[14:30:15] File changed: translations/en.json
[14:30:15] Rebuilding...
✓ Build complete (174 keys) [52ms]

[14:30:45] File changed: translations/es.json
[14:30:45] Rebuilding...
✓ Build complete (174 keys) [48ms]
```

Press `Ctrl+C` to stop watching.

---

## Configuration File

All commands use `slang-roblox.yaml` by default. Override with `-c, --config`:

```bash
roblox-slang build --config production.yaml
roblox-slang validate --config staging.yaml
```

## Environment Variables

Set environment variables for configuration:

```bash
# Base locale
export SLANG_BASE_LOCALE=es

# Output directory
export SLANG_OUTPUT_DIR=dist/translations

# Enable verbose logging
export SLANG_VERBOSE=1

# Run command
roblox-slang build
```

**Supported Variables:**

- `SLANG_BASE_LOCALE` - Override base_locale
- `SLANG_OUTPUT_DIR` - Override output_directory
- `SLANG_INPUT_DIR` - Override input_directory
- `SLANG_VERBOSE` - Enable verbose output (1 or true)
- `SLANG_QUIET` - Suppress output (1 or true)

## Exit Codes

All commands use standard exit codes:

| Code | Meaning |
|------|---------|
| 0 | Success |
| 1 | General error (validation, build, etc.) |
| 2 | Configuration error |
| 3 | File not found |
| 4 | Permission denied |
| 130 | Interrupted (Ctrl+C) |

**Usage in Scripts:**

```bash
#!/bin/bash

roblox-slang build
if [ $? -eq 0 ]; then
    echo "Build successful"
    # Deploy...
else
    echo "Build failed"
    exit 1
fi
```

## Logging

Control log output with verbosity flags:

```bash
# Normal output
roblox-slang build

# Verbose output (debug info)
roblox-slang build --verbose

# Quiet output (errors only)
roblox-slang build --quiet
```

**Log Levels:**

- **Error** - Critical errors (always shown)
- **Warning** - Non-critical issues (shown by default)
- **Info** - Progress information (shown by default)
- **Debug** - Detailed debug info (shown with --verbose)

## Shell Completion

Generate shell completion scripts:

```bash
# Bash
roblox-slang completions bash > ~/.local/share/bash-completion/completions/roblox-slang

# Zsh
roblox-slang completions zsh > ~/.zfunc/_roblox-slang

# Fish
roblox-slang completions fish > ~/.config/fish/completions/roblox-slang.fish

# PowerShell
roblox-slang completions powershell > roblox-slang.ps1
```

## Examples

### Basic Workflow

```bash
# 1. Initialize project
roblox-slang init

# 2. Create translations
# (edit translations/en.json, translations/es.json, etc.)

# 3. Validate
roblox-slang validate

# 4. Build
roblox-slang build

# 5. Use in game
# (copy output/Translations.lua to your project)
```

### Development Workflow

```bash
# Start watch mode
roblox-slang build --watch

# In another terminal, edit translations
# Watch mode auto-rebuilds on save
```

### CI/CD Workflow

```bash
#!/bin/bash
set -e

# Validate translations
roblox-slang validate --strict

# Build
roblox-slang build

# Run tests
# ...

# Deploy
# ...
```

### Migration Workflow

```bash
# 1. Export from Roblox Cloud
# (download CSV from Creator Dashboard)

# 2. Import to Roblox Slang
roblox-slang migrate csv roblox_export.csv

# 3. Build
roblox-slang build

# 4. Compare outputs
diff output/roblox_upload.csv roblox_export.csv
```

## Troubleshooting

### Command Not Found

```bash
# Check installation
which roblox-slang

# Reinstall
rokit install  # or aftman install, foreman install
```

### Permission Denied

```bash
# Check file permissions
ls -la slang-roblox.yaml

# Fix permissions
chmod 644 slang-roblox.yaml
chmod 755 translations/
```

### Build Fails

```bash
# Validate first
roblox-slang validate

# Check config
cat slang-roblox.yaml

# Verbose output
roblox-slang build --verbose
```

## See Also

- [Getting Started](../getting-started.md) - Initial setup
- [Configuration](../guides/configuration.md) - Config file reference
- [Roblox Cloud Integration](../guides/roblox-cloud.md) - Upload to Roblox Cloud
- [Rojo Integration](../integration/rojo.md) - Use with Rojo
