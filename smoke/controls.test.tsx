// 0.6.0 draws the mobile Switch and DateTimePicker with the system
// controls and iOS sheets with detents: look at them in both schemes.
// Run: `nsh test --target ios|android smoke/controls.test.tsx`; the
// captures go to docs/evidence/shell/controls/.
import { open, test } from "nsh/test";
import type { App } from "nsh/test";

const OUT = "docs/evidence/shell/controls";

async function shot(app: App, name: string): Promise<void> {
  try {
    await app.wait(800);
    await app.screenshot(`${OUT}/${name}.png`);
  } catch {
    // Headless draws nothing.
  }
}

test("forms switch and date picker, overlays sheet, light and dark", async () => {
  const app = await open("..", { clock: {} });
  await app.waitFor("text:Home");
  for (const scheme of ["light", "dark"] as const) {
    await app.setColorScheme(scheme);
    await app.scroll("id:home-scroll", -10000);
    await app.scroll("id:home-scroll", 150);
    await app.press("id:page-forms");
    await app.waitFor("id:forms-scroll");
    await shot(app, `forms-top-${scheme}`);
    await app.scroll("id:forms-scroll", 700);
    await shot(app, `forms-mid-${scheme}`);
    await app.press("id:signup-birth-date");
    await shot(app, `forms-date-${scheme}`);
    await app.back();
    await app.scroll("id:forms-scroll", 600);
    await shot(app, `forms-switch-${scheme}`);
    await app.back();
    await app.waitFor("text:Home");

    await app.scroll("id:home-scroll", 900);
    await app.press("id:page-overlays");
    await app.waitFor("text:Dialogs, popups, menus, toasts and desktop surfaces, each in a flow.");
    await app.scroll("text:Dialog", 1900);
    await shot(app, `overlays-page-${scheme}`);
    await app.press("text:Notification settings…");
    await shot(app, `overlays-sheet-${scheme}`);
    await app.back();
    await app.wait(800);
    await app.back();
    await app.wait(800);
    await app.waitFor("text:Home");
  }
}, 240_000);
