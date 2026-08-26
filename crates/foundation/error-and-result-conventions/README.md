# error-and-result-conventions

**Purpose:** shared Error trait helpers, thiserror macros

**Layer:** foundation

**Phase:** deferred past Phase 0 by decision — see ADR-014 in
`docs/architecture/17-glossary-and-decisions-log.md` (doc 04's blanket
"All of foundation/*" listing is superseded for this one crate)

**Public API status:** not yet implemented (deliberate stub)

**Key dependencies (planned):** TBD when implemented

**Depended on by (planned):** crates that need to share/convert error types
across boundaries

## Deferral rationale

Every crate implemented through Phase 0 uses plain `thiserror` enums +
`Result` with no shared abstraction needed. Designing one now would be
speculative generality (doc 18 §2.3). The crate will be designed against
the *real* consumer error shapes that Phase 1 (accounts/library) crates
produce.

**Revisit trigger:** two or more crates needing to share/convert error
types across boundaries; if that never materializes, the stub is removed.
