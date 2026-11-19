#!/usr/bin/env bash
# Fetch the sibling libraries that Cargo.toml and packages/remote-screen read by relative path.
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

fetch nsh-query SuperNSH/query 4f5ee0edcc1e153a381739e4063c952b7ceffdb7
fetch nsh-form SuperNSH/form 06053c367618e0feeec8cd3765636d552f987b9d
