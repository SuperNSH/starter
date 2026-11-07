# Status

What is done, what was verified how (command + evidence path under
`docs/evidence/<area>/`), and what is NOT verified. A claim with no evidence
is unverified. Every area updates its own section as it lands work.

## Baseline (bootstrap, pre-workflow)

- Scaffolded the Rust app at the repo root with `nsh init --template rust`
  shape: `nsh.json` (`"rust"`, `"root"`, theme at `themes/theme.json`),
  `Cargo.toml` with `nsh-query`/`nsh-form` path deps, `rust-i18n`, and
  `nsh-engine`/`nsh-testing` 0.4.0 dev-deps; module tree in `src/` with
  placeholder pages for every feature area; `App.test.tsx` + `tsconfig.json`.
- Verified on the baseline only: `cargo fmt --all -- --check`,
  `cargo clippy --all-targets -- -D warnings`, `cargo test`, `nsh check`,
  `nsh build` (no warnings), `nsh test` (1 passing test). No feature work
  exists yet; every satisfaction criterion is unverified.
