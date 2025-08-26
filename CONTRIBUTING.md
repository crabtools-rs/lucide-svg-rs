# Contributing to lucide-svg-rs

Thanks for helping make this better!

## Development Setup

- Install Rust (stable)
- Build: `cargo build`
- Run tests: `cargo test`
- Examples: `cargo run --example list`
- Lint: `cargo clippy --all-targets --all-features -- -D warnings`
- Format: `cargo fmt --all`

## Pull Requests

1. Create a feature branch from `main`.
2. Ensure CI passes locally (build, fmt, clippy, tests, docs).
3. Add tests for new functionality.
4. Update docs if behavior changes.
5. Open a PR — the template will prefill the checklist.

## Versioning & Releases

We follow **Conventional Commits** and use **Release Please**:

- Use types like `feat:`, `fix:`, `docs:`, `chore:`, `refactor:`, `test:`, etc.
- Release Please will open a release PR that bumps versions and updates `CHANGELOG.md`.
- Merging that PR will create a GitHub Release and tag (e.g., `v0.1.1`).

Examples:

- `feat(cli): add --json output to search`
- `fix(client): trim whitespace before matching icon names`
