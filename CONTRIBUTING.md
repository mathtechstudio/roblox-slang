# Contributing to Roblox Slang

## Development Setup

### 1. Fork the Repository

Fork the repository on GitHub to your account.

### 2. Clone Your Fork

```bash
git clone https://github.com/YOUR_USERNAME/roblox-slang.git
cd roblox-slang
```

### 3. Add Upstream Remote

```bash
git remote add upstream https://github.com/mathtechstudio/roblox-slang.git
```

### 4. Install Rust

```bash
rustup update stable
rustup override set stable
```

### 5. Build and Test

```bash
cargo build
cargo test
```

## Contributing Workflow

### 1. Create a Feature Branch

```bash
git checkout -b feat/your-feature-name
```

### 2. Make Your Changes

- Write clear, focused commits
- Follow the commit convention below
- Write tests for new features
- Document public APIs

### 3. Test Your Changes

```bash
cargo test              # Run all tests
cargo clippy -- -D warnings  # Run linter
cargo fmt              # Format code
```

### 4. Update CHANGELOG.md

Add your changes under `[Unreleased]` section.

### 5. Push to Your Fork

```bash
git push origin feat/your-feature-name
```

### 6. Submit Pull Request

Submit a PR from your feature branch to the `main` branch of the upstream repository.

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
