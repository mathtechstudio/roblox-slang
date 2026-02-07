# String Interpolation

Use parameters in translations for dynamic content.

## Basic Interpolation

### Simple Parameters

```json
{
  "greeting": "Hello, {name}!",
  "welcome": "Welcome back, {playerName}!"
}
```

**Usage:**

```lua
local t = Translations.new("en")

print(t.greeting({ name = "Player1" }))
-- "Hello, Player1!"

print(t.welcome({ playerName = "JohnDoe" }))
-- "Welcome back, JohnDoe!"
```

### Multiple Parameters

```json
{
  "trade": "{player1} traded {item} to {player2}"
}
```

**Usage:**

```lua
print(t.trade({
    player1 = "Alice",
    player2 = "Bob",
    item = "Diamond Sword"
}))
-- "Alice traded Diamond Sword to Bob"
```

## Format Specifiers

Control how parameters are formatted using type specifiers.

### Integer Format (`:int`)

Format numbers as integers (no decimals).

```json
{
  "score": "Score: {points:int}",
  "level": "Level {level:int}"
}
```

**Usage:**

```lua
print(t.score({ points = 1234 }))
-- "Score: 1234"

print(t.score({ points = 1234.56 }))
-- "Score: 1234" (decimals removed)
```

### Fixed Decimal Format (`:fixed`)

Format numbers with 2 decimal places.

```json
{
  "price": "Price: ${amount:fixed}",
  "balance": "Balance: {coins:fixed} coins"
}
```

**Usage:**

```lua
print(t.price({ amount = 99.99 }))
-- "Price: $99.99"

print(t.price({ amount = 100 }))
-- "Price: $100.00"

print(t.balance({ coins = 1234.5 }))
-- "Balance: 1234.50 coins"
```

### Number Format (`:num`)

Format numbers with thousands separators.

```json
{
  "population": "Population: {count:num}",
  "views": "{views:num} views"
}
```

**Usage:**

```lua
print(t.population({ count = 1234567 }))
-- "Population: 1,234,567"

print(t.views({ views = 1000000 }))
-- "1,000,000 views"
```

### DateTime Format (`:datetime`)

Format timestamps as readable dates.

```json
{
  "lastSeen": "Last seen: {time:datetime}",
  "joined": "Joined {date:datetime}"
}
```

**Usage:**

```lua
print(t.lastSeen({ time = os.time() }))
-- "Last seen: 2025-02-07 14:30:00"

print(t.joined({ date = 1704067200 }))
-- "Joined 2024-01-01 00:00:00"
```

### Translate Format (`:translate`)

Translate nested keys.

```json
{
  "status": {
    "online": "Online",
    "offline": "Offline",
    "away": "Away"
  },
  "playerStatus": "{name} is {status:translate}"
}
```

**Usage:**

```lua
print(t.playerStatus({
    name = "Player1",
    status = "status.online"
}))
-- "Player1 is Online"
```

## Advanced Usage

### Combining with Pluralization

```json
{
  "itemsOwned": {
    "one": "{name} owns {count} item worth ${value:fixed}",
    "other": "{name} owns {count} items worth ${value:fixed}"
  }
}
```

**Usage:**

```lua
print(t.itemsOwned(1, {
    name = "Player1",
    value = 99.99
}))
-- "Player1 owns 1 item worth $99.99"

print(t.itemsOwned(5, {
    name = "Player1",
    value = 499.95
}))
-- "Player1 owns 5 items worth $499.95"
```

### Optional Parameters

Parameters can be optional with default values:

```json
{
  "greeting": "Hello{name}!",
  "farewell": "Goodbye{name}!"
}
```

**Usage:**

```lua
print(t.greeting({ name = ", Player1" }))
-- "Hello, Player1!"

print(t.greeting({}))
-- "Hello!" (no name provided)
```

### Nested Parameters

```json
{
  "achievement": "{player} unlocked {achievement.name} ({achievement.rarity})"
}
```

**Usage:**

```lua
print(t.achievement({
    player = "Player1",
    achievement = {
        name = "First Win",
        rarity = "Rare"
    }
}))
-- "Player1 unlocked First Win (Rare)"
```

## Format Specifier Reference

| Specifier | Description | Example Input | Example Output |
|-----------|-------------|---------------|----------------|
| (none) | Plain string | `"Player1"` | `"Player1"` |
| `:int` | Integer | `1234.56` | `"1234"` |
| `:fixed` | 2 decimals | `99.9` | `"99.90"` |
| `:num` | Thousands separator | `1234567` | `"1,234,567"` |
| `:datetime` | Date/time | `1704067200` | `"2024-01-01 00:00:00"` |
| `:translate` | Nested translation | `"status.online"` | `"Online"` |

## Best Practices

### 1. Use Descriptive Parameter Names

**Bad:**

```json
{
  "message": "{a} sent {b} to {c}"
}
```

**Good:**

```json
{
  "message": "{sender} sent {item} to {receiver}"
}
```

### 2. Use Format Specifiers

**Bad:**

```json
{
  "price": "Price: ${amount}"  // No format specifier
}
```

**Good:**

```json
{
  "price": "Price: ${amount:fixed}"  // Always 2 decimals
}
```

### 3. Keep Parameters Consistent

If a parameter appears in multiple translations, use the same name:

✅ **Good:**

```json
{
  "greeting": "Hello, {name}!",
  "farewell": "Goodbye, {name}!",
  "welcome": "Welcome, {name}!"
}
```

### 4. Document Required Parameters

Add comments in translation files:

```json
{
  // Parameters: name (string), level (number)
  "playerInfo": "{name} - Level {level:int}"
}
```

### 5. Validate Parameters

Use validation to catch missing parameters:

```bash
roblox-slang validate
```

This checks:

- All parameters are provided
- Parameter types match format specifiers
- No unused parameters

## Common Patterns

### Player Information

```json
{
  "playerCard": "{name} | Level {level:int} | {coins:num} coins"
}
```

**Usage:**

```lua
print(t.playerCard({
    name = "Player1",
    level = 42,
    coins = 12345
}))
-- "Player1 | Level 42 | 12,345 coins"
```

### Shop Items

```json
{
  "itemPrice": "{itemName} - ${price:fixed}",
  "itemStock": "{itemName} ({stock:int} in stock)"
}
```

**Usage:**

```lua
print(t.itemPrice({
    itemName = "Diamond Sword",
    price = 99.99
}))
-- "Diamond Sword - $99.99"

print(t.itemStock({
    itemName = "Health Potion",
    stock = 50
}))
-- "Health Potion (50 in stock)"
```

### Time Remaining

```json
{
  "timeLeft": "{minutes:int}:{seconds:int} remaining",
  "cooldown": "Cooldown: {hours:int}h {minutes:int}m"
}
```

**Usage:**

```lua
print(t.timeLeft({
    minutes = 5,
    seconds = 30
}))
-- "5:30 remaining"

print(t.cooldown({
    hours = 2,
    minutes = 15
}))
-- "Cooldown: 2h 15m"
```

### Achievements

```json
{
  "unlocked": "🏆 {name} unlocked: {achievement}",
  "progress": "{achievement}: {current:int}/{total:int}"
}
```

**Usage:**

```lua
print(t.unlocked({
    name = "Player1",
    achievement = "First Win"
}))
-- "🏆 Player1 unlocked: First Win"

print(t.progress({
    achievement = "Collect 100 Coins",
    current = 75,
    total = 100
}))
-- "Collect 100 Coins: 75/100"
```

## Error Handling

### Missing Parameters

If a parameter is missing, Roblox Slang will:

1. Log a warning (if analytics enabled)
2. Show the parameter name in output

```lua
print(t.greeting({}))  -- Missing {name}
-- Output: "Hello, {name}!" (parameter not replaced)
-- Console: "Warning: Missing parameter 'name' in 'greeting'"
```

### Type Mismatches

If parameter type doesn't match format specifier:

```lua
print(t.score({ points = "abc" }))  -- String instead of number
-- Output: "Score: {points:int}" (not replaced)
-- Console: "Warning: Parameter 'points' must be a number"
```

### Enable Validation

Enable parameter validation in config:

```yaml
analytics:
  enabled: true
  track_missing: true
```

## Performance Tips

### 1. Cache Translation Instances

```lua
-- Bad: Creates new instance every time
local function showMessage()
    local t = Translations.new("en")
    print(t.greeting({ name = "Player" }))
end

-- Good: Reuse instance
local t = Translations.new("en")
local function showMessage()
    print(t.greeting({ name = "Player" }))
end
```

### 2. Avoid Complex Nested Parameters

```lua
-- Bad: Deep nesting
print(t.message({
    player = {
        info = {
            name = {
                first = "John",
                last = "Doe"
            }
        }
    }
}))

-- Good: Flat structure
print(t.message({
    firstName = "John",
    lastName = "Doe"
}))
```

### 3. Use Format Specifiers

Format specifiers are optimized and faster than manual formatting:

```lua
-- Slower
local formatted = string.format("%.2f", amount)
print(t.price({ amount = formatted }))

-- Faster
print(t.price({ amount = amount }))  -- Uses :fixed specifier
```

## See Also

- [Getting Started](../getting-started.md) - Initial setup
- [Pluralization](pluralization.md) - Handle plural forms
- [Configuration](configuration.md) - Config options
- [Roblox Cloud Integration](roblox-cloud.md) - Upload to cloud
- [Rojo Integration](../integration/rojo.md) - Use with Rojo
