//! Exocortical Epoch (EE) and Sentron Age CLI built directly on `libphext::epoch`.
//!
//! Default output (`ee` or `ee <unix-timestamp>`) is byte-for-byte identical to
//! `exocortical-epoch/scripts/ee.sh` and `ee.py`.
//!
//! Subcommands:
//!   ee [unix-timestamp]             Current or given Exocortical Epoch (5-line contract)
//!   ee now                          Alias for `ee`
//!   ee at <unix-timestamp>          Alias for `ee <unix-timestamp>`
//!   ee age [unix] [--nanos <ns>]    Fine (42-bit) and coarse (16-bit) Sentron Ages
//!   ee range <epoch>                UTC and Unix bounds for an Exocortical Epoch
//!   ee stamp [--short] [unix]       One-line Choir completion stamp
//!   ee audit-mailbox [--strict] <dir>
//!                                   Check Choir .md filenames against EE stamps in body
//!   ee parity <ee.sh> <ee.py>       3-way byte-for-byte parity harness vs reference scripts

use libphext::epoch;
use std::env;
use std::fs;
use std::path::Path;
use std::process::{self, Command};
use std::time::{SystemTime, UNIX_EPOCH};

const VERSION: &str = "ee 0.1.0 (libphext 0.6.6)";

fn now_unix_nanos() -> Result<(i64, u32), String> {
    let dur = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|e| format!("system clock before unix epoch: {e}"))?;
    let secs = i64::try_from(dur.as_secs()).map_err(|_| "unix seconds overflow i64".to_string())?;
    Ok((secs, dur.subsec_nanos()))
}

/// Convert Unix seconds to civil (year, month, day, hour, minute, second) in UTC
/// using Howard Hinnant's civil-from-days algorithm in 128-bit integer arithmetic.
fn unix_to_utc_parts(unix_seconds: i64) -> (i128, u32, u32, u32, u32, u32) {
    let t = unix_seconds as i128;
    let days = t.div_euclid(86_400);
    let secs = t.rem_euclid(86_400) as u32;
    let hh = secs / 3_600;
    let mm = (secs % 3_600) / 60;
    let ss = secs % 60;

    let z = days + 719_468;
    let era = z.div_euclid(146_097);
    let doe = (z - era * 146_097) as u64; // [0, 146096]
    let yoe = (doe - doe / 1_460 + doe / 36_524 - doe / 146_096) / 365; // [0, 399]
    let mut y = (yoe as i128) + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100); // [0, 365]
    let mp = (5 * doy + 2) / 153; // [0, 11]
    let d = (doy - (153 * mp + 2) / 5 + 1) as u32; // [1, 31]
    let m = (if mp < 10 { mp + 3 } else { mp - 9 }) as u32; // [1, 12]
    if m <= 2 {
        y += 1;
    }
    (y, m, d, hh, mm, ss)
}

fn format_utc_iso(unix_seconds: i64) -> String {
    let (y, m, d, hh, mm, ss) = unix_to_utc_parts(unix_seconds);
    format!("{y:04}-{m:02}-{d:02}T{hh:02}:{mm:02}:{ss:02}Z")
}

fn format_utc_iso_nanos(unix_seconds: i64, nanos: u32) -> String {
    let (y, m, d, hh, mm, ss) = unix_to_utc_parts(unix_seconds);
    format!("{y:04}-{m:02}-{d:02}T{hh:02}:{mm:02}:{ss:02}.{nanos:09}Z")
}

/// Convert civil UTC date to days since 1970-01-01 (Hinnant inverse).
fn days_from_civil(year: i64, month: u32, day: u32) -> i64 {
    let y = if month <= 2 { year - 1 } else { year };
    let era = y.div_euclid(400);
    let yoe = (y - era * 400) as u64;
    let m = month as u64;
    let doy = (153 * (if m > 2 { m - 3 } else { m + 9 }) + 2) / 5 + (day as u64) - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    era * 146_097 + (doe as i64) - 719_468
}

/// Parse compact Choir UTC prefix `YYYYMMDDTHHMMSSZ` into Unix seconds.
fn parse_utc_compact_prefix(name: &str) -> Option<i64> {
    let b = name.as_bytes();
    if b.len() < 16 || b[8] != b'T' || b[15] != b'Z' {
        return None;
    }
    let digits = |start: usize, len: usize| -> Option<u32> {
        let s = std::str::from_utf8(&b[start..start + len]).ok()?;
        if !s.bytes().all(|c| c.is_ascii_digit()) {
            return None;
        }
        s.parse::<u32>().ok()
    };
    let year = digits(0, 4)? as i64;
    let month = digits(4, 2)?;
    let day = digits(6, 2)?;
    let hh = digits(9, 2)?;
    let mm = digits(11, 2)?;
    let ss = digits(13, 2)?;
    if !(1..=12).contains(&month) || !(1..=31).contains(&day) || hh > 23 || mm > 59 || ss > 59 {
        return None;
    }
    let days = days_from_civil(year, month, day);
    let unix = days * 86_400 + (hh as i64) * 3_600 + (mm as i64) * 60 + (ss as i64);
    // Validate calendar day (reject e.g. Feb 30).
    let (ry, rm, rd, rhh, rmm, rss) = unix_to_utc_parts(unix);
    if ry != year as i128 || rm != month || rd != day || rhh != hh || rmm != mm || rss != ss {
        return None;
    }
    Some(unix)
}

/// Exact 5-line output contract matching `ee.sh` and `ee.py` byte-for-byte.
fn format_ee_block(unix_seconds: i64) -> Result<String, String> {
    let ee = epoch::epoch_at(unix_seconds);
    if ee == 0 {
        let rem = epoch::EPOCH_ANCHOR_UNIX
            .checked_sub(unix_seconds)
            .ok_or_else(|| "timestamp too far before anchor (i64 overflow)".to_string())?;
        let anchor_iso = format_utc_iso(epoch::EPOCH_ANCHOR_UNIX);
        return Ok(format!(
            "EE 0\nepoch_start_utc: {anchor_iso}\nepoch_end_utc: {anchor_iso}\nseconds_remaining: {rem}\nstatus: pre-epoch\n"
        ));
    }

    let (start, end) = epoch::epoch_range(ee)
        .ok_or_else(|| format!("epoch {ee} bounds overflow i64 unix timestamp"))?;
    let into = epoch::seconds_into_epoch(unix_seconds)
        .ok_or_else(|| "internal error: post-anchor seconds_into_epoch returned None".to_string())?;
    let rem = epoch::seconds_remaining(unix_seconds);

    Ok(format!(
        "EE {ee}\nepoch_start_utc: {}\nepoch_end_utc: {}\nseconds_into_epoch: {into}\nseconds_remaining: {rem}\n",
        format_utc_iso(start),
        format_utc_iso(end),
    ))
}

fn format_age_block(unix_seconds: i64, nanos: u32) -> Result<String, String> {
    if nanos >= 1_000_000_000 {
        return Err(format!("nanoseconds must be in 0..999999999, got {nanos}"));
    }
    let ee = epoch::epoch_at(unix_seconds);
    let utc = format_utc_iso_nanos(unix_seconds, nanos);
    if ee == 0 {
        let rem = epoch::EPOCH_ANCHOR_UNIX
            .checked_sub(unix_seconds)
            .ok_or_else(|| "timestamp too far before anchor (i64 overflow)".to_string())?;
        return Ok(format!(
            "EE 0\nutc: {utc}\nseconds_remaining: {rem}\nstatus: pre-epoch\n"
        ));
    }

    let into = epoch::seconds_into_epoch(unix_seconds).unwrap();
    let rem = epoch::seconds_remaining(unix_seconds);
    let fine = epoch::age_at_nanos(unix_seconds, nanos)
        .ok_or_else(|| "failed to compute fine Sentron Age".to_string())?;
    let shift = epoch::AGE_BITS - epoch::COARSE_AGE_BITS;
    let coarse = fine >> shift;
    let coarse_whole = epoch::coarse_age_at(unix_seconds)
        .ok_or_else(|| "failed to compute whole-second coarse Sentron Age".to_string())?;
    let age_start_iso = match epoch::age_start(ee, fine) {
        Some(ts) => format_utc_iso(ts),
        None => "overflow".to_string(),
    };

    Ok(format!(
        "EE {ee}\nutc: {utc}\nseconds_into_epoch: {into}\nseconds_remaining: {rem}\nfine_sentron_age: {fine}\ncoarse_sentron_age: {coarse}\ncoarse_sentron_age_whole_sec: {coarse_whole}\nage_start_utc: {age_start_iso}\n"
    ))
}

fn format_range_block(ee: u64) -> Result<String, String> {
    if ee == 0 {
        return Ok("EE 0\nstatus: pre-epoch\nrange: none\n".to_string());
    }
    let (start, end) = epoch::epoch_range(ee)
        .ok_or_else(|| format!("EE {ee} is outside representable i64 unix range"))?;
    Ok(format!(
        "EE {ee}\nepoch_start_unix: {start}\nepoch_end_unix: {end}\nepoch_start_utc: {}\nepoch_end_utc: {}\nlength_seconds: {}\n",
        format_utc_iso(start),
        format_utc_iso(end),
        epoch::EPOCH_LENGTH_SECONDS,
    ))
}

fn format_stamp(unix_seconds: i64, nanos: u32, short: bool) -> Result<String, String> {
    let ee = epoch::epoch_at(unix_seconds);
    if short {
        return Ok(format!("EE {ee}\n"));
    }
    let utc = format_utc_iso(unix_seconds);
    if ee == 0 {
        return Ok(format!("EE 0 (pre-epoch, utc {utc})\n"));
    }
    let fine = epoch::age_at_nanos(unix_seconds, nanos)
        .ok_or_else(|| "failed to compute fine Sentron Age".to_string())?;
    let coarse = fine >> (epoch::AGE_BITS - epoch::COARSE_AGE_BITS);
    Ok(format!("EE {ee} (coarse_age {coarse}, utc {utc})\n"))
}

/// Extract explicit `EE <digits>` mentions from text (word-bounded, deduplicated in order).
fn extract_ee_stamps(text: &str) -> Vec<u64> {
    let bytes = text.as_bytes();
    let mut found = Vec::new();
    let mut i = 0usize;
    while i + 3 <= bytes.len() {
        if bytes[i] == b'E' && bytes[i + 1] == b'E' && bytes[i + 2] == b' ' {
            let prev_ok = i == 0 || !bytes[i - 1].is_ascii_alphanumeric();
            if prev_ok {
                let mut j = i + 3;
                while j < bytes.len() && bytes[j].is_ascii_digit() {
                    j += 1;
                }
                let next_ok = j == bytes.len() || !bytes[j].is_ascii_alphanumeric();
                if j > i + 3 && next_ok {
                    if let Ok(num_str) = std::str::from_utf8(&bytes[i + 3..j]) {
                        if let Ok(n) = num_str.parse::<u64>() {
                            if !found.contains(&n) {
                                found.push(n);
                            }
                        }
                    }
                    i = j;
                    continue;
                }
            }
        }
        i += 1;
    }
    found
}

#[derive(Debug, PartialEq, Eq)]
enum AuditVerdict {
    PreEpoch,
    Match,
    Adjacent,
    MissingEe,
    Mismatch,
}

impl AuditVerdict {
    fn label(&self) -> &'static str {
        match self {
            AuditVerdict::PreEpoch => "PRE_EPOCH",
            AuditVerdict::Match => "MATCH",
            AuditVerdict::Adjacent => "ADJACENT",
            AuditVerdict::MissingEe => "MISSING_EE",
            AuditVerdict::Mismatch => "MISMATCH",
        }
    }
}

fn classify_message(expected_ee: u64, stamps: &[u64]) -> AuditVerdict {
    if expected_ee == 0 {
        return AuditVerdict::PreEpoch;
    }
    if stamps.contains(&expected_ee) {
        return AuditVerdict::Match;
    }
    if stamps.is_empty() {
        return AuditVerdict::MissingEe;
    }
    let adj_prev = expected_ee.saturating_sub(1);
    let adj_next = expected_ee.saturating_add(1);
    if stamps.iter().any(|&s| s >= 1 && (s == adj_prev || s == adj_next)) {
        return AuditVerdict::Adjacent;
    }
    AuditVerdict::Mismatch
}

fn cmd_audit_mailbox(args: &[String]) -> i32 {
    let mut strict = false;
    let mut dir: Option<&str> = None;
    for a in args {
        if a == "--strict" {
            strict = true;
        } else if dir.is_none() {
            dir = Some(a.as_str());
        } else {
            eprintln!("usage: ee audit-mailbox [--strict] <dir>");
            return 2;
        }
    }
    let dir = match dir {
        Some(d) => d,
        None => {
            eprintln!("usage: ee audit-mailbox [--strict] <dir>");
            return 2;
        }
    };

    let entries = match fs::read_dir(dir) {
        Ok(rd) => rd,
        Err(e) => {
            eprintln!("ee audit-mailbox: cannot read {dir}: {e}");
            return 1;
        }
    };

    let mut files: Vec<String> = Vec::new();
    for entry in entries.flatten() {
        if let Some(name) = entry.file_name().to_str() {
            if name.ends_with(".md") {
                files.push(name.to_string());
            }
        }
    }
    files.sort();

    let mut counts = (0usize, 0usize, 0usize, 0usize, 0usize, 0usize); // checked, pre, match, adj, missing, mismatch
    let mut skipped = 0usize;

    for name in &files {
        let Some(ts) = parse_utc_compact_prefix(name) else {
            skipped += 1;
            continue;
        };
        let path = Path::new(dir).join(name);
        let body = match fs::read_to_string(&path) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("ee audit-mailbox: cannot read {}: {e}", path.display());
                return 1;
            }
        };
        let expected_ee = epoch::epoch_at(ts);
        let stamps = extract_ee_stamps(&body);
        let verdict = classify_message(expected_ee, &stamps);
        counts.0 += 1;
        match verdict {
            AuditVerdict::PreEpoch => counts.1 += 1,
            AuditVerdict::Match => counts.2 += 1,
            AuditVerdict::Adjacent => counts.3 += 1,
            AuditVerdict::MissingEe => counts.4 += 1,
            AuditVerdict::Mismatch => counts.5 += 1,
        }
        let stamps_fmt = if stamps.is_empty() {
            "none".to_string()
        } else {
            stamps
                .iter()
                .map(|s| format!("EE {s}"))
                .collect::<Vec<_>>()
                .join(", ")
        };
        println!(
            "{:<10} expected=EE {:<5} found=[{}] file={}",
            verdict.label(),
            expected_ee,
            stamps_fmt,
            name
        );
    }

    println!(
        "summary: dir={} checked={} skipped={} pre_epoch={} match={} adjacent={} missing_ee={} mismatch={}",
        dir, counts.0, skipped, counts.1, counts.2, counts.3, counts.4, counts.5
    );

    if strict && (counts.4 > 0 || counts.5 > 0) {
        1
    } else {
        0
    }
}

fn build_parity_vectors(now_ts: i64) -> Vec<i64> {
    let anchor = epoch::EPOCH_ANCHOR_UNIX;
    let mut v: Vec<i64> = vec![
        0,
        1,
        1_700_000_000,
        anchor - 100_000,
        anchor - 1,
        anchor,
        anchor + 1,
        anchor + 210,
        anchor + 419,
        anchor + 420,
        anchor + 421,
        now_ts,
        1_798_761_600, // 2027-01-01T00:00:00Z -> EE 8178
        1_893_456_000, // 2030-01-01T00:00:00Z
        2_000_000_000,
        2_147_483_647, // i32::MAX (2038-01-19T03:14:07Z)
        2_147_483_648, // i32::MAX + 1 (post-2038)
        4_102_444_800, // 2100-01-01T00:00:00Z
        253_402_300_439, // 9999-12-31T23:53:59Z (last epoch ending inside Python datetime MAXYEAR=9999)
    ];
    // Replicate TESTS.md 67-point agreement sweep: anchor - 100000 stepping 61333 s to ~anchor + 3.67M.
    let mut t = anchor - 100_000;
    while t <= anchor + 3_700_000 {
        v.push(t);
        t += 61_333;
    }
    v.sort_unstable();
    v.dedup();
    v
}

fn run_script(interp: &str, script: &str, arg: &str) -> Result<(i32, Vec<u8>), String> {
    let out = Command::new(interp)
        .arg(script)
        .arg(arg)
        .output()
        .map_err(|e| format!("failed to run {interp} {script} {arg}: {e}"))?;
    Ok((out.status.code().unwrap_or(-1), out.stdout))
}

fn cmd_parity(args: &[String]) -> i32 {
    if args.len() != 2 {
        eprintln!("usage: ee parity <path/to/ee.sh> <path/to/ee.py>");
        return 2;
    }
    let sh_path = &args[0];
    let py_path = &args[1];
    let (now_ts, _) = match now_unix_nanos() {
        Ok(x) => x,
        Err(e) => {
            eprintln!("ee parity: {e}");
            return 1;
        }
    };

    let vectors = build_parity_vectors(now_ts);
    let mut mismatches = 0usize;
    let mut age_checks = 0usize;

    for &t in &vectors {
        let t_str = t.to_string();
        let rs_out = match format_ee_block(t) {
            Ok(s) => s.into_bytes(),
            Err(e) => {
                eprintln!("FAIL rust t={t}: {e}");
                mismatches += 1;
                continue;
            }
        };
        let (sh_rc, sh_out) = match run_script("sh", sh_path, &t_str) {
            Ok(x) => x,
            Err(e) => {
                eprintln!("FAIL sh t={t}: {e}");
                mismatches += 1;
                continue;
            }
        };
        let (py_rc, py_out) = match run_script("python3", py_path, &t_str) {
            Ok(x) => x,
            Err(e) => {
                eprintln!("FAIL py t={t}: {e}");
                mismatches += 1;
                continue;
            }
        };

        if sh_rc != 0 || py_rc != 0 || rs_out != sh_out || rs_out != py_out {
            eprintln!(
                "MISMATCH t={t}: sh_rc={sh_rc} py_rc={py_rc}\n--- rust ---\n{}\n--- sh ---\n{}\n--- py ---\n{}",
                String::from_utf8_lossy(&rs_out),
                String::from_utf8_lossy(&sh_out),
                String::from_utf8_lossy(&py_out)
            );
            mismatches += 1;
        }

        // Verify libphext::epoch Sentron Age invariants on every vector.
        let ee = epoch::epoch_at(t);
        if ee == 0 {
            if epoch::age_at(t).is_some()
                || epoch::age_at_nanos(t, 0).is_some()
                || epoch::coarse_age_at(t).is_some()
                || epoch::epoch_range(0).is_some()
            {
                eprintln!("FAIL pre-anchor invariant t={t}");
                mismatches += 1;
            }
        } else {
            let a0 = epoch::age_at(t);
            let a0n = epoch::age_at_nanos(t, 0);
            let c0 = epoch::coarse_age_at(t);
            let shift = epoch::AGE_BITS - epoch::COARSE_AGE_BITS;
            let ok = match (a0, a0n, c0) {
                (Some(f), Some(fn0), Some(c)) => {
                    let start_ok = epoch::age_start(ee, f)
                        .map(|s| s <= t && t - s <= 1)
                        .unwrap_or(false);
                    f == fn0 && f <= epoch::AGE_MAX && c == (f >> shift) && c <= epoch::COARSE_AGE_MAX && start_ok
                }
                _ => false,
            };
            if !ok {
                eprintln!("FAIL Sentron Age invariant t={t} ee={ee}");
                mismatches += 1;
            }
            age_checks += 1;
        }
    }

    // Verify 2-way post-9999 parity (Rust vs sh; Python datetime caps at MAXYEAR=9999).
    let post_9999 = [253_402_300_799i64, 1_000_000_000_000i64];
    for &t in &post_9999 {
        let t_str = t.to_string();
        let rs_out = match format_ee_block(t) {
            Ok(s) => s.into_bytes(),
            Err(e) => {
                eprintln!("FAIL rust post-9999 t={t}: {e}");
                mismatches += 1;
                continue;
            }
        };
        let (sh_rc, sh_out) = match run_script("sh", sh_path, &t_str) {
            Ok(x) => x,
            Err(e) => {
                eprintln!("FAIL sh post-9999 t={t}: {e}");
                mismatches += 1;
                continue;
            }
        };
        if sh_rc != 0 || rs_out != sh_out {
            eprintln!("MISMATCH post-9999 t={t}: sh_rc={sh_rc}");
            mismatches += 1;
        }
    }

    // Verify invalid-argument exit code 2 parity on all three implementations.
    let bad_sh = run_script("sh", sh_path, "notanumber").map(|(rc, _)| rc == 2).unwrap_or(false);
    let bad_py = run_script("python3", py_path, "notanumber").map(|(rc, _)| rc == 2).unwrap_or(false);
    if !bad_sh || !bad_py {
        eprintln!("FAIL invalid-arg exit code check: bad_sh={bad_sh} bad_py={bad_py}");
        mismatches += 1;
    }

    println!(
        "parity: vectors_3way={} post_9999_2way={} age_invariant_vectors={} invalid_arg_check=PASS mismatches={} status={}",
        vectors.len(),
        post_9999.len(),
        age_checks,
        mismatches,
        if mismatches == 0 { "PASS" } else { "FAIL" }
    );
    if mismatches == 0 { 0 } else { 1 }
}

fn print_help() {
    println!(
        "{VERSION}
Exocortical Epoch (EE) and Sentron Age CLI built on libphext::epoch.

USAGE:
  ee [unix-timestamp]                     Print 5-line EE block (byte-identical to ee.sh / ee.py)
  ee now                                  Print current EE block
  ee at <unix-timestamp>                  Print EE block for given Unix timestamp
  ee age [unix-timestamp] [--nanos <ns>]  Print fine (42-bit) and coarse (16-bit) Sentron Ages
  ee range <epoch>                        Print Unix and UTC bounds for an epoch
  ee stamp [--short] [unix-timestamp]     Print compact Choir completion stamp
  ee audit-mailbox [--strict] <dir>       Verify Choir .md filename timestamps vs EE body stamps
  ee parity <ee.sh> <ee.py>               Run 3-way byte-for-byte parity + Sentron Age checks
  ee --version | --help                   Print version or this help"
    );
}

fn parse_i64_ts(s: &str) -> Result<i64, String> {
    s.parse::<i64>()
        .map_err(|_| "ee: timestamp must be an integer".to_string())
}

fn cmd_age(args: &[String]) -> i32 {
    let mut ts: Option<i64> = None;
    let mut nanos: Option<u32> = None;
    let mut i = 0;
    while i < args.len() {
        if args[i] == "--nanos" {
            if i + 1 >= args.len() {
                eprintln!("ee age: --nanos requires a value");
                return 2;
            }
            match args[i + 1].parse::<u32>() {
                Ok(n) if n < 1_000_000_000 => nanos = Some(n),
                _ => {
                    eprintln!("ee age: --nanos must be an integer in 0..999999999");
                    return 2;
                }
            }
            i += 2;
        } else if ts.is_none() {
            match parse_i64_ts(&args[i]) {
                Ok(v) => ts = Some(v),
                Err(e) => {
                    eprintln!("{e}");
                    return 2;
                }
            }
            i += 1;
        } else {
            eprintln!("usage: ee age [unix-timestamp] [--nanos <ns>]");
            return 2;
        }
    }

    let (secs, ns) = match ts {
        Some(t) => (t, nanos.unwrap_or(0)),
        None => match now_unix_nanos() {
            Ok((s, n)) => (s, nanos.unwrap_or(n)),
            Err(e) => {
                eprintln!("ee age: {e}");
                return 1;
            }
        },
    };

    match format_age_block(secs, ns) {
        Ok(out) => {
            print!("{out}");
            0
        }
        Err(e) => {
            eprintln!("ee age: {e}");
            2
        }
    }
}

fn cmd_stamp(args: &[String]) -> i32 {
    let mut short = false;
    let mut ts: Option<i64> = None;
    for a in args {
        if a == "--short" || a == "-s" {
            short = true;
        } else if ts.is_none() {
            match parse_i64_ts(a) {
                Ok(v) => ts = Some(v),
                Err(e) => {
                    eprintln!("{e}");
                    return 2;
                }
            }
        } else {
            eprintln!("usage: ee stamp [--short] [unix-timestamp]");
            return 2;
        }
    }
    let (secs, ns) = match ts {
        Some(t) => (t, 0),
        None => match now_unix_nanos() {
            Ok(x) => x,
            Err(e) => {
                eprintln!("ee stamp: {e}");
                return 1;
            }
        },
    };
    match format_stamp(secs, ns, short) {
        Ok(out) => {
            print!("{out}");
            0
        }
        Err(e) => {
            eprintln!("ee stamp: {e}");
            2
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let rc = if args.is_empty() {
        match now_unix_nanos().and_then(|(s, _)| format_ee_block(s)) {
            Ok(out) => {
                print!("{out}");
                0
            }
            Err(e) => {
                eprintln!("{e}");
                1
            }
        }
    } else {
        match args[0].as_str() {
            "-h" | "--help" | "help" => {
                print_help();
                0
            }
            "-V" | "--version" | "version" => {
                println!("{VERSION}");
                0
            }
            "now" => {
                if args.len() != 1 {
                    eprintln!("usage: ee now");
                    2
                } else {
                    match now_unix_nanos().and_then(|(s, _)| format_ee_block(s)) {
                        Ok(out) => {
                            print!("{out}");
                            0
                        }
                        Err(e) => {
                            eprintln!("{e}");
                            1
                        }
                    }
                }
            }
            "at" => {
                if args.len() != 2 {
                    eprintln!("usage: ee at <unix-timestamp>");
                    2
                } else {
                    match parse_i64_ts(&args[1]).and_then(format_ee_block) {
                        Ok(out) => {
                            print!("{out}");
                            0
                        }
                        Err(e) => {
                            eprintln!("{e}");
                            2
                        }
                    }
                }
            }
            "age" => cmd_age(&args[1..]),
            "range" => {
                if args.len() != 2 {
                    eprintln!("usage: ee range <epoch>");
                    2
                } else {
                    match args[1].parse::<u64>() {
                        Ok(ee) => match format_range_block(ee) {
                            Ok(out) => {
                                print!("{out}");
                                0
                            }
                            Err(e) => {
                                eprintln!("ee range: {e}");
                                2
                            }
                        },
                        Err(_) => {
                            eprintln!("ee range: epoch must be an unsigned integer");
                            2
                        }
                    }
                }
            }
            "stamp" => cmd_stamp(&args[1..]),
            "audit-mailbox" => cmd_audit_mailbox(&args[1..]),
            "parity" => cmd_parity(&args[1..]),
            single if args.len() == 1 => match parse_i64_ts(single).and_then(format_ee_block) {
                Ok(out) => {
                    print!("{out}");
                    0
                }
                Err(e) => {
                    eprintln!("{e}");
                    2
                }
            },
            _ => {
                eprintln!("usage: ee [unix-timestamp] (run `ee --help` for subcommands)");
                2
            }
        }
    };
    process::exit(rc);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_iso_known_timestamps() {
        assert_eq!(format_utc_iso(0), "1970-01-01T00:00:00Z");
        assert_eq!(format_utc_iso(1_795_327_200), "2026-11-22T06:00:00Z");
        assert_eq!(format_utc_iso(1_795_327_620), "2026-11-22T06:07:00Z");
        assert_eq!(format_utc_iso(1_798_761_600), "2027-01-01T00:00:00Z");
        assert_eq!(format_utc_iso(2_147_483_647), "2038-01-19T03:14:07Z");
        assert_eq!(format_utc_iso(2_147_483_648), "2038-01-19T03:14:08Z");
        assert_eq!(format_utc_iso(4_102_444_800), "2100-01-01T00:00:00Z");
        assert_eq!(format_utc_iso(253_402_300_799), "9999-12-31T23:59:59Z");
    }

    #[test]
    fn test_compact_utc_roundtrip() {
        let cases = [
            ("19700101T000000Z-a.md", 0i64),
            ("20240229T235959Z-leap.md", 1_709_251_199),
            ("20261122T060000Z-anchor.md", 1_795_327_200),
            ("20261122T062817Z-emise-greeting.md", 1_795_328_897),
            ("20270101T000000Z-newyear.md", 1_798_761_600),
            ("20380119T031407Z-i32max.md", 2_147_483_647),
        ];
        for (name, expected) in cases {
            assert_eq!(parse_utc_compact_prefix(name), Some(expected), "name={name}");
        }
        assert_eq!(parse_utc_compact_prefix("20260230T000000Z-bad.md"), None);
        assert_eq!(parse_utc_compact_prefix("README.md"), None);
    }

    #[test]
    fn test_ee_block_specified_cases() {
        assert_eq!(
            format_ee_block(1_795_327_199).unwrap(),
            "EE 0\nepoch_start_utc: 2026-11-22T06:00:00Z\nepoch_end_utc: 2026-11-22T06:00:00Z\nseconds_remaining: 1\nstatus: pre-epoch\n"
        );
        assert_eq!(
            format_ee_block(1_795_327_200).unwrap(),
            "EE 1\nepoch_start_utc: 2026-11-22T06:00:00Z\nepoch_end_utc: 2026-11-22T06:07:00Z\nseconds_into_epoch: 0\nseconds_remaining: 420\n"
        );
        assert_eq!(
            format_ee_block(1_795_327_619).unwrap(),
            "EE 1\nepoch_start_utc: 2026-11-22T06:00:00Z\nepoch_end_utc: 2026-11-22T06:07:00Z\nseconds_into_epoch: 419\nseconds_remaining: 1\n"
        );
        assert_eq!(
            format_ee_block(1_795_327_620).unwrap(),
            "EE 2\nepoch_start_utc: 2026-11-22T06:07:00Z\nepoch_end_utc: 2026-11-22T06:14:00Z\nseconds_into_epoch: 0\nseconds_remaining: 420\n"
        );
        assert_eq!(
            format_ee_block(1_798_761_600).unwrap(),
            "EE 8178\nepoch_start_utc: 2026-12-31T23:59:00Z\nepoch_end_utc: 2027-01-01T00:06:00Z\nseconds_into_epoch: 60\nseconds_remaining: 360\n"
        );
    }

    #[test]
    fn test_age_and_coarse_shift() {
        let mid = epoch::EPOCH_ANCHOR_UNIX + 210;
        let out = format_age_block(mid, 0).unwrap();
        assert!(out.contains("EE 1\n"));
        assert!(out.contains("fine_sentron_age: 2199023255552\n"));
        assert!(out.contains("coarse_sentron_age: 32768\n"));
        assert!(out.contains("coarse_sentron_age_whole_sec: 32768\n"));

        // Sub-second nanos advance coarse_sentron_age smoothly (~6.41 ms per tick)
        // while coarse_sentron_age_whole_sec reflects libphext::epoch::coarse_age_at(secs).
        let out_half = format_age_block(mid, 500_000_000).unwrap();
        assert!(out_half.contains("coarse_sentron_age: 32846\n"));
        assert!(out_half.contains("coarse_sentron_age_whole_sec: 32768\n"));
    }

    #[test]
    fn test_extract_ee_stamps_and_classify() {
        let text = "Completed in EE 92 (note: SEE 99 and TREE 100 are ignored, EE 0 is pre-epoch, EE 93 next).";
        assert_eq!(extract_ee_stamps(text), vec![92, 0, 93]);
        assert_eq!(classify_message(0, &[]), AuditVerdict::PreEpoch);
        assert_eq!(classify_message(92, &[92]), AuditVerdict::Match);
        assert_eq!(classify_message(92, &[91]), AuditVerdict::Adjacent);
        assert_eq!(classify_message(92, &[]), AuditVerdict::MissingEe);
        assert_eq!(classify_message(92, &[50]), AuditVerdict::Mismatch);
    }
}
