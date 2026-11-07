// The motion page's demos, captured at their end states: light and dark at
// phone width, and the key sections at desktop width. Screenshots need a
// target that draws, so the vision phase runs this file on one (`nsh test
// vision/motion.test.tsx --target macos|ios`); headless, the steps still
// prove the end states. The shell mounts the page behind its Motion nav
// item (`id:page-motion` is the shell's route id for it). `open` resolves
// entries against this file, so the package is the parent.
import { expect, open, test } from "nsh/test";
import type { App } from "nsh/test";

const EVIDENCE = "docs/evidence/motion";

/** How far the page scroll moves the drag-and-drop section into frame. */
const TO_DROP = 1900;
/** To the bottom, where the scrubber (the last section) sits. */
const TO_SCRUB = 3000;
/** Where the swipe rows and the transform card sit in frame. */
const TO_SWIPE = 1600;
const TO_TRANSFORM = 2500;

async function motion(scheme: "light" | "dark"): Promise<App> {
  // The entry resolves against this file, so the package is the parent.
  const app = await open("..", { clock: {}, colorScheme: scheme });
  await app.press("id:page-motion");
  await app.waitFor("text:Motion and gestures");
  return app;
}

/** The desktop window: 1280 × 800, the width the desktop fit is judged at. */
async function desktop(scheme: "light" | "dark"): Promise<App> {
  const app = await motion(scheme);
  await app.setViewport(1280, 800);
  return app;
}

/** Drops One after Two, with the section scrolled into frame. */
async function dropped(app: App): Promise<void> {
  await app.scroll("id:motion-page-scroll", TO_DROP);
  await app.drag("id:Two", { x: 0, y: 64 });
  await app.waitFor("text:Order: One · Three · Two · Four");
}

test("the spring and tween demo settles at its target", async () => {
  const app = await motion("light");
  await app.press("text:Play to 240");
  await app.waitFor("text:Rest 240 · finished");
  expect(await app.screenshot(`${EVIDENCE}/spring-settled-light.png`)).toBeTruthy();
});

test("the route flip lands on the entering route", async () => {
  const app = await motion("light");
  await app.press("label:Route A — alpha");
  await app.waitFor("text:Route B — beta");
  expect(await app.screenshot(`${EVIDENCE}/route-beta-light.png`)).toBeTruthy();
});

test("the layout reorder records its new order", async () => {
  const app = await motion("light");
  await app.press("role:button[Reverse]");
  await app.waitFor("text:Reversed — order 5 · 4 · 3 · 2 · 1");
  expect(await app.screenshot(`${EVIDENCE}/layout-reversed-light.png`)).toBeTruthy();
});

test("a swipe dismisses its row", async () => {
  const app = await motion("light");
  await app.scroll("id:motion-page-scroll", TO_SWIPE);
  await app.drag("id:Beta", { x: -240, y: 0 });
  await app.waitFor("text:/^Beta dismissed/");
  expect(await app.screenshot(`${EVIDENCE}/swipe-dismissed-light.png`)).toBeTruthy();
});

test("a drop reorders the list", async () => {
  const app = await motion("light");
  await dropped(app);
  expect(await app.screenshot(`${EVIDENCE}/drop-reordered-light.png`)).toBeTruthy();
});

test("the transform card follows a drag", async () => {
  const app = await motion("light");
  await app.scroll("id:motion-page-scroll", TO_TRANSFORM);
  await app.drag("id:transform-card", { x: 40, y: -20 });
  await app.waitFor("text:Drag 45 pt · scale 1.00× · angle 0°");
  expect(await app.screenshot(`${EVIDENCE}/transform-dragged-light.png`)).toBeTruthy();
});

test("the scroll-linked header hides and the list rests", async () => {
  const app = await motion("light");
  await app.scroll("id:header-scroll", 600);
  await app.waitFor("text:At rest at 600");
  expect(await app.screenshot(`${EVIDENCE}/header-scrolled-light.png`)).toBeTruthy();
});

test("the scrubber keeps its playhead where the release left it", async () => {
  const app = await motion("light");
  await app.scroll("id:motion-page-scroll", TO_SCRUB);
  await app.drag("id:scrub-pad", { x: 80, y: 0 });
  await app.waitFor("text:/^1 grants · scrubbed to/");
  expect(await app.screenshot(`${EVIDENCE}/scrub-granted-light.png`)).toBeTruthy();
});

test("the calm path ends in the same states", async () => {
  const app = await motion("dark");
  await app.press("text:Calm motion on this page");
  await app.press("text:Reveal the drawer");
  await app.waitFor("text:Instant 0");
  await app.press("text:Play to 240");
  await app.waitFor("text:Instant 240");
  expect(await app.screenshot(`${EVIDENCE}/calm-instant-dark.png`)).toBeTruthy();
});

// Desktop width: the same sections at 1280 × 800, light.
test("desktop: the spring and tween demo settles", async () => {
  const app = await desktop("light");
  await app.press("text:Play to 240");
  await app.waitFor("text:Rest 240 · finished");
  expect(await app.screenshot(`${EVIDENCE}/desktop-springs-light.png`)).toBeTruthy();
});

test("desktop: a swipe dismisses its row", async () => {
  const app = await desktop("light");
  await app.scroll("id:motion-page-scroll", TO_SWIPE);
  await app.drag("id:Beta", { x: -240, y: 0 });
  await app.waitFor("text:/^Beta dismissed/");
  expect(await app.screenshot(`${EVIDENCE}/desktop-swipe-light.png`)).toBeTruthy();
});

test("desktop: a drop reorders the list", async () => {
  const app = await desktop("light");
  await dropped(app);
  expect(await app.screenshot(`${EVIDENCE}/desktop-drop-light.png`)).toBeTruthy();
});

test("desktop: the scroll-linked header hides and the list rests", async () => {
  const app = await desktop("light");
  await app.scroll("id:header-scroll", 600);
  await app.waitFor("text:At rest at 600");
  expect(await app.screenshot(`${EVIDENCE}/desktop-header-light.png`)).toBeTruthy();
});

test("desktop: the scrubber keeps its playhead", async () => {
  const app = await desktop("light");
  await app.scroll("id:motion-page-scroll", TO_SCRUB);
  await app.drag("id:scrub-pad", { x: 80, y: 0 });
  await app.waitFor("text:/^1 grants · scrubbed to/");
  expect(await app.screenshot(`${EVIDENCE}/desktop-scrub-light.png`)).toBeTruthy();
});

// Dark appearance across the page, phone width.
test("dark: the spring and tween demo settles", async () => {
  const app = await motion("dark");
  await app.press("text:Play to 240");
  await app.waitFor("text:Rest 240 · finished");
  expect(await app.screenshot(`${EVIDENCE}/spring-settled-dark.png`)).toBeTruthy();
});

test("dark: a swipe dismisses its row", async () => {
  const app = await motion("dark");
  await app.scroll("id:motion-page-scroll", TO_SWIPE);
  await app.drag("id:Beta", { x: -240, y: 0 });
  await app.waitFor("text:/^Beta dismissed/");
  expect(await app.screenshot(`${EVIDENCE}/swipe-dismissed-dark.png`)).toBeTruthy();
});

test("dark: a drop reorders the list", async () => {
  const app = await motion("dark");
  await dropped(app);
  expect(await app.screenshot(`${EVIDENCE}/drop-reordered-dark.png`)).toBeTruthy();
});

test("dark: the scroll-linked header hides and the list rests", async () => {
  const app = await motion("dark");
  await app.scroll("id:header-scroll", 600);
  await app.waitFor("text:At rest at 600");
  expect(await app.screenshot(`${EVIDENCE}/header-scrolled-dark.png`)).toBeTruthy();
});

test("dark: the transform card follows a drag", async () => {
  const app = await motion("dark");
  await app.scroll("id:motion-page-scroll", TO_TRANSFORM);
  await app.drag("id:transform-card", { x: 40, y: -20 });
  await app.waitFor("text:Drag 45 pt · scale 1.00× · angle 0°");
  expect(await app.screenshot(`${EVIDENCE}/transform-dragged-dark.png`)).toBeTruthy();
});
