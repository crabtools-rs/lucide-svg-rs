# 🚀 Release Quick Checklist

**Before tagging**
- ✅ All CI green (build, fmt, clippy, test, docs)
- ✅ Coverage uploaded (Codecov badge updated)
- ✅ Release Please PR merged, CHANGELOG looks correct
- ✅ Secrets set:
  - `CARGO_REGISTRY_TOKEN` (crates.io)
  - *(optional)* `GH_PAT`, `HOMEBREW_TAP_REPO`, `SCOOP_BUCKET_REPO`
  - *(optional)* `CODECOV_TOKEN`, `CODECOV_BADGE_TOKEN`
- ✅ Local checks:
  - `cargo fmt --all`
  - `cargo clippy -D warnings`
  - `cargo test --all`
  - `RUSTDOCFLAGS="-D warnings" cargo doc --no-deps`
  - `cargo package --locked && cargo publish --dry-run`

**Tagging**
```bash
git tag vX.Y.Z
git push origin vX.Y.Z
```

**After tagging**
- ✅ GitHub Release includes cargo-dist artifacts, SBOM, checksums
- ✅ Homebrew formula + Scoop manifest generated & attached
- ✅ Auto-push to tap/bucket repos succeeded
- ✅ Crate published to crates.io (Cargo Publish workflow)
- ✅ docs.rs updated for new version

**Packaging validation**
- 🍺 Homebrew: `brew install yourname/tap/lucide-offline-cli`
- 🪣 Scoop: `scoop install lucide-offline-cli`
