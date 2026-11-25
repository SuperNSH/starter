// The Items template for the vision pass: the skeletons, the loaded list,
// an opened item (pushed on a phone, presented over the list on desktop
// widths), and the failure with Retry, in light and dark,
// at phone width (390 × 844) and desktop width (1280 × 800). Run on a real
// target: `nsh test --target macos|ios tests/vision/items.test.tsx`; iOS
// and Android reject a width, so the desktop tests log a skip there.
import { open, test } from "nsh/test";
import type { App } from "nsh/test";

// Per-target runs write over these; copy them out between targets.
const OUT = "docs/evidence/project/vision";
const TEST_TIMEOUT = 180_000;

async function capture(app: App, name: string): Promise<void> {
  try {
    await app.screenshot(`${OUT}/${name}.png`);
  } catch (error) {
    if (!String(error).includes("needs a real platform")) throw error;
  }
}

/** The app at `scheme` on a manual clock, at desktop width when
 * `desktop`; null where the target fills the screen and rejects a width. */
async function openAt(scheme: "light" | "dark", desktop: boolean): Promise<App | null> {
  if (!desktop) return open("../..", { clock: {}, colorScheme: scheme });
  try {
    return await open("../..", { clock: {}, colorScheme: scheme, width: 1280, height: 800 });
  } catch (error) {
    if ((error as { code?: string }).code !== "ERR_NSH_TEST_UNSUPPORTED") throw error;
    console.log("skipped: this target has no desktop width");
    return null;
  }
}

/** Home's entries in order, as `tests/smoke/pages.test.tsx` counts them:
 * a phone's Home scrolls, and an entry under a bar takes the bar's press. */
const HOME_INDEX: Record<string, number> = {
  "page-data_devtools": 4,
  "page-items": 12,
  "page-settings": 13,
  "page-sign_in": 16,
};

/** Where Home's scroll stands in the test's app. */
let homeOffset = 0;

/** Presses a Home entry, scrolled into view where Home scrolls. */
async function openFromHome(app: App, id: string): Promise<void> {
  // Home keeps its offset: scroll by the difference, never past an end.
  const to = Math.max(0, HOME_INDEX[id] * 110 - 250);
  try {
    if (to !== homeOffset) await app.scroll("id:home-scroll", to - homeOffset);
    homeOffset = to;
  } catch {
    // The desktop Home fits without scrolling.
  }
  await app.press(`id:${id}`);
}

/** Back to Home: back on a phone, the sidebar's Home row on desktop. */
async function home(app: App): Promise<void> {
  await app.back();
  await app.wait(400);
  if (app.count("id:nav-home") > 0) await app.press("id:nav-home >> nth=0");
  await app.waitFor("text:Home");
}

for (const desktop of [false, true]) {
  const width = desktop ? "desktop" : "phone";
  for (const scheme of ["light", "dark"] as const) {
    test(`items, ${scheme}, ${width}`, async () => {
      const app = await openAt(scheme, desktop);
      if (!app) return;
      homeOffset = 0;
      await app.waitFor("text:Home");

      // The fake API answers after its latency: the skeletons first.
      await openFromHome(app, "page-items");
      await app.waitFor("id:items-loading");
      await capture(app, `items-loading-${scheme}-${width}`);
      await app.clock!.runFor(1000);
      await app.waitFor("id:items-row-3");
      await app.wait(400);
      await capture(app, `items-list-${scheme}-${width}`);

      await app.press("id:items-row-3");
      await app.clock!.runFor(1000);
      await app.waitFor("text:By Alan Kay");
      await app.wait(400);
      await capture(app, `items-detail-${scheme}-${width}`);

      // Every request fails: the error and its Retry.
      await home(app);
      await openFromHome(app, "page-data_devtools");
      await app.waitFor("text:Fake API");
      await app.press("role:button[All]");
      await home(app);
      await openFromHome(app, "page-items");
      await app.clock!.runFor(3000);
      await app.waitFor("text:Couldn't load the items", { timeout: 10_000 });
      await app.wait(400);
      await capture(app, `items-error-${scheme}-${width}`);
    }, TEST_TIMEOUT);
  }
}
