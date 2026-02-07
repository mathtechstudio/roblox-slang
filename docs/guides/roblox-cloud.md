# Roblox Cloud Integration

Upload translations to Roblox Cloud Localization to access Roblox's localization features.

## Overview

By uploading your translations to Roblox Cloud, you gain access to Roblox's localization features:

- **Automatic Text Capture (ATC)** - Roblox automatically captures UI strings from your game
- **Automatic translation** - Roblox AI translates strings to supported languages  
- **Translator Portal** - Collaborate with human translators
- **Analytics** - Track translation coverage and usage
- **Multi-game sync** - Share translations across multiple games

**Important:** Roblox Slang generates CSV files compatible with Roblox Cloud format. The actual translation features (ATC, automatic translation) are provided by Roblox, not by Roblox Slang.

## Prerequisites

1. A published Roblox game
2. Access to [Roblox Creator Dashboard](https://create.roblox.com/)
3. Localization Table created for your game

## Setup

### Step 1: Enable Localization in Your Game

1. Open your game in Roblox Studio
2. Go to **Home** → **Game Settings** → **Localization**
3. Enable **Use Translated Content**
4. Select supported languages

### Step 2: Create Localization Table

1. Go to [Roblox Creator Dashboard](https://create.roblox.com/)
2. Select your game
3. Navigate to **Localization** → **Localization Table**
4. Click **Create Table** (if not exists)

### Step 3: Build Translations

Generate CSV file for upload:

```bash
roblox-slang build
```

This creates `output/roblox_upload.csv` in Roblox Cloud format.

## CSV Format

Roblox Slang generates CSV with this structure:

```csv
Source,Context,Key,en,es,id
"Buy","","ui.buttons.buy","Buy","Comprar","Beli"
"Sell","","ui.buttons.sell","Sell","Vender","Jual"
"Hello, {name}!","","ui.messages.greeting","Hello, {name}!","¡Hola, {name}!","Halo, {name}!"
```

**Columns:**

- **Source** - English text (used by Roblox ATC for matching)
- **Context** - Disambiguation context (usually empty for manual keys)
- **Key** - Dot notation key for programmatic lookup
- **Locale columns** - One column per supported locale

## Upload to Roblox Cloud

### Method 1: Manual Upload (Recommended for First Time)

1. Go to [Creator Dashboard](https://create.roblox.com/)
2. Select your game
3. Navigate to **Localization** → **Localization Table**
4. Click **Update** → **Upload CSV**
5. Select `output/roblox_upload.csv`
6. Review changes
7. Click **Upload**

### Method 2: Open Cloud API (Coming Soon)

```bash
# Future feature
roblox-slang upload --api-key YOUR_API_KEY
```

## Roblox's Automatic Translation

After uploading your CSV, you can enable Roblox's automatic translation feature:

### Enable Automatic Translation

1. Go to Creator Dashboard → **Localization** → **Languages**
2. For each language, enable:
   - **Experience Information** (name and description)
   - **Experience Strings & Products** (in-game text)

### How It Works

- Roblox AI automatically translates blank entries in your localization table
- Uses per-character quota (initial + monthly)
- Translations improve over time
- You can lock entries to prevent automatic updates

**Note:** This is a Roblox Cloud feature, not provided by Roblox Slang.

## Roblox's Automatic Text Capture (ATC)

Roblox can automatically capture UI strings from your game:

### Enable ATC

1. Go to Creator Dashboard → **Localization** → **Settings**
2. Enable **Capture text from experience UI while users play**

### What ATC Captures

✅ Captures:

- `TextLabel`, `TextButton` with `AutoLocalize` enabled
- `TextBox.PlaceholderText`
- GUI text objects

❌ Does NOT capture:

- Default Roblox leaderboards and chat
- Player-owned items/tools
- Images with embedded text
- Badge/Pass names from platform

**Note:** This is a Roblox Cloud feature, not provided by Roblox Slang.

## Translator Portal

Collaborate with human translators via Roblox:

### Invite Translators

1. Go to **Localization** → **Translator Portal**
2. Click **Invite Translators**
3. Enter Roblox usernames
4. Assign languages

### Translator Workflow

Translators can:

- View all translations
- Edit translations
- Add context notes
- Mark translations as reviewed

### Download Updated Translations

After translators make changes:

```bash
# Future feature
roblox-slang pull --strategy=merge
```

This downloads updated translations and merges with local files.

## Bi-Directional Sync

Keep local translations in sync with Roblox Cloud:

### Upload Local Changes

```bash
roblox-slang build
# Upload output/roblox_upload.csv manually
```

### Download Cloud Changes

```bash
# Future feature
roblox-slang pull
```

### Merge Strategy

Choose how to handle conflicts:

```bash
# Keep local changes
roblox-slang pull --strategy=local

# Keep cloud changes
roblox-slang pull --strategy=cloud

# Merge both (manual conflict resolution)
roblox-slang pull --strategy=merge
```

## Best Practices

### 1. Use Descriptive Source Text

Roblox's automatic translation works better with natural language:

❌ **Bad:**

```json
{
  "btn_buy": "Buy"  // Code-like key
}
```

✅ **Good:**

```json
{
  "ui.buttons.buy": "Buy"  // Natural source text
}
```

### 2. Provide Context for Ambiguous Terms

Use context field for disambiguation:

```csv
Source,Context,Key,en,es
"Close","ui.buttons.close","ui.buttons.close","Close","Cerrar"
"Close","adjective.near","adjective.close","Close","Cerca"
```

### 3. Lock Manual Translations

Prevent Roblox's automatic translation from overwriting manual translations:

1. In Creator Dashboard, select rows
2. Click **Actions** → **Lock Translation**
3. Locked rows won't be updated by automatic translation

### 4. Regular Sync

Sync regularly to get translator updates:

```bash
# Weekly sync recommended
roblox-slang pull --strategy=merge
roblox-slang build
# Upload to cloud
```

### 5. Test in Game

Always test translations in-game before publishing:

```lua
-- Test all locales
local locales = {"en", "es", "id", "pt", "de"}
for _, locale in ipairs(locales) do
    local t = Translations.new(locale)
    print(locale, t.ui.buttons.buy())
end
```

## Analytics

Track translation usage via Roblox Cloud:

### Enable Analytics

In `slang-roblox.yaml`:

```yaml
analytics:
  enabled: true
  track_usage: true
```

### View Analytics

1. Go to Creator Dashboard
2. Navigate to **Localization** → **Analytics**
3. View:
   - Most used translations
   - Missing translations
   - Locale distribution

### Optimize Based on Analytics

- Remove unused translations
- Prioritize translating high-usage keys
- Add missing translations for popular locales

## Troubleshooting

### Issue: CSV Upload Fails

**Cause:** Invalid CSV format

**Solution:**

1. Validate CSV: `roblox-slang validate`
2. Check for:
   - Special characters (quotes, commas)
   - Missing columns
   - Duplicate keys

### Issue: Automatic Translation Not Working

**Cause:** Feature not enabled or quota exceeded

**Solution:**

1. Check **Languages** tab in Creator Dashboard
2. Verify automatic translation is enabled
3. Check quota usage

### Issue: Translations Not Appearing in Game

**Cause:** Localization not enabled or table not published

**Solution:**

1. Check Game Settings → Localization is enabled
2. Verify Localization Table is published
3. Restart game servers

### Issue: Parameters Not Working

**Cause:** Parameter syntax mismatch

**Solution:**
Use Roblox parameter syntax:

```lua
-- ✅ Correct
translator:FormatByKey("greeting", { name = "Player" })

-- ❌ Wrong
translator:FormatByKey("greeting", "Player")
```

## Supported Languages

Roblox Cloud supports automatic translation for these languages:

| Code | Language |
|------|----------|
| en | English |
| es | Spanish |
| pt | Portuguese |
| de | German |
| fr | French |
| it | Italian |
| ja | Japanese |
| ko | Korean |
| zh-CN | Chinese (Simplified) |
| zh-TW | Chinese (Traditional) |
| id | Indonesian |
| ru | Russian |
| th | Thai |
| tr | Turkish |
| vi | Vietnamese |
| pl | Polish |
| ar | Arabic |

For complete list, see [Roblox Language Codes](https://create.roblox.com/docs/production/localization/language-codes).

## Migration from Manual Localization

If you have existing manual localization:

### Step 1: Export Existing Table

1. Go to Creator Dashboard → Localization Table
2. Click **Export** → **Download CSV**
3. Save as `existing_translations.csv`

### Step 2: Import to Roblox Slang

```bash
roblox-slang migrate csv existing_translations.csv
```

This converts CSV to JSON format in `translations/` directory.

### Step 3: Build and Upload

```bash
roblox-slang build
# Upload output/roblox_upload.csv
```

## Advanced: Open Cloud API

For automated workflows, use Open Cloud API (coming soon):

### Setup API Key

1. Go to [Creator Dashboard](https://create.roblox.com/)
2. Navigate to **Open Cloud** → **API Keys**
3. Create new API key with **Localization** permissions
4. Save API key securely

### Upload via API

```bash
# Future feature
export ROBLOX_API_KEY="your_api_key"
roblox-slang upload --auto-translate
```

### Download via API

```bash
# Future feature
roblox-slang pull --strategy=merge
```

## See Also

- [Getting Started](../getting-started.md) - Initial setup
- [Configuration](configuration.md) - Config options
- [Rojo Integration](../integration/rojo.md) - Use with Rojo
- [Roblox Localization Docs](https://create.roblox.com/docs/production/localization) - Official documentation
- [Automatic Translations](https://create.roblox.com/docs/production/localization/automatic-translations) - Roblox's automatic translation feature
- [Open Cloud API](https://create.roblox.com/docs/cloud/open-cloud/localization-api) - API reference
