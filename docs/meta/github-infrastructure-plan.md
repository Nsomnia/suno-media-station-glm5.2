# GitHub Infrastructure Plan — Zero-to-V1 Repo Readiness

> **Last Updated:** 2026-08-22 · **Status:** Proposed — awaiting orchestrator decision (Audit item A-4)

Doc 16 §4 fixed the CI *bar* but deferred the mechanics. For unattended
autonomous operation the mechanics can't stay deferred: a fresh agent needs
the repo to verify itself. Everything below is Phase-0-appropriate and small.

## 1. Workflows (`.github/workflows/`)

### `ci.yml` — every push + PR to `main`

| Job | Steps |
|---|---|
| `fmt` | `cargo fmt --check` |
| `clippy` | `cargo clippy --workspace --all-targets -- -D warnings` |
| `test` | `cargo test --workspace` (tooling-gated tests skip via env default-off) |
| `guardrails` | `cargo xtask check-layering` + `cargo xtask check-file-caps` + `cargo tree --workspace --duplicates` (advisory first) |

Runners: `macos-latest` + `ubuntu-latest` matrix on `test` (Windows added when
Phase 2 audio lands). `Swatinem/rust-cache` for target-dir caching. The
projectM/FFMI spike work may need brew/apt system deps — keep a documented
`CI-SETUP.md` snippet the agent updates when it adds a system dependency.

### `docs.yml` (optional, cheap)

Markdown link/reference checker over `docs/` on PRs touching docs — catches
the "doc N §M" drift class mechanically.

## 2. Templates

- `.github/pull_request_template.md` — exactly doc 19 §5's template
  (What&Why / Docs Referenced / Self-Review Notes / Testing / Docs Updated).
- `.github/ISSUE_TEMPLATE/capture-request.md` — structured ask for Burp
  captures: endpoint category (doc 06 §2.x), action performed, sanitized
  request/response blocks. Turns doc 06's halt-and-request into a clickable
  workflow.
- `.github/ISSUE_TEMPLATE/idea.md` — routes straight into doc 99 backlog
  format.
- `.github/ISSUE_TEMPLATE/bug.md` — standard, plus "phase/crate affected".

## 3. Branch Protection & Repo Settings

- Require PR before merge on `main`; required checks: clippy+test
  (fmt/guardrails advisory until code volume justifies hard gates).
- Linear history enabled (matches squash-merge policy, doc 19 §5).
- Auto-delete merged branches.
- Milestones: one per roadmap phase (`Phase 0 — Foundation`, …) so `gh issue`
  work maps onto doc 04 without translation.
- Labels: `phase-N`, layer tags (`external-bridge`, `domain-store`,
  `service`, `ui`, `foundation`), `capture-needed`, `stalled`.

## 4. Toolchain & Config Pins (repo root)

| File | Content |
|---|---|
| `rust-toolchain.toml` | pin current stable channel; agent bumps deliberately, not incidentally |
| `rustfmt.toml` | start empty/default — any override requires ADR (doc 16 §4 already says this) |
| `.editorconfig` | UTF-8, LF, trailing whitespace trim; 4-space Rust; 2-space YAML/TOML-ish consistency |
| `deny.toml` | cargo-deny: licenses allowlist (MIT/Apache-2.0/BSD/ISC/Unicode/Zlib), advisory scanning, duplicate-version ban matching doc 18 Gate item |

## 5. Release Engineering (minimal now, per doc 19 §8)

- Tag `v0.1.0` at Phase 2 completion (unchanged policy).
- Add `.github/workflows/release.yml` skeleton then: build matrix artifacts +
  draft release notes from conventional commits. Write it in Phase 0 but leave
  gated behind tag pushes so it costs nothing until used.

## 6. Definition of "Repo Ready" (Phase 0 gate, new exit criterion)

All of these exist and are green before Phase 0 closes:

- [ ] `ci.yml` running green on `main` (even with stub crates)
- [ ] PR template + three issue templates present
- [ ] Branch protection active; milestone set for every phase
- [ ] `rust-toolchain.toml`, `.editorconfig`, `deny.toml` committed
- [ ] Root `Cargo.toml` parses; workspace builds clean (audit E-1/E-10)
- [ ] `.gitignore` complete per audit E-11
- [ ] TODO.md uses state-mark spec; AGENTS.md points at both meta specs

## 7. What This Buys the Daemon

Every stop-condition-free hour of autonomous work ends with machine-verifiable
state: CI green = "I didn't break it", branch protection = "nothing landed
unreviewed-by-checks", capture-request issues = structured asks instead of
prose. This is what lets the orchestrator's entire involvement be reading
PRs and answering `[?]` rows.
