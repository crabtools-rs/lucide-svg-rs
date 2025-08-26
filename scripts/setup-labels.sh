#!/usr/bin/env bash
# Requires GitHub CLI: https://cli.github.com/
# Usage: ./scripts/setup-labels.sh soulcorrea/lucide-svg-rs

set -euo pipefail
REPO="${1:-}"
if [[ -z "$REPO" ]]; then
  echo "Usage: $0 soulcorrea/lucide-svg-rs"
  exit 1
fi

gh label create bug -c "#d73a4a" -d "Something isn't working" -R "$REPO" || true
gh label create enhancement -c "#a2eeef" -d "New feature or request" -R "$REPO" || true
gh label create documentation -c "#0075ca" -d "Improvements or additions to documentation" -R "$REPO" || true
gh label create chore -c "#cfd3d7" -d "Maintenance or tooling" -R "$REPO" || true
gh label create good-first-issue -c "#7057ff" -d "Good for newcomers" -R "$REPO" || true
