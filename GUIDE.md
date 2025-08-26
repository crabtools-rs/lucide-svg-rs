# Lucide SVG Rust — Comprehensive Guide

This guide walks through building, testing, packaging, and distributing the **Lucide Offline CLI** and library. It integrates all directions covered in development, from unit testing to CI/CD workflows.

---

## 📂 Project Overview

- **`lucide_svg_rs`**: Rust library + CLI for Lucide SVG icons
- **File-based only**: ships with `icons/` directory, no network calls
- **Features**:
  - `list` all icons (plain text / JSON)
  - `search` icons by substring
  - `download-all` icons to another directory
- **Extras**:
  - Examples, integration tests, doctests
  - CI/CD workflows
  - Release automation with checksums, SBOM, and optional GPG signing
  - Packaging for Homebrew & Scoop
  - Nix flake + Dockerfile for reproducible builds

---

## 🛠 Library Usage

```rust
use lucide_svg_rs::{LucideClient, ICONS_DIR};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = LucideClient::new(ICONS_DIR)?;

    // List
    let icons = client.list_icons()?;
    println!("Found {} icons", icons.len());

    // Search
    let results = client.search_icons("alert")?;
    println!("Matches: {:?}", results);

    Ok(())
}
```

---

## 🖥 CLI Usage

```bash
# List icons
lucide-svg-rs list

# List icons as JSON
lucide-svg-rs list --json

# Search icons
lucide-svg-rs search alert

# Search as JSON
lucide-svg-rs search alert --json

# Download/export all icons
lucide-svg-rs download-all ./out
```

---

## 🧪 Testing

### Integration Tests

Run the CLI integration tests:

```bash
cargo test --test cli
```

Covers:

- Defaulting to `ICONS_DIR`
- Search functionality
- Download/export functionality
- JSON output validity

### JSON Output Test

Confirms machine-readable mode produces valid JSON:

```rust
let cli = Cli { dir: None, command: Commands::List { json: true } };
let output = run_cli(cli).unwrap();
let parsed: Vec<String> = serde_json::from_str(&output).unwrap();
assert!(!parsed.is_empty());
```

---

## 📖 Examples

### List

```bash
cargo run --example list
```

### Search

```bash
cargo run --example search
```

### Download All

```bash
cargo run --example download_all ./exported-icons
```

Examples explicitly show defaulting to `ICONS_DIR`.

---

## 📄 Doctests

Library doctests ensure docs compile + run:

````rust
/// ```
/// use lucide_svg_rs::{LucideClient, ICONS_DIR};
/// let client = LucideClient::new(ICONS_DIR).unwrap();
/// let icons = client.list_icons().unwrap();
/// assert!(!icons.is_empty());
/// ```
````

Run:

```bash
cargo test --doc
```

---

## ✅ CI Workflow

**`.github/workflows/ci.yml`** runs on push/PR:

- Build locked (`cargo build --locked`)
- Format check (`cargo fmt`)
- Lint (`cargo clippy -D warnings`)
- Docs build (`RUSTDOCFLAGS="-D warnings" cargo doc --no-deps`)
- Unit + integration tests
- Doctests + examples

---

## 🚀 Release Workflow

**`.github/workflows/release.yml`** runs on tag `v*.*.*`.

Builds:

- Linux x86_64
- macOS x86_64 + arm64
- Windows x86_64

Artifacts:

- Zipped binaries + README/LICENSE
- `CHECKSUMS.sha256` (+ `.asc` if GPG enabled)
- `SBOM.cdx.json` (CycloneDX)
- Auto-generated Homebrew & Scoop packaging files

Optional auto-push to your tap/bucket if secrets set:

- `GH_PAT`
- `HOMEBREW_TAP_REPO`
- `SCOOP_BUCKET_REPO`

Tag and push:

```bash
git tag v0.1.0
git push origin v0.1.0
```

---

## 🍺 Homebrew Packaging

Formula generated at release time.

Example tap:

```bash
brew tap yourname/tap https://github.com/yourname/homebrew-tap
brew install soulcorrea/tap/lucide-svg-rs
```

---

## 🪣 Scoop Packaging

Manifest generated at release time.

Install via bucket:

```bash
scoop bucket add lucide https://github.com/soulcorrea/scoop-bucket
scoop install lucide-svg-rs
```

---

## 🔐 Security

- **Checksums**: `CHECKSUMS.sha256` ensures integrity
- **GPG Signing**: optional `.asc` signatures if GPG secrets configured
- **SBOM**: `SBOM.cdx.json` published with releases

---

## ❄️ Nix Flake

`flake.nix` provides reproducible dev shell + builds.

### Dev Shell

```bash
nix develop
```

Includes Rust toolchain, `cargo`, `clippy`, `rustfmt`, `rust-analyzer`.

### Build

```bash
nix build
./result/bin/lucide-svg-rs list
```

> Run `nix build` once to obtain `cargoHash` if needed.

---

## 🐳 Docker

Multi-stage build with minimal runtime image.

### Build Image

```bash
docker build -t lucide-svg-rs .
```

### Run CLI

```bash
docker run --rm lucide-svg-rs list
docker run --rm lucide-svg-rs search alert
```

### Export Icons

```bash
mkdir -p ./out
docker run --rm -v $(pwd)/out:/out lucide-svg-rs download-all /out
```

---

# 🎯 Conclusion

With this setup you now have:

- Fully offline Lucide client + CLI
- Automated testing, docs, examples
- CI and release workflows
- Secure packaging with checksums/signatures
- Cross-platform binaries
- Reproducible builds via Nix
- Portable Docker image
