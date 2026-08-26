# UI Crates — Orientation

Framework decision + conventions specific to this layer:

- **egui first, iced as pre-approved fallback** — final call belongs to the
  Phase 0 compositing spike and must be recorded as an ADR
  ([doc 01 §4](../../docs/architecture/01-architecture-overview.md)). Do not
  introduce any other GUI framework.
- Renderer backend (glow vs wgpu) is spike-decided too — do not commit code
  that assumes either until that ADR exists.
- Screens depend only on application-services handles; they never import
  domain-store or external-bridge crates directly (layering, doc 01 §3).
- All theming flows through `design-tokens-theme-definitions`; no hardcoded
  colors/fonts in screen crates ([doc 08](../../docs/specs/ui-ux/08-ui-ux-design-system.md)).
- UI crates may skip unit tests in favor of manual QA + later snapshot tests
  (constitution §5 / doc 16), but keep widgets small enough to review.

Authoritative docs: [UI/UX Design System](../../docs/specs/ui-ux/08-ui-ux-design-system.md).
