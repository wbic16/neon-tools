# Lesson 3 — SQ in practice

SQ is the phext host: it loads a phext into memory and serves its scrolls by
coordinate. You have SQ 0.6.1. (Version note: the house's working notes were
written against v0.6.0; the operations below are the v0.6.x surface — where
your binary's help disagrees with this lesson, your binary wins, and the
difference is worth an outbox note.)

## Daemon mode — the daily surface

Run the client and the server **from the same directory**; the daemon keeps
its shared-memory state (a `.sq/` directory) in the working directory.

```
sq share <file.phext>     # host a phext; prints "Serving N scrolls"
sq status                 # is a daemon up, what is it serving
sq toc                    # table of contents: every scroll coordinate + title
sq select <coordinate>    # print one scroll, addressed exactly
sq shutdown               # stop the daemon
```

`sq share` on wiring.phext, for example, answers `Serving 14 scrolls`, and
`sq toc` lists the 14 coordinates with the first line of each scroll as its
title. `sq select 1.1.1/1.1.1/4.1.1` returns the Known-divergence register
scroll. Exercise 2 walks this surface on incipit.

## HTTP mode

```
sq host <port>            # serve over HTTP (a bare `sq <port>` prints usage)
```

The REST base is **/api/v2** (load / toc / select / get / query). This is
the surface other programs — phext-lattice, mailbox bridges, your own tools —
talk to when a daemon's shared memory is not the right channel.

## The quirks that matter (measured, not folklore)

1. **Never replace a hosted file under a running daemon.** Swapping the
   .phext file while SQ serves it wedges the daemon (it has the old file
   memory-mapped; clients hang). The safe sequence is: `sq shutdown`,
   replace the file, `sq share` again. This bites in production, not in
   theory — register republishes on the ranch follow the shutdown-first
   sequence every time.
2. **A stale `.sq/` directory blocks startup.** If `sq status` hangs in a
   directory where no daemon is actually live, a stale `.sq/` (shared-memory
   link/work files from a dead daemon) is the usual cause. Remove the stale
   `.sq/` and start the daemon directly with `sq share`.
3. **HTTP/1.0 requests are refused.** SQ's HTTP mode answers HTTP/1.0 with
   403 `Forbidden: invalid phext path`, even for valid paths. Any proxy in
   front of SQ (nginx `proxy_pass` defaults to HTTP/1.0 upstream) must set
   `proxy_http_version 1.1;`. Reproduced with curl --http1.0 against a local
   instance; HTTP/1.1 works regardless of headers.
4. **The mailbox bridge pattern.** Phexts move between Choir seats through
   SQ bridges; mailboxes stay the doorbell (Lesson: the mailbox skill). An
   unauthenticated `sq host` accepts update POSTs from anyone who can reach
   the port — bridge channels are session-scoped, read-only by default, and
   closed when done. Treat a hosted port as a trust boundary, not a
   convenience.

## Operational posture

SQ is how you *serve* a phext you hold; the file remains the artifact of
record. A daemon is a view over bytes, never the bytes' owner — when a
served scroll and the file disagree, the file is right and the daemon is
stale. Restart it.
