# Testing Strategy

> **Last Updated:** 2026-08-25 · **Status:** Active

## 1. Philosophy

Given an LLM writes all code, tests are the primary safety net against
silent regressions and hallucinated behavior — they are not optional
polish. Every domain/service/bridge crate ships tests alongside its
implementation in the same task, per the Definition of Done (doc 03 §5),
not as a follow-up ticket.

## 2. Test Levels by Layer

- **Foundation crates:** straightforward unit tests (config parsing edge
  cases, token redaction actually redacting, theme token completeness).
- **External-bridge crates:** unit tests against fakes/mocks of the external
  system wherever possible (e.g., `suno-http-client-core` tested against
  `suno-api-fixture-mocks`, not live suno.com — **live-network calls are
  banned in the automated test suite entirely**, per doc 03 §7 and doc 06's
  capture-driven approach; live calls only happen via manual, human-run
  exploratory sessions when capturing new endpoints). Process-wrapper bridges
  (`video-export-ffmpeg-process`, `whisper-transcription-bridge`) get a thin
  integration test gated behind a feature flag/env var
  (`STATION_TEST_REQUIRES_LOCAL_TOOLING=1`) so CI-less/tool-less environments
  can skip them without failing the default `cargo test --workspace` run.
- **Domain-store crates:** integration tests against a real (temp-file or
  in-memory) SQLite instance via `sqlx`'s test utilities — migrations run,
  CRUD round-trips verified, including the "versioned/never-overwrite"
  behaviors (e.g. lyric document history) and soft-delete behavior.
- **Application-service crates:** integration tests composing fake/stub
  bridges + real (temp) stores to verify orchestration logic (e.g., "does
  the karaoke render service correctly prefer remote lyrics over whisper
  when both exist") without needing the full real external stack.
- **UI crates:** primarily manual QA during early phases (egui's testing
  story is weaker than backend Rust). From Phase 5 onward, add **snapshot
  tests for serializable state** the UI produces (scene graph JSON,
  keyframe track JSON) even if the visual rendering itself isn't
  snapshot-tested — this catches "the editor silently corrupts saved data"
  bugs, which are the highest-value UI-adjacent bugs to catch automatically.

## 3. Fixture & Mock Data Conventions

- `suno-api-fixture-mocks` holds sanitized, real-capture-derived JSON
  fixtures (see doc 06 §0) organized to mirror doc 06's endpoint categories
  (`fixtures/library/list_tracks_page_1.json`, etc). A small
  `MockSunoHttpClient` (implementing the same trait `suno-http-client-core`
  exposes) serves these fixtures for tests, letting service/UI-logic tests
  run without any real HTTP dependency.
- `deterministic-test-clock-and-ids` provides a fixed/injectable clock and
  a seeded/sequential UUID generator so store-layer tests produce
  reproducible `created_at`/`id` values instead of asserting against
  wall-clock time or random UUIDs (which would make assertions flaky/
  awkward).
- Fixtures are sanitized per doc 06 §0's redaction rule even though they
  live only in the repo (not shared externally) — treat "never commit a
  real secret" as a hard rule regardless of the repo's visibility, since
  visibility/hosting policy may change later.

## 4. What CI Enforces (mechanics finalized later per project instructions, principles fixed now)

Actual CI/git workflow tooling is explicitly deferred (per the user's
instruction to worry about worktrees/branches/releases later), but the
**bar CI will eventually enforce** is fixed now so development proceeds
toward it from day one:

- `cargo build --workspace` — zero errors, zero new warnings.
- `cargo clippy --workspace --all-targets -- -D warnings` — clean.
- `cargo test --workspace` — all pass, live-network-requiring and local-
  tooling-requiring tests properly gated/skipped as described in §2.
- `cargo fmt --check` — consistent formatting (standard rustfmt defaults
  unless a documented project-wide `rustfmt.toml` override is added early
  and recorded via ADR).
- A file-size-cap check (custom `xtask` script, per doc 02) flags any `.rs`
  file over the 300-line hard cap (doc 03 §1) — this can start as advisory/
  warning-only in `xtask` and later become a hard CI gate once the codebase
  is large enough for it to matter.

## 5. Manual QA Checklists

For each phase (doc 04), the phase's own feature doc (written when that
phase begins, per the project's step-by-step prompting plan) should include
a short manual QA checklist specific to that phase's exit criteria — e.g.
Phase 1's checklist walks through adding an account via all three auth
methods, switching accounts, and confirming library re-scoping. These
checklists are how the human orchestrator spot-verifies agent-claimed
"exit criteria met" status without needing to read every diff line-by-line.

## 6. Performance/Load Testing (light touch until it matters)

- No formal perf benchmarking suite before Phase 7 (Automation). Phase 7's
  own exit criteria (a real 20-50 track batch run, doc 04) IS the first
  real load test, and should be treated as such — capture timing/resource
  usage observations from that run into that phase's doc as a baseline for
  later "thousands of tracks" scale work.
- Audio playback (Phase 2) and visualizer rendering (Phase 4) get informal
  "does it glitch/drop frames under normal manual use" checks rather than
  automated benchmarks initially — formalize only if a real problem
  surfaces.

## 7. Security-Adjacent Testing

- Redaction logic (doc 05 §3, `secrecy`-wrapped credential types) gets an
  explicit unit test asserting that `Debug`/`Display` formatting of a
  credential-bearing struct never contains the raw secret substring — this
  is cheap to test and guards against a genuinely damaging class of bug
  (accidental token leakage into logs).
- Keyring storage round-trip (`os-keyring-secret-storage`) gets an
  integration test on each target platform's actual keyring where CI
  environment permits, with a documented manual-test fallback note for
  platforms/CI runners where a real OS keyring isn't available headlessly.
