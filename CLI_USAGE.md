# CLI Usage

## roblox-slang

Type-safe internationalization code generator for Roblox games.

### USAGE

```
roblox-slang [OPTIONS] <SUBCOMMAND>
```

### FLAGS

```
-h, --help       Prints help information
-V, --version    Prints version information
```

### SUBCOMMANDS

```
init        Initialize a new Roblox Slang project
build       Build translations and generate Luau code
import      Import translations from a Roblox CSV file
validate    Validate translations for errors and inconsistencies
migrate     Migrate translations from another format
help        Prints this message or the help of the given subcommand(s)
```

---

## roblox-slang init

Initialize a new Roblox Slang project.

### USAGE

```
roblox-slang init [FLAGS]
```

### FLAGS

```
    --with-overrides    Create overrides.yaml template
-h, --help              Prints help information
```

### EXAMPLES

```bash
# Basic initialization
roblox-slang init

# With overrides template
roblox-slang init --with-overrides
```

### CREATES

- `slang-roblox.yaml` - Configuration file
- `translations/` - Empty directory for translation files
- `overrides.yaml` - (if --with-overrides) Override template

---

## roblox-slang build

Build translations and generate Luau code.

### USAGE

```
roblox-slang build [FLAGS]
```

### FLAGS

```
-w, --watch    Watch mode (auto-rebuild on changes)
-h, --help     Prints help information
```

### EXAMPLES

```bash
# One-time build
roblox-slang build

# Watch mode
roblox-slang build --watch
```

### GENERATES

- `output/Translations.lua` - Main module
- `output/types/Translations.d.luau` - Type definitions
- `output/roblox_upload.csv` - CSV for Roblox Cloud

### EXIT CODES

- `0` - Success
- `1` - Build error

---

## roblox-slang import

Import translations from a Roblox CSV file.

### USAGE

```
roblox-slang import <CSV_FILE>
```

### ARGS

```
<CSV_FILE>    Path to the CSV file to import
```

### FLAGS

```
-h, --help    Prints help information
```

### EXAMPLES

```bash
# Import from Roblox CSV
roblox-slang import existing_table.csv

# Import downloaded translations
roblox-slang import downloaded_translations.csv
```

### OUTPUT

Converts CSV to JSON format in `translations/` directory.

---

## roblox-slang validate

Validate translations for errors and inconsistencies.

### USAGE

```
roblox-slang validate [FLAGS] [OPTIONS]
```

### FLAGS

```
    --missing      Check for missing translations
    --unused       Check for unused keys
    --conflicts    Check for conflicts
    --coverage     Show coverage report
    --all          Run all checks
-h, --help         Prints help information
```

### OPTIONS

```
    --source <DIR>    Source directory to scan for unused keys
```

### EXAMPLES

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

### CHECKS

- **Missing translations** - Keys in base locale but not in others
- **Unused keys** - Defined but never used in source code
- **Conflicts** - Duplicate keys or conflicting definitions
- **Coverage** - Translation coverage percentage per locale

### EXIT CODES

- `0` - No issues found
- `1` - Issues found

---

## roblox-slang migrate

Migrate translations from another format.

### USAGE

```
roblox-slang migrate [OPTIONS]
```

### OPTIONS

```
    --from <FORMAT>           Format to migrate from
                              [possible values: custom-json, gettext]
    --input <FILE>            Input file path
    --output <FILE>           Output file path
    --transform <TRANSFORM>   Key transformation strategy
                              [possible values: snake-to-camel, upper-to-lower, 
                               dot-to-nested, none]
-h, --help                    Prints help information
```

### EXAMPLES

```bash
# Migrate from custom JSON
roblox-slang migrate --from custom-json --input old.json --output translations/en.json

# Migrate from gettext
roblox-slang migrate --from gettext --input translations.po --output translations/en.json

# With key transformation
roblox-slang migrate --from custom-json --input old.json --output new.json --transform snake-to-camel
```

### SUPPORTED FORMATS

- `custom-json` - Custom JSON format
- `gettext` - GNU gettext (.po files)

### KEY TRANSFORMATION STRATEGIES

- `none` - Keep keys as-is
- `snake-to-camel` - Convert `ui_button_buy` → `uiButtonBuy`
- `upper-to-lower` - Convert `UI_BUTTON_BUY` → `ui_button_buy`
- `dot-to-nested` - Convert flat keys to nested structure

---

## Configuration File

All commands use `slang-roblox.yaml` in the current directory.

### Example Configuration

```yaml
base_locale: en
supported_locales:
  - en
  - es
  - id
input_directory: translations
output_directory: output
```

See [Configuration Guide](docs/guides/configuration.md) for complete reference.

---

## Environment Variables

Set environment variables for configuration:

```bash
# Enable verbose logging
export RUST_LOG=debug

# Run command
roblox-slang build
```

### Supported Variables

- `RUST_LOG` - Set log level (error, warn, info, debug, trace)

---

## Exit Codes

All commands use standard exit codes:

| Code | Meaning |
|------|---------|
| 0 | Success |
| 1 | Error |

---

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

---

## See Also

- [README](README.md) - Project overview
- [Getting Started](docs/getting-started.md) - Initial setup
- [CLI Reference](docs/reference/cli-reference.md) - Detailed command reference
- [Configuration](docs/guides/configuration.md) - Config file reference
