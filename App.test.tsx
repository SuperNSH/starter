// The Rust root tested from TypeScript: `nsh test` runs this file in a
// headless runner it builds with the crate, and `open(".")` starts the root
// (`nsh.json` "root"). The shell area keeps this file current with the root.
import { expect, open, test } from "nsh/test";

test("the root mounts", async () => {
  const app = await open(".", { clock: {} });
  await app.waitFor("text:To-dos");
  // The phone shell names the page in the system header; the page's own
  // add field is what the root drew.
  await app.waitFor("text:What needs doing?");
  expect(app.texts()).toContain("No tasks yet — add the first one above.");
});

test("a task adds from the root", async () => {
  const app = await open(".", { clock: {} });
  await app.waitFor("text:To-dos");
  await app.type("id:todo-add", "From the TSX root\n");
  await app.waitFor("text:From the TSX root");
  // Back on the page asks to exit: the one page is the stack's root.
  await app.back();
  await app.waitFor("text:Exit todo-sample?");
  await app.press("role:button[Cancel]");
});
