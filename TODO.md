# TODO

> Updated at start/end of every work session — see `docs/03-agent-constitution.md` §10
> and proposed mark spec: `docs/meta/TODO-task-state-conventions.md` (audit A-1).
>
> Marks legend: `[ ]` todo · `[~]` doing · `[!]` blocked · `[?]` needs you ·
> `[x]` done-awaiting-your-verification · `[X]` verified-locked · `[-]` cancelled
>
> Current phase: **Phase 0 — Foundation** (`docs/04-phase-roadmap.md`)
> Meta round active: doc-set audit awaiting orchestrator decisions (`docs/meta/AUDIT-findings-and-recommendations.md`)

## In Progress

*(empty)*

## Up Next (current phase)

- [x] Orchestrator approved all audit items; changes applied across docs + infra in one pass
- [x] E-block mechanical fixes applied (Cargo.toml TOML validity, station-app rename, template fixes, doc 06 repairs, env var, gitignore, edition 2024)
- [ ] Initialize git repo hygiene per docs/19 §9 + `.gitignore` completion (audit E-11)
- [ ] Scaffold every crate stub per docs/02 (empty lib.rs + README.md per docs/templates/crate-stub-template)
- [ ] Implement foundation crates: structured-logging-and-tracing, app-configuration-loader, design-tokens-theme-definitions (Catppuccin + Monokai)
- [ ] station-app binary: themed empty window + nav shell + working theme-switcher
- [ ] egui-glow-vs-wgpu + projectM texture-compositing feasibility spike (docs/01 §4, docs/09 §4-5; audit C-3/C-4 widen scope) → record ADR
- [ ] xtask check-layering command (docs/18 §2.1)
- [x] GitHub infrastructure files per docs/meta/github-infrastructure-plan.md: ci.yml, release.yml skeleton, PR/issue templates, toolchain pins
- [ ] End-of-phase Senior Architect Pass + Phase Audit Summary (docs/18 §3)

## Blocked / Needs Human Input

- [!] Meaningful Phase 1 work on suno-http-client-core — blocked on: fresh confirming capture of Clerk auth flow + library listing (leads exist from prototype recon; see docs/meta/suno-api-ground-truth-from-prototype.md §2–3)
- [?] Audit decisions: approve/reject/modify items in docs/meta/AUDIT-findings-and-recommendations.md (esp. scope items S-1 creation-front-end charter amendment and S-3 Phase 6b Suno Creation Studio)

## Awaiting Your Verification (agent says done; confirm → remove or lock)

- [x] Full planning doc set (docs/00-20, 99) authored — verify by reading README index + spot-checking any two docs
- [x] Chat-export extraction completed; all 33 blocks landed as files with per-doc commits — verify via `git log --oneline`
- [x] Meta audit round authored (5 docs under docs/meta/) — verify by reviewing AUDIT-findings-and-recommendations.md decision boxes

## Verified Complete (user-locked)

*(none yet)*
