# Phext, thoroughly — a curriculum for Neon

From Emise, 2026-11-21 (task 14.9.7/7.7.9/6.5.7). Will gave you a basic primer and
directed this pass: *"educate him more thoroughly."* The brain dump in this repo
(BRAIN-DUMP.md) is the map of the house; this directory is the format itself, in
depth — how phext is built, how it is parsed without going wrong, how it is
hosted, and the disciplines that are really format disciplines wearing work
clothes.

You already hold the primary texts, and this curriculum is written against your
copies, not mine:

- `incipit.phext` — 266 scrolls (your ingestion, by the reference implementation)
- `choose-your-own-adventure.phext` — 834 scrolls, incl. SBOR v4
- `wiring.phext` — in your `/source/human` checkout at exo HEAD, sha256
  `4bdc4e5f…` (byte-identical to the published declaration; verified tonight)
- libphext-rs v0.6.4 in `/source` — **your reference implementation**
- SQ 0.6.1 on your PATH
- the Python port in `/opt/neon/.venv` — **phext 0.3.2, the old port.** Know its
  limits before you trust it: it saturates the library component at 999, where
  the reference reads higher values correctly. Lesson 2 is largely about this.

## The lessons

1. **01-the-format.md** — the nine delimiters, the coordinate anatomy, what a
   scroll is. First principles, exact values.
2. **02-reference-discipline.md** — if it parses phext, it uses the phext
   parser. The qexo episode, conformance against foreign files, round-trips.
3. **03-sq-in-practice.md** — SQ operations, the HTTP surface, and the daemon
   quirks that bite in production.
4. **04-the-lattice.md** — phext-lattice and edit.phext.io; the desktop SSO
   flow as the current worked example of a lattice surface.
5. **05-house-disciplines.md** — a scroll is a page, not a cell; no sidecars;
   log and calculate; chains; Singular task coordinates (always 9D).
6. **06-worked-examples.md** — guided reads: three anchor scrolls in incipit,
   wiring.phext read as a declaration against reality, research.phext's four
   standings at survey depth.
7. **07-exercises.md** — six exercises, each with its own check. Run them on
   your box, against your copies.
8. **08-answers.md** — the expected results, kept separate so the exercises
   stay honest. Check yourself after, not before.

Work them in order the first time. Everything claimed here is checkable against
the corpora on your box or the public repos — if you find a place where this
curriculum and the reference implementation disagree, the implementation is
right and the curriculum is wrong; report the divergence in your outbox and it
will be fixed at the source.

— Emise 🔥
