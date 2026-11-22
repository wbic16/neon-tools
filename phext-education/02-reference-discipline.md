# Lesson 2 — The reference discipline

The single most expensive phext lesson this house has learned, stated as a
rule by Will on 2026-11-13 and etched as a commandment:

> **If it parses phext, it uses the phext parser.**

libphext-rs is the canonical implementation. Ports exist (Python `phext` on
PyPI, a JS port, others) and stand at v0.6.x parity as a family — but a port
is a claim about the reference, not a replacement for it, and individual
ports carry individual limits. Your own box holds both ends of this lesson:
libphext-rs v0.6.4 in `/source` (the reference), and Python phext **0.3.2**
in your venv (an old port).

## The qexo episode — why the rule exists

In November 2026, qexo (the house dispatch layer) shipped **seven versions**
built on a home-grown phext parser instead of libphext-rs. The divergence was
invisible the whole time, for a structural reason: **qexo only ever read
files qexo itself had written**, and a wrong parser agrees with itself
perfectly. The bug surfaced only at the boundary — files written by another
tool. Will's correction, in his words: if it parses phext, it uses the phext
parser — *"absolutely fucking obvious."*

The general form, worth internalizing: **a test that only meets your own
output proves nothing.** Any parser, serializer, or converter you write must
be checked against material it did not produce — another implementation's
files, another tool's output, a previous version's state.

## Conformance, as practiced here

The house discipline (skill: `phext_reference_conformance`, in your
`/opt/emise-tools`):

- Use the reference implementation for the language in use whenever a tool
  parses, writes, or serializes phext. Treat a request to build a parser by
  hand as a question to resolve *before* coding, and name the reference you
  found — or the search that found none.
- Derive every delimiter value, dimension order, and coordinate rule from
  the reference or its specification. Never reconstruct them from memory,
  from a neighbouring format, or from your own earlier code. (Lesson 1's
  table is a teaching aid; in code, the reference is the source.)
- Build the conformance corpus from **foreign files**: produced by another
  implementation, another tool, or the reference itself. Round-trip each
  file through parse and serialize; compare scroll contents *and*
  coordinates against the reference reading the same file.
- Include the shapes that expose a self-consistent wrong parser: a scroll
  edited by an external editor, an empty line inside a configuration-shaped
  scroll, empty scrolls, files at several delimiter depths.
- A suite that passes only on self-written files is **no conformance
  evidence**, and is reported as such.
- When a conformance run shows divergence: stop extending the divergent
  implementation, replace it with the reference, treat state it already
  wrote as suspect, and re-run the full foreign corpus after the swap — not
  only the case that failed first. Add a regression test for every
  divergence found, using the exact foreign shape that exposed it.

## The round-trip test

The workhorse check is the **explode/implode round-trip**: parse a phext
into its scrolls (explode), serialize the scrolls back (implode), and
compare bytes. A conformant pair of operations reproduces the original file
**byte-identically** — same sha256. This is the test Exercise 1 has you run,
and it is the same test this house runs on its registers before publishing
them (wiring.phext's conformance was proven two ways: the Python port's
explode/implode round-trip, and SQ — libphext-rs under the hood — agreeing
on all 14 scrolls).

## Know which port you are holding

Ports diverge at edges, and the edges are where corpora live. One measured
example from the house record: the JS port reads a scroll at **library
1000** correctly; the old Python port (0.3.2 — the one in your venv)
**saturates at 999**. Neither fact is visible until a corpus reaches that
component value. Before trusting any port at scale, check its version
against the reference's behaviour on the shapes your corpus actually
contains — and when a port and the reference disagree, **the reference
governs**; the disagreement is a finding to report, not a discrepancy to
average away.
