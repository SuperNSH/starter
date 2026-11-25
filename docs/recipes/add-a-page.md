# Add a page

A page is one folder, one `Route` variant and one line in the page table. The compiler asks for the line if you forget it. `src/pages/settings/` is the model to copy.

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

   `view.rs`: `pub fn orders_page() -> impl IntoElement`, built from `crate::ui` controls.

2. **Add the address** in `src/routes.rs`: the `Orders` variant, its arm in `path` (`"orders"`) and in `parse` (`["orders"]`).

3. **Add it to the table** in `src/pages/mod.rs`: `pub mod orders;`, the arm `Route::Orders => &orders::PAGE` in `Route::page`, and a `list.push(Route::Orders)` in `sidebar()` where it should appear. Add it to `tabs()` too if it is one of the phone's tabs (five at most).

4. **Add its strings** to every file in `i18n/` under `orders.*`, and read them with `store::locale::text("orders.title")`.

5. **Write `tests/orders.rs`**: boot `nsh_starter::App`, press `id:page-orders` on Home, drive the page. Break the code once to see the test fail. Add the page to `PAGES` in `tests/smoke/pages.test.tsx`.

6. **Check:** `just rules && just test orders && just check`.

A page of one item (`Route::Order(u64)`) lives in the same folder (`order.rs`), has its own record with `content: Content::WithId(..)`, returns its id from `Route::id` and its parent from `Route::parent`. `src/showcase/data/` shows it.
