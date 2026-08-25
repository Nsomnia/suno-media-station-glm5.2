# Plugin System — Deferred Design (Phase 8)

> **Last Updated:** 2026-08-25 · **Status:** Active

## 1. Current Status (Phases 0-7)

`plugin-host-stub` exists purely as trait scaffolding + a no-op registry —
per doc 03 §8 and ADR-004, this is intentional and should NOT be fleshed
out ahead of Phase 8. This doc records the *intended* design now so the
seam left in earlier phases (see §4) is shaped correctly, without actually
building the implementation early.

## 2. Tiered Runtime Decision (recap of earlier decision)

- **Tier 1 (Phase 8 initial): Rhai scripting.** Pure-Rust embedded scripting
  engine, no external toolchain/compiler needed by end users, naturally
  sandboxed (no filesystem/network access unless explicitly exposed via
  registered host functions). Chosen over Lua (`mlua`) for being pure-Rust
  (simpler cross-platform distribution, no C dependency) — confirm this
  reasoning still holds via a quick prior-art check at Phase 8 start in
  case the ecosystem has shifted.
- **Tier 2 (later, only if real need emerges): WASM component host** via
  `wasmtime`, for users wanting to author plugins in other languages or
  needing capabilities beyond what a scripting sandbox comfortably allows
  (e.g. genuinely heavy compute). Not built until a concrete Tier-1
  limitation is hit in practice — per doc 18 §2.3's anti-speculative-
  generality rule, do not build Tier 2 "just in case."

## 3. Planned Extension Points (Phase 8 targets)

- **Canvas custom effect hook:** a Rhai script implementing a
  per-frame property-modifier function — given an element's current
  computed properties + time `t`, returns modified properties. Registered
  as a new "effect type" alongside the built-ins (doc 10 §2).
  ```
  // illustrative Rhai function signature
  fn apply(properties, t) {
      // read/modify a fixed, documented set of numeric/string properties
      // return the modified properties map
  }
  ```
- **Automation pipeline custom step hook:** a Rhai script that can run as
  an additional step in a pipeline (doc 07 §7) — e.g. custom output-file
  naming logic, or a conditional skip rule — given a well-defined read-only
  context object (track metadata, run info) and returning a simple
  decision/value, not arbitrary host control.

## 4. What Earlier Phases Must NOT Do (guardrail against premature coupling)

- Earlier phases (5, 7) may reference `plugin-host-stub`'s trait types as
  extension seams (e.g. an `Effect` enum in the scene JSON schema having a
  `Custom(PluginRef)` variant that's simply unused/unreachable until Phase
  8) — this is fine and encouraged as forward-compatible schema design.
- Earlier phases must NOT implement any actual script execution, sandbox
  logic, or plugin-file-loading — that's 100% Phase 8 scope. If a Phase 5/7
  task feels tempted to "just quickly support a simple script hook already
  since the seam is right there," that's scope creep per doc 03 §8 — note
  it in `docs/99-ideas-backlog.md` and move on.

## 5. Sandbox & Trust Model (Phase 8 requirement, recorded now for clarity)

- A plugin script has access ONLY to explicitly-registered host functions/
  data — no ambient filesystem, network, or process access via Rhai's
  default engine configuration (Rhai is sandboxed-by-default unless you
  register additional capabilities — confirm this is still accurate for
  whatever Rhai version is current at Phase 8 time).
- Plugins are loaded from local files the user explicitly points the app
  at (a plugins folder in the app's config directory) — no auto-discovery
  from arbitrary locations, no remote plugin fetching/marketplace (doc 04
  Phase 8 non-goals).
- A misbehaving plugin (infinite loop, error) must not hang or crash the
  host app — Rhai supports execution step limits/timeouts; Phase 8's
  implementation must configure and test this explicitly as a hard
  requirement, not an afterthought.

## 6. Example Plugins to Ship (Phase 8 exit criteria per doc 04)

- A simple custom canvas effect (e.g. a "strobe/flash on beat" property
  modifier) — demonstrates the effect hook.
- A simple custom pipeline step (e.g. "skip tracks shorter than N
  seconds") — demonstrates the pipeline hook.
Both ship as documented, readable example scripts (not obfuscated/minified)
specifically so end users can learn the API by reading them.
