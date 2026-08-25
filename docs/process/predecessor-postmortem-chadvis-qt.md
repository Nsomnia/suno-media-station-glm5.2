# Predecessor Project Post-Mortem — chadvis-projectm-qt

> **Last Updated:** 2026-08-25 · **Status:** Active
>
> Source of record: full architectural survey of
> `~/Documents/chadvis-projectm-qt` (~22k LOC C++23/Qt6 QML, git-clean at
> `bbb3ff5`) performed 2026-08-25. That repo is the early-draft ground truth
> for this project's core logic (everything except Suno front-end aspects).
> This doc distills what the Rust rewrite must **port faithfully**, and what it
> must **never replicate**. Treat it as binding input alongside the spec docs.

## 1. What to Port Faithfully (hard-won working logic)

These are the crown jewels of the prototype — non-obvious, battle-tested
logic. When implementing the corresponding Rust crates, consult these exact
files in `~/Documents/chadvis-projectm-qt/`:

| Prototype file | Lesson / logic to carry over | Maps to crate(s) |
|---|---|---|
| `src/suno/SunoClient.cpp` | Deque + 1 Hz timer rate-limiter; JWT extraction from `__session` cookie **including suffixed variants** (e.g. `__session_Jnxw-muT`); Clerk SID parsed from JWT middle base64url segment; refresh via `GET /v1/client?_is_native=true` → `POST /v1/client/sessions/{sid}/tokens`; lazy "refresh-then-retry" on missing token | `suno-http-client-core`, auth crates |
| `src/suno/SunoLyricsManager.cpp` | Polite-fetch policy: ≤3 concurrent lyric fetches, 50–250 ms random jitter, re-queue when error starts `"Lyrics processing:"`, pause-on-401 → token refresh → auto-resume | library sync service |
| `src/suno/SunoEndpoints.hpp` | Centralized constexpr endpoint map incl. B-side Orpheus base (`suno-ai--orpheus-prod-web.modal.run`) and CDN fallback construction (`cdn1.suno.ai/{id}.mp3` when `audio_url` empty) | `suno-http-client-core` |
| `src/suno/SunoDownloader.cpp` | ID3 tagging: USLT lyrics + custom TXXX frames (`SUNO_ID`, `SUNO_PROMPT`, `SUNO_STYLE`, `SUNO_MODEL`); FLAC Xiph equivalents; `.txt` metadata + `.srt` sidecars next to downloads | download orchestration service |
| `src/lyrics/LyricsData.cpp` | `alignWordsToLines()` — prompt-text line-splitting + normalized token matching. The trickiest algorithm in the codebase; port the *algorithm*, not the code | `karaoke-lyric-timing-resolution-service` |
| `src/audio/AudioAnalyzer.*` | 2048-sample span-based circular buffer → pffft FFT → magnitude smoothing (0.3). **Improve**: Hann window instead of rectangular; spectral-flux onset detection instead of energy-ratio beats | audio decode bridge + new analysis module |
| `src/audio/AudioQueue.hpp` | Lock-free SPSC fan-out concept (moodycamel) with cache-line-aligned frames and per-consumer drop counters — but consolidate to ONE ring buffer, multi-consumer (their own audit note) | audio-io bridge design |
| `src/visualizer/projectm/Bridge.cpp` | All preset ops marshalled onto the GL thread via atomics consumed during render; native↔UI state echo de-dupe (`syncingFromNative_`) | projectM frame bridge |
| `src/qml_bridge/VisualizerQFBO.cpp` | ⭐ The most expensive lesson, in three acts: plain QML item embedding broke under Qt6 RHI (Vulkan/Metal/D3D defaults); `QQuickFramebufferObject` was tried next; the **finally-proven path is a dedicated native GL window** (standalone `QWindow` embedded via `WindowContainer`) with the toolkit HUD composited alongside. Rust analogue: isolate projectM on its own GL context/thread with explicit handoff — exactly why doc 01 §4 mandates the compositing spike before any renderer commitment | Phase 0 spike ADR input |
| `src/recorder/FrameGrabber.cpp` | Dual-PBO async GPU readback (triple-buffered), bounded queue with drop-oldest + counter | video export bridge design |
| `src/recorder/EncoderSettings.cpp` | Named encoder presets (`youtube1080p60`, `youtube4k60`, `twitter720p`, `discord8mb`, `lossless`, `editing`) + HW-accel enum (NVENC/VAAPI/AMF/QSV) — good UX vocabulary worth keeping verbatim | ffmpeg export bridge |
| `config/default.toml` | Useful as a config-*schema checklist* (sections `[audio] [general] [keyboard] [overlay.elements…] [recording.audio|video] [ui] [visualizer]`) — not its values | `app-configuration-loader` |

Also directly reusable as assets: ~30 clean SVG icons in
`resources/icons/` (+`resources/icons/qml/`), the 10-line text-shadow shader
`src/qml/shaders/shadow.frag`, and `config/offical-projectM-config.inp`.

## 2. Do-NOT-Replicate List

Each item below is a real defect or decay pattern observed in the prototype.
The rewrite's architecture docs (doc 01 layering, doc 18 guardrails) exist
specifically to prevent recurrence — cite item IDs when closing them out.

| # | Anti-pattern observed | Guardrail that prevents it here |
|---|---|---|
| DNR-1 | God object: `core/Application.cpp` (584 LOC) owning every subsystem | doc 01 §2.7 No-God-Objects rule; composition root = `station-app` wiring only |
| DNR-2 | Broken committed state: dangling references after file deletion (`SunoPersistentAuth` removed from disk but not CMakeLists/callers) | CI builds green on `main`; trunk-based merges only (doc 19 §2) |
| DNR-3 | Recorder P0s: unchecked `avcodec_alloc_context3` null; brace-scope bug silently dropping all recorded audio | Definition of Done (doc 03 §5): clippy `-D warnings` + minimal tests per behavior |
| DNR-4 | Inconsistent 401 handling scattered across call sites; orchestrator bypassing the shared rate-limit queue; unguarded JSON `.value()` crashes | Single authenticated-client interceptor in `suno-http-client-core`; all traffic through one queue |
| DNR-5 | Plaintext credentials in TOML; CSRF state generated but never validated; M3U path traversal | `os-keyring-secret-storage` mandatory (doc 05 §3); security-lens review hat (doc 03 §14) |
| DNR-6 | RT-path violations: scratch allocation inside audio callback; analyzer thread busy-wait; dual position-update race in lyrics sync | doc 01 §2.4 no-blocking-on-UI-thread rule extended to RT threads; single-source-of-truth position feed |
| DNR-7 | Dead/speculative code accumulating (~20 stale cmake modules, placeholder classes, vestigial QSS theme layer) | doc 18 §2.3 dead-code ban |
| DNR-8 | Decorative tests: stub assertions; test targets never wired into the build | doc 16 testing strategy; CI runs `cargo test --workspace` |
| DNR-9 | Config drift: TOML defaults diverging from struct defaults; `debug=true` default; hardcoded `/home/nsomnia/...` paths; migration rerunning every startup | config loader derives defaults from one source; migrations idempotent & versioned (doc 07) |
| DNR-10 | SQLite schema decay: ad-hoc `ALTER TABLE` migrations, LIKE-based search, duration stored as string | doc 07 storage schema owns migrations; typed columns; FTS where search matters |
| DNR-11 | Uncancellable long operations (wav polling loop max 60 × 2 s, no abort) | all long-running jobs cancellable via tokio cancellation tokens from day one |

## 3. Greenfield Confirmations

Features the automation/UI plan expects that have **no counterpart at all**
in the prototype — they are greenfield, not ports: canvas/keyframe overlay
system (doc 10), batch automation pipelines (doc 13), plugin host (doc 11),
LLM/image-gen adapters (doc 12), multi-account switching (prototype had one
persisted session), playlist management against Suno's API.

## 4. Build/Dependency Reference

Prototype stack for parity-checking scope: Qt 6 (Core/Gui/Multimedia/
Network/Quick/Qml/QuickControls2/Sql), spdlog+fmt, tomlplusplus, pffft,
moodycamel readerwriterqueue, taglib, glew/glm, FFmpeg libs, projectM-4
(+playlist lib, custom FindProjectM4.cmake with `-l:` linker fix), optional
PulseAudio (referenced but never found — latent bug).

## 5. Related Material Already In This Repo

- Captured Suno API knowledge migrated wholesale into
  [docs/captures/raw/recon-from-chadvis/](../captures/raw/recon-from-chadvis/README.md)
  (endpoint inventory, auth, generation, upload, billing, personas, B-side,
  social, feature flags) — feeding [doc 06](../specs/suno-integration/06-suno-api-integration-contract.md).
- Ground-truth provenance notes: [docs/meta/suno-api-ground-truth-from-prototype.md](../meta/suno-api-ground-truth-from-prototype.md).
