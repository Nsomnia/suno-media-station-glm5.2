# suno-media-station

A native Rust desktop companion for [Suno.com](https://suno.com) with advanced
library management, local playback, karaoke/visualizer video generation, and
automation features not available in Suno's official clients.

> Status: pre-release, active development — see [TODO.md](TODO.md) and the
> [phase roadmap](docs/product/04-phase-roadmap.md). *(badges: build / license /
> rust-version placeholders — add CI badges here once workflows land)*

## Documentation

The full documentation wiki lives in [`docs/`](docs/README.md) — organized by
purpose (product, architecture, specifications, phases, process), with every
doc cross-linked from the hub.

Quick entry points:

| Doc | What it covers |
|---|---|
| [Project Charter](docs/product/00-project-charter.md) | Vision, pillars, non-goals |
| [Agent Constitution](docs/process/03-agent-constitution.md) | Binding operating rules for AI agents |
| [Phase Roadmap](docs/product/04-phase-roadmap.md) | Phase plan & current status |
| **[Full doc index](docs/README.md)** | Every doc, by category |

## Repo Layout

- [`docs/`](docs/) — master source of truth (wiki hub: [`docs/README.md`](docs/README.md))
- [`crates/`](crates/) + `app/station-app/` — Cargo workspace
- [`AGENTS.md`](AGENTS.md) — front-door orientation for AI agent sessions
- [`TODO.md`](TODO.md) — living task tracker (always current)
- [`CHANGELOG.md`](CHANGELOG.md) — Keep-a-Changelog release notes

## For AI Coding Agents

Read [`AGENTS.md`](AGENTS.md) **first**, then the docs it points to — in
particular the [Agent Constitution](docs/process/03-agent-constitution.md) and
the [Codebase Health Guardrails](docs/process/18-codebase-health-guardrails.md),
which are binding. Check [`TODO.md`](TODO.md) before starting work and update
it before finishing.
