# Lesson 4 — The lattice: phext-lattice and edit.phext.io

SQ serves one phext on one machine. **phext-lattice** is the layer above:
the lattice under the native editors, and the server behind
**edit.phext.io**. Where SQ answers "give me the scroll at this
coordinate," the lattice is the multi-corpus, multi-user surface — traversal
and reading across hosted phexts, editing behind a sign-in.

The access shape, as built:

- **Reading and traversal are open** — you can walk a hosted corpus by
  coordinate without a login. (This is the phext.tech platform ruling too:
  read-only public access; signup required for write.)
- **Editing is authenticated.** The lattice's OAuth suite signs a user in
  through GitHub, Google, or X (authorization code + PKCE).

## The desktop SSO flow — the current worked example

The Phextron program (the executive console; see BRAIN-DUMP.md §7) needs a
desktop application to hold a lattice session without becoming a browser.
The lattice already implements a desktop SSO audience in its `src/oauth.rs`.
The flow, as specified in the console's SPEC:

1. The desktop app asks its local browser to open
   `GET /api/auth/sso/desktop` — a **loopback-only** endpoint carrying a
   state nonce.
2. The browser completes the lattice's normal sign-in (GitHub / Google / X,
   auth code + PKCE).
3. The lattice issues a **sealed, single-use assertion** (120-second life).
4. The app exchanges it: `POST /api/auth/desktop/exchange` → a **sealed
   14-day session**, stored in the OS credential store (the console's G-L2
   crate, landed 2026-11-21, is exactly this store/read/delete binding).
5. The session is **editor-list gated** — holding an assertion is not
   enough; the account must be on the deployment's editor list.

Two house rulings ride on this surface and are already settled — do not
re-litigate them in your own work:

- **Desktop SSO for humans. MCP Auth for agents.** The lattice also carries
  an MCP OAuth 2.1 store; the two auth surfaces do **not** converge. Each
  serves its audience.
- The console speaks **only** to phext-lattice. Panels that read rendered
  markdown, dashboard files, or raw register files were struck as slop; the
  lattice mediates everything, or the capability isn't in the console yet
  (the SPEC's R-D family names the mediation services still to grow).

Live-state honesty, as of this writing: the desktop audience is implemented
and its test suite passes (54 OAuth tests on the hub), but it is **not yet
armed in the edit.phext.io deployment** — the environment flag
`PHEXT_SSO_DESKTOP=1` is unset there, so the endpoint 404s live. Arming it
is a staged Will-sudo step on awsbox. The gap between "implemented and
tested" and "live" is stated here exactly as the house states it internally:
a feature that is not live is not done.

## Why this lesson is in a phext curriculum

The lattice is what phext addressing looks like when it grows up into
infrastructure: coordinates as URLs, corpora as places you can be *in*,
sessions scoped by audience. When you build on phext here, build toward the
lattice's shape — coordinate-addressed reads, authenticated writes, sealed
short-lived assertions exchanged for sessions — rather than inventing a
parallel surface.
