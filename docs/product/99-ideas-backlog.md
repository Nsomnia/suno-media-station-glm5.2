# Ideas Backlog

> **Last Updated:** 2026-08-26 · **Status:** Active

A running log of good ideas surfaced during development that are
explicitly OUT of current scope (per doc 03 §8's scope-discipline rule).
Nothing here is committed to — this is a parking lot, reviewed
periodically (e.g. at Core Maintainability Gate time, doc 18 §4, and at
the start of any new phase) to decide what (if anything) graduates into an
actual phase/doc update.

Format: `- [ ] <idea> — (surfaced during: <phase/task>, date if useful)`

## Parked Ideas

- [ ] Auto-upload recorded takes (Phase 2) directly to a Suno project —
      deferred, Phase 2 is capture-and-save-locally only.
- [ ] Cross-account content-hash deduplication for downloaded audio files
      — deferred, not a v1 concern (doc 15 §7).
- [ ] Snapping/alignment guides in the canvas editor — deferred, not a
      Phase 5 exit criterion (doc 10 §5).
- [ ] Additional easing curve types beyond the v1 minimum set — additive,
      low priority (doc 10 §3).
- [ ] Spend/budget tracking UI for LLM/image-gen provider usage — deferred
      (doc 12 §4), only if real user demand emerges.
- [ ] Saved smart-filters/search queries reused as pipeline input
      selectors — deferred until library browser filtering (Phase 1) is
      mature enough to reuse cleanly (doc 13 §3).
- [ ] Per-item scene overrides within a single automation pipeline run —
      deferred, v1 pipelines are one-scene-per-run only (doc 13 §3).
- [ ] Plugin marketplace/distribution infrastructure — explicitly deferred
      indefinitely; local file-based plugin loading only (doc 04 Phase 8
      non-goals, doc 11 §5).
- [ ] WASM plugin host (Tier 2) — build only if a concrete Tier-1 (Rhai)
      limitation is actually hit in practice (doc 11 §2).
- [ ] Full mobile/tablet responsive layout — explicitly out of scope,
      desktop-first app (doc 08 §5).
- [ ] Full screen-reader accessibility support — stretch goal, not a v1
      blocker given current egui maturity (doc 08 §8).
- [ ] Heuristic lyric-aligner fallback tier (cheap local alignment of plain
      lyrics when remote timing is absent but timing is desired — sits between
      remote timing and full Whisper runs in the karaoke resolution chain) —
      (surfaced during: audit round 2026-08, predecessor prototype had working
      LyricsSync heuristic aligner).
- [ ] Overlay timeline `preset` segments — visualizer preset changes as
      draggable clips on typed timeline tracks (`audio|text|karaoke|image|
      preset`), keyframes incl. `hold` easing — promote into doc 10 scope at
      Phase 5 start if it fits v1 cut (source:
      [design-input-from-prior-takes §4](../meta/design-input-from-prior-takes.md)).
- [ ] Bounded overlay expression language over fixed variable set (`t, bass,
      mid, treb, vol, beat, line_index, progress`) — declarative effects JSON
      first; explicitly NOT Rhai/general scripting (that stays Phase 8, doc 11)
      (source: prior-takes §5).
- [ ] Song relationship taxonomy + derivation DAG view (cover_of / remix_of /
      stem_of / extended_from / cropped_from / video_export_of / inspired_by)
      as schema tables in doc 07 (source: prior-takes §7).
- [ ] Prompt Vault: versioned prompt library with style presets, section
      scaffolds, SFX tag dictionary, `negative_prompt` field; real specimens
      in `~/Documents/*.rtf` for validation (source: prior-takes §11).
- [ ] Capability-probe feature flags gating Suno b-side/experimental endpoints
      at runtime (source: prior-takes §11; complements doc 06 provenance tiers).
- [ ] Stems rack UI (per-stem mute/solo/volume/color) when Suno stems become
      downloadable via captured API (source: sunoPulse mockup data model).
- [ ] Mobile-preview responsive toggle in app shell header (resizes window to
      ~450 px for responsive QA) — only if any screen ever supports narrow
      layouts; desktop-first per doc 08 §5 (source: QML shell mockup).

## Phase 0 Known Debt

Recorded at the Phase 0 close audit (doc 18 §3 ritual). These are accepted,
tracked gaps — not silent accumulation. Triage happens per doc 18 §4.

- [ ] Log rotation period / log directory not yet config-expressible —
      v1 config ships minimal defaults; expose in the logging section when
      a real need appears (surfaced during: Phase 0 close audit, 2026-08).
- [ ] `assets/themes/*.toml` lack automated equality tests against the
      code-built themes — drift between shipped TOML and registry values
      would go unnoticed (surfaced during: Phase 0 close audit, 2026-08).
- [ ] Dual `LoggingSettings` naming: the config v1 section type
      (app-configuration-loader) and the logging-crate runtime settings
      (structured-logging-and-tracing) share a name but are distinct types.
      Rename candidate: `LoggingSection` for the config-side one
      (surfaced during: Phase 0 close audit, 2026-08; assessed as justified
      decoupling, watch-item only).
- [x] Mutex-poison handling style was inconsistent in station-app startup
      (`.expect("config lock")` vs graceful persist path) — RESOLVED
      2026-08-26: boot-time access unified on the graceful style
      (`lock_config_with_fallback`).
- [ ] `DesignTokens` is a ~20-field flat struct — documented deliberate
      exception to doc 18 §2.4's 10–12 field heuristic (doc 08 §3 defines
      it as one payload; splitting would be two structs pretending to be
      one) (surfaced during: Phase 0 close audit, 2026-08).
- [ ] egui exact-version pin deferred until `backdrop-blur-egui` lands in
      Phase 5 (ADR-013 consequence: it version-pins egui, so pin exactly
      and isolate behind ui-shared-widget-library then)
      (surfaced during: Phase 0 close audit, 2026-08).
- [ ] Formal visualizer fps measurement deferred to Phase 5 (no baseline
      numbers exist yet; needed before compositing work starts)
      (surfaced during: Phase 0 close audit, 2026-08).

## Review Log

*(Populated as backlog reviews happen — empty at doc-set creation.)*

| Date | Reviewed By | Items Graduated to a Phase Doc | Items Removed (no longer relevant) |
|---|---|---|---|
| — | — | — | — |
