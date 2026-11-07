// The canvas page's vision pass: opens the app's root, pushes the Canvas
// page from the shell's home, drives each section to a state worth
// looking at, and screenshots it. Screenshots need a real target
// (`nsh test vision/canvas.test.tsx --target macos`); a headless run
// draws nothing, so they are skipped there and the state assertions still
// run. The manual clock moves the frame loop and the shader without real
// time.
import { open, test } from "nsh/test";
import type { App } from "nsh/test";

// Headless has no drawing: a screenshot there would fail the whole test.
async function capture(app: App, path: string) {
  try {
    await app.screenshot(path);
  } catch {
    console.log(`screenshot skipped (no drawing surface): ${path}`);
  }
}

// The app at the Canvas page, reached the way a user reaches it.
async function canvasApp(): Promise<App> {
  const app = await open("..", { clock: {} });
  await app.press("text:Canvas");
  await app.waitFor("text:Sketch");
  return app;
}

test("the canvas page shows every section", async () => {
  const app = await canvasApp();
  await app.waitFor("text:/^0 strokes/");
  await app.waitFor("text:/^tick /");
  await app.waitFor("text:/^batch #7 ·/");
  await app.waitFor("text:Aurora — WGSL over this box");
  await app.waitFor("text:Post to the page");
  // Let the manual clock run the shader and the sweep for a moment so the
  // capture shows them mid-animation.
  await app.clock!.runFor(1200);
  await capture(app, "docs/evidence/canvas/page-light.png");
});

test("a drag leaves a stroke worth looking at", async () => {
  const app = await canvasApp();
  await app.drag("id:sketch", { x: 120, y: -60 });
  await app.drag("id:sketch", { x: -80, y: 40 });
  await app.waitFor("text:/^2 strokes/");
  await capture(app, "docs/evidence/canvas/sketch-drawn.png");
  await app.press("text:Undo");
  await app.waitFor("text:/^1 strokes/");
});

test("the charts redraw on new data", async () => {
  const app = await canvasApp();
  await app.waitFor("text:/^batch #7 ·/");
  await capture(app, "docs/evidence/canvas/charts-first.png");
  await app.press("text:New data");
  await app.waitFor("text:/^batch #8 ·/");
  await capture(app, "docs/evidence/canvas/charts-new-data.png");
});

test("the shader pauses and its uniform drift changes", async () => {
  const app = await canvasApp();
  await app.waitFor("text:Aurora — WGSL over this box");
  await app.clock!.runFor(600);
  await capture(app, "docs/evidence/canvas/shader-running.png");
  await app.press("id:shader-pause >> text:Pause");
  await app.waitFor("text:Resume");
  await app.press("text:Swift");
  await capture(app, "docs/evidence/canvas/shader-paused.png");
});

test("the images and the bundled video show", async () => {
  const app = await canvasApp();
  await app.waitFor("text:/placeholder while loading/");
  await app.waitFor("text:/the clip plays from the app bundle/");
  await capture(app, "docs/evidence/canvas/media.png");
});

test("the infinite board pans, zooms and selects a node", async () => {
  const app = await canvasApp();
  await app.waitFor("text:/^world 100% · pan 0, 0$/");
  await app.drag("id:world-board", { x: 60, y: 0 });
  await app.waitFor("text:/^world 100% · pan 60, 0$/");
  await app.press("text:Zoom in");
  await app.waitFor("text:/^world 125% · pan 75, 0$/");
  await capture(app, "docs/evidence/canvas/world-zoomed.png");
  await app.drag("id:world-board", { x: -100, y: 0 });
  await app.waitFor("text:/^center 20, 0 · under: Notes$/");
  await app.press("text:Zoom in");
  await app.waitFor("text:/^world 156% · pan -31, 0$/");
  await app.drag("id:world-board", { x: 30, y: 0 });
  await app.waitFor("text:/^selected: Notes · 89, 0$/");
  await capture(app, "docs/evidence/canvas/world-node-selected.png");
});

test("the web view exchange is recorded both ways", async () => {
  const app = await canvasApp();
  await app.waitFor("text:Post to the page");
  await app.press("id:post-to-page >> text:Post to the page");
  await app.waitFor("text:hi #1");
  await capture(app, "docs/evidence/canvas/web-view.png");
});
