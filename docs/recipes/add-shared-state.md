# Add shared state

State goes in `src/store/` when two screens read it, or when it must outlive the screen that made it. A screen's own state stays in its folder (`state.rs`).

1. **Make `src/store/<name>.rs`** and list it in `src/store/mod.rs`.

2. **Declare a type for the state.** `global::<T>()` keeps one value per type, so the type is the state's name:

   ```rust
   /// The orders the user has starred.
   #[derive(Clone, Default, PartialEq)]
   struct Starred(Vec<u64>);
   ```

3. **Keep the signal private; export what reads and changes it.**

   ```rust
   /// Whether `id` is starred. Tracked.
   pub fn is_starred(id: u64) -> bool {
       global::<Starred>().with(|starred| starred.0.contains(&id))
   }

   /// Stars or unstars `id`.
   pub fn toggle_star(id: u64) { /* update the signal */ }
   ```

4. **Do not import `ui`, `pages` or `shell`** from a store file: state knows nothing of how it is shown. It may call `services`.

5. **Check:** `just rules && just test`.

`src/store/locale.rs` is a small example; `src/store/nav.rs` a larger one.
