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

# Always start from the current upstream/main, also on a repeat run: a branch
# left over from an earlier attempt sits on an older upstream, and a commit
# made on top of it deletes every package added upstream since — the diff
# looked like 119 deleted files of other people's packages once.
echo "Resetting branch '$PR_BRANCH' to upstream/main …"
git checkout -q -B "$PR_BRANCH" upstream/main

# ── Copy files ───────────────────────────────────────────────────────────────

if [[ -d "$TARGET_DIR" ]]; then
  echo "Target directory $TARGET_DIR already exists – overwriting."
  rm -rf "$TARGET_DIR"
fi

mkdir -p "$TARGET_DIR"

rsync -a --delete --delete-excluded \
  --exclude=".git" \
  --exclude=".gitignore" \
  --exclude=".gitattributes" \
  --exclude=".github/" \
  --exclude="*.pdf" \
  --exclude=".DS_Store" \
  --exclude="scripts/" \
  --exclude="docs/build/" \
  "$BLOCKST_DIR/" \
  "$TARGET_DIR/"

# ── Commit ───────────────────────────────────────────────────────────────────

cd "$PACKAGES_REPO"
git add -A

# Nothing outside the package's own directory may change — a stray deletion
# here would remove somebody else's package from the registry.
STRAY="$(git diff --cached --name-status upstream/main \
  | grep -v $'\tpackages/preview/blockst/'"$VERSION"'/' || true)"
if [[ -n "$STRAY" ]]; then
  echo "Error: the branch touches files outside packages/preview/blockst/$VERSION:" >&2
  echo "$STRAY" | head -20 >&2
  exit 1
fi

if git diff --cached --quiet; then
  echo "No changes to commit in packages repo."
else
  git commit -m "Add blockst $VERSION"
  echo ""
  echo "✓ Committed to branch '$PR_BRANCH' in $PACKAGES_REPO"
  git --no-pager diff --stat upstream/main | tail -1
fi

echo ""
echo "Pushing branch '$PR_BRANCH' to origin ..."
git push -u origin "$PR_BRANCH"

echo ""
echo "Next steps:"
echo "  1. Open https://github.com/Loewe1000/packages and click 'Compare & pull request'"
echo "     → Base: typst/packages:main  ←  Compare: Loewe1000/packages:$PR_BRANCH"
