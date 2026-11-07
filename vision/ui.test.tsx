// The UI gallery for the vision pass: the whole styled layer captured in
// both appearances at phone and desktop width. Run on a real target so the
// screenshots draw (`nsh test --target macos vision/ui.test.tsx`); headless
// draws nothing. Navigation assumes the shell exposes the gallery under the
// label "UI gallery".
import { open, test } from "nsh/test";

const OUT = "docs/evidence/ui";

/** Opens the app on the gallery, scrolled back to the top. */
async function gallery(scheme: "light" | "dark", width: number, height: number) {
  const app = await open(".", {
    colorScheme: scheme,
    width,
    height,
  });
  await app.press("label:UI gallery");
  await app.waitFor("text:UI gallery");
  return app;
}

for (const scheme of ["light", "dark"] as const) {
  test(`buttons, text and fields draw, in ${scheme} at phone width`, async () => {
    const app = await gallery(scheme, 390, 844);
    await app.waitFor("text:Buttons");
    await app.screenshot(`${OUT}/gallery-buttons-${scheme}-phone.png`);
    await app.scroll("text:Text and marks", 600);
    await app.waitFor("text:Fields");
    await app.screenshot(`${OUT}/gallery-fields-${scheme}-phone.png`);
  });

  test(`controls and choices draw, in ${scheme} at phone width`, async () => {
    const app = await gallery(scheme, 390, 844);
    await app.scroll("text:Controls", 1600);
    await app.waitFor("text:Controls");
    await app.screenshot(`${OUT}/gallery-controls-${scheme}-phone.png`);
    await app.scroll("text:Choices from a list", 1600);
    await app.waitFor("text:Choices from a list");
    await app.screenshot(`${OUT}/gallery-choices-${scheme}-phone.png`);
  });

  test(`pickers draw, in ${scheme} at phone width`, async () => {
    const app = await gallery(scheme, 390, 844);
    await app.scroll("text:Pickers", 2600);
    await app.waitFor("text:Pickers");
    await app.screenshot(`${OUT}/gallery-pickers-${scheme}-phone.png`);
  });

  test(`overlays open on the gallery, in ${scheme} at phone width`, async () => {
    const app = await gallery(scheme, 390, 844);
    await app.scroll("text:Overlays", 3400);
    await app.waitFor("text:Overlays");
    // The menu, open at its trigger.
    await app.press("role:button[Actions]");
    await app.waitFor("text:Rename…");
    await app.screenshot(`${OUT}/gallery-menu-open-${scheme}-phone.png`);
    await app.press("text:Rename…");
    // The dialog card, open with its decision visible.
    await app.press("role:button[Open dialog]");
    await app.waitFor("text:Delete the draft?");
    await app.screenshot(`${OUT}/gallery-dialog-open-${scheme}-phone.png`);
    await app.press("role:button[Cancel]");
    await app.waitFor("text:Delete the draft?", { state: "gone" });
  });

  test(`data views and surfaces draw, in ${scheme} at phone width`, async () => {
    const app = await gallery(scheme, 390, 844);
    await app.scroll("text:Data views", 4400);
    await app.waitFor("text:Data views");
    await app.screenshot(`${OUT}/gallery-data-${scheme}-phone.png`);
    await app.scroll("text:Surfaces", 4400);
    await app.waitFor("text:Surfaces");
    await app.screenshot(`${OUT}/gallery-surfaces-${scheme}-phone.png`);
  });

  test(`the gallery draws at desktop width, in ${scheme}`, async () => {
    const app = await gallery(scheme, 1200, 800);
    await app.waitFor("text:Buttons");
    await app.screenshot(`${OUT}/gallery-top-${scheme}-desktop.png`);
    await app.scroll("text:Controls", 2000);
    await app.waitFor("text:Controls");
    await app.screenshot(`${OUT}/gallery-controls-${scheme}-desktop.png`);
    await app.scroll("text:Surfaces", 5000);
    await app.waitFor("text:Surfaces");
    await app.screenshot(`${OUT}/gallery-surfaces-${scheme}-desktop.png`);
  });
}
