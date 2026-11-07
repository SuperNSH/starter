//! The canvas area's tests, on a manual clock: the responder drag leaves
//! a stroke and undo removes it, the drawing survives a re-layout, the
//! plots redraw on new data, and the frame loop advances only while the
//! manual clock runs and its pause holds. The shader and the web view
//! tests cover what headless can see: that the WGSL compiled and the
//! page's Rust side of the messaging is wired (no page runs headless).

use std::time::Duration;

use nsh_core::{Size, prelude::*};
use nsh_engine::{Clock, EngineConfig, Root};
use nsh_starter::features::canvas::canvas_page;
use nsh_testing::Headless;

/// The page alone: no shell, so selectors name only this area's views.
#[component]
fn CanvasRoot() -> impl IntoElement {
    canvas_page()
}

/// Tall enough that every section is on screen at once; a manual clock,
/// so the frame loop moves only when a test runs it.
fn app() -> Headless {
    Headless::start(EngineConfig {
        viewport: Size::new(px(390.0), px(3200.0)),
        clock: Clock::manual(std::time::SystemTime::UNIX_EPOCH),
        ..EngineConfig::new(Root::of::<CanvasRoot>())
    })
    .expect("the engine starts")
}

/// The tick the frame loop shows, if it is on screen.
fn tick_of(app: &mut Headless) -> Option<u32> {
    app.screen()
        .texts()
        .into_iter()
        .find_map(|t| t.strip_prefix("tick ")?.parse().ok())
}

#[test]
fn a_dragged_stroke_stays_and_undo_removes_it() -> Result<(), String> {
    let mut app = app();
    app.wait_for("text:/^0 strokes/")?;
    app.drag("id:sketch", point(px(90.0), px(50.0)))?;
    app.wait_for("text:/^1 strokes/")?;
    app.press("text:Undo")?;
    app.wait_for("text:/^0 strokes/")?;
    Ok(())
}

#[test]
fn the_drawing_survives_a_re_layout() -> Result<(), String> {
    let mut app = app();
    // `start` can return before the first frame, so wait for one first.
    app.wait_for("text:Sketch")?;
    app.drag("id:sketch", point(px(40.0), px(30.0)))?;
    app.wait_for("text:/^1 strokes/")?;
    // A wider window lays the canvas out again; the strokes are data, so
    // they redraw in the new width.
    app.set_viewport(Size::new(px(760.0), px(3200.0)))?;
    app.wait_for("text:/^1 strokes/")?;
    assert_eq!(app.screen().count("text:/^1 strokes/")?, 1);
    Ok(())
}

#[test]
fn the_charts_redraw_when_the_data_changes() -> Result<(), String> {
    let mut app = app();
    app.wait_for("text:/^batch #7 ·/")?;
    app.press("text:New data")?;
    app.wait_for("text:/^batch #8 ·/")?;
    Ok(())
}

#[test]
fn the_frame_loop_advances_on_the_manual_clock_and_a_pause_holds_it() -> Result<(), String> {
    let mut app = app();
    app.wait_for("text:/^tick /")?;
    // Pause, reset, play: the effect restarts the interval, so the ticks
    // that follow come from this moment on the manual clock.
    app.press("id:sim-toggle >> text:Pause")?;
    app.press("text:Reset")?;
    app.wait_for("text:tick 0")?;
    app.press("id:sim-toggle >> text:Play")?;
    app.run_for(Duration::from_millis(1000))?;
    let played = tick_of(&mut app).unwrap_or_else(|| panic!("the tick readout is on screen"));
    assert!(played >= 25, "a second of clock ticks the loop: {played}");
    app.press("id:sim-toggle >> text:Pause")?;
    app.run_for(Duration::from_millis(1000))?;
    let paused = tick_of(&mut app).unwrap_or_else(|| panic!("the tick readout is on screen"));
    assert_eq!(paused, played, "a paused loop holds its tick");
    Ok(())
}

#[test]
fn the_shader_compiles_and_its_children_draw_over_it() -> Result<(), String> {
    let mut app = app();
    // The caption is the shader's child: it mounts only over a compiled
    // program (a failed compile takes the fallback branch, whose text
    // differs). Headless draws no pixels; this proves the WGSL validated.
    app.wait_for("text:Aurora — WGSL over this box")?;
    assert!(!app.shows("Shader unavailable"));
    app.press("id:shader-pause >> text:Pause")?;
    app.wait_for("text:Resume")?;
    Ok(())
}

#[test]
fn posting_to_the_web_view_page_records_the_message() -> Result<(), String> {
    let mut app = app();
    // No page runs headless, so nothing comes back from it; the app's own
    // side of the exchange is what a headless run can see.
    app.wait_for("text:Post to the page")?;
    app.press("id:post-to-page >> text:Post to the page")?;
    app.wait_for("text:hi #1")?;
    Ok(())
}
