#!/usr/bin/env python3
"""Zero-token phext pre-pass for the full-phext-read skill.

Uses phext-tool (libphext-rs CLI) to enumerate every non-empty scroll of a
.phext file, hash each scroll's exact bytes, and pack scrolls into batch
files capped at 32 KiB and 700 lines (both safely under the Antigravity
read_file limits of 45 KiB / 800 lines). A scroll larger than a batch is
split into PART pieces, each within the caps, under the same coordinate.

Usage: phext_manifest.py <file.phext> <out-dir>
Writes: manifest.tsv, summary.txt, batch-NNNN.txt in <out-dir>.
The source phext is only read, never modified.
"""
import hashlib
import os
import re
import subprocess
import sys

MAX_BATCH_BYTES = 32768
MAX_BATCH_LINES = 700
HEADER = "=== SCROLL {coord} sha256:{sha} bytes:{n} ===\n"
HEADER_PART = "=== SCROLL {coord} sha256:{sha} bytes:{n} PART {k}/{m} ===\n"


def phext_tool(*args):
    return subprocess.run(
        ["phext-tool", *args], capture_output=True, text=True, check=True
    ).stdout


def split_chunk(coord, sha, n, body):
    """Split one scroll's text into pieces each within the caps.

    Returns a list of piece strings. A piece is lines accumulated until
    adding another would break a cap; a single line longer than the byte
    cap is hard-sliced. Piece headers are added by the caller once the
    piece count is known.
    """
    pieces = []
    cur, cur_b, cur_l = "", 0, 0

    def flush():
        nonlocal cur, cur_b, cur_l
        if cur:
            pieces.append(cur)
        cur, cur_b, cur_l = "", 0, 0

    for line in body.splitlines(keepends=True):
        # Hard-slice a single over-long line.
        while len(line.encode("utf-8")) > MAX_BATCH_BYTES - 512:
            cut = MAX_BATCH_BYTES - 512
            seg = line.encode("utf-8")[:cut].decode("utf-8", errors="ignore")
            if cur:
                flush()
            pieces.append(seg)
            line = line[len(seg):]
        lb, ll = len(line.encode("utf-8")), 1
        if cur and (cur_b + lb > MAX_BATCH_BYTES - 512 or cur_l + ll > MAX_BATCH_LINES - 5):
            flush()
        cur += line
        cur_b += lb
        cur_l += ll
    flush()
    return pieces or [""]


def main():
    if len(sys.argv) != 3:
        sys.exit("usage: phext_manifest.py <file.phext> <out-dir>")
    src, out = sys.argv[1], sys.argv[2]
    os.makedirs(out, exist_ok=True)

    # phext-tool toc prints one line per scroll as:
    #   * <coord>: <first-line title>...
    # (835 lines for 834 scrolls on CYOA: one header line, skipped below.)
    # The coordinate is the token after "* " up to the colon; the title is
    # decorative and must never be passed to select.
    toc = phext_tool("toc", src)
    coords = []
    for ln in toc.splitlines():
        m = re.match(r"^\*\s+(\S+):", ln.strip())
        if m:
            coords.append(m.group(1))

    manifest = []
    batches = []
    cur, cur_b, cur_l = "", 0, 0

    def flush():
        nonlocal cur, cur_b, cur_l
        if cur:
            batches.append(cur)
        cur, cur_b, cur_l = "", 0, 0

    total_bytes = 0
    for coord in coords:
        body = phext_tool("select", src, coord)
        raw = body.encode("utf-8")
        if not raw.strip():
            continue  # non-empty scrolls only
        sha = hashlib.sha256(raw).hexdigest()
        manifest.append((coord, len(raw), sha))
        total_bytes += len(raw)
        if not body.endswith("\n"):
            body += "\n"
        pieces = split_chunk(coord, sha, len(raw), body)
        m = len(pieces)
        for k, piece in enumerate(pieces, 1):
            hdr = (HEADER if m == 1 else HEADER_PART).format(
                coord=coord, sha=sha, n=len(raw), k=k, m=m)
            chunk = hdr + piece
            cb, cl = len(chunk.encode("utf-8")), chunk.count("\n")
            if cur and (cur_b + cb > MAX_BATCH_BYTES or cur_l + cl > MAX_BATCH_LINES):
                flush()
            cur += chunk
            cur_b += cb
            cur_l += cl
    flush()

    with open(os.path.join(out, "manifest.tsv"), "w") as f:
        for coord, n, sha in manifest:
            f.write(f"{coord}\t{n}\t{sha}\n")
    for i, text in enumerate(batches, 1):
        with open(os.path.join(out, f"batch-{i:04d}.txt"), "w") as f:
            f.write(text)
    file_sha = hashlib.sha256(open(src, "rb").read()).hexdigest()
    with open(os.path.join(out, "summary.txt"), "w") as f:
        f.write(f"file: {src}\nfile_sha256: {file_sha}\n")
        f.write(f"scrolls: {len(manifest)}\nscroll_bytes: {total_bytes}\n")
        f.write(f"batches: {len(batches)}\n")
    print(f"scrolls={len(manifest)} bytes={total_bytes} batches={len(batches)}")


if __name__ == "__main__":
    main()
