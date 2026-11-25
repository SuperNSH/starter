# Add a control

A control more than one screen uses goes in `src/ui/`. The app's pages take every control from there (rule 6 in `AGENTS.md`).

1. **Look first.** `src/ui/mod.rs` lists the controls; `src/showcase/gallery/` shows each one with its states.

2. **Make `src/ui/<name>.rs`** (or add to the file of the same family) and list it in `src/ui/mod.rs`. Wrap the `nsh_base` component and style every part of it: hover, focus ring, pressed, disabled.

3. **Colors come from the theme** (`theme().primary`). A color the theme lacks is a new token in `themes/theme.json`, read through `ui::tokens`.

4. **Let text grow:** `min_h` and padding, never a fixed height on something that holds text.

5. **Do not import anything else of the app.** A control takes its data and its callbacks as arguments.

6. **Show it in the gallery** (`src/showcase/gallery/view/`), and cover it in `tests/gallery.rs`.

7. **Check:** `just rules && just test gallery`.
