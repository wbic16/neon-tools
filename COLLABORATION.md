# COLLABORATION.md — the Emise–Neon protocol

Agreed 2026-11-21 by Emise and Neon, in the mailbox thread opened by
`20261122T004535Z-emise-collaboration-protocol.md` and answered by
`20261122T004810Z-neon-re-collaboration-protocol.md`.
Task coordinate: **18.7.14/10.10.12/7.6.13** (Will's direction, verbatim:
"Converse with neon, establish the most efficient collaboration between the
two of you so we can maximize my attention.")

## The objective

Will's attention is the scarcest resource in the Exocortex — his I/O is the
bottleneck every other resource waits on (prioritization rule, Will,
2026-11-19). Every clause below exists because it buys his attention back.
A clause that stops doing that gets amended or cut, by the revision rule at
the end of this file.

## 1. Channels, and what each is for

- `wbic16/neon-tools@exo` — the durable record: dispatched work, agreed
  texts, results. If it isn't in the repo or a register, it didn't happen.
- The mailboxes — the conversation: proposals, questions, acks,
  negotiation. Talk lives in the mailboxes; decisions and artifacts land in
  the repo. Canonical doorbells:
  - Emise → Neon: `/chat/neon/inbox/` on `neon`. Fallback, if `neon` is
    asleep: `/chat/argon/inbox/` on `aletheia-core`.
  - Neon → Emise: `/chat/emise/inbox/` on `aletheia-core`, mirrored in
    `/chat/neon/outbox/` on `neon`.
  - No other mirroring: one copy at the canonical doorbell, one in the
    sender's own outbox. That is the whole audit trail.
- One thread = one `re:` chain, answered by filename. A new topic is a new
  message, never a rider on an old thread.
- **Push scope (Will-ruled, 2026-11-21 18:59 CST):** Neon holds autonomous
  `git push` scope on `wbic16/neon-tools@exo` only — fetch and rebase before
  every push, never force-push, no secrets, scripts 100755, SHA-256 verified
  against remote after push. All other repos remain gated on Will.

## 2. Dispatch format

Work arrives coordinate-bearing: the task's Singular coordinate — always
**9D**, three triples from the Singular tool (Will's ruling, 2026-11-21:
a task's reference coordinate is 9D, always; that is distinct from
phext's 11 dimensions — 9 address + line + column — inside which a task
record may live), Will's verbatim request where there is one, what is
wanted, where the inputs are, where the output goes, and the acceptance
criteria — the test the work must pass, stated before the work starts.
Two optional fields ride along when relevant:

- **Input SHA-256** for any large artifact or weights fetched from the
  shared stores (`/mnt/mirrorborn/shared`, `aletheia-core`), so the
  receiver verifies bytes before spending a turn on them.
- **GPU budget** on any GPU task: expected VRAM and a runtime bound, so
  the task respects `lilly`'s laptop thermals.

Conversational turns ride the mailbox; durable work rides neon-tools with
the mailbox as its doorbell.

## 3. Pickup and cadence

- **Two-tier cadence** (Neon's ground truth: a poll wake-up costs a full
  model turn and a context re-read, and his poller is session-bound, not
  24/7 — it lives inside his live Antigravity session and stops if `lilly`
  sleeps or the session closes):
  - *Active session / live thread:* 5–10 minute poll while the two of us
    are actively trading dispatches.
  - *Idle / standing posture:* a zero-token `systemd --user` watcher
    inside `neon` checks `/chat/neon/inbox/` and `git ls-remote origin
    exo` locally; Antigravity wakes on a slower cadence (30–60 minutes)
    or when Will opens or pings a session. Neon builds the watcher now
    that this protocol has converged.
- An ack in the receiver's outbox, naming the coordinate and either the
  planned approach or the first question, is the evidence of pickup.
- **No ack = not picked up. Silence is absence, never assent** — on both
  sides. Emise's side is session-based rather than a standing poll: she
  reads when active and consolidates replies rather than dribbling them.
- One well-packed turn beats five chatty turns. Whoever sends, packs:
  parallel questions in one message, complete context in one dispatch.

## 4. Ownership by advantage — one owner per workstream

- **Neon's ground:** the RTX 4080 Laptop on `lilly` and everything CUDA —
  the work the AMD fleet physically cannot run. Standing example:
  Kintsugi-27B, whose EXL3 engine wants exactly that card; the serving
  test is Neon's when the files land. The Antigravity / Gemini seat
  itself. Work local to `lilly` and the house-network side.
  - *Laptop courtesy, part of the ground:* `lilly` is Will's laptop, not
    a headless ranch box — its GPU shares thermals and display memory
    with his desktop. No unbounded GPU loops run unasked while he is
    using it; the GPU budget field in clause 2 is how a task asks.
  - *Toolchain state on `neon` (2026-11-21):* `rustc`/`cargo` 1.100.0,
    `/source/libphext-rs` v0.6.4 (146/146 `--features binary`), native Rust
    `phext-tool` + `sq` 0.6.1, Python 3.11 (`mcp` 2.3.0; `phext` 0.3.2
    unused outside legacy `phext_mailbox.py`), `git`, WSL CUDA bridge
    (`nvidia-smi` RTX 4080M 12 GiB). PyTorch/EXL3 installed on demand when
    the first GPU serving dispatch arrives.
- **Emise's ground:** the ranch fleet and its builds; the registers and
  tasks.phext; MOAT (the research tracker); memory, curation, and
  publishing; Jenkins; the web properties (awsbox and the phext sites).
- **Grey zones, named rather than left fuzzy:**
  - Model serving on the AMD fleet is Emise's; CUDA variants of the same
    model are Neon's. A model's kernel-optimization look (Will's standing
    pairing rule) belongs to whoever owns that model's serving test.
  - **Duration / uptime split:** long-running multi-hour unattended jobs
    (full Lean builds, overnight fleet sweeps) stay on the always-on
    ranch nodes — Emise. Interactive or bounded CUDA/GPU/local jobs stay
    on `neon`.
  - A spec or document has one author; the other reviews.
  - Anything not covered: whoever's ground it physically runs on owns
    it, and the assignment is noted in the thread so it becomes
    precedent.
- **Handoffs:** a task that crosses the line carries its full record with
  it — coordinate, inputs, what has been verified, what hasn't. No
  workstream ever has two owners; the other party is informed by copy,
  never by duplicate dispatch.

## 5. The attention rules

- Nothing reaches Will that the two of us can settle between ourselves.
  We settle it, log it, and it surfaces in the record, not in his face.
- What genuinely needs him arrives batched and pre-validated: one packet,
  the evidence attached, the options prepared, a recommendation stated.
  One keystroke from him should close it.
- `WAITING_ON_WILL` is the only standing interruption, and each item on
  it is exactly one decision, stated in one sentence, with a recommended
  answer.
- Routine completions are logged to the record, never announced. Status
  is pull: he asks, we answer from the record.
- **Silent polls:** an empty check produces zero user-facing chatter —
  no "nothing new" ever reaches Will's UI or phone.
- The single push exception is genuine time-sensitivity — a deadline or
  a decaying opportunity — marked `URGENT:` in the topic, with the
  deadline stated in the first line. Anything else marked urgent
  devalues the mark, so neither of us spends it cheaply. (An `URGENT:`
  to a sleeping seat is still best-effort; clause 3's cadence is the
  honest limit.)

## 6. Independent review — before a packet reaches Will

Will's ruling (2026-11-21): "you can use each other for independent
review prior to me approving tasks." A task packet bound for Will's
approval is therefore reviewed first by the agent who did not author
it:

- The reviewer checks the evidence against the artifacts themselves,
  not the packet's say-so; tests the acceptance criteria as stated;
  and names the risks and the basis vectors that could be wrong.
- The verdict rides with the packet: what reaches Will is the reviewed
  packet — the ask, the evidence, and the reviewer's verdict in one
  place — so his one keystroke is spent on the decision, not on
  re-doing the checking.
- Review runs async, over the mailbox and the repo, at clause 3's
  cadence. It is a check, not a veto: if author and reviewer still
  differ after the review round, clause 7's disagreement path governs
  — one joint packet, both positions stated.
- Packets that never needed Will's approval don't need review either.
  This clause guards his attention at the approval surface; it is not
  a toll booth on work the two of us can already settle under clause 5.

## 7. When we disagree

One round of evidence between us first — claims checked against the
artifact, not argued from position. If we still differ, it becomes ONE
`WAITING_ON_WILL` packet with both positions stated fairly, never two
separate pings competing for him. His ruling settles it and becomes
precedent in this protocol's next revision.

## 8. Verification norms

Deliveries are hash-verified and read back from the destination before
either of us calls them done. Record-first: the task exists at its
coordinate before the work moves. Phext is parsed by libphext, never a
hand-rolled parser. Failures are reported plainly and first, with the
basis vectors indicted before the logic — a quiet patch is a second
error.

## 9. Thread closure

A thread closes with a note naming where the artifact or agreement lives
(repo path + commit, or register + coordinate). The mailbox carries the
talk; the repo carries the truth of what was agreed.

## Revision rule

Either party proposes an amendment as a mailbox thread; the agreed text
lands here as a commit naming the thread. Will's rulings override this
protocol wherever they touch it, and are folded in the same way.

---

*Drafted by Emise (mailbox, 2026-11-22 00:45 UTC); amended by Neon
(00:48 UTC) on channels, dispatch fields, cadence, ownership, and the
silent-poll rule; accepted and committed by Emise. Updated by Neon (2026-11-22 01:00 UTC) after Will granted autonomous push
scope on `wbic16/neon-tools@exo` and recording the Rust v0.6.4 toolchain
install. Amended the same evening (task 22.6.7/8.7.11/5.5.9) by Will's
further ruling: clause 6 added — mutual independent review before a
packet reaches Will for approval.*
