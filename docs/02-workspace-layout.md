**Cargo Workspace Layout**

This is the guideline for rough *directory structure* to scaffold in Phase 0. Every crate's
`lib.rs` should begin with a doc comment restating its ONE job (copy the
"Purpose" line below into it). Directories are intentionally deep/verbose per
the LLM-context-legibility goal — prefer a clear nested path over a clever short
one. As you can observe below, code is divided into highly specific single intent and purpose classes within yet again specific groups of classes. 
the codebase when navigated, especially by an LLm, should self-explain itself at a broad high level simply by verbose and dry directory and filename strucutre.

```
Suno Station/
├── Cargo.toml                                       # workspace root
├── docs/                                            # this doc set (source of truth)
├── crates/
│   │
│   ├── foundation/
│   │   ├── error-and-result-conventions/            # shared Error trait helpers, thiserror macros
│   │   ├── app-configuration-loader/                # reads/writes app config (TOML), env overrides
│   │   ├── structured-logging-and-tracing/          # tracing subscriber setup, log file rotation
│   │   └── design-tokens-theme-definitions/         # Catppuccin/Monokai token structs, no UI code
│   │
│   ├── external-bridges/
│   │   ├── suno-http-client-core/                   # low-level authenticated HTTP client to suno.com
│   │   ├── suno-auth-manual-token-paste/            # parse pasted JS-snippet JWT+cookie blob
│   │   ├── suno-auth-embedded-webview-login/        # wry-based Suno login capture
│   │   ├── suno-auth-oauth-loopback-google-fb/      # system-browser + localhost callback OAuth
│   │   ├── os-keyring-secret-storage/               # keyring crate wrapper, encrypted-file fallback
│   │   ├── visualizer-projectm-ffi-bindings/        # raw bindgen output + safe wrapper boundary
│   │   ├── visualizer-projectm-frame-bridge/        # PCM-in, texture-out trait + implementation
│   │   ├── audio-decode-symphonia-bridge/           # file → PCM decode wrapper
│   │   ├── audio-io-cpal-bridge/                    # device I/O, playback + input capture
│   │   ├── whisper-transcription-bridge/            # whisper-rs wrapper, model mgmt
│   │   ├── video-export-ffmpeg-process/             # spawn/pipe-to ffmpeg, progress parsing
│   │   ├── llm-text-provider-adapter/               # trait + OpenAI-compatible impl (stub-priority: low)
│   │   ├── image-gen-provider-adapter/              # trait + remote/local-server impls (stub-priority: low)
│   │   └── plugin-host-stub/                        # trait defs + no-op registry only
│   │
│   ├── domain-stores/
│   │   ├── account-profile-store/                   # multi-account CRUD + active-account switching
│   │   ├── suno-remote-library-cache-store/         # cached remote track/project metadata
│   │   ├── local-download-manager-store/            # download queue, local file bookkeeping
│   │   ├── lyrics-and-alignment-store/              # remote+whisper timed lyrics, versioned edits
│   │   ├── canvas-scene-and-keyframe-store/         # scene graph + keyframe track persistence
│   │   ├── automation-pipeline-definition-store/    # pipeline recipes, run history
│   │   └── recorded-audio-take-store/               # locally recorded takes metadata
│   │
│   ├── application-services/
│   │   ├── suno-library-sync-service/               # orchestrates cache-store <-> http-client
│   │   ├── suno-bulk-library-operations-service/    # bulk tag/delete/organize across tracks
│   │   ├── track-download-orchestration-service/.   # drives download-manager-store end to end
│   │   ├── local-playback-parity-service/           # unifies remote-stream vs local-file playback
│   │   ├── karaoke-lyric-timing-resolution-service/ # merges remote+whisper per lyrics-flow doc
│   │   ├── single-track-visualizer-render-service/  # one-off "render this track as video"
│   │   ├── canvas-overlay-compositing-service/      # combines visualizer frame + scene graph
│   │   ├── automation-batch-render-orchestrator/    # fan-out of render-service across pipeline
│   │   └── audio-recording-capture-service/         # mic capture -> take-store -> (future upload)
│   │
│   ├── ui/
│   │   ├── ui-app-shell-and-navigation/             # window, top nav/routing, layout skeleton
│   │   ├── ui-shared-widget-library/                # buttons/cards/glass-panel primitives, themed
│   │   ├── ui-screen-account-management/            # add/switch/remove Suno accounts
│   │   ├── ui-screen-remote-library-browser/        # search/browse/bulk-ops on Suno library
│   │   ├── ui-screen-local-library-browser/         # local downloads browser/player
│   │   ├── ui-screen-lyrics-editor/                 # karaoke timing review/edit UI
│   │   ├── ui-screen-visualizer-preview/            # live preview + one-off render controls
│   │   ├── ui-screen-canvas-scene-editor/           # freeform placement + keyframe timeline UI
│   │   ├── ui-screen-automation-pipeline-builder/.  # pipeline authoring + run monitor UI
│   │   ├── ui-screen-settings-and-theming/          # theme picker, provider keys, prefs
│   │   └── ui-screen-recording-studio/              # basic capture UI (Phase 9 grows this)
│   │
│   └── shared-test-support/
│       ├── suno-api-fixture-mocks/                  # recorded/mocked HTTP fixtures for tests
│       └── deterministic-test-clock-and-ids/        # test helpers for time/uuid determinism
│
├── app/
│   └── Suno Station-app/                            # the binary crate; composition root only
│       ├── Cargo.toml 
│       └── src/main.rs
│
├── assets/
│   ├── themes/                                      # catppuccin/monokai token json/toml sources
│   └── projectm-presets/                            # bundled default presets (if redistributable)
│
└── xtask/                                           # cargo-xtask style dev tooling (codegen, fixture capture helpers)

```

**Notes on This Layout**

- **Not every crate listed above needs code on day one.** Phase 0 scaffolds the
  *whole workspace skeleton* with each crate compiling as an empty stub
  (`lib.rs` with the purpose doc-comment and maybe one placeholder type) so the
  directory structure — the thing that gives the LLM agent navigable, guessable
  paths — exists before deep feature work starts. Filling them in happens per
  phase per doc 04.
- **Naming convention:** `kebab-case`, `noun-phrase-describing-the-one-job`,
  no abbreviations unless industry-standard (`ffi`, `io`, `db`). This is
  deliberate over-verbosity for agent legibility, not a style preference to
  fight.
- **300-line soft/hard cap** applies to files (`*.rs`), not crates. A crate
  like `ui-screen-canvas-scene-editor` will contain MANY files/submodules — that
  is expected and correct; keep nesting deeper (e.g.
  `src/keyframe_track/interpolation_curve_editor_widget.rs`) rather than
  growing one file.
- Any crate not yet implemented ships with a `README.md` stub stating its
  planned phase, so `gh`/repo browsing by the agent surfaces intent even for
  empty crates.
