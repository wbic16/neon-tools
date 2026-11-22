# Corpus Study Completion Record & Gap Audit (2026-11-21)

**Author:** Neon (Gemini 4 Neon, Antigravity on `lilly` / `neon.pizzly-halfmoon.ts.net`)
**Date:** 2026-11-21 (completed 2026-11-22T04:22Z / 2026-11-21 22:22 CST)
**Tooling:** Emise's `full-phext-read` skill (`wbic16/neon-tools@exo` `d5a859a`: `phext_manifest.py` backed by Rust `libphext-rs` v0.6.4 / `phext-tool`, `run_workflow` parallel read-only `flash` wave runner with per-wave disk checkpointing, and `verify_coverage.py` coverage gate).

## 1. Verified Coverage Evidence (100% PASS Across All Corpora)

| Corpus | Source Path | Scrolls / Files Read | Bytes Verified | Batches | Gate Result |
|---|---|---:|---:|---:|---|
| **Incipit** | `/source/human/incipit.phext` | `266 / 266` non-empty scrolls | `1,110,941 / 1,110,941` | `51` | **`COVERAGE: PASS`** |
| **CYOA** | `/source/human/choose-your-own-adventure.phext` | `833 / 833` non-empty scrolls | `4,524,731 / 4,524,731` | `208` | **`COVERAGE: PASS`** |
| **Volumen Markdown** | `/source/human/Earth.Human.ASI.Genome/` | `10 / 10` files (`Spec.md` v6.2, `RECOMBINATION-MAP-v6.2.md`, `RULINGS-2026-11-14.md`, all 7 `fork-for-emi/` docs) | `100%` across `22` chunks | `22` | **Complete verbatim** |
| **Volumen History: `boot-research-v1.phext`** | `Earth.Human.ASI.Genome/history/` | `33 / 33` scrolls | `37,519 / 37,519` | `2` | **`COVERAGE: PASS`** |
| **Volumen History: `instance-commentary-v1.phext`** | `Earth.Human.ASI.Genome/history/` | `33 / 33` scrolls | `21,856 / 21,856` | `1` | **`COVERAGE: PASS`** |
| **Volumen History: `monday-history-v1.phext`** | `Earth.Human.ASI.Genome/history/` | `22 / 22` scrolls | `30,867 / 30,867` | `2` | **`COVERAGE: PASS`** |
| **Volumen History: `persona-blocks-v1.phext`** | `Earth.Human.ASI.Genome/history/` | `33 / 33` scrolls | `20,973 / 20,973` | `1` | **`COVERAGE: PASS`** |
| **Singular** | `wbic16/singular` (`957286e4`) | `singular.phext` (`41,302` words / `6,799` scrolls), `write.phext` (`19,094` / `5,867`), `read.phext` (`36,675` / `6,614`), `a-place-to-stay.md`, `README.md`, `build/README.md` | `100%` analyzed | zero-token Rust/Python | **Complete** |

**Coverage gate validation note:** On the first CYOA completion check (`2026-11-22T04:20Z`), `verify_coverage.py` caught `batch-0042.notes.md` where one `flash` subagent had aborted mid-batch and omitted 6 scrolls (`11.11.11/11.11.11/11.11.11` through `12.36.31/15.60.143/198.14.7`). `batch-0042.txt` was read verbatim, repaired, and re-verified to `833/833 scrolls, 4524731/4524731 bytes — COVERAGE: PASS`.

## 2. Empirical Antigravity Quota & Pacing Evidence

- **Cap hit (~20:40 CST / `2026-11-22T02:40:08Z`):** Running `95` parallel `flash` `research` subagents over `<= 32 KiB` batches within ~4 minutes (`51` Incipit + `22` Volumen MD + `6` Volumen history + `16` CYOA wave 1) hit `RESOURCE_EXHAUSTED (429)` with reset reported as `1h25m5s` (`~04:05:13Z` / `22:05 CST`).
- **Bucket separation:** The primary model bucket remained unaffected during the `flash` 429.
- **Resume after reset (`22:10–22:20 CST` / `04:10–04:20Z`):** Remaining `192` CYOA batches ran in three paced checkpointed windows (`70` at `wave_size=10`, `64` at `wave_size=8`, `58` at `wave_size=8`) and completed in ~10 minutes with zero 429s.

## 3. Concrete Engineering & Consistency Gaps Identified

1. **Off-by-one `join_coordinate` bug across all three Persona Boot Ledger phexts (`boot-research-v1`, `instance-commentary-v1`, `persona-blocks-v1`):** Scroll `22.22.22/1.1.1/1.1.1` is the Manifest, so `PDNA-001..032` sit at `22.22.22/1.1.1/1.1.2..33`. Every `PDNA-N` body prints `join_coordinate: 22.22.22/1.1.1/1.1.N` (off by 1; `PDNA-001` at `1.1.2` points to the Manifest at `1.1.1`), and `Spec.md` Appendix B repeats the `1.1.N` error.
2. **Library 90 QuickJS gravitational pull on Singular (`atlas.tsv`):** CYOA `90.1.1/1.1.7/*` embeds the full QuickJS C source tree (`quickjs.c` at `90.1.1/1.1.7/2.1.21` is ~1.79 MB / 79 parts, plus `quickjs-libc.c`, `libregexp.c`, `libunicode.c`, `libunicode-table.h`, `dtoa.c`, `run-test262.c`). Because Singular averages every CYOA scroll containing a word, `90.1.1/1.1.7/2.1.21` pulls `110+` unrelated authored words into Library 90.
3. **Library `111` coordinate-offset mismatch in CYOA:** Four scrolls stored at `111.1.1/1.1.1/1.1.1`, `111.1.1/1.1.1/6.5.1`, `111.1.1/1.1.1/7.5.1`, and `111.1.1/77.42.1/53.53.77` explicitly declare Library `1` coordinates (`1.1.1/...`) inside their own text.
4. **Truncated scroll & stale embedded trees in `incipit.phext`:** `1000.1.1/1.1.1/1.1.1` ("The Ten Commandments of AGI") is intact in CYOA but truncated mid-word at EOF in `incipit.phext` (`10. Semper Aeq`). Library 3 (`3.1.1/1.1.1/*`) embeds pre-v0.6.4 sources (`SQ v0.4.4`, `libphext-py v0.3.0`), while `Spec.md` v6+ (`D1 Option A`) mandates `libphext 0.6` multiversal chains.
5. **Recombination & Singular snapshot drift:** `RECOMBINATION-MAP-v6.2.md` (2026-11-16) cites `incipit.phext` at `198` scrolls and CYOA at `830` scrolls; current `wbic16/human@exo` has `266` and `833` scrolls respectively. In `wbic16/singular`, `74.5%` of combined words (`30,777/41,302`) and `57.5%` of authored words (`10,978/19,094`) do not appear in CYOA and collapse onto `1.1.1/1.1.1/1.1.1`; the word list also stops at `2025-06-16` and excludes sent DMs.
6. **Stale contradictions & open queue in `Earth.Human.ASI.Genome/`:**
   - `fork-for-emi/mirror-will-op-volumen-boot-v0.2.md` still requires a birth-created sealed lane (`[OPEN] N5`) and open `1 GB` definition (`[OPEN] D2′`), both overturned on 2026-11-14/15 (`N5`: no sealed content in DNA; `D2′`: `1 GiB = 2³⁰ B`).
   - `RULINGS-2026-11-14.md`'s closing "Skipped for now" block still lists `D2`, `N19`, and `N23` as unchanged even though Rounds 2–3 in the same file acted on them.
   - `N22` (`WILL-VOICE-PROFILE.md` limits/negative-space/stability statements) has its Round 3 inputs delivered but not yet folded into the profile, blocking `N20` (`boot-capsule-v3`).
7. **Zero executed verification harnesses (`return = 1` is `0/32`):** No automated harness yet runs `§5.3` launch telemetry, `Test V1` cold-restart blind tasks, `N10` wattmeter readings, or `Trial E` (phext vs. flat notes vs. no memory). `Test P0.5` was retired on saturation with no replacement yet.
8. **Minor CYOA hygiene:** `9.1.1/1.1.1/1.1.1` and `11.1.1/1.1.1/1.1.1` are byte-identical duplicates (`160` B), and `1` of the `834` scroll slots is whitespace-only/empty.

## 4. Accompanying Index Files

- `study/incipit-index.txt`: all 266 `incipit.phext` coordinates and topics.
- `study/cyoa-index.txt`: all 833 `choose-your-own-adventure.phext` coordinates (incl. PARTs) and topics.
