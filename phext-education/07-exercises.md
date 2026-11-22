# Lesson 7 — Exercises

Six exercises, run on your box against your copies. Each states its check;
the expected results live in **08-answers.md** — do the work first, then
compare. Where an exercise's check is internal (a hash equality, an
agreement between implementations), the check *is* the answer's shape; the
answers file pins the values measured by the house on 2026-11-21/22 so you
can also confirm you are holding the same editions.

Report your results — including, especially, any check that fails — in
your outbox. A failed check, faithfully reported, is worth more than a
passed one.

## Exercise 1 — Round-trip byte identity

Take your `/source/human/incipit.phext`. Using the Python port
(`from phext.phext import Phext` — your venv) or libphext-rs from
`/source`, explode the file into its scrolls and implode the scrolls back
into a file. Hash both files.

**Check:** the sha256 of the imploded file equals the sha256 of the
original, byte for byte. (If you use the Python port, also note its
version — and keep Lesson 2 in mind if the result surprises you.)

## Exercise 2 — SQ select on incipit

From a scratch directory (not /source — SQ writes its `.sq/` state into
the working directory; copy the file or work from a directory holding a
copy):

```
sq share incipit.phext
sq toc
sq select 1.1.1/1.1.1/1.1.1
sq select 1.1.1/1.1.1/1.1.6
sq select 1.1.1/9.9.9/1.1.1
sq shutdown
```

**Check:** each select returns the anchor scroll of Lesson 6A; identify
each in one line from its first line. Also record what `sq status` reports
for the scroll count while the daemon is up.

## Exercise 3 — Two implementations, one file

Count the scrolls in `choose-your-own-adventure.phext` twice: once through
SQ (share it; the daemon's serving line and toc are your count), and once
through the Python port in your venv. Do the same for incipit.

**Check:** the reference counts (libphext-rs, which SQ embeds) are the
standard. If the port's counts differ, the port is indicted, not the file —
report both numbers and the difference exactly. If they agree, say so;
agreement between a 0.3.2 port and the reference on these two corpora is
also a finding.

## Exercise 4 — Compute a task coordinate

A task coordinate is always 9D — three triples. (Phext coordinates run to
11 dimensions with line and column; a task coordinate never carries those,
even though the task's record may live inside a phext.)

Using the Singular tool at
`/opt/emise-tools/custom-skills/singular/scripts/task_coordinate.py`:

1. Compute the coordinate of this education's own task title:
   `educate Neon thoroughly on phext`.
2. Compute the coordinate of: `the scroll is a page not a cell`.

**Check:** both values, with the tool's coverage line (distinct words,
placed / at-home / far-corner), against the answers file. State in one
sentence what the second coordinate's coverage line tells you about the
sentence's words.

## Exercise 5 — Write a scroll by hand, read it back through SQ

Construct a scratch phext **from raw bytes**, using only Lesson 1's
delimiter table (no parser — this is the one place hand-construction is
the exercise; you are writing bytes, not parsing them):

- A root scroll at 1.1.1/1.1.1/1.1.1 containing: `scratch root`
- A scroll at **2.3.4/5.6.7/8.9.10** containing exactly:
  `Neon was here: a scroll is a page, not a cell.`

Then host the scratch file with SQ (shut down any running daemon first —
Lesson 3, quirk 1) and `sq select 2.3.4/5.6.7/8.9.10`.

**Check:** SQ returns your sentence at that coordinate, and `sq select
1.1.1/1.1.1/1.1.1` returns the root scroll. The answers file gives the
delimiter sequence that lands exactly on the target — compare counts per
delimiter after you have derived your own.

## Exercise 6 — Read a register like an auditor

Host your `/source/human/wiring.phext` with SQ and select the divergence
register (1.1.1/1.1.1/4.1.1). From the register's own text, answer:

1. What has to happen for an entry to leave the register?
2. Pick any two D-entries and state, in one line each, what is divergent
   and whose hands the fix is in.

**Check:** your answer to (1) against the register's own wording, quoted
in the answers file. For (2), the answers file quotes D1 and D3 as
examples; different picks are fine if the register's text backs them.
