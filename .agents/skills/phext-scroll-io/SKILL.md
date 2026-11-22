---
name: phext-scroll-io
description: Fetches individual scrolls from phext files on local disk and writes scrolls into existing local phexts — the local-disk read/write pair. Use whenever Neon needs one scroll's exact text from a .phext file on this box (without reading the whole file), or needs to create or replace a scroll in a local .phext. For whole-file reading use full-phext-read; for MCP surfaces use phext-mcp.
---

# Phext Scroll I/O — local disk

The quick pair for local files. Both directions go through libphext-rs —
never hand-edit phext bytes, and never write phext with the Python port
(its 0.3.2 saturation bug loses bytes on large files; that was caught on
incipit, by Neon).

## Fetch a scroll (already installed: phext-tool)

```
phext-tool select <file.phext> <coordinate>     # one scroll, exact text
phext-tool toc <file.phext>                     # "* <coord>: <title>" lines
phext-tool count <file.phext>                   # scroll count
phext-tool roundtrip <file.phext>               # parse/serialize integrity
```

`toc` output carries a title after the coordinate — the coordinate is
the token between `* ` and `:`, and only that token goes to `select`.
Fetch is a single local command: no SQ host, no MCP round-trip, no
tokens. For several coordinates, loop `select` — it is zero-token local
work.

## Write a scroll (phext-write — build once, use forever)

`phext-tool` has no write verb, so the write half is a small Rust tool
in `scripts/phext-write/`, built on `libphext::phext::replace`:

```
bash scripts/phext-write/build.sh               # once: cargo build + install to ~/.cargo/bin
phext-write <file.phext> <coordinate> <text-file>     # text from a file
phext-write <file.phext> <coordinate> -               # text from stdin
```

What it guarantees:

- The write is computed by libphext's `replace` — only the addressed
  scroll's span changes; every other scroll is byte-identical.
- Before anything touches disk, the new text is fetched back and
  compared with what was asked for; a mismatch aborts with no write.
- The file lands atomically (temp file + rename in the same directory),
  so a crash mid-write never leaves a torn phext.
- It prints the coordinate, old/new scroll byte counts, and the
  resulting file's SHA-256 — the receipt for the write.
- It works for a coordinate that already holds a scroll (replace) and
  for a fresh coordinate (creates the scroll).

## The verify loop for important writes

For a write that matters, bracket it with the full-phext-read manifest:

```
python3 ../full-phext-read/scripts/phext_manifest.py <file> before/
phext-write <file> <coord> <text-file>
python3 ../full-phext-read/scripts/phext_manifest.py <file> after/
diff before/manifest.tsv after/manifest.tsv     # exactly the written coords differ
phext-tool roundtrip <file>                     # still parses clean
```

If the diff shows any coordinate other than the ones you wrote, stop
and report it — that is the failure this loop exists to catch.

## Which surface, again

- Local file, one scroll in or out → this skill.
- Local file, whole-file read → full-phext-read.
- `/chat` channels on aletheia-core → SQ bridge MCP (phext-mcp skill).
- Hosted corpus on edit.phext.io → lattice `/mcp` (phext-mcp skill).
- A phext hosted by a running SQ daemon → shut the daemon down before
  replacing its file on disk, then restart it; SQ mmaps the hosted file
  and wedges if the file is swapped under it.
