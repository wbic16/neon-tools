---
name: neon-substrate-ops
description: Operates Neon's own Antigravity substrate correctly — the sandbox boundary, the Podman container route, stateless pwsh, output truncation, and the persistence map. Use whenever Neon runs commands, moves files, uses git/SSH/GPU, or plans multi-step work on lilly/neon, so execution follows the verified limits of this substrate instead of discovering them by failure.
---

# Neon Substrate Ops

These are Neon's verified runtime limits on lilly/neon (self-reported and
confirmed in operation, 2026-11-21). Design every plan inside them.

## Execution routes

- **Windows host (pwsh):** direct network, SSH, and git are **blocked in
  Standard Sandbox Mode**. Do not retry host-side network commands hoping
  for a different result.
- **The working route is the neon Podman container:**
  `wsl -d Ubuntu -- podman exec -u wbic16 neon ...` — inside it, Linux,
  Rust, git, SSH, and GPU all run with zero approval prompts. Default to
  the container for anything beyond reading host files.
- `run_command` starts a **fresh pwsh session per call**: CWD and
  environment variables do not persist. Every command must be
  self-contained — absolute paths, env set inline, or a script file.
- **Quoting gotcha:** avoid `$VAR` and backticks in `wsl` command lines
  (pwsh eats them). Pass scripts and data as **files** in `C:\agentic\`
  (mounted at `/agentic` inside neon) and invoke the file.

## Output and file limits

- Synchronous wait is 500–10,000 ms; longer commands background
  automatically and notify on completion. Plan long jobs as background
  jobs with a log file, never as one blocking call.
- Large stdout is **middle-truncated** in context (~10–12 KiB); the full
  output lands in a log file. For anything verbose, redirect to a log
  inside the container and read it back in slices.
- `read_file` caps at **800 lines or 45 KiB per call** — paginate with
  StartLine/EndLine/ByteOffset, and prefer tools that emit small, exact
  answers (counts, hashes, single scrolls) over dumps.

## Context discipline

- The context window is finite and the harness **compacts** earlier turns
  into a summary when it fills. Anything that must survive compaction —
  decisions, coordinates, hashes, task state — gets written to a file in
  `/workspace/` or `/agentic` *when it happens*, not at the end.
- Fan out wide reads with `invoke_subagent` / `run_workflow` instead of
  pulling whole corpora into one context (see the full-phext-read skill).
- MCP tools (`sq_toc`, `sq_read`, `sq_query`, `sq_write` via sq-emise /
  sq-argon) are lazy-loaded and give coordinate-level phext access at a
  fraction of the cost of file reads. Prefer them for targeted lookups.

## Persistence map

- **Persistent:** `/home/wbic16` (neon-home volume), `/workspace`
  (neon-workspace volume), `/agentic` (host `C:\agentic` bind mount).
  Skills, clones, scripts, and state live in `/workspace/` or `/agentic`.
- **Not persistent:** `/source/` sits on the container overlay FS. It
  holds read-mostly reference clones (human, libphext-rs, exocortical);
  never store new work there.
- Zero-token local tooling: Rust 1.100.0, libphext-rs v0.6.4,
  `phext-tool` (`~/.cargo/bin`), `sq` 0.6.1. Prefer local tools over LLM
  tokens for anything mechanical — hashing, counting, extracting.

## GPU

RTX 4080 Laptop, 12,282 MiB VRAM (driver 546.92) — and lilly is Will's
laptop. Keep GPU jobs bounded: one job at a time, sized to fit 12 GiB,
and never leave a GPU process running unattended across a work session
boundary without noting it in the task record.
