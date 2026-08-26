# External-Bridges Layer — Orientation

This layer concentrates **FFI and unsafe-adjacent code**. Rules that differ
from the rest of the workspace:

- `unsafe_code = "deny"` at the crate level; any unavoidable `unsafe` block
  lives in the `*-ffi-bindings` crates only (`visualizer-projectm-ffi-bindings`
  is the expected home for raw bindgen output), wrapped behind a safe API
  before any other crate touches it.
- Every external process/library sits behind a trait boundary so the concrete
  backend stays swappable (doc 01 §2.3) — callers never import a vendor's
  types across their own crate's public API.
- Never invent Suno API shapes: [doc 06](../../docs/specs/suno-integration/06-suno-api-integration-contract.md)
  is capture-driven; halt on missing captures.
- Security lens is mandatory here: credentials never log, never persist in
  plaintext ([doc 05](../../docs/specs/suno-integration/05-auth-and-multi-account.md)).

Authoritative docs: [Architecture Overview](../../docs/architecture/01-architecture-overview.md) ·
[Constitution](../../docs/process/03-agent-constitution.md).
