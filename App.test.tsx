// The Rust root tested from TypeScript: `nsh test` runs this file in a
// headless runner it builds with the crate, and `open(".")` starts the root
// (`nsh.json` "root"). The shell area keeps this file current with the root.
import { expect, open, test } from "nsh/test";

test("the root mounts", async () => {
  const app = await open(".");
  expect(app.texts()).toContain("nsh-starter");
});
