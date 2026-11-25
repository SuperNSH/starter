# Add a page

A page is one folder, one `Route` variant and one line in the page table. The compiler asks for the line if you forget it. `src/pages/settings/` is the model to copy; `src/pages/items/` for a list with a detail (`nsh-query`), `src/pages/sign_in/` for a form (`nsh-form`).

1. **Make the folder** `src/pages/<name>/` with two files.

   `mod.rs`: a `//!` header saying what the page is and `Test: tests/<name>.rs`, then its record:

   ```rust
   mod view;

   pub use view::orders_page;

   use nsh_core::prelude::IntoElement;

   use crate::routes::{Content, Frame, Page};
   use crate::ui::icons::lucide;

   pub const PAGE: Page = Page {
       key: "orders",                       // the folder's name
       title: "Orders",
       summary: "What you bought, newest first.",
       icon: lucide!(r#"<path d="M3 5h18M3 12h18M3 19h18"/>"#),
       symbol: "list.bullet",               // an SF Symbol
       frame: Frame::Scrolls,
       action: None,
       content: Content::Plain(|| orders_page().into_element()),
   };
   ```

   `view.rs`: `pub fn orders_page() -> impl IntoElement`, built from `crate::ui` controls (never `nsh_base`, except `theme`: `just rules` checks it). Server reads go in `queries.rs` (keys and `use_query*` calls), state only this page holds in `state.rs` or local signals.

2. **Add the address** in `src/routes.rs`: the `Orders` variant, its arm in `path` (`"orders"`) and in `parse` (`["orders"]`).

3. **Add it to the table** in `src/pages/mod.rs`: `pub mod orders;`, the arm `Route::Orders => &orders::PAGE` in `Route::page`, and a `list.push(Route::Orders)` in `sidebar()` where it should appear. Add it to `tabs()` too if it is one of the phone's tabs (five at most: iOS folds the rest into More). A page that is not in the sidebar (a pushed item, a sign-in) gets a row in `home_rows()` instead, so Home and the smoke suite can reach it.

4. **Add its strings** to every file in `i18n/` under `orders.*`, and read them with `store::locale::text("orders.title")`.

5. **Write `tests/orders.rs`**: boot `nsh_starter::App`, press `id:page-orders` on Home (its card, or its row from `home_rows()`), drive the page. A headless app is a phone; for the desktop layout set the viewport and a desktop environment (`Os::MacOs`, `Idiom::Desktop`), as `tests/items.rs` does. Break the code once to see the test fail, and name the break in the file's header. Add the page to `PAGES` in `tests/smoke/pages.test.tsx` (and its id to `HOME_ROWS` there when Home reaches it by a row).

6. **Say where it is:** the page list in `AGENTS.md`'s map, `docs/architecture.md` if it is a pattern others copy, and `CHANGELOG.md`.

7. **Check:** `just rules && just test orders && just check`. If you changed a line of `src/routes.rs` or `src/pages/mod.rs` next to a `showcase` line, run `just new-app` into a scratch directory too: `scripts/new-app.sh` edits those lines by their text and stops when one moved.

A page of one item (`Route::OrdersDetail(u64)`) lives in its list's folder (`detail.rs`), keyed `orders_detail`, with its own record (`content: Content::WithId(..)`); `Route::id` returns its id, `Route::parent` returns `Orders`, and its link is `orders/<id>`. The list opens it with `nav::push_child(Route::OrdersDetail(id))`: on phones the shell pushes it; where that answers `false` (desktop widths), the list presents the item itself. `src/pages/items/` shows it.
