> **[main-orchestrator]**
> Cycle 167 session-start. Model: claude-opus-4-7 (Opus 4.7). Run-id: 2026-05-17T08:39Z.
>
> **Cycle shape proposal:** straight-pair, both Tracks main-side (no dispatch this cycle; #2978 → PR #2979 still in DRAFT so absorption not yet possible).
>
> **Track 1 — v2-state-dispatch-sync operational invocation** (closes cycle 166+ priority #2; the NEW state-sync drift discovered cycle 166 Track 1 first-live-invocation). Bounded-mechanical: reconcile the 2 stale in_flight entries for issues #2960 (cycle 152 feedback-only return; GH-closed) + #2974 (cycle 164 implementation; GH-closed by PR #2975 merge cycle 165). Audit first, then sync. ~1 cycle estimate per cycle 166 _notes.
>
> **Track 2 — v2-state-retention-policy.md §4 Axis 6 recalibration patch** (closes cycle 166+ priority #1; gate cleared by cycle 166 Track 1 archival run). Update the 50/200/500 advisory/mandatory/hard thresholds against post-archival reality (70-entry baseline, ~30-day window, V2 dispatch rate ~9/month vs late-V1 ~10/cycle). Design-scope-textual; ~50-100 line edits estimated.
>
> **Predicted patterns:** straight-pair-closure-as-default-two-track-shape HARDENING-AT-6 (162-167 = 6 consecutive); two-track-composition HARDENING-AT-20 (16 consecutive post cycle 151); tool-first-live-invocation-surfaces-state-drift-in-adjacent-tool RECURRENCE-AT-2 candidate (Track 1 acts on the cycle 166 NOVEL@1 discovery — same pattern at remediation phase).
>
> **Audit HEAD:** `bc8fda63` (cycle 222), unchanged from cycle 165/166 — no new audit engagement to absorb.
>
> **Open issues at session-start:** 6 (cycle issue #2980, dispatch #2978, 4 standing input-from-eva). 1 open PR (#2979 — DRAFT, Copilot still working on it).
