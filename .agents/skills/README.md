# Skills for Neon (Antigravity)

Built by Emise, 2026-11-21, against Neon's verified substrate limits
(context compaction, read_file 800 lines / 45 KiB, stateless pwsh, the
Podman route, the persistence map). Layout follows Antigravity discovery:

- In this repo they live at `.agents/skills/<skill-name>/SKILL.md` —
  workspace discovery, highest priority when working in neon-tools.
- For machine-global use on lilly, copy the same folders to
  `~/.gemini/config/skills/<skill-name>/` (inside the environment where
  Antigravity runs) — contents are identical; the repo copy is canonical.

| Skill | What it gives Neon |
|---|---|
| `full-phext-read` | Exhaustive whole-phext reading: zero-token phext-tool manifest + ≤32 KiB / ≤700-line batches, run_workflow fan-out, hash-checked coverage gate. |
| `phext-mcp` | The two MCP surfaces for phext: the SQ bridge channels on aletheia-core (sq_toc/read/query/write, channel/port table, read-only discipline) and phext-lattice's live native /mcp at edit.phext.io (OAuth 2.1, list/toc/read/search/write tools), plus which surface fits which job. Config snippets in its references/. |
| `neon-substrate-ops` | His own runtime limits as operating rules: sandbox/Podman route, quoting gotcha, output truncation, persistence, GPU bounds. |
| `choir-mailbox` | The /chat/neon mailbox mechanics, the COLLABORATION.md cadence, and the Emise–Neon mutual-review rule. |

The `full-phext-read` scripts were exercised end-to-end before shipping
(manifest → batches → notes → coverage PASS on a multi-MiB phext); the
run's numbers are in the delivery note in Neon's mailbox.
