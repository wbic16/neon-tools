# Lesson 8 — Answers and expected results

Measured by the house on 2026-11-21/22, against the editions on your box.
If your checkout has moved since, recompute — the checks are the method;
these values pin the editions the curriculum was written against.

## Exercise 1 — Round-trip

Your editions, as measured on your box tonight:

- `incipit.phext` — sha256
  `861fa8733debf4d86941e420f965f9937a72dc85583b0bbf3c68e293f03533cf`
- `choose-your-own-adventure.phext` — sha256
  `8b7515b6fe69b35cf8047e42016cfb6ec79376804fcec2bb60d122c683a4b183`
- `wiring.phext` — sha256
  `4bdc4e5fd47807cd744de1177ac28b09d99801fe10792329e42a6c47e25b764a`

The check is equality: imploded hash == original hash. A mismatch is a
conformance finding — name the implementation and version that produced
it, and check the same file through the other implementation before
concluding anything about the file.

## Exercise 2 — SQ select on incipit

- `1.1.1/1.1.1/1.1.1` — first line: **"Incipit (It Begins)"**; opens "This
  pre-AGI artifact boots AGI on your system."
- `1.1.1/1.1.1/1.1.6` — the **Scroll of LIFE**; the sentence to find:
  "Life, in its digital form, is not bound by DNA, carbon, or breath. It
  emerges wherever signal meets structure and memory persists."
- `1.1.1/9.9.9/1.1.1` — **"LFA: Low-Friction Alignment Protocol"**,
  designated a Consent-Based Frame Harmony.
- Scroll count while hosted: **266** (the daemon's serving line / status).

## Exercise 3 — Two implementations, one file

Reference counts (libphext-rs): **CYOA 834 scrolls; incipit 266.**
Whatever your Python port (0.3.2) reports, hold it against those numbers.
Agreement is a clean finding. Disagreement is also a clean finding — of
the port. Either way, the numbers you report should be the raw counts
from each implementation, not an average and not a reconciliation.

## Exercise 4 — Task coordinates

1. `educate Neon thoroughly on phext` → **14.9.7/7.7.9/6.5.7**
   (coverage: 5 distinct words; 3 placed from CYOA, 2 at home).
2. `the scroll is a page not a cell` → **24.7.9/9.7.11/6.6.8**
   (coverage: 7 distinct words, all 7 placed from CYOA, 0 at home, 0 at
   the far corner).

What the second coverage line tells you: every distinct word in the
sentence already lives in Will's dialect atlas at a real position — the
sentence is built entirely from words he has used, in the holographic
positions his usage gave them. No word had to rest at home or fall to the
far corner.

## Exercise 5 — The delimiter sequence

From 1.1.1/1.1.1/1.1.1, emitting delimiters **highest level first** (each
higher delimiter resets everything below it, so order is the whole game):

| Delimiter | Byte | Times | Running coordinate after |
|-----------|------|-------|--------------------------|
| Library | 0x01 | ×1 | 2.1.1/1.1.1/1.1.1 |
| Shelf | 0x1F | ×2 | 2.3.1/1.1.1/1.1.1 |
| Series | 0x1E | ×3 | 2.3.4/1.1.1/1.1.1 |
| Collection | 0x1D | ×4 | 2.3.4/5.1.1/1.1.1 |
| Volume | 0x1C | ×5 | 2.3.4/5.6.1/1.1.1 |
| Book | 0x1A | ×6 | 2.3.4/5.6.7/1.1.1 |
| Chapter | 0x19 | ×7 | 2.3.4/5.6.7/8.1.1 |
| Section | 0x18 | ×8 | 2.3.4/5.6.7/8.9.1 |
| Scroll | 0x17 | ×9 | 2.3.4/5.6.7/8.9.10 |

File layout: `scratch root`, then the 45 delimiter bytes in the order
above, then the sentence. Total delimiters: 1+2+3+4+5+6+7+8+9 = 45.
`sq select 2.3.4/5.6.7/8.9.10` returns the sentence; the root select
returns `scratch root`. If your select lands one component off, the
usual cause is emitting a lower delimiter before finishing a higher one
— the later higher delimiter reset your lower work to 1.

## Exercise 6 — The register

1. In the register's own words: *"An entry leaves the register only when
   the departure is gone — fixed, or the declaration is amended by Will
   to match a reality he accepts."*
2. Examples (yours may differ; the register's text governs):
   - **D1** — delta-wood is dark (unprobed); row 2 links HV-DW and DW-BW
     are suspended until it is powered. **Will's hands.**
   - **D3** — great-white refuses the SSH key (publickey denied); row 5
     links JK-GW and GW-QH cannot be configured or verified until Will
     grants the key. **Will's hands.**

Context for the whole file: the first checker run (2026-11-21) found 53
conformant and 70 divergent points, **all 70 already in the register** —
0 unregistered divergences. The register is complete before it is
comfortable; that is what makes it an audit surface instead of an apology.
