-- Test script for Roblox Slang translations
-- This demonstrates how to use the generated translation module

local ReplicatedStorage = game:GetService("ReplicatedStorage")
local Translations = require(ReplicatedStorage.Translations)

-- Create translation instance for English
local t = Translations.new("en")

print("=== Basic Translations ===")
-- Simple translations (no parameters)
print("Buy button:", t.ui.buttons.buy())
print("Sell button:", t.ui.buttons.sell())
print("Cancel button:", t.ui.buttons.cancel())
print("Welcome label:", t.ui.labels.welcome())

print("\n=== Translations with Parameters ===")
-- Translations with parameters
print("Greeting:", t.ui.messages.greeting({ name = "Player1" }))
print("Item count:", t.ui.messages.itemCount({ count = 5 }))

print("\n=== Format Specifiers ===")
-- Format specifiers
print("Price:", t.ui.messages.price({ price = 19.99 }))
print("Score:", t.ui.messages.score({ score = 1234.56 }))

print("\n=== Pluralization ===")
-- Plural translations
print("1 item:", t.ui.messages.items(1))
print("5 items:", t.ui.messages.items(5))

print("\n=== Locale Switching ===")
-- Register locale change callback
t:onLocaleChanged(function(newLocale, oldLocale)
	print(string.format("Locale changed from %s to %s", oldLocale, newLocale))
end)

-- Switch to Indonesian
print("Current locale:", t:getLocale())
t:setLocale("id")
print("New locale:", t:getLocale())

print("\n=== Indonesian Translations ===")
print("Buy button:", t.ui.buttons.buy())
print("Greeting:", t.ui.messages.greeting({ name = "Pemain1" }))
print("Price:", t.ui.messages.price({ price = 19.99 }))
print("1 item:", t.ui.messages.items(1))

print("\n=== Asset Localization ===")
-- Asset localization (example)
-- print("Logo asset:", t:getAsset("logo"))
-- print("Background music:", t:getAsset("bgm"))
