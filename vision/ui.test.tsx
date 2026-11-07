// The UI gallery for the vision pass: the whole styled layer captured in
// both appearances (light, dark) at both widths (phone 390 × 844, desktop
// 1280 × 800), through the real shell (its home row is labeled
// "UI gallery"). Run on a real target so the screenshots draw:
// `nsh test --target macos vision/ui.test.tsx`; headless draws nothing.
//
// `shows()` matches every mounted text, so visibility cannot be probed:
// each test sweeps the gallery scroll in fixed flicks and screenshots
// every stop. The stops tile the full content height, so every section
// (Buttons, Choices from a list, Text and marks, Fields, Controls,
// Navigation rows, Selection and toggles, Pickers, Overlays, Data views,
// Surfaces, Link and scroll bar) appears in at least one capture of each
// of the four combos.
import { open, test } from "nsh/test";

const OUT = "docs/evidence/ui/vision";
const SCROLL_VIEW = "id:gallery-scroll";
/** Points per flick down the gallery scroll. */
const STEP = 1300;
/** Stops per sweep, top to bottom. */
const STOPS = 14;
const TEST_TIMEOUT = 300_000;

type GalleryApp = Awaited<ReturnType<typeof open>>;

for (const scheme of ["light", "dark"] as const) {
  test(`gallery sweep, ${scheme}, phone`, async () => {
    const app = await open("..", { colorScheme: scheme });
    await app.press("label:UI gallery");
    await app.waitFor("text:UI gallery");
    // A nudge makes the next step wait out the push slide before the
    // first capture (stop-00 otherwise catches the page mid-slide).
    await app.scroll(SCROLL_VIEW, 2);
    await sweep(app, scheme, "phone");
  }, TEST_TIMEOUT);

  test(`gallery sweep, ${scheme}, desktop`, async () => {
    const app = await open("..", { colorScheme: scheme });
    await app.press("label:UI gallery");
    await app.waitFor("text:UI gallery");
    await app.setViewport(1280, 800);
    await app.waitFor("text:UI gallery");
    await app.scroll(SCROLL_VIEW, 2);
    await sweep(app, scheme, "desktop");
  }, TEST_TIMEOUT);
}

/** Screenshots the gallery at every stop of one top-to-bottom sweep. */
async function sweep(app: GalleryApp, scheme: string, width: string) {
  for (let stop = 0; stop < STOPS; stop += 1) {
    if (stop > 0) {
      await app.scroll(SCROLL_VIEW, STEP);
    }
    const name = `gallery-stop-${String(stop).padStart(2, "0")}-${scheme}-${width}.png`;
    await app.screenshot(`${OUT}/${name}`);
  }
}
