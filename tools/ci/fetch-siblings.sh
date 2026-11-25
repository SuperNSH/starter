#!/usr/bin/env bash
# Fetch the sibling libraries whose TSX packages packages/remote-screen reads by relative path.
# The SHAs are the revisions this repo was built against locally; both are ancestors of origin's master.
set -euo pipefail

parent="$(cd .. && pwd)"

fetch() {
  local name="$1" repo="$2" sha="$3" dir="$parent/$1"
  rm -rf "$dir"
  git init --quiet "$dir"
  git -C "$dir" remote add origin "https://github.com/$repo.git"
  git -C "$dir" fetch --quiet --depth 1 origin "$sha"
  git -C "$dir" checkout --quiet FETCH_HEAD
  echo "$name at $(git -C "$dir" rev-parse --short HEAD)"
}

fetch nsh-query SuperNSH/query 1088fddd053cb10d71e9978a3c6704420f8ed970
fetch nsh-form SuperNSH/form 7b26c60f365d7a3892cd47abea9ecb7aad1f3ac2

# The TSX packages import TanStack from npm; install what they depend on.
for name in nsh-query nsh-form; do
  (cd "$parent/$name/packages/$name" && npm install --omit=dev --no-package-lock --no-audit --no-fund)
done
