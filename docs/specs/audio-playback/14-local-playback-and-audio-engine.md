# Local Playback & Audio Engine Spec

> **Last Updated:** 2026-08-25 · **Status:** Active

## 1. Scope

Covers `audio-decode-symphonia-bridge`, `audio-io-cpal-bridge`,
`local-playback-parity-service` (Phase 2), `recorded-audio-take-store` +
`audio-recording-capture-service` (Phase 2's recording slice), and
forward-notes for Phase 9's DAW evolution. This is the "make local files
feel exactly as good to use as remote streaming" layer, plus the seed of
audio capture.

## 2. Decode Layer (`audio-decode-symphonia-bridge`)

- Wraps `symphonia` (pure-Rust, actively maintained, wide format support —
  confirmed prior-art choice per doc 03 §3) to decode local audio files
  (whatever formats Suno downloads actually arrive as) into a
  normalized PCM stream: fixed sample format (e.g. `f32` interleaved),
  explicit sample rate and channel count metadata attached to every
  decoded chunk.
- **Probe actual downloaded file formats during Phase 1 before locking any
  decode-profile assumptions** — likely MP3/WAV/FLAC, but do not assume
  formats ahead of observation; confirm what Suno's CDN actually serves
  via doc 06 capture/observation, then finalize the supported-format list
  and decode profiles from evidence.
- This crate's public API should be a simple pull-based iterator/stream
  abstraction (`fn next_chunk(&mut self) -> Option<PcmChunk>`) — consumers
  (playback engine, visualizer feed, Whisper transcription, headless
  export's audio walk) all pull from the same normalized shape, so
  **decode logic exists in exactly one place** despite being consumed by
  four different downstream features (doc 18 §2.2 duplicate-logic
  guardrail, directly relevant here).

## 3. Playback Engine (`audio-io-cpal-bridge` + `local-playback-parity-service`)

- `audio-io-cpal-bridge` wraps `cpal` for output device enumeration/
  selection and the actual audio callback loop; kept thin (device I/O
  only), mirroring the FFI-bridge-crate pattern from doc 09 §3 (thin raw
  wrapper crate + a fuller-logic crate above it).
- `local-playback-parity-service` owns: play/pause/seek/volume/queue
  state, gapless-ish handling between queued tracks (buffer-ahead the next
  track's initial decode while the current one nears its end — "gapless-
  ish" because true sample-accurate gapless across arbitrary formats is a
  stretch goal, not a hard v1 requirement; note this honestly rather than
  overclaiming per doc 08 §8's accessibility-honesty precedent), and the
  unification of "playing a local file" vs. "previewing a remote stream
  URL" behind one player interface so the UI doesn't need two separate
  player implementations (doc 04 Phase 2 exit criteria: parity between
  remote-preview and local-file playback UX).
- This service is also the audio tap point for the visualizer's live
  preview feed (doc 09 §5) — it exposes a way to subscribe to the
  currently-playing PCM stream (a broadcast channel or similar) rather
  than the visualizer needing its own separate playback/decode instance
  for the same audio.

## 4. Recording Capture (`audio-recording-capture-service`, Phase 2 slice)

- Minimal v1: enumerate input devices (via the same `audio-io-cpal-bridge`
  crate, input-side), record to a local WAV (or similar lossless/simple
  format — avoid needing an encoder dependency for v1, PCM WAV is
  trivially correct and sufficient), save via `recorded-audio-take-store`
  (doc 07 §8) with basic metadata.
- No real-time monitoring effects, no punch-in/out, no multitrack — this
  is "hit record, capture a clean take, stop, save, play it back" only.
  Explicitly deferred to Phase 9: mixing, multitrack, effects processing,
  any JUCE-equivalent DSP work.

## 5. Phase 9 Forward-Notes (non-binding, for future phase-doc authoring)

Concrete Phase 9 scope (multitrack sessions, mixing/DSP crate evaluation,
etc.) is deferred to doc 04's Phase 9 definition — see that phase entry
when planning begins. The one warning worth keeping here: "recreate
something like JUCE" is an extremely large undertaking if taken literally,
and scoping it down to what Suno Station actually needs is the single
largest scope-inflation risk in the entire roadmap.

## 6. Error Handling & Device-Change Resilience

- Both playback and recording paths must handle a device disconnecting
  mid-use gracefully (e.g. Bluetooth headphones dropping, USB interface
  unplugged) — surface a clear UI notification and fall back to system
  default device rather than silently hanging or crashing the audio
  callback thread. This is a common real-world annoyance worth explicit
  handling rather than an edge case to skip.
