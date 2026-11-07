//! Lists and data views, through the UI on a manual clock: sort orders,
//! off-screen jumps keeping per-card state, load-more-at-start prepending
//! without jumping, calendar selection, and pagination.

use std::time::{Duration, SystemTime};

use nsh_base::{DataTableSort, DataTableSortDirection};
use nsh_core::{Size, prelude::*};
use nsh_engine::{Clock, EngineConfig, Root};
use nsh_starter::features::lists::{
    calendar_view::CalendarDemo,
    chat::ChatDemo,
    grid::GridDemo,
    pager::PagerDemo,
    quotes::{QuotesDemo, quote_rows, sort_quotes},
};
use nsh_testing::Headless;

/// The engine's time during the runs: 2026-11-11, so "today" in the
/// calendar is fixed whatever machine runs this.
fn noonish() -> SystemTime {
    SystemTime::UNIX_EPOCH + Duration::from_secs(1_794_355_200)
}

/// Starts a demo as the whole app, phone-height and on the manual clock.
fn start<C: Component<Props = ()>>() -> Headless {
    let config = EngineConfig {
        viewport: Size::new(px(600.0), px(844.0)),
        clock: Clock::manual(noonish()),
        ..EngineConfig::new(Root::of::<C>())
    };
    let mut app = Headless::start(config).expect("the engine starts");
    // `start` can return before the first frame, so wait for something.
    app.first_render().unwrap();
    app
}

/// The quotes' expected order, the way the app sorts them.
fn expected_first(key: &str, direction: DataTableSortDirection) -> String {
    // `#[non_exhaustive]`: built from the default, never as a literal.
    let mut sort = DataTableSort::default();
    sort.key = key.to_owned().into();
    sort.direction = direction;
    let mut rows = quote_rows(3000);
    sort_quotes(&mut rows, &sort);
    rows[0].symbol.clone()
}

#[test]
fn sorting_a_column_orders_the_rows() {
    let mut app = start::<QuotesDemo>();
    // In table order the first quote leads.
    app.wait_for("text:AAPL").unwrap();

    // One press of the price title: ascending, cheapest first.
    app.press("id:sort:price").unwrap();
    let cheapest = expected_first("price", DataTableSortDirection::Ascending);
    app.wait_for(&format!("text:{cheapest}")).unwrap();
    // The old first row sat mid-ranked; it is off screen now.
    assert!(
        app.screen().count("text:AAPL") == Ok(0),
        "the unsorted first row should have left the screen"
    );

    // A second press: descending, priciest first.
    app.press("id:sort:price").unwrap();
    let priciest = expected_first("price", DataTableSortDirection::Descending);
    app.wait_for(&format!("text:{priciest}")).unwrap();
    assert!(
        app.screen().count(&format!("text:{cheapest}")) == Ok(0),
        "the cheapest row should have left the screen"
    );
}

#[test]
fn a_scroll_past_the_screen_keeps_the_stars_with_their_cards() {
    let mut app = start::<GridDemo>();
    app.wait_for("text:Card 1").unwrap();

    // Star the second card: the header counts the card's own state.
    app.press("label:Star 2").unwrap();
    assert_eq!(app.screen().count("text:★"), Ok(1));
    assert_eq!(
        app.screen().count("text:1 starred"),
        Ok(1),
        "the star lives in the cell, not in the card's data"
    );

    // Scroll far down, then further, then half a screen back up: no
    // starred card is in view anywhere there.
    app.scroll("id:grid", px(4000.0)).unwrap();
    app.scroll("id:grid", px(2000.0)).unwrap();
    app.wait_for("text:Card 145").unwrap();
    app.scroll("id:grid", px(-600.0)).unwrap();
    app.wait_for("text:Card 135").unwrap();
    assert_eq!(
        app.screen().count("text:★"),
        Ok(0),
        "a star travelled with a cell"
    );

    // Back almost at the top: the star is still on the second card.
    app.scroll("id:grid", px(-5200.0)).unwrap();
    app.wait_for("text:Card 2").unwrap();
    assert_eq!(app.screen().count("text:★"), Ok(1));
    assert!(
        app.screen().count("label:Star 2") != Ok(0),
        "the star is not on card 2"
    );
}

#[test]
fn loading_newer_messages_prepends_without_jumping() {
    let mut app = start::<ChatDemo>();
    // The inverted chat catches up as it opens: the start loader brought
    // the room's newer pages in, and the newest shows at the bottom edge.
    app.wait_for("text:msg 260").unwrap();

    // A screen up, away from the start.
    app.scroll("id:chat-list", px(-700.0)).unwrap();
    app.wait_for("text:msg 240").unwrap();

    // The messages the screen holds before the arrival:
    let visible = |app: &Headless| {
        let mut ids: Vec<u32> = app
            .screen()
            .texts()
            .iter()
            .filter_map(|t| t.strip_prefix("msg ").and_then(|n| n.parse().ok()))
            .collect();
        ids.sort_unstable();
        ids.dedup();
        ids
    };
    let before = visible(&app);

    // A message arrives at the start while the user is away.
    app.press("label:Receive a message").unwrap();

    // The oldest message the screen holds is still the same one: the
    // visible rows kept their places, and the arrival stayed out of sight.
    let after = visible(&app);
    assert!(
        after.first() == before.first(),
        "the visible messages shifted when the newer one joined: \
         {before:?} -> {after:?}"
    );
    assert!(
        app.screen().count("text:msg 261") == Ok(0),
        "the arrival landed in the visible rows"
    );
}

#[test]
fn picking_a_day_selects_it_and_weekends_refuse() {
    let mut app = start::<CalendarDemo>();
    // The schedule opens holding November 12th, 2026.
    app.wait_for("text:November 2026").unwrap();
    app.wait_for("text:Selected 2026-11-12. Weekends are off.")
        .unwrap();

    // A Wednesday is picked and reported.
    app.press("label:2026-11-18").unwrap();
    app.wait_for("text:Selected 2026-11-18. Weekends are off.")
        .unwrap();

    // The Saturday of that week is off: the press is refused.
    app.press("label:2026-11-14").unwrap_err();
    assert!(
        app.screen()
            .count("text:Selected 2026-11-18. Weekends are off.")
            != Ok(0),
        "a refused press changed the selection"
    );
}

#[test]
fn choosing_a_page_shows_its_rows() {
    let mut app = start::<PagerDemo>();
    app.wait_for("text:Archive 1").unwrap();
    app.wait_for("text:Page 1 of 50").unwrap();

    app.press("text:3").unwrap();
    app.wait_for("text:Page 3 of 50").unwrap();
    app.wait_for("text:Archive 21").unwrap();
    assert!(
        app.screen().count("text:Archive 1") == Ok(0),
        "the first page's rows lingered"
    );
}
