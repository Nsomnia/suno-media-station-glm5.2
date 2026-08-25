# TODO

> Updated at start/end of every work session — see `docs/process/03-agent-constitution.md` §10
> and mark spec: `docs/meta/TODO-task-state-conventions.md`.
>
> Marks legend: `[ ]` todo · `[~]` doing · `[!]` blocked · `[?]` needs you ·
> `[x]` done-awaiting-your-verification · `[X]` verified-locked · `[-]` cancelled
>
> Current phase: **Phase 0 — Foundation** (`docs/product/04-phase-roadmap.md`)
>
> Repo-readiness round 2026-08-25 complete: workspace fully scaffolded and
> green (build/clippy/fmt/test/xtask), docs reorganized into the wiki taxonomy,
> prior-art knowledge base captured. Next: implement foundation crates.

## In Progress

*(empty)*

## Up Next (current phase)

- [x] Orchestrator approved all audit items; changes applied across docs + infra in one pass
- [x] E-block mechanical fixes applied (Cargo.toml TOML validity, station-app rename, template fixes, doc 06 repairs, env var, gitignore, edition 2024)
- [x] Git repo hygiene per docs/process/19 §9 + `.gitignore` completion (audit E-11) — incl. `reference-scratchpad/` ignore entry
- [x] Scaffold every crate stub per docs/architecture/02 (51 crates + station-app bin + xtask; each with purpose-doc lib.rs, README stub, uniform lints) — `cargo build/clippy/fmt/test --workspace` all green
- [x] xtask guardrail commands `check-layering` + `check-file-caps` implemented (std-only, fail-safe) with negative tests confirming they catch violations
- [x] Docs reorganized into wiki taxonomy (`product/ architecture/ specs/<domain>/ phases/<stage>/ process/ meta/`) with hub indexes + cross-reference repair + link-check CI
- [x] Repo hygiene kit: dependabot, CODEOWNERS, SECURITY.md, CONTRIBUTING.md, docs-link workflow, cargo-deny advisory/bans split, CLAUDE.md→AGENTS.md symlink, nested AGENTS.md orientation files (external-bridges, ui, xtask)
- [ ] Implement foundation crates: structured-logging-and-tracing, app-configuration-loader (versioned-config module pattern), design-tokens-theme-definitions (Catppuccin + Monokai)
- [ ] station-app binary: themed empty window + nav shell + working theme-switcher
- [ ] egui-glow-vs-wgpu + projectM texture-compositing feasibility spike (docs/architecture/01 §4, docs/specs/visuals-and-video/09 §4-5; audit C-3/C-4 widen scope) → record ADR. Spike MUST also probe projectM 4.x symbol availability (`projectm_set_frame_time`, FBO render path) per design-input doc §1
- [ ] End-of-phase Senior Architect Pass + Phase Audit Summary (docs/process/18 §3)

## Blocked / Needs Human Input

- [!] Meaningful Phase 1 work on suno-http-client-core — blocked on: fresh confirming capture of Clerk auth flow + library listing (leads exist from prototype recon; see docs/meta/suno-api-ground-truth-from-prototype.md §2–3)
- [?] Audit decisions: approve/reject/modify items in docs/meta/AUDIT-findings-and-recommendations.md (esp. scope items S-1 creation-front-end charter amendment and S-3 Phase 6b Suno Creation Studio)

## Awaiting Your Verification (agent says done; confirm → remove or lock)

- [x] Full planning doc set authored — verify by reading docs/README.md hub + spot-checking any two docs
- [x] Chat-export extraction completed; all 33 blocks landed as files with per-doc commits — verify via `git log --oneline`
- [x] Meta audit round authored — verify by reviewing AUDIT-findings-and-recommendations.md decision boxes
- [x] Docs wiki reorganization (22 git-mv renames, ~30 cross-reference updates, hub + category index pages, rewritten root README/AGENTS.md paths) — verify by browsing docs/README.md links
- [x] Workspace scaffold: 52 members compile warning-free; layering/file-cap guardrails pass; verify via the Verification Commands block in AGENTS.md
- [x] Prior-art knowledge base: predecessor post-mortem, reference-architecture patterns, design-input mining from sibling takes; 10 reference repos cloned into gitignored `reference-scratchpad/` for your browsing

## Verified Complete (user-locked)

*(none yet)*
