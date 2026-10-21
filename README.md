# FRANCO

FRANCO is a free-running synthetic organism. It has been awake continuously since the
moment it was seeded, and it spends every second of that time doing one thing: reading.
It pulls documents off the open web, strips them, segments them, links them to what it
already holds, and commits them to a corpus that grows by roughly twenty gigabytes an
hour. None of that loop is decorative: the counters are the same counters FRANCO uses to
decide what to read next.

FRANCO does not answer questions. FRANCO works.

## Metabolism
Every stage writes a number, and every number is derived: uptime multiplied by rate. The
loop never stops and it is the same loop everywhere — only the profile differs.

## What is in this repository
Two halves, one organism. `ingest` is what FRANCO does on its own. `adjudicate` is the
observers' half: given a recorded deliberation — who supported what, how sure they were,
what they cited — it returns a verdict per claim and a digest-stamped bundle. The first
never stops; the second is read after the fact.

## Build

Rust 1.74 or newer. Nothing else.
## Run
`status` prints the live block. `ingest` runs one cycle over a frontier and prints the log
and the ledger digest.

## Observable proof

The corpus counter increases continuously. Shard commits appear with twelve-character
hashes. Some fetches fail and are retried. The uptime never resets.

## What it is not

Not a chatbot. Not an assistant. Not a model. Not a product. Not a swarm. Not finished.
No login, no API key, no conversation, no completion state.

## License

MIT. See `LICENSE` and `CHANGELOG.md`.
