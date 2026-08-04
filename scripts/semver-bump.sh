#!/usr/bin/env sh
# Compute the next semver from Conventional Commit subjects since the last tag.
#
# WHY THIS IS SHELL AND NOT COMMITIZEN
# The `standards` gate names commitizen as the changelog/bump driver, and that is
# the right tool in a Python project. Here it would mean installing a Python
# package onto the runner at release time. The fleet has already been bitten by
# exactly that shape: the standards workflow degrades to a warning because
# python3-yaml is absent from the runner image, and its own error text says the
# fix is to bake the dependency in rather than install it in-job. A release must
# not be the thing that discovers a missing interpreter package, so the bump rule
# is implemented here in POSIX sh against git alone — no interpreter, no network,
# no third-party action, consistent with release.yml's supply-chain posture.
#
# CONTRACT
#   stdout: the next version, bare (e.g. 0.465.0) — nothing else goes to stdout
#   stderr: human-readable reasoning
#   exit 0: a release is warranted, next version on stdout
#   exit 3: NO releasable commits — caller must skip the release, not fail the build
#   exit 1: something was wrong and we refuse to guess (never-silent)
#
# Exit 3 is deliberately distinct from both success and failure. "Nothing to
# release" is a normal outcome on a docs-only merge, and collapsing it into
# either 0 or 1 would mean either cutting an empty release or reporting a red
# build for a healthy merge.
set -eu

CUR="${1:-}"
[ -n "$CUR" ] || { echo "semver-bump: current version required as \$1" >&2; exit 1; }

case "$CUR" in
  *[!0-9.]*|"") echo "semver-bump: current version '$CUR' is not bare X.Y.Z" >&2; exit 1 ;;
esac
# EXACTLY three components. Checking only the characters is not enough: for '1.2',
# ${REST#*.} returns '2' unchanged (no second dot to strip), so the version would
# parse as 1.2.2 and be silently accepted. A malformed version must refuse, not
# guess -- guessing here would tag and publish a release at a version nobody chose.
MAJ=${CUR%%.*}; REST=${CUR#*.}; MIN=${REST%%.*}; PAT=${REST#*.}
case "$CUR" in
  *.*.*.*) echo "semver-bump: '$CUR' has more than three components" >&2; exit 1 ;;
  *.*.*)   ;;
  *)       echo "semver-bump: '$CUR' is not X.Y.Z (needs exactly three components)" >&2; exit 1 ;;
esac
for part in "$MAJ" "$MIN" "$PAT"; do
  case "$part" in
    ''|*[!0-9]*) echo "semver-bump: component '$part' of '$CUR' is not numeric" >&2; exit 1 ;;
  esac
done

# Range: last reachable tag..HEAD. With no tags at all, consider full history.
LAST_TAG=$(git describe --tags --abbrev=0 2>/dev/null || true)
if [ -n "$LAST_TAG" ]; then RANGE="$LAST_TAG..HEAD"; else RANGE="HEAD"; fi
echo "semver-bump: scanning $RANGE (current $CUR)" >&2

# %s subject, %b body. Body matters: BREAKING CHANGE conventionally lives there.
LOG=$(git log --no-merges --format='%s%n%b%n--COMMIT--' "$RANGE" 2>/dev/null || true)
if [ -z "$(printf '%s' "$LOG" | tr -d '[:space:]')" ]; then
  echo "semver-bump: no commits in range — nothing to release" >&2; exit 3
fi

# Precedence: major > minor > patch. Scan once, keep the strongest signal.
LEVEL="none"
# `type!:` or `type(scope)!:` marks a breaking change, as does BREAKING CHANGE in the body.
if printf '%s\n' "$LOG" | grep -qE '^[a-z]+(\([^)]*\))?!:' \
   || printf '%s\n' "$LOG" | grep -qE '^BREAKING[ -]CHANGE:'; then
  LEVEL="major"
elif printf '%s\n' "$LOG" | grep -qE '^feat(\([^)]*\))?:'; then
  LEVEL="minor"
elif printf '%s\n' "$LOG" | grep -qE '^(fix|perf|refactor|revert)(\([^)]*\))?:'; then
  LEVEL="patch"
fi

# docs/chore/ci/test/style-only ranges are intentionally NOT releasable: shipping a
# new binary whose behaviour is provably identical only adds a version users must
# reason about.
if [ "$LEVEL" = none ]; then
  echo "semver-bump: no feat/fix/perf/refactor/revert/breaking commits — not releasable" >&2
  exit 3
fi

# Pre-1.0 posture: a breaking change moves MINOR, not MAJOR. This project is at
# 0.x and treats 0.MINOR as its compatibility axis (v0.464.0 with 464 minors is
# the evidence). Auto-promoting 0.x to 1.0.0 on the first `feat!:` would be an
# irreversible product decision made by a script, so it is refused here instead.
if [ "$MAJ" = "0" ] && [ "$LEVEL" = major ]; then
  echo "semver-bump: breaking change on a 0.x line -> bumping MINOR, not MAJOR." >&2
  echo "semver-bump: promoting to 1.0.0 is a deliberate human decision; tag it by hand." >&2
  LEVEL="minor"
fi

case "$LEVEL" in
  major) MAJ=$((MAJ+1)); MIN=0; PAT=0 ;;
  minor) MIN=$((MIN+1)); PAT=0 ;;
  patch) PAT=$((PAT+1)) ;;
esac

echo "semver-bump: level=$LEVEL  $CUR -> $MAJ.$MIN.$PAT" >&2
printf '%s.%s.%s\n' "$MAJ" "$MIN" "$PAT"
