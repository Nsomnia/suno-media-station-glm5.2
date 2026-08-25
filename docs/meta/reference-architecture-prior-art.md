# Reference Architecture & Prior-Art Notes

> **Last Updated:** 2026-08-25 · **Status:** Active
>
> Distilled from a live survey of the owner's own related repositories and
> best-in-class OSS projects (projectM ecosystem, Rust players, karaoke/video
> apps, unofficial Suno wrappers). These are *patterns to apply*, subordinate
> to the binding docs (doc 01 layering, doc 02 layout, doc 18 guardrails).
> Cloned copies of most referenced repos live in the gitignored
> `reference-scratchpad/` at repo root for offline study (constitution §3/§12).

## 1. Confirmed by the Owner's Own Prior Attempts

These decisions independently recurred across the owner's repos — treat as
settled direction, not open questions:

1. **Shell + layered libs.** `apps/app_shell` + `libs/{engine,network,ui}`
   (C++ sibling) maps 1:1 onto this repo's `station-app` bin + layer-ordered
   Cargo workspace. Trust the shape of [doc 02](../architecture/02-workspace-layout.md).
2. **Seams before engines** (from the Tauri/React sibling's "UI Mockup mode"):
   define trait boundaries early (`PlaybackEngine`, `VisualizerHost`,
   provider adapters) with mock impls so UI/tests run without GPU/network/
   backend toolchains. See also doc 01 §2.3 bridge-trait rule.
3. **Library = local SQLite mirror + incremental cursor sync**, never live API
   queries against lists (`suno-sync` repo: cursor pagination over `feed/v3`,
   ~20/page, fetch-newer-than-last). Add **clip lineage tracking**
   (covers/remixes/inspiration chains) as a self-referencing schema table —
   the owner designed this once already; keep it in [doc 07](../specs/data-and-storage/07-data-model-and-storage-schema.md) scope.
4. **Preset lifecycle is product, not plumbing**: favorites, broken-preset
   quarantine, history, random/sequential cycling (`aurora-projectm-visualizer`
   shipped all four; chadvis carried them forward). Deserves its own module
   boundary inside the visualizer crates rather than blob state.
5. **Recording = raw-frame pipe into a templated FFmpeg command**
   (`{WIDTH}/{HEIGHT}/{FPS}/{FRAMES}` placeholders in config) — simple,
   testable, OS-agnostic default for the video-export bridge before any
   libav* bindings are considered.
6. **Docs hub with numbered cross-references is house style** — continued here
   via the [wiki hub](../README.md).
7. Known recurring anti-pattern to avoid: giant single-tree codebases with
   `.backup_graveyard/` quarantine dirs and vendored forks inside the working
   tree. The granular workspace + `[workspace.dependencies]` +
   `reference-scratchpad/` convention exists precisely to prevent this.

## 2. Structural Patterns From Best-in-Class Projects

| Source | Pattern worth adopting |
|---|---|
| [termusic](https://github.com/tramhao/termusic) (4-crate Rust workspace) | Playback crate with pluggable backends behind one interface selected by features · **versioned config modules** (`config/v1`, `config/v2`) surviving breaking changes · DB migrations co-located with their owning domain crate · single `[workspace.dependencies]` + `[workspace.lints]` source of truth |
| [spotify_player](https://github.com/aome510/spotify-player) | Classic player-loop decomposition (`client/ state/ event/ ui/`) · explicit `event/` module instead of scattered channels · strict clippy-pedantic lint policy with targeted allows — pairs perfectly with the 300-LOC cap |
| [projectM](https://github.com/projectM-visualizer/projectm) (★4.4k) | Two separately-versioned C APIs (engine vs playlist) → keep `visualizer-projectm-ffi-bindings` and preset-playlist logic as distinct public surfaces · parser fixture tests over real `.milk` files (incl. subdir fixtures) · ship a minimal headless test harness binary exercising the engine without full GUI |
| [Karaoke Mugen](https://gitlab.com/karaokemugen/code/karaokemugen-app) | Content pipeline split: immutable base files + **derived, rebuildable search index** (SQLite FTS over cached clip metadata fits doc 07) · tags/aliases as first-class entities, not free-text fields · ffmpeg hardsub compositing as robust fallback export path |
| [UltraStar Play](https://github.com/UltraStar-Deluxe/Play) | Song-format parser/writer isolation (UltraStar txt ↔ our LRC/SRT) · validation **issues as first-class objects** surfaced in UI · every external ML tool behind a common shell-runner trait (their `AiTools/*Runner` ≈ our whisper bridge) · architecture-conformance tests in CI |
| [UltraStar Deluxe](https://github.com/UltraStar-Deluxe/USDX) (★1.3k) | Root-level process docs (`PIPELINE.md`, `RELEASING.md`) ↔ our `docs/process/`; game-data vs engine vs tooling top-level split |
| [gcui-art/suno-api](https://github.com/gcui-art/suno-api) (★3.2k, informational only — contract stays capture-driven per doc 06) | Word-level aligned lyrics endpoint is the karaoke keystone · captcha/auth strategy isolated behind a replaceable module · poll-until-ready job loops with CDN URL fetching |
| [UltraSinger](https://github.com/rakuri255/UltraSinger) | Closest OSS analog to "song → karaoke track" automation: vocal/pitch extraction pipeline autogenerating timed lyric files |

## 3. Agent-Harness & Repo-Hygiene Conventions (2026 consensus)

- `AGENTS.md` is read natively by Codex/Cursor/Copilot/Gemini CLI/opencode et al.;
  nearest-file-wins resolution; keep each file ≤ ~100–150 lines, exact
  commands included, contradictions-with-defaults stated explicitly.
- `CLAUDE.md` should be a **symlink to AGENTS.md** (never a duplicate copy).
- Solo-maintainer free-tier CI stack: pinned `dtolnay/rust-toolchain` +
  `Swatinem/rust-cache`, clippy `-Dwarnings`, cargo-deny with advisories
  `continue-on-error` so surprise CVEs don't red CI, dependency-scoped
  `paths:` filters, Dependabot for cargo + github-actions ecosystems.
- Release engineering (release-plz / cargo-dist) is deliberately deferred to
  Phase 2+ per doc 19 §8 — noted here so the pattern isn't re-litigated.
- An owner precedent exists for comment-triggered agent CI (`/oc` on issue
  comments, least-privilege OIDC); revisit only when useful.

## 4. Application Checklist (fold into phase work, don't build ahead)

- Phase 0: `[workspace.lints]` in root Cargo.toml (deny unsafe_code, warn
  missing_docs, clippy pedantic baseline) · versioned config module skeleton ·
  mock-friendly trait seams in service crates.
- Phase 1: cursor-based incremental sync + lineage table in doc 07 schema ·
  auth chain implemented exactly as captured (see
  [ground truth](suno-api-ground-truth-from-prototype.md)).
- Phase 4: preset favorites/broken-quarantine/history subsystem · `.milk`
  parser fixture tests · headless engine smoke-test binary.
- Phase 5+: ffmpeg command-template export default; hardsub fallback path.
