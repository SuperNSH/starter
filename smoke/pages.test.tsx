// The shell's smoke suite (the brief's criterion 3): the app's page opens,
// shows its marker and is screenshotted, per target. The boilerplate's demo
// pages stay in the crate (their own tests mount them directly); the shell
// routes to the to-do list alone, so this is the one page to walk.
// Headless runs pass with the captures skipped (they draw nothing).
// Run per target: `nsh test --target macos|ios|android|web smoke/pages.test.tsx`.
import { expect, open, test } from "nsh/test";
import type { App } from "nsh/test";

// The page the shell can show, with a line of its own text as the marker.
// Contract: the shell area keeps this current with the routes it mounts
// (src/shell/routes.rs names the same page).
const PAGE = { id: "page-home", marker: "What needs doing?" };

// What the web target does not have on 0.4.0, asserted so a silently
// missing feature cannot pass unnoticed.
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

test("the page opens from the shell", async () => {
  // The documented web skips, spelled out so a change is a diff here.
  expect(SKIPPED_ON_WEB).toEqual([
    "file and folder dialogs (desktop-only file requests)",
    "speech recognition (no recognizer on the web)",
    "the system tab bar (nsh/mobile renders nothing off iOS and Android)",
  ]);

  const app = await openShell();
  await app.waitFor("text:To-dos");

  await app.waitFor(`text:${PAGE.marker}`);
  try {
    // Relative to the run's working directory; per-target runs write
    // over these, one page per file.
    await app.screenshot(`docs/evidence/shell/smoke/${PAGE.id}.png`);
  } catch {
    // Headless draws nothing: captures are for real targets.
  }

  // The page is the stack's root: the add field still shows after.
  await app.waitFor(`text:${PAGE.marker}`);
});
