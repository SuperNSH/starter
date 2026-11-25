// The Rust root tested from TypeScript: `nsh test` runs this file in a
// headless runner it builds with the crate, and `open("../..")` starts the root
// (`nsh.json` "root").
import { expect, open, test } from "nsh/test";

test("the root mounts", async () => {
  const app = await open("../..", { clock: {} });
  await app.waitFor("text:Home");
  // The phone shell names the app in the system header only; Home's own
  // line is what the root drew.
  expect(app.texts()).toContain("Everything the boilerplate demonstrates, one page each.");
});

test("the shell navigates from Home", async () => {
  const app = await open("../..", { clock: {} });
  await app.press("id:page-settings");
  await app.waitFor("text:Language");
  // Back pops the pushed page; back again on Home asks to exit.
  await app.back();
  await app.back();
  await app.waitFor("text:Exit nsh-starter?");
  await app.press("role:button[Cancel]");
});
