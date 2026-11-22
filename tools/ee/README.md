# `ee` — Exocortical Epoch and Sentron Age CLI

A zero-extra-dependency Rust binary built directly on `libphext::epoch` (v0.6.6+).

## What it does

- **Shared clock (`ee [unix-timestamp]`):** Computes the Exocortical Epoch from UTC Unix seconds (`anchor = 1795327200`, `length = 420 s`). Output is byte-for-byte identical to the reference `ee.sh` and `ee.py` scripts.
- **Sentron Ages (`ee age [unix] [--nanos <ns>]`):** Exposes both the 42-bit fine Sentron Age (`2^42` ticks/epoch, ~95.5 ps/tick, local clock) and the 16-bit register-grade coarse Sentron Age (`2^16 = 65,536` ticks/epoch, ~6.41 ms/tick, NTP-survivable).
- **Epoch bounds (`ee range <epoch>`):** Prints Unix and UTC `[start, end)` bounds for any epoch (`EE 0` returns `range: none`).
- **Choir completion stamps (`ee stamp [--short] [unix]`):** Prints a compact one-line completion stamp (`EE <n>` or `EE <n> (coarse_age <c>, utc <ISO>)`).
- **Mailbox EE audit (`ee audit-mailbox [--strict] <dir>`):** Checks Choir `.md` files named `YYYYMMDDTHHMMSSZ-*.md` against explicit `EE <n>` stamps in each message body to verify compliance with the EE 1 standing rule.
- **3-way parity harness (`ee parity <ee.sh> <ee.py>`):** Verifies byte-for-byte output agreement across Rust (`libphext::epoch`), POSIX `sh`, and Python across boundary, sweep, live, and post-2038 timestamps, plus Sentron Age invariants.

## Build and install

Requires `/source/libphext-rs` at v0.6.6 or newer (override the path via `.cargo/config.toml` if your checkout lives elsewhere):

```bash
cd tools/ee
./build.sh
```

This runs `cargo test --offline`, compiles `--release`, and installs `~/.cargo/bin/ee` (`0755`).

## Examples

```bash
ee                                   # current EE (5-line contract)
ee 1795327200                        # EE 1 anchor
ee age                               # live fine + coarse Sentron Age
ee range 92                          # UTC and Unix bounds for EE 92
ee stamp --short                     # EE <n>
ee audit-mailbox /chat/neon/outbox   # verify outbox EE stamps
ee parity /path/to/ee.sh /path/to/ee.py
```
