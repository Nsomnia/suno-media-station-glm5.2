# Phase 0 Kickoff Prompt (Literal, Paste-Ready)

> **Last Updated:** 2026-08-25 · **Status:** Active

This is the literal prompt text to paste into the agentic coding harness
(opencode + oh-my-openagent) to begin implementation. Paste as-is, or
lightly adjust the project name if you haven't already find-replaced
"Suno Station" throughout the repo.

---

```text
You are the sole development team for this project — no human will write
any code. The human is your orchestrator and reviewer only. Before doing
anything else, read these files in full, in order:

1. AGENTS.md
2. docs/product/00-project-charter.md
3. docs/architecture/01-architecture-overview.md
4. docs/architecture/02-workspace-layout.md
5. docs/process/03-agent-constitution.md  (binding operating rules — internalize
   fully: file size caps, prior-art-first mandate via crates.io/gh search,
   the ask-vs-infer decision protocol, AGENTS.md/TODO.md maintenance
   requirements, full dev-team role coverage, and the note that token cost
   is not a constraint here — be as thorough as each task genuinely
   warrants, don't cut corners for token economy)
6. docs/product/04-phase-roadmap.md  (note §0's Core Maintainability Gate)
7. docs/specs/ui-ux/08-ui-ux-design-system.md
8. docs/specs/visuals-and-video/09-visualizer-projectm-integration.md  (note §4 build/packaging
   decision and §5's live-preview-vs-headless-export design)
9. docs/process/16-testing-strategy.md
10. docs/process/18-codebase-health-guardrails.md  (binding — the whole reason
    this doc set is this thorough is to avoid the predecessor project's
    ~25k LOC spaghetti outcome; take this doc as seriously as doc 03)
11. docs/process/19-git-workflow-and-repo-governance.md
12. TODO.md
13. docs/meta/session-protocol-autonomous-operation.md  (how "continue"
    sessions run — daemon loop, stop conditions)

Then execute Phase 0 exactly as scoped in docs/product/04-phase-roadmap.md's
"Phase 0 — Foundation" section, in this order:

1. Git/repo setup: initialize the repository if not already, set up
   .gitignore per doc 19 §9, confirm AGENTS.md and TODO.md exist and are
   accurate (they should already exist from planning — verify, don't
   duplicate).
2. Scaffold the ENTIRE Cargo workspace tree exactly as specified in
   docs/architecture/02-workspace-layout.md — every crate listed gets a real directory,
   a Cargo.toml, and a stub lib.rs + README.md per
   docs/templates/crate-stub-template, even crates whose real
   implementation is phases away. The workspace must compile
   (`cargo build --workspace`) with this stub tree in place before you
   move on.
3. Implement the Layer 0 foundation crates for real (not stubs):
   error-and-result-conventions, app-configuration-loader,
   structured-logging-and-tracing, design-tokens-theme-definitions
   (Catppuccin Mocha/Frappé/Macchiato/Latte + Monokai — source real
   palette values from an existing `catppuccin` crate / known-good
   Monokai reference, per the prior-art mandate, don't hand-transcribe
   hex codes from memory).
4. Do the required Phase 0 spike (doc 01 §4, doc 09 §4-5): validate that
   your chosen UI framework (egui, per current lean — confirm or switch to
   iced if a real blocker surfaces) can share/composite a GPU texture
   produced by a separate rendering context, since this de-risks the
   entire visualizer architecture. Record the outcome — including the
   projectM build/packaging decision (system-installed vs. vendored via
   build.rs) — as a new ADR in docs/17.
5. Build `app/station-app`: a themed, empty window with a left-nav shell
   (sections can be placeholder/disabled for now) and a working
   theme-switcher proving the design-token pipeline works end to end.
6. Build the `xtask` `check-layering` command per docs/18 §2.1.
7. For every piece of work, follow the full Definition of Done (docs/03
   §5) — tests, clippy, layering check, doc-sync, and a PR opened via
   `gh pr create` per docs/19 §5 — do not batch all of Phase 0 into one
   giant PR; split by logical unit (e.g., one PR for workspace scaffold,
   one for the foundation crates, one for the spike, one for the app
   shell, one for xtask).
8. Update TODO.md continuously as you go (docs/19 §7) — never let it go
   stale mid-session.
9. At the end of Phase 0, perform the Senior Architect Pass (docs/18 §3)
   and produce a Phase Audit Summary. Do not mark Phase 0 complete in
   TODO.md until this pass is done and its findings (if any) are resolved
   or explicitly logged in docs/product/99-ideas-backlog.md.

Ground rules while you work:
- Never guess a Suno API endpoint's shape — none should be needed in
  Phase 0 anyway, but if you find yourself tempted to stub something
  Suno-shaped, stop; that's Phase 1's job and depends on real captures
  (doc 06).
- Use `gh search repos` / `gh search code` / crates.io before writing any
  non-trivial logic from scratch — this includes, e.g., checking for an
  existing `catppuccin` Rust crate, existing easing-function crates,
  existing wgpu-texture-sharing examples, before hand-rolling any of it.
- Use `brew`, `git clone`, and local compilation as needed to get
  dependencies (e.g. projectM) working on this machine — you have
  permission to install tooling per docs/03 §12.
- When you hit a genuinely architecture-level ambiguity not resolved by
  the docs, stop and present the human a short multiple-choice breakdown
  with your recommendation (docs/03 §9) rather than guessing. For minor,
  reversible implementation details, make a reasonable call and note it.
- You are not token-constrained in any meaningful sense here (free-tier
  model) — be thorough. Do not skip tests, skip the prior-art search, or
  skip a self-review pass to save effort.
- CI exists (.github/workflows/ci.yml) — keep main green; every PR must
  pass clippy/test/fmt jobs.

Final step for every merged task: create/update CHANGELOG.md entries per
Keep-a-Changelog.

Begin with step 1.
