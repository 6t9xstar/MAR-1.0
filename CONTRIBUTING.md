# Contributing to MAR 1.0

Thank you for your interest in contributing! We welcome contributions of all forms.

## Code of Conduct

Please read our [Code of Conduct](CODE_OF_CONDUCT.md) first.

## How to Contribute

### Report a Bug

Open an issue with:
- A clear title and description
- Steps to reproduce
- Expected vs actual behavior
- Environment details (OS, Rust version, etc.)

### Request a Feature

Open an issue with:
- Use case and motivation
- Proposed solution (if any)
- Alternative approaches considered

### Submit a Pull Request

1. Fork the repository
2. Create a feature branch from `main`:
   ```bash
   git checkout -b feat/my-feature
   ```
3. Make your changes
4. Run tests:
   ```bash
   # Rust
   cargo test --workspace --exclude inference

   # Python training
   cd training && python -m pytest tests/

   # Frontend
   npm run typecheck
   npm test
   ```
5. Run linting:
   ```bash
   cargo clippy --workspace --exclude inference -- -D warnings
   npm run lint
   ```
6. Commit with a clear message (see conventions below)
7. Push and open a PR against `main`

### Commit Conventions

```
<type>(<scope>): <description>

feat:    New feature
fix:     Bug fix
docs:    Documentation
style:   Formatting (no code change)
refactor: Code refactoring
test:    Adding or fixing tests
chore:   Build, CI, dependencies
```

Examples:
```
feat(model): add scaled RoPE for long context
fix(memory): correct SQL OR precedence bug
docs(readme): add quickstart for Windows
```

## Development Setup

See [README.md](README.md#quick-start) for setup instructions.

## Architecture

- **api-server/**: Axum REST API with tower-http middleware
- **training/**: PyTorch training pipeline (model architecture, tokenizer, data)
- **inference/**: llama.cpp GGUF inference server
- **src/**: SolidJS frontend with fine-grained reactivity
- **src-tauri/**: Tauri desktop shell

## Code Style

- Rust: Follow `cargo clippy` with `-D warnings`
- TypeScript: Follow `biome check --apply`
- Python: Follow PEP 8 (run `ruff check`)
- Keep PRs focused on a single concern

## Questions?

Open a discussion or reach out to the maintainer.
