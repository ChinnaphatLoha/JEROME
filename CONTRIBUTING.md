# Contributing to JEROME

Thank you for your interest in contributing to JEROME! This document outlines our development workflow, code standards, and submission guidelines.

---

## Development Workflow

### Prerequisites

Ensure you have a recent stable Rust toolchain installed:
```bash
rustup update stable
rustup component add rustfmt clippy
```

### Building & Checking

Verify the entire workspace compiles cleanly:
```bash
cargo check --workspace --all-targets
```

### Running Tests

Run the test suite across all crates in the workspace:
```bash
cargo test --workspace
```

To verify benchmarks compile:
```bash
cargo bench --workspace --no-run
```

---

## Code Style & Standards

We enforce strict formatting and zero compiler or linter warnings across the workspace.

1. **Formatting**: Always format code using `rustfmt`:
   ```bash
   cargo fmt --all -- --check
   ```
2. **Clippy**: Ensure code passes Clippy with warnings treated as errors:
   ```bash
   cargo clippy --workspace --all-targets -- -D warnings
   ```

---

## Commit Message Convention

We follow conventional commits structured as:
```text
<type>(<scope>): <summary>
```

### Allowed Types
- `feat`: A new feature or capability
- `fix`: A bug fix
- `refactor`: Code changes that neither fix a bug nor add a feature
- `test`: Adding or correcting tests
- `docs`: Documentation changes only
- `ci`: Changes to CI/CD workflows and configuration
- `chore`: Maintenance tasks, dependency bumps, tooling updates

### Scope Examples
- Crate names: `core`, `analysis`, `probability`, `decision`, `strategy`, `simulation`
- Cross-cutting: `bench`, `workspace`, `deps`

### Example
```text
feat(analysis): implement fast 7-card evaluator lookup
```

---

## Pull Request Process

1. **Branching**: Create a feature branch off `main` with a descriptive name (e.g., `feat/range-builder`, `fix/equity-tie`).
2. **Pre-flight Checks**: Before opening a PR, ensure all checks pass locally:
   ```bash
   cargo fmt --all -- --check
   cargo clippy --workspace --all-targets -- -D warnings
   cargo test --workspace
   ```
3. **Open PR**: Submit a pull request against the `main` branch with a clear title and description explaining the motivation and changes.
4. **Review**: Address review feedback and keep commits clean and atomic.
