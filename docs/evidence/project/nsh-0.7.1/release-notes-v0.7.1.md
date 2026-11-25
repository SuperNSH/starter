This is a patch release on top of v0.7.0. It fixes the Windows title bar accent color leaking into window content, and brings canvas panning, sound playback and media linking fixes for macOS and Linux.

## Windows

- Fix the title bar accent color showing up inside window content. #3207
- Fix the mouse wheel stopping short of the end of a multiline `Input` at HiDPI scales. #3231

## Performance

### macOS

- Panning a canvas now moves its layer instead of redrawing every frame, and keeps drawing off the main thread while dragging. #3203 #3227
- Image sprites are cut once and reused instead of recreated each frame. #3241

### Linux

- A resizing view no longer re-bakes its continuous shadow every frame. #3202
- Panning a canvas moves its raster instead of replaying it every frame, and only the changed 64-point cells are redrawn. #3196 #3213

## GStreamer and media

- Linux loads GStreamer at run time for sounds: building no longer needs the GStreamer `-dev` packages, and an app on a machine without GStreamer still starts (WAV sounds still play). #3215
- Linux: canvases and shaders inside `<Window>` windows draw again (icons were blank), sounds play to the end, and shaders open on OpenGL. #3237
- `nsh` links GStreamer (Linux) and Media Foundation (Windows) only into runners whose app uses `Sound`, `AudioPlayer` or `<Video>`, so apps without media no longer need GStreamer headers to package. #3228



### CLI
- Read an nshm's inherited (workspace) version and `nsh-core` requirement the way Cargo does (#3232).
- Name the nshm that was built for another nsh version when a build fails (#3225).

### Web
- Leave unused `nsh/base` part gates out of a wasm-bindgen runtime, shrinking Rust web bundles (#3238).

### Docs & tests
- Chinese design guides now cover the `ui` controls section (#3214).
- Test that list markers stay unhighlighted during a Linux drag selection (#3212).

### Additional fixes
- macOS: Don't apply a batch from a frame inside a nested run loop (#3191)
- macOS: Place a scroll view's indicator again when its frame changes size (#3192)
- List: Keep the rows of a list or data table hidden with `display: none` (#3193)
- Docs: Add a writing guide and fix a few unclear passages (#3199)
- Skills: Require `ui` controls to wrap nsh/base elements (#3189)

<details>
<summary>Contributors</summary>

@huacnlee @JeromeLee-ljl @1havecoke @madcodelife @RebelBIrd @sunli829

</details>

<details>
<summary>What's Changed</summary>

* release: Prepare version 0.7.1 by @huacnlee in https://github.com/longbridge/nsh/pull/3250
* macos: Name the image part key type to satisfy Clippy by @huacnlee in https://github.com/longbridge/nsh/pull/3273

**Full Changelog**: https://github.com/longbridge/nsh/compare/v0.7.0...v0.7.1

</details>

