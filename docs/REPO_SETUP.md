# Repository Setup

## Branch Protection (recommended)

Protect `main` (Settings → Branches → Add rule):
- Require PRs before merging
- Require status checks to pass
  - CI / build-and-test
- Require linear history (optional)
- Require code owner review (optional)
- Include administrators (optional)

## Default Labels

- bug
- enhancement
- documentation
- chore
- good first issue

Create labels with the GitHub UI or run `scripts/setup-labels.sh`.

## Secrets Needed for Workflows

- `GITHUB_TOKEN` — provided automatically by GitHub Actions
- `CARGO_REGISTRY_TOKEN` — crates.io publish
- (optional) `GPG_PRIVATE_KEY` / `GPG_PASSPHRASE`
- (optional) `GH_PAT`, `HOMEBREW_TAP_REPO`, `SCOOP_BUCKET_REPO` for auto-pushing packaging files
- (optional) `CODECOV_TOKEN`, `CODECOV_BADGE_TOKEN`
