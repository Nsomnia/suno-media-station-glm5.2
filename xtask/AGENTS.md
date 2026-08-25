# xtask — Guardrail Tooling Orientation

- Zero external dependencies policy: std-only. These commands must build and
  run instantly, everywhere, forever — dependency rot here breaks every CI run.
- Commands live one-per-file under `src/commands/` and must fail-safe:
  ambiguity → exit non-zero with a clear message, never silently pass.
- `check-layering`: encodes doc 01 §3's downward-only rule
  (`ui → application-services → domain-stores → external-bridges → foundation`).
  If you add a workspace layer or rename layers, update the layer map in the
  same task — a stale map is worse than no map.
- `check-file-caps`: enforces constitution §1 (300 hard cap / 200 warn).
- Both run locally per-task AND in CI (`ci.yml` guardrails job); keep their
  output single-line-per-violation so agent logs stay scannable.

Authoritative docs: [Health Guardrails](../docs/process/18-codebase-health-guardrails.md).
