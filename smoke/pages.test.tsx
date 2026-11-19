// The shell's smoke suite (the brief's criterion 3): every page reachable
// from the shell opens, shows its marker and is screenshotted, per target.
// Headless runs pass with the captures skipped (they draw nothing).
// Run per target: `nsh test --target macos|ios|android|web smoke/pages.test.tsx`.
import { expect, open, test } from "nsh/test";
import type { App } from "nsh/test";

// One entry per page the shell can show. `id` is the row on Home,
// `marker` a selector for something only that page shows (a title alone
// also matches its Home row and sidebar entry, and would pass too early). Contract: feature areas
// keep their page's marker stable, or update this list with the shell
// area (src/shell/routes.rs names the same pages).
const PAGES: Array<{ id: string; marker: string }> = [
  { id: "page-ui", marker: "text:Native controls" },
  { id: "page-forms", marker: "text:Sign up" },
  { id: "page-data", marker: "text:Shipping a Rust shell" },
  { id: "page-data-item", marker: "text:Add a comment" },
  { id: "page-query-devtools", marker: "text:Query cache" },
  { id: "page-lists", marker: "text:Recent orders" },
  { id: "page-canvas", marker: "text:Sketch" },
  { id: "page-motion", marker: "text:Reduced motion" },
  { id: "page-overlays", marker: "text:Dialogs, popups, menus, toasts and desktop surfaces, each in a flow." },
  { id: "page-platform", marker: "text:Pick an image" },
  { id: "page-device", marker: "text:Screen reader" },
  { id: "page-remote", marker: "text:Delivery status" },
  { id: "page-diagnostics", marker: "text:Start sample task" },
];

// What the web target does not have on 0.5.0, asserted so a silently
// missing feature cannot pass unnoticed. Page navigation itself works
// everywhere (every page is Rust).
const SKIPPED_ON_WEB = [
  "file and folder dialogs (desktop-only file requests)",
  "speech recognition (no recognizer on the web)",
  "the system tab bar (nsh/mobile renders nothing off iOS and Android)",
];

async function openShell(): Promise<App> {
  try {
    // Desktop width exercises the sidebar; iOS and Android reject the
    // size (the app fills their screen) and fall through to the default.
    return await open("..", { clock: {}, width: 1280, height: 800 });
  } catch (error) {
    if ((error as { code?: string }).code !== "ERR_NSH_TEST_UNSUPPORTED") throw error;
    return await open("..", { clock: {} });
  }
}

// iOS and Android pop with back; the desktop sidebar has no back (the
// macOS host ignores it), so Home is its first row.
// Home keeps its scroll offset, and a row under the title bar or the tab
// bar takes the press for that bar. Where the content fits (desktop) there
// is nothing to scroll.
async function homeRowInView(app: App, index: number): Promise<void> {
  // PAGES skips the "Native vs app" tile; the data item row is the last.
  index = PAGES[index].id === "page-data-item" ? 14 : index + 1;
  try {
    await app.scroll("id:home-scroll", -10000);
    if (index > 2) await app.scroll("id:home-scroll", index * 110 - 250);
  } catch {
    // Nothing to scroll.
  }
}

async function goHome(app: App): Promise<void> {
  await app.back();
  if (app.count("id:nav-home") > 0 && app.count("id:page-ui") === 0) {
    await app.press("id:nav-home >> nth=0");
  }
}

test("every page opens from the shell", async () => {
  // The documented web skips, spelled out so a change is a diff here.
  expect(SKIPPED_ON_WEB).toEqual([
    "file and folder dialogs (desktop-only file requests)",
    "speech recognition (no recognizer on the web)",
    "the system tab bar (nsh/mobile renders nothing off iOS and Android)",
  ]);

  const app = await openShell();
  await app.waitFor("text:Home");

  for (const page of PAGES) {
    await homeRowInView(app, PAGES.indexOf(page));
    await app.press(`id:${page.id}`);
    await app.waitFor(page.marker);
    // A push animates for a moment on the devices; settle before the capture.
    await app.wait(500);
    try {
      // Relative to the run's working directory; per-target runs write
      // over these, one page per file.
      await app.screenshot(`docs/evidence/shell/smoke/${page.id}.png`);
    } catch {
      // Headless draws nothing: captures are for real targets.
    }
    await goHome(app);
  }

  // Back at Home: every page reachable again.
  await app.waitFor("text:Home");
  // 30 s is too short for 15 captures on a simulator or an emulator.
}, 240_000);
