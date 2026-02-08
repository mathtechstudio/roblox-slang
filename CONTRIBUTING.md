# Contributing to Roblox Slang

## Development Setup

```bash
# Clone and setup
git clone https://github.com/mathtechstudio/roblox-slang.git
cd roblox-slang

# Install Rust (stable recommended)
rustup update stable
rustup override set stable

# Build and test
cargo build
cargo test
```

## Pull Request Process

1. Fork the repository and create a feature branch
2. Make your changes with clear, focused commits
3. Ensure all tests pass: `cargo test`
4. Run linter: `cargo clippy -- -D warnings`
5. Format code: `cargo fmt`
6. Update CHANGELOG.md under `[Unreleased]`
7. Submit PR to `main` branch

## Commit Convention

Follow [Conventional Commits](https://www.conventionalcommits.org/):

```bash
<type>(<scope>): <description>

[optional body]
[optional footer]
```

Types: `feat`, `fix`, `docs`, `style`, `refactor`, `test`, `chore`

Examples:

```yaml
feat(parser): add YAML support
fix(generator): escape quotes in Luau strings
docs: update installation guide
```

## Code Standards

- Write tests for new features
- Document public APIs with doc comments
- Keep functions focused
- Use `Result<T>` for error handling
- Follow Rust naming conventions

## Testing

```bash
cargo test              # Run all tests
cargo test test_name    # Run specific test
cargo bench            # Run benchmarks
```
