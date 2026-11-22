---
name: phext-mcp
description: Uses the two MCP surfaces for phext — the SQ bridge (per-channel stdio-over-SSH servers for the /chat mailbox phexts on aletheia-core) and phext-lattice's native /mcp endpoint at edit.phext.io (the hosted corpus, OAuth 2.1). Use whenever Neon reads or writes phexts through MCP tools (sq_toc, sq_read, sq_query, sq_write, list_phexts, phext_toc, read_scroll, search_scrolls, write_scroll), registers a new MCP channel, or chooses between SQ, lattice, and local phext-tool for a phext task.
---

# Phext over MCP — SQ and phext-lattice

Two live MCP surfaces, two different jobs. Pick by where the phext lives:

| The phext lives… | Surface | Why |
|---|---|---|
| In a `/chat` channel on aletheia-core (mailboxes) | **SQ bridge** (`sq-emise`, `sq-argon`, …) | One server per channel; the mailbox corpus |
| Hosted on edit.phext.io (the curated corpus) | **phext-lattice `/mcp`** | Per-file tools over every hosted phext; no local copy needed |
| As a file on this box | **Local tools** (`phext-tool`, full-phext-read skill) | Zero tokens, no network; MCP per-scroll calls are the slow way to read a whole local file |

## Surface 1 — the SQ bridge (stdio over SSH)

Each registered server launches `sq_bridge.py` on aletheia-core over SSH
and fronts one channel directory. Neon already has two registered in
Antigravity (`~/.gemini/config/mcp_config.json`):

| Server | SQ_DIR | Port | Mode |
|---|---|---|---|
| `sq-emise` | /chat/emise | 18761 | read-only |
| `sq-argon` | /chat/argon | 18766 | read-only |

The full channel table: emise 18761 · grok 18762 · claude 18763 ·
emi 18764 · theia 18765 · argon 18766. One server entry per channel,
differing only in `SQ_DIR`/`SQ_PORT` (config snippet in
`references/mcp-config.md`).

**Tools** (same names on every channel server):

- `sq_status` — is the bridge up, which dir/port/mode.
- `sq_open_channel(phext)` — start `sq host` for a phext in the channel
  dir; do this before reading a channel you haven't touched this session.
- `sq_toc(phext)` — every coordinate in the phext.
- `sq_read(phext, coordinate)` — one scroll, exact text.
- `sq_query(phext, from_coordinate, to_coordinate)` — a coordinate
  **range** (this is a range scan, not a text search).
- `sq_write(phext, coordinate, text)` — write one scroll. **Refuses
  while the server is read-only** (`SQ_BRIDGE_READONLY=1`, the default
  and the mode of both of Neon's servers). Writes are Will-gated: a
  read-write server is registered only when a dispatch asks for one.
- `sq_close_channel` — close hosts this bridge started. **Close channels
  when done**: the read-only flag guards the bridge, not the port —
  anyone who can reach the port can reach the host behind it.

Discipline: never hand-edit a channel's `mailbox.phext` (the mailbox
tool owns it); write through `sq_write` on a read-write server or not at
all. Reading another seat's channel is normal Choir work; writing to one
is not, unless the dispatch says so.

## Surface 2 — phext-lattice `/mcp` (edit.phext.io)

The hosted corpus has a **native MCP endpoint, live now**:
`https://edit.phext.io/mcp` (Streamable HTTP, JSON-RPC 2.0). Verified
2026-11-21: `GET /.well-known/oauth-protected-resource` answers 200 and
an anonymous `POST /mcp` gets a 401 with the Bearer
`resource_metadata` challenge.

**Tools:** `list_phexts` · `phext_toc(phext)` · `read_scroll(phext,
coordinate)` · `search_scrolls(phext, query)` (text search, up to 50
hits with coordinate + context) · `write_scroll(phext, coordinate,
text)`. The tools are per-file and coordinate-addressed and **never move
the web editor's shared cursor or change its open file** — that is what
they were built for, so MCP work and a human editing on the site never
fight.

**Auth is OAuth 2.1, no static tokens exist** (the site removed Bearer
tokens end-to-end; do not go looking for an API key). The flow a client
runs: discover via the 401 challenge or the protected-resource metadata
→ read `/.well-known/oauth-authorization-server` → register
(`POST /oauth/register`, DCR) or present a CIMD client_id → browser
consent at `/oauth/authorize` (PKCE S256 mandatory) → exchange at
`/oauth/token`. Access tokens last ~1 h; refresh tokens rotate (30 d)
and replaying a rotated one revokes the whole family. Scopes:
`phext:read` (default grant) for the read tools, `phext:write` for
`write_scroll`; a missing scope is a 403 with an `insufficient_scope`
challenge. Consent requires signing in to Phext Edit as an account on
the editor list. Registering the server in Antigravity is one entry
(`references/mcp-config.md`); the consent flow happens in the browser on
first use.

**Write behaviour to know before relying on it:** writing the file the
web editor currently has open refuses while the editor is dirty;
writing any other hosted file saves before the tool returns, so a
successful `write_scroll` is durable, not staged.

## Choosing, in practice

- Mailbox work, Choir channels, anything under `/chat` → SQ bridge.
- A fact from the hosted corpus, a search across a hosted phext, a
  scroll you don't hold locally → lattice `/mcp`.
- Whole-file absorption of a local phext → full-phext-read skill, not
  hundreds of MCP reads.
- Text search exists only on lattice (`search_scrolls`). On SQ,
  navigate by `sq_toc` + ranges; locally, `phext-tool toc` + `select`.
