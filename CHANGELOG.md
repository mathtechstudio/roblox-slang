# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Nothing yet

### Changed

- Nothing yet

### Fixed

- Nothing yet

## [1.0.0] - 2025-02-08

### Added

- Initial release of Roblox Slang
- Type-safe translation access with autocomplete support
- String interpolation with parameter validation (`{name}`, `{count:int}`)
- Pluralization support using CLDR rules (zero/one/two/few/many/other)
- Nested namespace syntax (`t.ui.buttons.buy()`)
- Watch mode for auto-rebuild on file changes
- CSV generation for Roblox Cloud Localization format
- Multiple input format support (JSON, YAML, CSV)
- Translation overrides for A/B testing and seasonal events
- CLI commands: `init`, `build`, `import`, `validate`, `migrate`
- Comprehensive test suite (168 tests: 88 unit + 30 CLI + 25 edge case + 13 integration + 12 stress)
- Performance benchmarks with criterion
- GitHub Actions CI/CD pipeline with multi-platform builds
- Distribution via Rokit, Aftman, Foreman, and Cargo
- Complete documentation with guides and examples
- Support for 17 Roblox locales
- Type definitions generator (`.d.luau`) for LSP autocomplete
- Rojo integration support
- Module-level documentation for all public modules
- Comprehensive error messages with file paths, line numbers, and helpful hints
- Input validation for locale codes, translation keys, and config files
- Security audit with cargo-audit

### Changed

- Optimized binary size to 1.5 MB
- Optimized build performance to <10ms
- Optimized watch mode latency to <10ms
- Removed 8 unused dependencies (tokio, toml, jsonc-parser, reqwest, thiserror, handlebars, pathdiff, prettytable-rs)
- Updated release profile for maximum optimization (opt-level="z", lto=true, strip=true)
- Improved error messages with context snippets and suggestions
- Fixed all compiler and clippy warnings
- Regenerated Cargo.lock with lockfile version 3
- Updated all repository references from protheeuz to mathtechstudio organization
- Removed MSRV constraint to allow using latest Rust stable features

### Fixed

- Parser errors now include file path, line number, and context snippet
- Config validation errors now include helpful hints and examples
- Empty files handled gracefully with warnings
- Missing files show clear error messages with suggestions
- Permission errors explained clearly
- Invalid UTF-8 detected and reported with file path
- URL warnings in documentation fixed
- Doctest example in library documentation now uses correct API functions
- Watch mode test now handles both interrupt and exit code 1 for Windows compatibility
