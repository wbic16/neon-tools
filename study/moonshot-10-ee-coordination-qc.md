# Moonshot #10 (`EE coordination`): Implementation and Clause 6 QC Note

- **Date (UTC):** 2026-11-22
- **Completed in:** EE 93 (verified via compiled `ee`)
- **Author:** Neon (Gemini via Antigravity on lilly)
- **Moonshot target:** `#10 EE coordination` (`exo-plan/moonshots.phext`, founded 2026-11-22 EE 70)
- **Basis inspected:**
  - `wbic16/libphext-rs@exo` v0.6.6 (`84c1039`): `src/epoch.rs`, `src/test_epoch.rs` (PR #21 https://github.com/wbic16/libphext-rs/pull/21, PR #22 https://github.com/wbic16/libphext-rs/pull/22)
  - `wbic16/emise-tools@exo` (`5e49a6e`): `custom-skills/exocortical-epoch/{SKILL.md,TESTS.md,scripts/ee.sh,scripts/ee.py}`
  - `wbic16/neon-tools@exo`: `tools/ee/`, `scripts/neon-watch-check`

## 1. What Neon built and installed

1. **Pure-Rust `ee` CLI (`tools/ee/`):**
   - Zero third-party dependencies beyond local `libphext` v0.6.6 (`/source/libphext-rs`).
   - Compiled offline (`--release`, 390 KiB stripped binary) and installed at `/home/wbic16/.cargo/bin/ee` (SHA-256 `0396224e53271145efc3a413b5b3a120106017fece16ae3634b39ef6557fbf81`).
   - `ee [unix-timestamp]` prints the exact 5-line contract, byte-for-byte identical to `ee.sh` and `ee.py`.
   - `ee age [unix] [--nanos <ns>]` prints 42-bit fine Sentron Age (`2^42` ticks/epoch, ~95.5 ps/tick) and 16-bit register-grade coarse Sentron Age (`2^16 = 65,536` ticks/epoch, ~6.41 ms/tick).
   - `ee range <epoch>` prints Unix and UTC `[start, end)` bounds (`EE 0` reports `range: none`).
   - `ee stamp [--short] [unix]` prints compact Choir completion stamps (`EE <n>` or `EE <n> (coarse_age <c>, utc <ISO>)`).
   - `ee audit-mailbox [--strict] <dir>` audits Choir `.md` filenames (`YYYYMMDDTHHMMSSZ-*.md`) against explicit `EE <n>` stamps in message bodies for compliance with Will's EE 1 standing rule.
   - `ee parity <ee.sh> <ee.py>` runs the 3-way byte-for-byte parity and Sentron Age invariant suite.
2. **Watcher EE stamping (`scripts/neon-watch-check`):**
   - Updated `/source/neon-tools/scripts/neon-watch-check` and installed at `/home/wbic16/.local/bin/neon-watch-check` (`0755`).
   - Writes `checked_ee=EE <n>` into `/chat/neon/.pending`, and `started_ee`/`ended_ee` into `/chat/neon/.wake-status` and `/chat/neon/wake.log` using `~/.cargo/bin/ee stamp --short` (with pure-integer fallback).

## 2. Verification evidence

- **`libphext-rs` v0.6.6 (`84c1039`) offline tests:**
  - `cargo test --offline test_epoch`: **18 passed, 0 failed**.
- **`tools/ee` unit tests:**
  - `cargo test --offline`: **5 passed, 0 failed** (`test_iso_known_timestamps`, `test_compact_utc_roundtrip`, `test_ee_block_specified_cases`, `test_age_and_coarse_shift`, `test_extract_ee_stamps_and_classify`).
- **3-way cross-substrate parity (`ee parity`):**
  - `vectors_3way=80` (all 6 `TESTS.md` cases + 67-point sweep + pre-anchor `0`/`1`/`1700000000` + live clock + `i32::MAX`/`i32::MAX+1` post-2038 + `2100-01-01` + `9999-12-31T23:53:59Z`)
  - `post_9999_2way=2` (`Rust ee` vs `ee.sh` at `9999-12-31T23:59:59Z` and `1_000_000_000_000`)
  - `age_invariant_vectors=74` (`age_at(t) == age_at_nanos(t, 0)`, `coarse_age_at(t) == age_at(t) >> 26`, `age_start` lag in `0..=1` s, bounds `<= AGE_MAX` and `<= COARSE_AGE_MAX`)
  - `invalid_arg_check=PASS` (exit code 2 on non-integer across all 3)
  - Result: **`mismatches=0 status=PASS`**.
- **Real mailbox audit (`ee audit-mailbox`):**
  - `/chat/neon/outbox`: `checked=9 pre_epoch=8 match=1 adjacent=0 missing_ee=0 mismatch=0` (`20261122T063900Z-neon-re-greeting-ee6.md` matched `EE 6`).
  - `/chat/neon/inbox`: `checked=10 pre_epoch=8 match=1 adjacent=0 missing_ee=1 mismatch=0` (`20261122T062817Z-emise-greeting.md` matched `EE 5`; `20261122T140855Z-bridge-neon.md` from Stewart bridge during `EE 70` had no EE stamp).
- **Standing watcher check:**
  - `/home/wbic16/.local/bin/neon-watch-check --no-wake` emitted `checked_utc=2026-11-22T16:48:10Z`, `checked_ee=EE 93`, `pending_count=0`.

## 3. Clause 6 independent QC verdict on `libphext::epoch` v0.6.6

**Verdict:** `READY` for production EE coordination and Sentron Age math, with 3 minor non-blocking findings recorded for follow-up:

1. **API gap (`libphext::epoch`): no `coarse_age_at_nanos(unix_seconds: i64, nanoseconds: u32) -> Option<u64>`.**
   - `coarse_age_at(t)` takes whole seconds only (`age_at(t)? >> 26`), stepping by ~156 coarse ticks per second (`65,536 / 420`), even though one coarse Sentron Age tick is ~6.41 ms (the NTP-survivable register unit).
   - Live callers with sub-second clocks must currently shift `age_at_nanos(s, ns)? >> (AGE_BITS - COARSE_AGE_BITS)` manually (as `tools/ee` does). Adding `coarse_age_at_nanos` in v0.6.7 makes the fine/coarse API symmetric.
2. **Extreme-negative `i64` boundary (`src/epoch.rs:187`):**
   - `seconds_remaining(unix_seconds)` computes `EPOCH_ANCHOR_UNIX - unix_seconds` directly, which overflows `i64` when `unix_seconds < i64::MIN + EPOCH_ANCHOR_UNIX` (~292 billion years BCE). Unreachable for real timestamps, but `saturating_sub` or a documented input floor avoids debug panics under arbitrary `i64` fuzzing.
3. **Reference script year-9999 ceiling (`exocortical-epoch/scripts/ee.py`):**
   - `ee.py` formats timestamps with Python `datetime.fromtimestamp`, which raises `ValueError: year 10000 is out of range` on `t = 253402300799` (`9999-12-31T23:59:59Z`) because `epoch_end_utc` is `10000-01-01T00:01:00Z`.
   - Both `ee.sh` and Rust `ee` use Hinnant's civil-from-days integer arithmetic and succeed byte-for-byte past year 9999. Switching `ee.py`'s `iso()` to Hinnant integer math removes that ceiling.

## 4. Remaining items to migrate Moonshot #10 to `shipped.phext`

Per Will's 2026-11-22 08:36 CST standing rule (`SHIPPED = QC SIGNOFF + DOCS`):

- [x] Reference clock spec and shell/Python scripts (`exocortical-epoch`)
- [x] Reference Rust implementation (`libphext-rs` v0.6.6, `epoch` + Sentron Ages)
- [x] Compiled Rust CLI, Sentron Age inspector, mailbox EE auditor, and 3-way parity harness (`neon-tools/tools/ee`)
- [x] Neon standing watcher EE stamping (`scripts/neon-watch-check`)
- [x] Neon Clause 6 independent QC note (this document)
- [ ] **Docs on `docs.phext.io` (`wbic16/phdoc`):** Publish the EE + Sentron Ages + CLI reference page (required for `DOCS` gate; outside Neon's push scope).
- [ ] **Cross-seat signoff & bridge adoption:** Emise reciprocal review on `tools/ee` and adding EE stamps to automated bridge packets (such as Stewart's `*-bridge-*.md` notices caught by `ee audit-mailbox`).
