#!/usr/bin/env bash
# Enforces the Conventional Commits format used in the ariadne repo
# (commitlint's config-conventional with the subject-case rule disabled),
# without a node toolchain. Invoked by pre-commit at the commit-msg stage
# with the message file as $1.
set -euo pipefail

msg_file="$1"
header="$(head -n 1 "$msg_file")"

# Messages commitlint ignores by default: merges, git-generated reverts,
# and autosquash commits.
case "$header" in
  "Merge "* | "Revert \""* | "fixup! "* | "squash! "*) exit 0 ;;
esac

types="build|chore|ci|docs|feat|fix|perf|refactor|revert|style|test"

if ! printf '%s' "$header" | grep -qE "^(${types})(\([a-z0-9-]+\))?!?: .+$"; then
  cat >&2 <<EOF
Commit message header does not follow Conventional Commits:

  $header

Expected: <type>(<optional-scope>): <subject>
Types:    ${types//|/, }
Examples: feat(slider): center the thumb on the track
          fix: derive input placeholder color from the text style
EOF
  exit 1
fi

if [ "${#header}" -gt 100 ]; then
  echo "Commit message header exceeds 100 characters (${#header})." >&2
  exit 1
fi
