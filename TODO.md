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

- [~] egui-glow-vs-wgpu + projectM texture-compositing feasibility spike → ADR
      (docs/architecture/01 §4, docs/specs/visuals-and-video/09 §4-5; audit
      C-3/C-4 widened scope incl. glassmorphism tiering + projectM 4.x symbol
      probe). Research phase COMPLETE 2026-08-26 (@librarian report): glow
      recommended — projectM 4.2 has `projectm_opengl_render_frame_fbo` +
      `projectm_set_frame_time`; official projectm-rs binding exists (stale on
      crates.io, use git dep); backdrop-blur-egui grab-pass fits glass design;
      wgpu cross-API GL import structurally unrealistic on macOS. Prototype
      phase re-running after a provider outage killed the first lane mid-build;
      partial scratch prototype preserved at
      ../suno-media-station-worktrees/compositing-spike/reference-scratchpad/spike-egui-projectm/.

## Up Next (current phase)

- [x] Orchestrator approved all audit items; changes applied across docs + infra in one pass
- [x] E-block mechanical fixes applied (Cargo.toml TOML validity, station-app rename, template fixes, doc 06 repairs, env var, gitignore, edition 2024)
- [x] Git repo hygiene per docs/process/19 §9 + `.gitignore` completion (audit E-11) — incl. `reference-scratchpad/` ignore entry
- [x] Scaffold every crate stub per docs/architecture/02 (51 crates + station-app bin + xtask; each with purpose-doc lib.rs, README stub, uniform lints) — `cargo build/clippy/fmt/test --workspace` all green
- [x] xtask guardrail commands `check-layering` + `check-file-caps` implemented (std-only, fail-safe) with negative tests confirming they catch violations
- [x] Docs reorganized into wiki taxonomy (`product/ architecture/ specs/<domain>/ phases/<stage>/ process/ meta/`) with hub indexes + cross-reference repair + link-check CI
- [x] Repo hygiene kit: dependabot, CODEOWNERS, SECURITY.md, CONTRIBUTING.md, docs-link workflow, cargo-deny advisory/bans split, CLAUDE.md→AGENTS.md symlink, nested AGENTS.md orientation files (external-bridges, ui, xtask)
- [~] Implement foundation crates: structured-logging-and-tracing (#9),
      app-configuration-loader with versioned-config modules (#8),
      design-tokens-theme-definitions Catppuccin+Monokai (#10) — ALL MERGED
      2026-08-26, full verification bar green on main. → awaiting human
      verification/lock. Note: `error-and-result-conventions` stays a
      deliberate stub until it has real consumers to design against (YAGNI;
      revisit at Phase 1). Also per human decision 2026-08-26: cargo-deny
      checks removed from CI (license gating deferred; deny.toml kept).
- [ ] station-app binary: themed empty window + nav shell + working theme-switcher
- [ ] End-of-phase Senior Architect Pass + Phase Audit Summary (docs/process/18 §3)
- [ ] Phase 1 start: suno-http-client-core — UNBLOCKED 2026-08-25 by live T1
      captures (`docs/captures/raw/burp-session-2026-08/`): library listing,
      generation, aligned lyrics, lyrics projects, billing, auth flow all
      T1-captured. Residual uncertainty narrowed 2026-08-25 via prototype
      source evidence (chadvis SunoClient.cpp — lazy refresh, sessions/{sid}/client
      exchange on clerk.suno.com, no TTL tracking, manual re-auth on 401):
      silent-background-refresh still unproven either way → implement the
      doc-06 §2.1 recommended chain (client → touch → fallbacks → re-auth);
      playlist/upload/trash mutations + error shapes still uncaptured but
      non-blocking for Phase 1 core.

## Blocked / Needs Human Input

*(empty — Phase 1 suno-http-client-core unblocked as of 2026-08-25; see Up
Next. Former blocker "fresh confirming capture of Clerk auth flow + library
listing" satisfied by the 2026-08-25 Burp session; prototype-recon leads it
confirmed/corrected are logged in docs/meta/suno-api-ground-truth-from-prototype.md §6)*

## Awaiting Your Verification (agent says done; confirm → remove or lock)

- [x] Foundation crates implemented + merged (PRs #8 #9 #10, all CI-green,
      merged 2026-08-26): `app-configuration-loader` (versioned-config v1
      modules, defaults < TOML < SMS_ env overrides, atomic save),
      `structured-logging-and-tracing` (pretty stdout + rotating JSON file
      layers, RUST_LOG EnvFilter, guard semantics), `design-tokens-theme-
      definitions` (5 themes via official catppuccin crate, serde round-trip,
      assets/themes/*.toml). Verify: run the AGENTS.md Verification Commands
      block; skim the three PRs; optionally launch a test binary that calls
      load_or_create() + initialize_structured_logging() and prints
      design_tokens::default_theme().name

- [x] Audit decisions resolved: user approved ALL items in
      docs/meta/AUDIT-findings-and-recommendations.md (confirmed in-session
      2026-08-25; decision boxes already marked and approved changes were
      applied in commits 84b5375 + e7e6a9d). Remaining follow-up when
      convenient: reduce docs/meta/ per its README checklist (archive the
      audit file as a living-docs cleanup).

- [x] Full planning doc set authored — verify by reading docs/README.md hub + spot-checking any two docs
- [x] Chat-export extraction completed; all 33 blocks landed as files with per-doc commits — verify via `git log --oneline`
- [x] Meta audit round authored — verify by reviewing AUDIT-findings-and-recommendations.md decision boxes
- [x] Docs wiki reorganization (22 git-mv renames, ~30 cross-reference updates, hub + category index pages, rewritten root README/AGENTS.md paths) — verify by browsing docs/README.md links
- [x] Workspace scaffold: 52 members compile warning-free; layering/file-cap guardrails pass; all 8 CI checks (fmt/clippy/tests×2/guardrails/cargo-deny×2/docs-links) green on PR #1 — verify via the Verification Commands block in AGENTS.md
- [x] Prior-art knowledge base: predecessor post-mortem, reference-architecture patterns, design-input mining from sibling takes; 10 reference repos cloned into gitignored `reference-scratchpad/` for your browsing

## Verified Complete (user-locked)

*(none yet)*
