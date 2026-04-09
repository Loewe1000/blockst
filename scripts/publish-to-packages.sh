#!/usr/bin/env bash
# publish-to-packages.sh
#
# Syncs the current blockst repo into the typst/packages repo and opens
# a new branch ready for a Pull Request.
#
# Usage:
#   ./scripts/publish-to-packages.sh <version>
#
# Example:
#   ./scripts/publish-to-packages.sh 0.2.0
#
# Prerequisites:
#   - The packages repo must be cloned next to this repo at ../packages
#     (or set PACKAGES_REPO env variable to point to a different path)
#   - You must be on a clean main branch (no uncommitted changes) in blockst

set -euo pipefail

VERSION="${1:-}"
if [[ -z "$VERSION" ]]; then
  echo "Error: Please provide a version number."
  echo "Usage: $0 <version>  (e.g. $0 0.2.0)"
  exit 1
fi

BLOCKST_DIR="$(cd "$(dirname "$0")/.." && pwd)"
PACKAGES_REPO="${PACKAGES_REPO:-$(cd "$BLOCKST_DIR/../packages" && pwd)}"
TARGET_DIR="$PACKAGES_REPO/packages/preview/blockst/$VERSION"

# ── Sanity checks ────────────────────────────────────────────────────────────

if [[ ! -d "$PACKAGES_REPO/.git" ]]; then
  echo "Error: packages repo not found at $PACKAGES_REPO"
  echo "Clone it there or set the PACKAGES_REPO environment variable."
  exit 1
fi

cd "$BLOCKST_DIR"

if [[ -n "$(git status --porcelain)" ]]; then
  echo "Error: blockst has uncommitted changes. Commit or stash them first."
  git status --short
  exit 1
fi

CURRENT_BRANCH="$(git rev-parse --abbrev-ref HEAD)"
if [[ "$CURRENT_BRANCH" != "main" ]]; then
  echo "Warning: You are not on 'main' (current branch: $CURRENT_BRANCH)."
  read -rp "Continue anyway? [y/N] " confirm
  [[ "$confirm" == [yY] ]] || exit 1
fi

# ── Prepare packages repo branch ─────────────────────────────────────────────

cd "$PACKAGES_REPO"

# Make sure we're up-to-date with upstream (typst/packages)
git fetch upstream

PR_BRANCH="add-blockst-$VERSION"

if git show-ref --verify --quiet "refs/heads/$PR_BRANCH"; then
  echo "Branch '$PR_BRANCH' already exists in packages repo. Switching to it."
  git checkout "$PR_BRANCH"
else
  echo "Creating branch '$PR_BRANCH' from upstream/main …"
  git checkout -b "$PR_BRANCH" upstream/main
fi

# ── Copy files ───────────────────────────────────────────────────────────────

if [[ -d "$TARGET_DIR" ]]; then
  echo "Target directory $TARGET_DIR already exists – overwriting."
fi

mkdir -p "$TARGET_DIR"

rsync -a --delete \
  --exclude=".git" \
  --exclude=".gitignore" \
  --exclude=".gitattributes" \
  --exclude="*.pdf" \
  --exclude=".DS_Store" \
  --exclude="scripts/" \
  "$BLOCKST_DIR/" \
  "$TARGET_DIR/"

# ── Commit ───────────────────────────────────────────────────────────────────

cd "$PACKAGES_REPO"
git add -A
if git diff --cached --quiet; then
  echo "No changes to commit in packages repo."
else
  git commit -m "Add blockst $VERSION"
  echo ""
  echo "✓ Committed to branch '$PR_BRANCH' in $PACKAGES_REPO"
fi

echo ""
echo "Next steps:"
echo "  1. Review the diff:  cd $PACKAGES_REPO && git diff upstream/main"
echo "  2. Push to your fork: git push -u origin $PR_BRANCH"
echo "  3. Open https://github.com/Loewe1000/packages and click 'Compare & pull request'"
echo "     → Base: typst/packages:main  ←  Compare: Loewe1000/packages:$PR_BRANCH"
