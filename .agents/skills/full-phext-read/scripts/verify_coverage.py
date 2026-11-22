#!/usr/bin/env python3
"""Coverage gate for the full-phext-read skill.

Usage: verify_coverage.py <manifest-dir> <notes-dir>

<manifest-dir> holds manifest.tsv + summary.txt from phext_manifest.py.
<notes-dir> holds batch-NNNN.notes.md files; each scroll read is recorded
with a line of the form:  coord: <coordinate>

Prints PASS only when every manifest coordinate appears in the notes and
the counts match exactly; otherwise lists the missing coordinates and
exits 1.
"""
import os
import re
import sys


def main():
    if len(sys.argv) != 3:
        sys.exit("usage: verify_coverage.py <manifest-dir> <notes-dir>")
    mdir, ndir = sys.argv[1], sys.argv[2]

    manifest = {}
    with open(os.path.join(mdir, "manifest.tsv")) as f:
        for ln in f:
            parts = ln.rstrip("\n").split("\t")
            if len(parts) == 3:
                manifest[parts[0]] = int(parts[1])

    seen = set()
    if os.path.isdir(ndir):
        for name in sorted(os.listdir(ndir)):
            if name.endswith(".notes.md"):
                text = open(os.path.join(ndir, name)).read()
                seen.update(re.findall(r"^coord:\s*(\S+)", text, re.M))

    missing = sorted(set(manifest) - seen)
    extra = sorted(seen - set(manifest))
    read_bytes = sum(manifest[c] for c in seen & set(manifest))
    total_bytes = sum(manifest.values())
    print(f"COVERAGE: {len(seen & set(manifest))}/{len(manifest)} scrolls, "
          f"{read_bytes}/{total_bytes} bytes")
    if missing:
        print("MISSING:")
        for c in missing:
            print(f"  {c}")
    if extra:
        print("NOT IN MANIFEST (fabricated or stale):")
        for c in extra:
            print(f"  {c}")
    if missing or extra:
        sys.exit(1)
    print("COVERAGE: PASS")


if __name__ == "__main__":
    main()
