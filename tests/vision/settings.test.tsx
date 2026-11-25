// The Settings page for the vision pass: the page top and its end (the
// telemetry card), signed out and signed in, in Chinese, in light and dark,
// at phone width (390 × 844) and desktop width (1280 × 800). Run on a real
// target: `nsh test --target macos|ios tests/vision/settings.test.tsx`;
// iOS and Android reject a width, so the desktop tests log a skip there.
import { open, test } from "nsh/test";
import type { App } from "nsh/test";

// Per-target runs write over these; copy them out between targets.
const OUT = "docs/evidence/project/vision";
const TEST_TIMEOUT = 180_000;
/** Points from the page top to its end on a phone, roughly. */
const TO_BOTTOM = 700;

async function capture(app: App, name: string): Promise<void> {
  // Let a push or a scroll settle on a device before the picture.
  await app.wait(400);
  try {
    await app.screenshot(`${OUT}/${name}.png`);
  } catch (error) {
    if (!String(error).includes("needs a real platform")) throw error;
  }
}

/** The app at `scheme`, at desktop width when `desktop`; null where the
 * target fills the screen and rejects a width. */
async function openAt(scheme: "light" | "dark", desktop: boolean): Promise<App | null> {
  if (!desktop) return open("../..", { colorScheme: scheme });
  try {
    return await open("../..", { colorScheme: scheme, width: 1280, height: 800 });
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

/** Presses a Home entry, scrolled into view where Home scrolls. */
async function openFromHome(app: App, id: string): Promise<void> {
  try {
    const index = HOME_INDEX[id];
    if (index > 2) await app.scroll("id:home-scroll", index * 90 - 250);
  } catch {
    // The desktop Home fits without scrolling.
  }
  await app.press(`id:${id}`);
}

async function openSettings(app: App): Promise<void> {
  await app.waitFor("text:Home");
  await openFromHome(app, "page-settings");
  await app.waitFor("id:settings-scroll");
}

/** Pops Sign in; the macOS sidebar shell ignores back, so its row reopens
 * Settings there. */
async function backToSettings(app: App): Promise<void> {
  await app.back();
  await app.wait(400);
  if (app.count("id:settings-scroll") === 0) await app.press("id:page-settings >> nth=0");
  await app.waitFor("id:settings-scroll");
}

async function bottom(app: App): Promise<void> {
  try {
    await app.scroll("id:settings-scroll", TO_BOTTOM);
  } catch {
    // Already showing the whole page.
  }
}

for (const desktop of [false, true]) {
  const width = desktop ? "desktop" : "phone";
  for (const scheme of ["light", "dark"] as const) {
    test(`settings, ${scheme}, ${width}`, async () => {
      const app = await openAt(scheme, desktop);
      if (!app) return;
      await openSettings(app);
      await app.waitFor("text:You are not signed in.");
      await capture(app, `settings-top-${scheme}-${width}`);
      await bottom(app);
      await app.waitFor("text:Telemetry");
      await capture(app, `settings-bottom-${scheme}-${width}`);

      // Signed in through the Account card's button, then back.
      await app.scroll("id:settings-scroll", -TO_BOTTOM);
      await app.press("role:button[Sign in]");
      await app.waitFor("id:sign_in-email");
      await app.type("id:sign_in-email", "ada@example.com");
      await app.type("id:sign_in-password", "hunter22");
      await app.press("role:button[Sign in]");
      await app.waitFor("id:sign_in-account", { timeout: 10_000 });
      await backToSettings(app);
      await app.waitFor("text:Mock User");
      await capture(app, `settings-signed-in-${scheme}-${width}`);

      // Every string of the page re-renders in Chinese.
      await app.press("role:button[中文]");
      await app.waitFor("text:语言");
      await capture(app, `settings-zh-${scheme}-${width}`);
      await app.press("role:button[English]");
      await app.waitFor("text:Language");
    }, TEST_TIMEOUT);
  }
}
