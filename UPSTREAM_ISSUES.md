# Issue drafts for appergb/OpenTake (from the video-edit trial fork)

## 1. External MCP serves a stale timeline after GUI edits (FIXED in fork d66b6f6)

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

- Window close can leave the process running (Linux); reproduced repeatedly. (FIXED in fork 0676d06: close saves and exits on non-macOS.)
- Export progress can read 0% while working when the main thread stalls
  (root cause = issue 2).
- Compositor horizontally squeezes wide (16:9) sources placed in a portrait
  project instead of letterboxing.
- Native menu does not rebuild on language switch (restart applies it).

## 5. set_clip_properties on one linked clip silently mutates its partner (FIXED in fork 4aa0a26: refusal by default, allowLinkDivergence for J/L)

Probed live 2026-09-01 (v1.0.0-beta.5 + trial patches): changing
`trimStartFrame` on the AUDIO clip of a linked pair returns success, but the
change is propagated to the linked VIDEO clip as well — the caller asked to
modify one clip and two changed, with no indication in the response. Either
refusing divergence explicitly or reporting the propagated change would be
honest; silent partner mutation is neither. (Fork batch 2 replaces this
behavior with explicit link-divergence semantics for J/L cuts.)

## MCP lifecycle tools vs the identity guards (fork, 2026-09-03)

The external-MCP LiveProjectMcpGate cancelled any tool whose execution
changed the project identity — making open_project (and the fork-added
new_project) non-functional over MCP, and (once exempted naively) a
self-deadlock: the gate holds the identity read lease while the tool
takes the same lock exclusively. Fixed in the fork with a dedicated
lifecycle dispatch path (no lease, no cancellable registration) plus a
clean refusal in the in-app chat's ProjectTurnGate. Documented residual
limitations (internal notes, NOT for upstream filing per fork-only
policy): new_project loses a prior UNSAVED scratch (rollback reopens a
saved predecessor only); bundle-exists check is TOCTOU against
OTHER-process creators; a GUI project switch racing the post-dispatch
capture window can make the returned result stale by one identity.

## MCP placement persistence + cover decode (fork, 2026-09-03)

Codex pipeline review found the "0 clips persist" root cause: MCP
save_project took the identity WRITE lock while the LiveProjectMcpGate
held the identity READ lease → self-deadlock → 60s timeout. Fixed by
routing save_project through the lease-free lifecycle path (like
open/new/list). Also fixed: retained-file frame decode fed ffmpeg a
non-seekable pipe with `-ss` after `-i` (decode-from-start), blowing the
15s cover deadline → "decode failed"; now attaches the cloned file as a
seekable fd with `-ss` before `-i`. Deferred hardening (internal notes,
fork-only): import publishes on ffprobe failure via ProbedMedia::default
(should fail-closed); add_clips doesn't validate mediaRef existence/trim
against the manifest; cover decoder collapses all errors to one string.
