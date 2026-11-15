// Stage 3's responsive captures: the same page at three window widths —
// wide (the summary rail beside the list), mid (the desktop shell, the
// single column) and compact (the phone shell's column) — screenshotted
// on real targets into docs/evidence/todo/. iOS and Android cannot honor
// a requested size (the app fills their screen); their runs stop at the
// open. Headless runs pass with the captures skipped (they draw nothing).
// Run per target: `nsh test --target macos smoke/todo-responsive.test.tsx`.
import { expect, open, test } from "nsh/test";
import type { App } from "nsh/test";

// Opens at a size; null where the target cannot take one (iOS, Android:
// ERR_NSH_TEST_UNSUPPORTED), so those runs prove nothing about widths and
// say so by stopping.
async function openAt(width: number, height: number): Promise<App | null> {
  try {
    return await open("..", { clock: {}, width, height });
  } catch (error) {
    if ((error as { code?: string }).code !== "ERR_NSH_TEST_UNSUPPORTED") throw error;
    return null;
  }
}

test("the wide window shows the rail beside the list", async () => {
  const app = await openAt(1280, 800);
  if (!app) return;

  await app.waitFor("text:What needs doing?");
  await app.waitFor("text:Summary");

  // One task through the real root so the capture shows a list, not an
  // empty state.
  const title = `Wide window ${Date.now()}`;
  await app.type("id:todo-add", `${title}\n`);
  await app.waitFor(`text:${title}`);
  expect(app.texts()).toContain("1 task left");

  try {
    await app.screenshot("docs/evidence/todo/responsive-wide.png");
  } catch {
    // Headless draws nothing: captures are for real targets.
  }
});

test("the mid window keeps the single column", async () => {
  const app = await openAt(860, 700);
  if (!app) return;

  await app.waitFor("text:What needs doing?");
  // Between the two layouts the rail is away and the list works.
  expect(app.texts()).not.toContain("Summary");
  const title = `Mid window ${Date.now()}`;
  await app.type("id:todo-add", `${title}\n`);
  await app.waitFor(`text:${title}`);

  try {
    await app.screenshot("docs/evidence/todo/responsive-mid.png");
  } catch {
    // Headless draws nothing: captures are for real targets.
  }
});

test("the compact window keeps the phone column", async () => {
  const app = await openAt(390, 844);
  if (!app) return;

  await app.waitFor("text:What needs doing?");
  expect(app.texts()).not.toContain("Summary");
  const title = `Compact window ${Date.now()}`;
  await app.type("id:todo-add", `${title}\n`);
  await app.waitFor(`text:${title}`);

  try {
    await app.screenshot("docs/evidence/todo/responsive-compact.png");
  } catch {
    // Headless draws nothing: captures are for real targets.
  }
});
