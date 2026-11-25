// The Sign in template for the vision pass: the empty form, its field
// errors after a submit, the provider's refusal, and the signed-in account
// card, in light and dark, at phone width (390 × 844) and desktop width
// (1280 × 800). The provider is the app's local mock: no account, no
// network. Run on a real target: `nsh test --target macos|ios
// tests/vision/sign_in.test.tsx`; iOS and Android reject a width, so the
// desktop tests log a skip there.
import { open, test } from "nsh/test";
import type { App } from "nsh/test";

// Per-target runs write over these; copy them out between targets.
const OUT = "docs/evidence/project/vision";
const TEST_TIMEOUT = 180_000;

async function capture(app: App, name: string): Promise<void> {
  // Let a push or the keyboard settle on a device before the picture.
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

/** Home's rows sit under its cards; `tests/smoke/pages.test.tsx` counts
 * the same offset. A row under a bar takes the bar's press. */
async function openSignIn(app: App): Promise<void> {
  await app.waitFor("text:Home");
  try {
    await app.scroll("id:home-scroll", 16 * 90 - 250);
  } catch {
    // The desktop Home fits without scrolling.
  }
  await app.press("id:page-sign_in");
  await app.waitFor("id:sign_in-email");
}

for (const desktop of [false, true]) {
  const width = desktop ? "desktop" : "phone";
  for (const scheme of ["light", "dark"] as const) {
    test(`sign in, ${scheme}, ${width}`, async () => {
      const app = await openAt(scheme, desktop);
      if (!app) return;
      await openSignIn(app);
      await capture(app, `sign-in-empty-${scheme}-${width}`);

      // Rules speak after the first submit.
      await app.press("role:button[Sign in]");
      await app.waitFor("text:Enter a valid email address");
      await app.waitFor("text:Enter your password");
      await capture(app, `sign-in-errors-${scheme}-${width}`);

      // The mock refuses this one password.
      await app.type("id:sign_in-email", "ada@example.com");
      await app.type("id:sign_in-password", "wrong-password");
      await app.press("role:button[Sign in]");
      await app.waitFor("text:Wrong email or password", { timeout: 10_000 });
      await capture(app, `sign-in-refused-${scheme}-${width}`);

      await app.clear("id:sign_in-password");
      await app.type("id:sign_in-password", "hunter22");
      await app.press("role:button[Sign in]");
      await app.waitFor("id:sign_in-account", { timeout: 10_000 });
      await capture(app, `sign-in-account-${scheme}-${width}`);

      await app.press("role:button[Sign out]");
      await app.waitFor("id:sign_in-email");
    }, TEST_TIMEOUT);
  }
}
