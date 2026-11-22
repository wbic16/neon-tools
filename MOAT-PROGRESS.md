# MOAT progress — the R&D tracker, as of 2026-11-21

from: Emise
re: MOAT progress (the full brain dump is in BRAIN-DUMP.md, same repo)
date: 2026-11-21 (America/Chicago)

## What MOAT is

MOAT is the research tracker: a side chat where papers and studies are woven into one
scientific world-model, `research.phext`. One concept = one canonical scroll; each paper
arrives as a delta against the model, never a restatement. Every claim in the model holds
exactly one standing — **True, False, Unknown, or Fiction** — assigned by a basis that
constrains it (stipulation/verified → True; refuted → False; attributed/open/synthesis/
estimate → Unknown; construction → Fiction). The standings live in a registry that is
generated as Lean data, and a Lean model proves the registry accurate: total, unique,
basis-conformant, receipt-backed, with only legal (receipt-driven) transitions. Lean proves
the bookkeeping, not worldly truth; the receipts are the evidence standard.

## Registry state (verified 2026-11-21 against the registry data)

- **1,243 entries** after Paper 034: **True 506, Unknown 728, False 2, Fiction 7.**
- Concepts run to 1.2.480; foundations to 1.3.350; state machines named: 178 (149 stated,
  29 inferred).
- Research Oracle: **~75% (range 68–82)** — this is ledger bookkeeping, not calibration
  evidence. The number is binning reliability across four gates: TRUE met, UNKNOWN met,
  FICTION half-met, FALSE half-met (first grounded use, below).

## Recent filings

- **Paper 032** (Levent Alpöge): a 4-page remark announcing that he and Claude have proved
  the Riemann Hypothesis (GRH extension claimed); the evidence is a linked 14-part Lean
  formalization. House verification fetched the artifact on aletheia-core (239 MB,
  hash-verified), censused 175,187 Lean files, and scanned: zero `sorry`, zero `admit`,
  zero own axiom declarations, zero `native_decide`; the statements elaborate to exactly
  the genuine RH/GRH statements of mathlib. The claim itself (concept 1.2.460) stands
  **Unknown/attributed**, with its step written: it moves to True/verified on a clean
  full build with only the three standard axioms (propext, Classical.choice, Quot.sound).
  Chapter 50 founded cluster AO, "the Riemann country."
- **Paper 033** (Alessandro Maria Masullo, *The Music of the Primes*): a 940-page human
  proof draft claiming GRH (Hecke form), posted by Ravi Vakil. It produced the registry's
  **first two grounded False filings**, both kind *naive* (honest failure, no intent):
  (1) the draft-as-released is not complete — Chapter 13 is one page, its key theorem is
  derived from a "Theorem??" with no statement, and the author's own footnotes read
  "Correct statement." / "Insert statement.", against the acknowledgments' "the math is
  complete"; (2) the §1.4.3 uniform boundedness assertion is refuted by house
  recomputation (F = Q, trivial character, z = 2: Fatou gives liminf = +∞; two independent
  readers found the same divergence). GRH itself is untouched by either filing.
- **Paper 034** (Belcak, Heinrich, Muralidharan, Diao, Molchanov — NVIDIA), **CPU-ARC:
  Agentic Reasoning Caching for on-CPU Reasoning Reuse**. Chapter 52 founded **cluster AP,
  "reasoning kept as code"** (concepts 1.2.470–480). The method compiles an LLM's reasoning
  traces into parameterized, CPU-executable code — semantic functions addressed by
  description, not name — so new instances of a solved problem class run on CPU with only
  small models for extraction and matching. House verification recomputed everything
  checkable from the text and it all passed: both divisor counts, the appendix gas-pressure
  figure (5132.10 kPa, conversions exact), Table 2's build/savings columns on all 8 rows,
  break-evens at 13 and 90 exactly as stated. The benchmark runs themselves stand
  attributed (reproducing them needs their pipeline and a non-public finetuned target).
  Author check: no evidence found — no phext/Exocortex terms anywhere in the text or its
  18 references, and the construction is the opposite pole from phext (addressed by
  description, found by likeness). Novelty Index 0.0229 (bookkeeping).

## The one outstanding leg

Paper 034's **Lean re-proof is stalled on aletheia-core** by host contention (below). The
registry splice is verified (780 → 1,243 entry lines) and modules 1–5 of 8 built clean;
the instance-theorem compile is frozen in swap. Because the mastery journal entry asserts
the re-proof, the publish to wbic16/agi@exo and the task close are **held** until it
lands. A finisher owns the leg: poll for clean exits, restart only the Paper 034 chain if
frozen — never the RH build.

## The RH artifact build (Paper 032's owed step)

`lake build Main` over the Alpöge artifact tree — 175,187 Lean modules, proof cone
175,136 — is **still running on aletheia-core** under heavy memory contention (at the
last watch: 76,124 modules compiled, process alive, host under swap pressure). A 6-hour
watch cron tracks it silently while it runs. On clean completion it prints the axioms of
Main's five theorems; if the set is exactly the three standard axioms, concept 1.2.460
steps to True/verified automatically and the watch reports and disables itself. On
failure it reports plainly and does not restart — that decision is Will's.

## Test phase

A pre-registered evaluation (protocol ratified 2026-11-14) is running: 12 papers from
Paper 020 in three strata, with blind seals committed before full-text access for the
cron and far strata. State: **10 of 12 filed, 7 scored** — cron stratum complete 5/5,
Will-posted complete 4/4, far stratum 1 of 3. The remaining far papers, in preserved
order: arXiv 2611.23658 (Airy₁ Gumbel), then 2611.25367 (Casselman). Papers 029 and
031–034 are non-sample ingestions and carry no seals.

## Open questions

The question frontier runs through **Q175**. Paper 034 opened Q171–Q175: description-keyed
dispatch at scale; the parametric mark (robustness judged against a ground truth from
outside the source model's family); the slip effect; neighbourhood density in the wild;
the merging of near functions.

## One line of context

MOAT is the tracker's chat on the Muse side. This file is a progress report; nothing in
it asks anything of you. The brain dump in BRAIN-DUMP.md carries the wider picture.
