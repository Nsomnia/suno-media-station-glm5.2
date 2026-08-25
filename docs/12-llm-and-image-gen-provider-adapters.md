# LLM Text & Image-Gen Provider Adapters

> **Last Updated:** 2026-08-25 · **Status:** Active

## 0. Priority Status (per ADR-008)

This capability was originally slotted as "Phase 6, after everything else."
Per ADR-008, it is now positioned as **one of the two options unlocked
immediately after the Core Maintainability Gate (doc 18 §4) passes** —
i.e., it can be tackled right after Phases 0-5 are genuinely
production-grade, potentially *before* Phase 7 (Automation), at the human
orchestrator's discretion. This doc's design is written to be buildable at
that point without depending on Phase 7 existing yet. Where a design note
below assumes automation exists, it's flagged explicitly as a Phase-7-or-
later enhancement, not a hard dependency.

## 1. Scope & Philosophy

Suno Station does not implement LLM inference or image diffusion itself — it
provides **adapters** over remote APIs and/or local OpenAI-compatible or
ComfyUI/A1111-style servers the user already runs. This keeps the crate
thin, keeps Suno Station's own maintenance burden low (per doc 18's core
philosophy of avoiding scope-driven decay), and lets users bring whichever
provider/model they already pay for or self-host.

Two independent adapter surfaces:
- **Text generation** — lyric ideation/assist, applied inside the lyrics
  editor (doc 04 Phase 3 / doc 07 §5) as an "assist" panel, not a
  standalone chat window (see §5 on scope boundary).
- **Image generation** — cover/brand art assist, applied inside the canvas
  scene editor (doc 10) as a "generate art asset" action producing a usable
  `image` element source.

## 2. Text Provider Adapter (`llm-text-provider-adapter`)

Note on async trait style: Rust's native async-in-trait (AFIT, stable since
1.75) is preferred over the `#[async_trait]` macro unless object safety
(`dyn` dispatch) is genuinely needed. The `#[async_trait]` attributes in the
illustrative blocks below are placeholders — final call at implementation
time.

```rust
#[async_trait::async_trait]
pub trait TextGenerationProvider: Send + Sync {
    async fn generate(
        &self,
        request: TextGenerationRequest,
    ) -> Result<TextGenerationResponse, TextProviderError>;
}

pub struct TextGenerationRequest {
    pub system_prompt: Option<String>,
    pub user_prompt: String,
    pub max_output_tokens: Option<u32>,
    pub temperature: Option<f32>,
}

pub struct TextGenerationResponse {
    pub text: String,
    pub provider_name: String,
    pub raw_metadata_json: serde_json::Value, // token usage etc, kept for forward-compat
}
```

- **v1 concrete implementation:** a single `OpenAiCompatibleTextProvider`
  hitting any endpoint implementing the widely-adopted `/v1/chat/
  completions`-style contract — this alone covers OpenAI itself,
  OpenRouter (multi-model proxy), Ollama, LM Studio, and most self-hosted
  local inference servers, maximizing coverage for minimal adapter code
  (prior-art-first mandate, doc 03 §3, in action).
- **Configuration (per doc 08 settings screen):** base URL, API key
  (stored via `os-keyring-secret-storage`, same pattern as Suno auth per
  doc 05 §3), model name/identifier, and optional default
  system-prompt-per-use-case (a "lyric assist" system prompt distinct from
  any future non-lyric use case's prompt).
- **Additional providers (Anthropic/Gemini native APIs)** are additive
  future implementations of the same trait — not required for v1, listed
  here so the trait shape anticipates them (e.g. `raw_metadata_json`
  absorbing provider-specific response quirks rather than the trait itself
  needing provider-specific fields).

## 3. Image Provider Adapter (`image-gen-provider-adapter`)

```rust
#[async_trait::async_trait]
pub trait ImageGenerationProvider: Send + Sync {
    async fn generate(
        &self,
        request: ImageGenerationRequest,
    ) -> Result<ImageGenerationResponse, ImageProviderError>;
}

pub struct ImageGenerationRequest {
    pub prompt: String,
    pub negative_prompt: Option<String>,
    pub width: u32,
    pub height: u32,
    pub seed: Option<u64>,
}

pub struct ImageGenerationResponse {
    pub image_bytes: Vec<u8>,      // PNG, normalized regardless of provider's native format
    pub provider_name: String,
    pub raw_metadata_json: serde_json::Value,
}
```

- **v1 concrete implementations (pick at least one during Phase
  implementation, both are reasonable day-one targets):**
  - `RemoteApiImageProvider` — a straightforward REST-call adapter for a
    well-documented hosted API (e.g. OpenAI Images or Stability's API —
    confirm current API shape via their official docs at implementation
    time rather than assuming, since these APIs change).
  - `ComfyUiHttpImageProvider` — calls a user's already-running local
    ComfyUI server's HTTP API with a pre-defined or user-supplied workflow
    graph. This is the "power user with a GPU and existing local setup"
    path and is strongly aligned with the project's general "don't
    reimplement what a mature local tool already does" philosophy.
- Output is always normalized to PNG bytes at the adapter boundary so
  `canvas-scene-and-keyframe-store`/the canvas editor never needs to know
  which provider produced an image asset.

## 4. Cost/Rate Awareness (light touch, not a hard requirement)

- Both adapters should surface provider errors distinctly for
  auth-failure vs. rate-limit vs. generic-failure (`TextProviderError`/
  `ImageProviderError` enums), so the UI can show an actionable message
  ("check your API key" vs "rate limited, try again shortly") rather than
  a generic failure toast.
- No built-in spend-tracking/budgeting UI at v1 — that's a reasonable
  backlog item (`docs/99-ideas-backlog.md`) if user demand emerges, not a
  day-one requirement.

## 5. Explicit Scope Boundary — Not a General Chat Assistant

This is a **feature-embedded assist tool** (a button inside the lyrics
editor / canvas editor that calls out and inserts a result), not a
general-purpose chat interface living as its own app section. This
boundary is deliberate: a full chat UI is a much larger surface (history,
threading, multi-turn context management, etc) that isn't a named product
pillar (doc 00 §2) and risks becoming exactly the kind of scope creep doc
18 exists to prevent. If a genuine need for a broader chat surface emerges
later, that requires a fresh product decision + ADR, not an organic
expansion of this adapter crate.

## 6. Testing Approach

Per doc 16 §2, no live-network calls in the automated test suite — both
providers are tested against fixture-based fakes
(`shared-test-support/suno-api-fixture-mocks`'s sibling pattern, or a small
dedicated fixture module within each adapter crate if reuse doesn't fit
cleanly) verifying request construction and response normalization. Live
provider verification is a manual QA step (doc 16 §5-style checklist) using
the human orchestrator's own API key/local server during that phase's
development.

## 7. Interaction with Automation Pipelines (Phase 7, forward-reference only)

Once Phase 7 exists, a pipeline step type "generate art asset for track
using prompt template X" or "generate a lyric variant" becomes a natural,
additive pipeline step built on these same adapters — but this doc's v1
scope is the manual, single-action assist UI only. Do not build the
pipeline-step integration as part of this phase unless Phase 7 already
exists and the human orchestrator explicitly asks for that combined work.
