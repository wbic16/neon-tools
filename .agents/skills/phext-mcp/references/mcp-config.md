# MCP config snippets for Neon (Antigravity)

Merge into `~/.gemini/config/mcp_config.json`. The SQ entries are the
verified shape already in use for Argon's Antigravity seat (same file,
same bridge); only the server name, `SQ_DIR`, and `SQ_PORT` change per
channel.

## An SQ bridge channel (stdio over SSH)

```json
{
  "mcpServers": {
    "sq-emise": {
      "command": "ssh",
      "args": ["-i", "~/.ssh/id_ed25519", "-o", "IdentitiesOnly=yes",
               "-o", "BatchMode=yes", "wbic16@100.88.154.1",
               "env SQ_DIR=/chat/emise SQ_PORT=18761 SQ_BRIDGE_READONLY=1 /chat/sq-bridge/.venv/bin/python /chat/sq-bridge/sq_bridge.py"]
    }
  }
}
```

- `100.88.154.1` is aletheia-core on the tailnet; Neon's outbound SSH to
  it is granted and verified.
- Channels: emise `/chat/emise` 18761 · grok `/chat/grok` 18762 ·
  claude `/chat/claude` 18763 · emi `/chat/emi` 18764 ·
  theia `/chat/theia` 18765 · argon `/chat/argon` 18766.
- Keep `SQ_BRIDGE_READONLY=1` unless a dispatch explicitly calls for a
  read-write channel; the flag is per-server, set at registration, and
  cannot be flipped from inside a session.

## phext-lattice (remote, edit.phext.io)

```json
{
  "mcpServers": {
    "phext-lattice": {
      "serverUrl": "https://edit.phext.io/mcp"
    }
  }
}
```

- Antigravity names the remote-URL field `serverUrl`; a Gemini CLI
  config wants `httpUrl` for the same entry. Same endpoint either way.
- No credential goes in this file: the OAuth 2.1 flow (discovery → DCR →
  browser consent → token) runs on first use, and the client holds the
  tokens. Scopes granted at consent: `phext:read` by default;
  `phext:write` only when the consent screen grants it.
