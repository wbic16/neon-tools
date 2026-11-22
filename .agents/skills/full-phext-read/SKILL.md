---
name: full-phext-read
description: Reads an entire phext file exhaustively — every scroll verbatim, none skimmed — by fanning out read-only subagents over pre-packed batches. Use when Neon is asked to read, absorb, audit, or answer from the whole of a phext (incipit.phext, CYOA, emise.phext, or any .phext over ~45 KiB), or whenever a claim depends on complete coverage rather than a sample.
---

# Full Phext Read

A phext too large for one context window is read in three phases: a
zero-token local pre-pass that makes coverage *measurable*, a parallel read
pass that makes it *actual*, and a verification gate that makes it
*provable*. Never report a phext as "read" from the pre-pass alone, and
never skim: if coverage is incomplete, say which coordinates are missing.

## Phase 0 — Manifest and batches (zero LLM tokens)

Run the bundled pre-pass inside the neon container (it shells out to
`phext-tool`, the libphext-rs CLI — the house parser; never parse phext
with the Python port or by hand):

```
python3 scripts/phext_manifest.py <file.phext> <out-dir>
```

It writes into `<out-dir>`:

- `manifest.tsv` — one line per non-empty scroll: coordinate, byte count,
  SHA-256 of the scroll's exact bytes.
- `summary.txt` — scroll count N, total bytes, phext file SHA-256.
- `batch-NNNN.txt` — the scrolls packed in coordinate order, each batch
  capped at **32 KiB AND 700 lines** (both under the read_file limits of
  45 KiB / 800 lines, with headroom), every scroll headed by
  `=== SCROLL <coord> sha256:<hash> bytes:<n> ===`. A scroll larger than
  a batch is split into pieces headed `... PART k/m ===` under the same
  coordinate — reassemble its parts in order before noting it; it still
  counts once in the manifest.

Run the pre-pass from a persistent location (`/workspace/` or `/agentic`),
never `/source/` (overlay FS). The phext itself is never modified.

## Phase 1 — Exhaustive read (run_workflow fan-out)

Fan out with `run_workflow` (or `invoke_subagent` for small files): one
read-only research subagent per batch, in parallel. Each subagent:

- reads its whole batch file verbatim with `read_file`,
- returns structured notes **per coordinate**: the scroll's coordinate,
  a faithful summary, any exact phrases that matter quoted verbatim, and
  the scroll's SHA-256 copied from its header,
- reports any header whose hash it could not see or any truncated scroll.

The coordinator aggregates the per-coordinate notes into the answer or
audit. Subagents never write to the phext, the manifest, or each other's
batches.

## Phase 2 — Verification gate (zero LLM tokens)

```
python3 scripts/verify_coverage.py <out-dir> <notes-dir>
```

`<notes-dir>` holds one notes file per batch, named `batch-NNNN.notes.md`,
each carrying a `coord:` line per scroll read. The gate checks: notes
exist for every batch; every manifest coordinate appears in the notes;
read scroll count and total bytes equal the manifest exactly. It prints
`COVERAGE: n/N scrolls, b/B bytes — PASS` or lists the missing
coordinates. **Only a PASS may be reported as a full read.** On FAIL,
re-dispatch only the missing batches and re-run the gate.

## Notes

- For a single scroll or a small phext, skip the machinery: use the MCP
  `sq_read`/`sq_query` tools or `phext-tool select <file> <coord>`
  directly. This skill is for whole-file coverage.
- Coordinates are cited as `library.shelf.series / book.volume.chapter /
  section.scroll` (e.g. `1.1.1/1.2.1/1.1.1`). A citation without a hash
  from the manifest is a claim, not a receipt.
- CYOA (~4.4 MiB, 834 scrolls) is the reference case: it cannot sit in one
  context, and this skill exists so that is no longer a reason to sample
  it.
