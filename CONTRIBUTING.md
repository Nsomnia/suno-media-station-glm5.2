# Contributing

**Read first: [`AGENTS.md`](AGENTS.md)** — it is the binding front door for
every contributor, human or AI.

## The unusual part

No humans write code on this project. Development is performed by LLM coding
agents inside an opencode + oh-my-opencode harness; the repository owner acts
as orchestrator/reviewer. Contributions follow the same rules whether authored
by an agent session or a human using agent tooling:

1. **Constitution compliance** — `docs/process/03-agent-constitution.md`
   (file-size caps, prior-art-first mandate, ask-vs-infer protocol) is binding.
2. **Layering** — dependencies point only downward
   `ui → application-services → domain-stores → external-bridges → foundation`;
   `cargo xtask check-layering` enforces it mechanically.
3. **Definition of Done** — builds warning-free, clippy clean, minimal tests,
   size caps respected, docs updated in sync, `TODO.md` updated, PR opened via
   `gh pr create`.
4. **Never invent Suno API shapes** — the contract in
   `docs/specs/suno-integration/06-suno-api-integration-contract.md` is
   capture-driven; request a capture via the issue template instead of guessing.

## Mechanics

- Trunk-based, short-lived branches named `phase-<N>/<slug>`; squash-merge via
  PR only (`docs/process/19-git-workflow-and-repo-governance.md`).
- Conventional commits (`feat:` / `fix:` / `docs:` / `chore:` …).
- CI must be green before merge: fmt, clippy `-Dwarnings`, tests, guardrails,
  cargo-deny (see `.github/workflows/ci.yml`).

## Ideas & bugs

Use the issue templates — ideas route into
[`docs/product/99-ideas-backlog.md`](docs/product/99-ideas-backlog.md);
Suno-API questions route to a capture request.
