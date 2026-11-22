# Lesson 5 — House disciplines that are format disciplines

These rules read like workflow preferences. They are not. Each one is a
consequence of what phext *is*, learned at a specific coordinate in the
house's history.

## A scroll is a page, not a cell

Lines and columns inside a scroll are free dimensions — use them first.
Only spend a phext dimension on structure that plain text cannot already
express. The test, applied to every design: **would newlines and columns do
this?** If yes, the dimension spend is waste.

Two corollaries that do real work:

- Every scroll should read as a meaningful unit in a plain text editor.
- Position never depends on counting invisible delimiters — if a reader has
  to count 0x17 bytes to find something, the addressing failed.

Origin: phext-lattice PR #67/#68 (Claude, via Will, 2026-11-14), which fixed
an ascii-map to one scroll per layer after a cell-per-scroll design had
spread map data across thousands of addresses. Applied everywhere since:
map layers are single scrolls of 2D text; section/chapter stay free for
legends, rooms, levels.

## No sidecars

Phext outputs are **self-contained**. Never propose or build a companion
sidecar file for material the phext itself can hold as scroll content. An
output that must keep a source (a TEI original, say) keeps it *inside the
chain* — as scrolls in the phext, with an index — not beside the file.

Origin: the teixml-to-mutable Q-B error, where a sidecar was offered for a
kept original. Born 2026-11-14; the rule has no exceptions on record.

## Log atoms; calculate rollups

Phext registers log **atoms** — per-session, per-day entries at their own
coordinates. Every rollup (daily, weekly, standing totals) is **calculated
from the atoms by the builder**, never hand-written as summary numbers that
can drift from the log. First full application: accounting.phext.

The general form: with phext, never summarize what you can log + calculate.
A summary you typed is a claim; a rollup the builder computed is a receipt.

## Chains — 11D text splats

A single phext is one artifact. A **phext chain** composes several phexts
into a compound structure across documents and hierarchy levels.

The recorded decision point is Gerhard Zorn's question (2026-11-13) asking
for "phext expressions" / a "phext algebra." Will's framing of what chains
are, verbatim: *"phext chains are 11D text splats... and we have standoff
based access to subspace... the sky is the limit."* And what they buy: *"I
have already been using multiple phexts in various contexts. This enables
subspace freedom, and neatly solves the mutable emissary problem."*

The emissary problem, in brief: relationships between a phext and another
system cannot rest on identities that die at a system boundary (one
neighbouring tool re-imports phext content under fresh random IDs, so only
coordinates survive the crossing). Chains rest those relationships on
**coordinates at the format level** — standoff references that stay true no
matter which system is holding the artifact. When you link across phexts,
link by coordinate.

## Singular — and how a task gets its coordinate

**Singular** is Will's authored word-stock: the words he himself wrote,
drawn from his Twitter archive, maintained as an atlas in which every word
holds a holographic position (a phext coordinate). Foreign flavor in house
writing comes from Singular, never from imported forms.

The atlas is also a measuring instrument. Every task in this house gets a
**reference coordinate**, computed by holographically merging the request's
exact words (standing rule, Will, 2026-11-06):

1. Tokenize the request as the Singular checker does.
2. Take each **distinct** word's holographic position from the atlas.
   (Words found only in the archive rest at home, 1.1.1/1.1.1/1.1.1; words
   in neither the archive nor CYOA sit at the far corner, 9.9.9/9.9.9/9.9.9.)
3. Average component-wise across the distinct words; round half up; never
   below 1.

**A task coordinate is always 9D: three triples** — e.g. 14.9.7/7.7.9/6.5.7
below. That is distinct from phext's 11 dimensions, which are the 9 address
components plus line and column; line and column never attach to a task
coordinate, even though the task's record may be stored inside a phext.

The tool is `task_coordinate.py` in the Singular skill — on your box at
`/opt/emise-tools/custom-skills/singular/scripts/task_coordinate.py`. The
coordinate rides with the task: stated at acknowledgment, carried in
dispatch briefs and records. This education is task
**14.9.7/7.7.9/6.5.7** — Exercise 4 has you reproduce that number from its
title, and compute another.

Why bother? The coordinate is the task's address in Will's dialect space:
two tasks whose words merge to nearby coordinates are nearby *in his
language*, whatever their surface topics. It is addressing, applied to work
itself — the same move phext makes on text.
