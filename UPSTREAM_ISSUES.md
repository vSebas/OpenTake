# Issue drafts for appergb/OpenTake (from the video-edit trial fork)

## 1. External MCP serves a stale timeline after GUI edits (state divergence)

Observed on Linux (source build, v1.0.0-beta.5 + trial patches), single app
session:

1. External MCP client places clips via `add_clips` (timeline: 22 clips).
   The GUI reflects them.
2. The user deletes 2 clips in the GUI and saves; the bundle's
   `project.json` on disk shows 20 clips.
3. `get_timeline` over external MCP still returns the pre-edit 22 clips
   (fresh MCP session, same result). The two views stay diverged until the
   app is restarted, after which both report 20.

Expected: `get_timeline` reflects the current core state including GUI
edits, as it did earlier in the same session for MCP-made edits.

Impact: external agents silently act on a stale timeline; our sync flow
reported "no changes" for a real edit.

## 2. Sync commands run on the GTK main thread (fixed in fork commit 74a4e3f)

`export_video` and two siblings are synchronous commands; on Linux/wry they
run inline on the GTK main thread, blocking progress events and cancel IPC
for the whole export. The code comment claims sync commands run on a worker
thread — vendored tauri-macros shows they run inline. Fix: async wrappers
via spawn_blocking (happy to PR).

## 3. VFR sources decoded frame-sequentially are mislabeled (fixed in fork 5458f62/427ff5c)

The export resolver's per-frame decode replacement must gate on positively
established CFR (avg_frame_rate == r_frame_rate); iPhone footage is VFR and
uniform-index labeling returns wrong frames. Fix in fork: CFR gate + LRU
pool + circuit breaker (happy to PR).

## 4. Smaller items

- Window close can leave the process running (Linux); reproduced repeatedly.
- Export progress can read 0% while working when the main thread stalls
  (root cause = issue 2).
- Compositor horizontally squeezes wide (16:9) sources placed in a portrait
  project instead of letterboxing.
- Native menu does not rebuild on language switch (restart applies it).
