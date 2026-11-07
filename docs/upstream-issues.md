# Upstream issues

Bugs and sharp edges found in the sibling libraries `nsh-query` and `nsh-form` while building this app. Both repos are read-only for us: if something here blocks you, work around it in your area and add a row below with a reproduction and the workaround. Do not fork or patch the libraries.

Format: one row per issue — what breaks, how to reproduce it, the workaround this app uses, and whether it is still open. When the library fixes one, move it to the fixed list with the version that fixed it.

## Open

*(none recorded yet — this page is written from the areas' `docs/STATUS.md` notes during the audit pass)*

## Known constraints (not bugs, but they bite)

These come from NSH itself and shape how the libraries are used here; `docs/PROMPT.md` "Facts that bite on 0.4.0" is the full list.

- `Signal::update` needs `T: Clone`. State machines holding non-cloneable user types pair `stored(..)` with a version signal (see `~/fo/nsh-query/crates/nsh-query/src/tracked.rs` for the pattern nsh-query ships).
- A component's signals are disposed when its `cleanup` runs: reading one there panics. Capture what cleanup needs in an `Rc<Cell<..>>`, and use `create_root` for an owner that outlives the component (nsh-query uses this for caches and in-flight writes).

## Fixed

*(none)*
