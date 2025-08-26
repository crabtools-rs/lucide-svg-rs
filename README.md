# lucide-svg-rs

## A Lucide Library and Offline CLI

[![Contributions Welcome](https://img.shields.io/badge/contributions-welcome-brightgreen.svg?style=flat)](CONTRIBUTING.md)
[![License: MIT/Apache-2.0](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE)
[![CI](https://github.com/soulcorrea/lucide-svg-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/soulcorrea/lucide-svg-rs/actions/workflows/ci.yml)
[![Release](https://github.com/soulcorrea/lucide-svg-rs/actions/workflows/release.yml/badge.svg)](https://github.com/soulcorrea/lucide-svg-rs/actions/workflows/release.yml)
[![Crates.io](https://img.shields.io/crates/v/lucide-svg-rs.svg)](https://crates.io/crates/lucide-svg-rs)
[![Docs.rs](https://docs.rs/lucide-svg-rs/badge.svg)](https://docs.rs/lucide-svg-rs)
[![codecov](https://codecov.io/gh/soulcorrea/lucide-svg-rs/branch/main/graph/badge.svg)](https://codecov.io/gh/soulcorrea/lucide-svg-rs)

lucide-svg-rs is a Rust library + command-line tool for working with
[Lucide](https://lucide.dev) SVG icons **offline**.
All icons are shipped in a local `icons/` directory — no network access required.

<img width="800" height="533" alt="lucide-svg-rs" src="https://github.com/user-attachments/assets/f9b3e74a-d5d7-46b7-a8fc-cd03f8949035" />

---

## 🚀 Quick Start

### Installation

Clone and build:

```bash
cargo build --release
```

The binary will be available at:

```
target/release/lucide-svg-rs
```

---

## 📖 Usage

### List icons

```bash
lucide-svg-rs list
lucide-svg-rs list --json
```

### Search icons

```bash
lucide-svg-rs search alert
lucide-svg-rs search alert --json
```

### Export icons

```bash
lucide-svg-rs download-all ./out
```

---

## 🧪 Testing

Run the test suite:

```bash
cargo test
```

This covers:

- Defaulting to `ICONS_DIR`
- Searching icons
- Download/export
- JSON output validity

---

## 🧑‍💻 Developer Guide

For a comprehensive reference covering:

- Integration & JSON tests (`cargo test --test cli`)
- Examples & doctests
- CI/CD workflows
- Release automation (checksums, GPG signing, SBOM)
- Reproducible builds (Nix flake)
- Portable usage (Docker)

👉 see [GUIDE.md](GUIDE.md).

## 🤝 Contributing

We welcome contributions! Please read [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

Key points:

- Follow [Conventional Commits](https://www.conventionalcommits.org/) (e.g. `feat:`, `fix:`, `docs:`)
- Our release process is automated via **Release Please** — version bumps and CHANGELOG are handled for you
- Ensure `cargo fmt`, `cargo clippy -D warnings`, and `cargo test` all pass before opening a PR

---

## 📚 Docs build

CI verifies the docs compile cleanly (warnings as errors):

```bash
RUSTDOCFLAGS="-D warnings" cargo doc --no-deps
```

---

## 🚦 Publish guard

On tags (`v*.*.*`), CI runs a **publish guard** to catch issues before a real publish:

- `cargo package --locked`
- `cargo publish --dry-run`
- docs.rs-style build on nightly with `--cfg docsrs -D warnings`

---

## 📦 crates.io publish

A separate workflow `.github/workflows/cargo-publish.yml` runs when a GitHub Release is published.

- Requires `CARGO_REGISTRY_TOKEN` secret (from crates.io account)
- Only runs on the main repository (not forks)
- Executes `cargo publish --locked`

---

## 🔗 Unified Release Flow

Releases are handled by a single workflow (**.github/workflows/release.yml**) that:

- plans/builds via **cargo-dist**
- generates an aggregate **CHECKSUMS.sha256** (and optional **.asc** signature)
- produces a **CycloneDX SBOM**
- publishes a GitHub Release with all artifacts

Trigger by pushing a tag:

```bash
git tag v0.1.0
git push origin v0.1.0
```

---

## 🚀 Releases

Maintainers: use the **Release Checklist** issue form to guide each release.
Tag format: `vX.Y.Z` (e.g., `v0.1.0`).

- Open a new issue → **Release Checklist**
- Complete pre-tag checks, then tag and push:

  ```bash
  git tag v0.1.0
  git push origin v0.1.0
  ```

- CI will build artifacts, generate SBOM & checksums, publish GitHub Release, and (optionally) push Homebrew/Scoop files.
- A separate workflow can publish the crate to crates.io when a GitHub Release is published.

---

> 🔧 **Note:** CI/Release/Codecov badges are auto-patched by `.github/workflows/patch-badges.yml` after pushes to `main`.
