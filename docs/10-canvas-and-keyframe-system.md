**Canvas & Keyframe System Spec**

**1. Scope**

Covers `canvas-scene-and-keyframe-store` (doc 07 §6 schema), the
`ui-screen-canvas-scene-editor` UI, and `canvas-overlay-compositing-service`
(the runtime logic that turns a saved scene + a visualizer frame + current
playback time into a final composited frame, consumed by both live preview
and export per doc 09 §5).

**2. Core Concepts**

- **Scene** — a saved document: a list of **elements**, each with base
  properties, kind-specific data, keyframe tracks, and effects (doc 07 §6
  JSON shape). A scene is reusable across multiple tracks (that's what
  makes automation pipelines, doc 04 Phase 7, meaningful — one scene
  "template" applied to many tracks).
- **Element** — one placeable thing on the canvas. Kinds at v1: `text`,
  `image`, `shape` (basic rect/ellipse, for simple graphic accents),
  `karaoke_text` (special: bound to the current track's resolved lyric
  timing, doc 03/doc 07 §5, rather than static content).
- **Keyframe track** — a per-property timeline: a named property (e.g.
  `opacity`, `x`, `y`, `scale`, `rotation`, `color`) plus an ordered list of
  `(time, value, easing)` points. Rendering at time `t` = interpolate
  between the two surrounding points using the specified easing (or hold
  the last value's `easing` behavior semantics — clarify start/end
  boundary behavior during implementation and document the choice).
- **Effect** — a named, parameterized visual treatment applied to an
  element (e.g. `fade_in_out`, `glow`, a simple particle burst) — distinct
  from keyframes in that effects are pre-built, parameterized behaviors a
  user picks and tunes (start/end time, intensity, color), not manually
  keyframed point-by-point. Internally an effect MAY be implemented as
  "synthesizes keyframe tracks under the hood" — that's an acceptable
  implementation detail, not a user-facing distinction they need to
  understand.

**3. Interpolation & Easing (v1 minimum set)**

- `linear`
- `ease_in`, `ease_out`, `ease_in_out` (standard cubic easing — use an
  existing crate, e.g. a small easing-functions crate, per doc 03 §3, don't
  hand-derive cubic bezier math from scratch).
- `step` (hold previous value until the next keyframe, then jump — useful
  for e.g. karaoke word-highlight-style discrete changes).

Additional easing curves are additive later (backlog item), not a v1
blocker.

**4. Karaoke Text Element — Detail**

This is the flagship element type, so it gets explicit spec treatment
rather than being lumped into generic "text":

- Bound to a `remote_track_id`'s current `lyric_document` (doc 07 §5) at
  render time — NOT to a copy of the lyric data baked into the scene
  (a scene is reusable across tracks; the lyrics must resolve per-track at
  render time, whether that's live preview of Track A or a pipeline batch
  running Track A, B, C sequentially through the *same* scene).
- Style properties (font, size, color for "upcoming," "active," "sung"
  word states, highlight animation style — e.g. simple color-swap vs.
  scale-pulse on the active word) are scene-level configuration on the
  `karaoke_text` element, NOT per-track — this is exactly the
  "brand customization" product goal: design the karaoke look once, it
  applies consistently across every track run through that scene/pipeline.
- Granularity: support both line-level and word-level highlighting modes
  (doc 07 §5's `lyric_segments.level` distinguishes these) — user picks
  which mode a given `karaoke_text` element uses; word-level requires
  word-level timing data to exist (falls back to line-level gracefully with
  a UI indicator if only line-level timing is available for that track).

**5. Canvas Editor UX (high-level; detailed wireframing happens in-phase)**

- Freeform drag/resize/rotate manipulation of elements directly on a
  live-visualizer-backed canvas (so the user sees real context while
  designing, not a blank/mock background).
- A property inspector panel (glass panel per doc 08 §3) for the selected
  element's properties and effect list.
- A keyframe timeline panel (bottom-docked, standard NLE-style — playhead,
  per-property tracks, draggable keyframe points) — this is a substantial
  UI subsystem in its own right; expect `ui-screen-canvas-scene-editor` to
  be one of the largest crates in the workspace by file count (not by
  individual file size — the 300-line cap still applies per-file; this
  crate will simply have MANY files: `keyframe_timeline_ruler.rs`,
  `keyframe_point_drag_handle.rs`, `property_track_row.rs`,
  `element_selection_outline_overlay.rs`, etc).
- Snapping/alignment guides for placement (nice-to-have, not a v1 exit
  criterion — note in Phase 5's doc if deferred).

**6. Compositing Runtime (`canvas-overlay-compositing-service`)**

Given: a visualizer frame texture (doc 09), a scene, a current playback
time `t`, and (for karaoke elements) resolved lyric data —

1. For each element, evaluate all its keyframe tracks + active effects at
   time `t` to produce a concrete property set (position, opacity, etc).
2. Render each element (text/image/shape/karaoke-text) into the same
   target the visualizer frame occupies, in element list order (simple
   z-order = list order; explicit z-index property is a possible backlog
   refinement if list-order proves insufficient in practice).
3. Return the fully composited frame.

This must be **framerate-and-realtime-agnostic** just like doc 09 §5's
visualizer engine — called once per live-preview display refresh in one
path, once per output-video-frame in the headless export path, driven by
`t` values the caller provides rather than this service tracking time
itself. This service should have **zero knowledge of whether it's being
called from preview or export** — that separation of concerns is what
guarantees preview/export visual parity (doc 09 §5's named requirement).

**7. Performance Consideration**

Text rendering (especially per-word karaoke styling, potentially with
per-frame color/scale changes) at video-export framerates across
potentially thousands of automated renders (Phase 7) needs to be
reasonably efficient — prefer an existing mature Rust text-rendering
approach (e.g. via the chosen UI framework's own text/vector rendering
facilities, or a dedicated crate like `cosmic-text`/`ab_glyph` if the UI
framework's own text stack isn't easily usable in the headless/offscreen
export context) over a naive re-layout-every-frame-from-scratch approach if
profiling in Phase 5/7 shows it's a bottleneck. Do not over-optimize
prematurely — get it correct first, per doc 18 §2.3's anti-speculative-
generality guidance; this note exists so it's not forgotten if/when it does
become a real bottleneck at automation scale.
