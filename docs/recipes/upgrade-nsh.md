# Move to a new NSH release

Every NSH crate must come from the same release, and the `nsh` CLI should match it.

1. Read the release notes for removed or renamed APIs.
2. In `Cargo.toml`, change every `nsh-*` version together (dependencies, the four platform crates, the dev-dependencies).
3. `cargo update`, then `just check && just test`. Fix compile errors against the new source in `~/.cargo/registry/src/*/nsh-core-<version>`, never from memory.
4. Compile the targets this Mac cannot run: `cargo clippy --lib --target aarch64-apple-ios-sim -- -D warnings`.
5. `just smoke`, then run the app on each target you ship.
6. Re-check the entries of "Facts that bite" in `docs/PROMPT.md` that the release could have changed, and record what you re-ran in `docs/STATUS.md`.
