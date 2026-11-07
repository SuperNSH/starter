// The forms pages for the vision pass: the sign-up fresh and with its
// validation errors shown, in both appearances, then the settings page.
// Run on a real target so the screenshots draw (`nsh test --target macos
// vision/forms.test.tsx`); headless draws nothing. Navigation assumes the
// shell's index exposes the feature under the label "Forms".
import { expect, open, test } from "nsh/test";

// The runner resolves the entry against this file's directory, so the app
// opens from the project root, while screenshot paths resolve against the
// process CWD. The directory must exist; the app's sandbox grants no
// fs.write to make one.
const OUT = "docs/evidence/forms";

/** Headless runs draw nothing; only a real target can keep the captures. */
async function capture(app: { screenshot(path: string): Promise<unknown> }, path: string) {
  try {
    await app.screenshot(`${OUT}/${path}`);
  } catch (error) {
    console.log(`skipped ${path}: ${error}`);
  }
}

for (const scheme of ["light", "dark"] as const) {
  test(`sign-up fresh, then with errors, in ${scheme}`, async () => {
    const app = await open("..", { colorScheme: scheme });
    await app.press("label:Forms");
    await app.waitFor("text:Create account");
    await capture(app, `sign-up-fresh-${scheme}.png`);
    expect(app.texts()).toContain("Sign up");

    // A failed submit shows every rule's message and focuses the email.
    await app.press("text:Create account");
    await app.waitFor("text:Enter your email");
    await capture(app, `sign-up-errors-${scheme}.png`);

    // The settings form the link opens.
    await app.press("text:App settings");
    await app.waitFor("text:Save settings");
    await capture(app, `settings-${scheme}.png`);
  });
}
