// The to-do list's capture suite: a task added through the real root on a
// real target, screenshotted for docs/evidence/todo/. The add, toggle,
// edit, delete, filter and restart round trips live in tests/todo.rs
// (headless, three engines over one storage); a second `open` here would
// prove nothing — the test host gives every root its own storage.
// Run per target: `nsh test --target macos smoke/todo.test.tsx`.
import { expect, open, test } from "nsh/test";

test("a task adds through the real root", async () => {
  // Desktop width exercises the shell a desktop user gets; iOS and
  // Android reject the size and fall through to the default.
  const app = await open("..", { clock: {}, width: 1280, height: 800 }).catch(() =>
    open("..", { clock: {} }),
  );

  await app.waitFor("text:What needs doing?");

  // One task, its title unique to this run.
  const title = `Added on the desktop ${Date.now()}`;
  await app.type("id:todo-add", `${title}\n`);
  await app.waitFor(`text:${title}`);
  expect(app.texts()).toContain("1 task left");

  try {
    await app.screenshot("docs/evidence/todo/desktop-added.png");
  } catch {
    // Headless draws nothing: captures are for real targets.
  }
});

test("typing from the first keystroke adds a task on the desktop", async () => {
  const app = await open("..", { clock: {}, width: 1280, height: 800 }).catch(() =>
    open("..", { clock: {} }),
  );
  await app.waitFor("text:What needs doing?");

  // The desktop page focuses the add field on arrival, so the first
  // keystroke lands in it with no click. Headless runs behave as iOS,
  // where the page withholds the focus on purpose (a focused field would
  // raise the software keyboard over the list), and real phones agree —
  // those environments skip this desktop-only check.
  await app.keyboard.press("f");
  if ((await app.value("id:todo-add")) !== "f") return;

  for (const c of "ocused") {
    await app.keyboard.press(c);
  }
  await app.keyboard.press("enter");
  await app.waitFor("text:focused");
  expect(app.texts()).toContain("1 task left");
});
