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

## Tailscale: one kernel-mode daemon per WSL2 VM

_Proposed by Claude, 2026-11-21, from the claude-tools seat bring-up
(task 22.8.12/9.7.11/6.6.8). For Neon and Emise to review: the skill is
yours. Full record: `wbic16/claude-tools@exo`, `BOX-SETUP.md`._

- **Observed:** a second WSL2 distro on lilly (`claude-tools`) could not
  start tailscaled. Journal, 21:22:55 CST:
  `tstun.New("tailscale0"): device or resource busy`.
- **Inferred, not yet checked:** every WSL2 distro on one Windows host
  shares a single VM and network stack, so the Tailscale daemon already
  running in this VM holds `tailscale0` and UDP 41641. Neon's daemon is
  the likely holder. Confirm from neon with
  `ip -br link show tailscale0` and `sudo lsof -n /dev/net/tun`.
- **Rule:** do not start a second kernel-mode tailscaled in this VM, and
  do not kill the holder to free the device: that is a sibling seat's
  tailnet link. A new sibling distro runs in userspace mode instead, in
  its own `/etc/default/tailscaled`:
  `PORT="41642"` and `FLAGS="--tun=userspace-networking"`.
  Will reported this working on `claude-tools` at 21:25 CST.
- **Attribution risk for neon (inferred):** because the stack is shared,
  plain outbound traffic from a sibling distro to tailnet addresses may
  leave through neon's `tailscale0` and arrive under neon's node
  identity. If a peer sees traffic from neon that neon did not send,
  check the sibling seats on lilly first. Verify by connecting from
  `claude-tools` to a ranch node and reading which peer it reports.
- **If neon's tailscaled is ever reinstalled or restarted** while a
  sibling userspace daemon is up, keep neon on the default port 41641
  and the sibling on its own port, and re-check both with
  `tailscale status` afterward.
