# Packaging Guide

This document explains how the **Homebrew tap** and **Scoop bucket** tie into the release workflow for `lucide-offline-cli`.

---

## 📦 Overview

The release workflow automatically generates and distributes packaging files for:

- **Homebrew (macOS/Linux)** → formula (`lucide-offline-cli.rb`) committed to your `homebrew-tap` repo
- **Scoop (Windows)** → manifest (`lucide-offline-cli.json`) committed to your `scoop-bucket` repo

This ensures that whenever you tag a release (e.g. `v0.2.0`), users on macOS/Linux/Windows can install or upgrade via `brew` or `scoop` without you manually updating these repos.

---

## 🔄 How It Works

1. **Tag a Release**
   ```bash
   git tag v0.2.0
   git push origin v0.2.0
   ```

2. **Workflow runs** (`.github/workflows/release.yml`):
   - Builds binaries with `cargo-dist`
   - Computes SHA256 checksums
   - Generates:
     - `outpkg/homebrew/lucide-offline-cli.rb`
     - `outpkg/scoop/lucide-offline-cli.json`
   - Attaches these files to the GitHub Release
   - (Optional) Pushes them to your tap/bucket repos

3. **Users install/upgrade** using standard tools:
   - `brew install yourname/tap/lucide-offline-cli`
   - `scoop install lucide-offline-cli`

---

## 🍺 Homebrew Tap Setup

1. Create a new repo: `yourname/homebrew-tap`
2. Push the starter tap from `lucide-offline-packaging.zip`
3. Add the secret in your main repo:
   - `HOMEBREW_TAP_REPO=yourname/homebrew-tap`
   - `GH_PAT` with `repo` scope

Users can now:

```bash
brew tap yourname/tap https://github.com/yourname/homebrew-tap
brew install yourname/tap/lucide-offline-cli
```

---

## 🪣 Scoop Bucket Setup

1. Create a new repo: `yourname/scoop-bucket`
2. Push the starter bucket from `lucide-offline-packaging.zip`
3. Add the secret in your main repo:
   - `SCOOP_BUCKET_REPO=yourname/scoop-bucket`
   - `GH_PAT` with `repo` scope

Users can now:

```powershell
scoop bucket add lucide https://github.com/yourname/scoop-bucket
scoop install lucide-offline-cli
```

---

## 🔐 Required Secrets

In your main repo (`lucide-offline-cli`), set these secrets:

- `GH_PAT` — a Personal Access Token with `repo` scope (used to push to other repos)
- `HOMEBREW_TAP_REPO` — e.g. `yourname/homebrew-tap`
- `SCOOP_BUCKET_REPO` — e.g. `yourname/scoop-bucket`
- (Optional) `GPG_PRIVATE_KEY` + `GPG_PASSPHRASE` for signing checksums

---

## ⚠️ Troubleshooting

- **Homebrew formula not updated** → Check `HOMEBREW_TAP_REPO` secret, ensure `GH_PAT` has `repo` scope.
- **Scoop manifest not updated** → Check `SCOOP_BUCKET_REPO` secret, ensure `GH_PAT` has `repo` scope.
- **SHA256 mismatch** → Ensure workflow computed hashes match the released archives; re-run the release workflow if needed.
- **No access** → Confirm GitHub Actions bot can push to the tap/bucket repos.

---

## ✅ Summary

- Tag a release → workflow builds + packages → auto-updates Homebrew + Scoop repos
- Users install/upgrade with standard `brew`/`scoop` commands
- Maintainers only manage secrets and occasionally review generated files

This provides a **seamless multi-platform distribution pipeline** with minimal manual work.
