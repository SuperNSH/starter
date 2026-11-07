// The vision pass over the platform pages: it navigates from the shell's
// home, captures each page, and turns the locale switch so the zh-CN render
// is captured too — the device page's word values translate with it.
// `app.screenshot` draws only on a target run — start it with
// `nsh test --target macos` (or `--target ios`); a headless run has no
// pixels, and the captures report as skipped.
//
// Shell contract: the home page's rows carry the route's key (`page-platform`,
// `page-device`), and back pops the pushed page.
import { expect, open, test } from "nsh/test";
import type { App } from "nsh/test";

const OUT = "docs/evidence/platform";

async function capture(app: App, name: string, locale: string) {
  const path = `${OUT}/${name}-${locale}.png`;
  try {
    await app.screenshot(path);
    console.log(`captured ${path}`);
  } catch (error) {
    console.log(`SKIPPED: ${path} (${error})`);
  }
}

test("captures the platform and device pages in both locales", async () => {
  // The app root is one level up: `open` resolves a Rust root through the
  // nearest nsh.json, starting at the test file's directory.
  const app = await open("..");

  await app.press("id:page-platform");
  await app.waitFor("Clipboard");
  await capture(app, "platform", "en");

  // The language card on the page itself: every string re-renders.
  await app.press("中文");
  await app.waitFor("剪贴板");
  await capture(app, "platform", "zh-CN");

  await app.back();
  await app.press("id:page-device");
  await app.waitFor("像素比");
  await capture(app, "device", "zh-CN");

  // Back to English through the language card for the second device pass.
  await app.back();
  await app.press("id:page-platform");
  await app.waitFor("剪贴板");
  await app.press("English");
  await app.waitFor("Clipboard");
  await app.back();

  await app.press("id:page-device");
  await app.waitFor("Pixel ratio");
  await capture(app, "device", "en");

  expect(app.texts().length).toBeGreaterThan(0);
});
