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

**Examples:**

```bash
roblox-slang --version
roblox-slang --help
roblox-slang build --help
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

**Examples:**

```bash
# Basic initialization
roblox-slang init

# With overrides template
roblox-slang init --with-overrides
```

**Creates:**

- `slang-roblox.yaml` - Configuration file
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

**Examples:**

```bash
# One-time build
roblox-slang build

# Watch mode
roblox-slang build --watch
```

**Generates:**

- `output/Translations.lua` - Main module
- `output/types/Translations.d.luau` - Type definitions
- `output/roblox_upload.csv` - CSV for Roblox Cloud

**Exit Codes:**

- `0` - Success
- `1` - Build error

---

### `import`

Import translations from a Roblox CSV file.

**Usage:**

```bash
roblox-slang import <CSV_FILE>
```

**Arguments:**

- `<CSV_FILE>` - Path to the CSV file to import

**Examples:**

```bash
# Import from Roblox CSV
roblox-slang import existing_table.csv

# Import downloaded translations
roblox-slang import downloaded_translations.csv
```

**Output:**

Converts CSV to JSON format in `translations/` directory.

---

### `validate`

Validate translations for errors and inconsistencies.

**Usage:**

```bash
roblox-slang validate [OPTIONS]
```

**Options:**

- `--missing` - Check for missing translations
- `--unused` - Check for unused keys
- `--conflicts` - Check for conflicts
- `--coverage` - Show coverage report
- `--source <DIR>` - Source directory to scan for unused keys
- `--all` - Run all checks

**Examples:**

```bash
# Check for missing translations
roblox-slang validate --missing

# Check for unused keys
roblox-slang validate --unused --source src/

# Run all checks
roblox-slang validate --all

# Multiple checks
roblox-slang validate --missing --conflicts --coverage
```

**Checks:**

- **Missing translations** - Keys in base locale but not in others
- **Unused keys** - Defined but never used in source code
- **Conflicts** - Duplicate keys or conflicting definitions
- **Coverage** - Translation coverage percentage per locale

**Exit Codes:**

- `0` - No issues found
- `1` - Issues found

---

### `migrate`

Migrate translations from another format.

**Usage:**

```bash
roblox-slang migrate --from <FORMAT> --input <FILE> --output <FILE> [OPTIONS]
```

**Options:**

- `--from <FORMAT>` - Format to migrate from (custom-json, gettext)
- `--input <FILE>` - Input file path
- `--output <FILE>` - Output file path
- `--transform <TRANSFORM>` - Key transformation strategy (snake-to-camel, upper-to-lower, dot-to-nested, none)

**Examples:**

```bash
# Migrate from custom JSON
roblox-slang migrate --from custom-json --input old.json --output translations/en.json

# Migrate from gettext
roblox-slang migrate --from gettext --input translations.po --output translations/en.json

# With key transformation
roblox-slang migrate --from custom-json --input old.json --output new.json --transform snake-to-camel
```

**Supported Formats:**

- `custom-json` - Custom JSON format
- `gettext` - GNU gettext (.po files)

**Key Transformation Strategies:**

- `none` - Keep keys as-is
- `snake-to-camel` - Convert `ui_button_buy` → `uiButtonBuy`
- `upper-to-lower` - Convert `UI_BUTTON_BUY` → `ui_button_buy`
- `dot-to-nested` - Convert flat keys to nested structure

---

## Configuration File

All commands use `slang-roblox.yaml` in the current directory.

**Example:**

```yaml
base_locale: en
supported_locales:
  - en
  - es
  - id
input_directory: translations
output_directory: output
```

See [Configuration Guide](../guides/configuration.md) for complete reference.

## Exit Codes

All commands use standard exit codes:

| Code | Meaning |
|------|---------|
| 0 | Success |
| 1 | Error |

## Examples

### Basic Workflow

```bash
# 1. Initialize project
roblox-slang init

# 2. Create translations
# (edit translations/en.json, translations/es.json, etc.)

# 3. Validate
roblox-slang validate --all

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

### Import Workflow

```bash
# 1. Export from Roblox Cloud
# (download CSV from Creator Dashboard)

# 2. Import to Roblox Slang
roblox-slang import roblox_export.csv

# 3. Build
roblox-slang build
```

### Validation Workflow

```bash
# Check for missing translations
roblox-slang validate --missing

# Check for unused keys in source code
roblox-slang validate --unused --source src/

# Full validation
roblox-slang validate --all --source src/
```

## Troubleshooting

### Command Not Found

```bash
# Check installation
which roblox-slang

# Reinstall
rokit install  # or aftman install, foreman install
```

### Build Fails

```bash
# Validate first
roblox-slang validate --all

# Check config
cat slang-roblox.yaml

# Check translation files
ls -la translations/
```

### Import Fails

```bash
# Check CSV format
head roblox_export.csv

# Verify file exists
ls -la roblox_export.csv
```

## See Also

- [Getting Started](../getting-started.md) - Initial setup
- [Configuration](../guides/configuration.md) - Config file reference
- [Roblox Cloud Integration](../guides/roblox-cloud.md) - Upload to Roblox Cloud
- [Rojo Integration](../integration/rojo.md) - Use with Rojo
