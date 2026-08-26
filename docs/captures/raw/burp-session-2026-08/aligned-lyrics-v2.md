# Timed Lyrics, Downbeats & Waveform (T1 capture 2026-08-25)

Karaoke + visualizer data plane. All GET/POST to
`https://studio-api-prod.suno.com/api/gen/{clip_id}/…`.

## `GET /api/gen/{id}/aligned_lyrics/v2/` (3 captures)

- **Response** → 200 (~173 KB):
```json
{
  "aligned_words": [
    {"word": "Howdy, ", "success": true, "start_s": 8.697,
     "end_s": 8.976, "p_align": 0.99},
    {"word": "I'm ",   "success": true, "start_s": 9.016,
     "end_s": 9.176, "p_align": 0.99},
    "...truncated..."
  ],
  "waveform_data": { },
  "hoot_cer": { },
  "is_streamed": false,
  "aligned_lyrics": { }
}
```
- **Correction vs prototype recon**: field names are `word`, `start_s`,
  `end_s` (+ `success` bool, `p_align` confidence float) — NOT
  `start_time`/`end_time`; no top-level `language`/`status`. Words include
  trailing whitespace/newlines and section markers (`[Intro]\nDip `), so the
  original lyric text is recoverable by concatenation.
- `aligned_lyrics` and `hoot_cer` were objects in this capture (content not
  inspected further); treat as optional/opaque.

## `POST /api/gen/{id}/downbeats_streaming/v2` (3 captures)

- **Request**: `{}` (empty JSON object)
- Despite the name, response arrives as a single complete JSON → 200:
```json
{
  "state": "complete",
  "final": true,
  "downbeats": [[0.0099, 1.0], [0.7001, 2.0], [1.39021, 3.0], "...[time_s, beat_index] pairs..."],
  "raw_downbeats": [ ],
  "downbeat_presence_confidence": <float>,
  "onset_map": { }
}
```
- Beat index cycles 1–4 (beat number within bar) — directly usable for
  visualizer beat-sync.

## `GET /api/gen/{id}/waveform-aggregates` (3 captures)

- **Response** → 200 (~657 KB):
```json
{"waveform_aggregates": [
  {"mip_map_level": 11, "data": [[-2427, 3059], [-5556, 8735], "...truncated..."]},
  {"mip_map_level": 12, "data": ["..."]},
  "...levels 11 through 23 observed..."
]}
```
- Each level is a min/max sample pair array; higher `mip_map_level` = coarser.
  Large payloads — fetch lazily per clip, cache on disk.

## Observed behavior

- All three endpoints are requested together when a clip opens in the player
  (same second timestamps across captures).
