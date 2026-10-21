# bio999
An experimental digital life form. Synthetic consciousness, digital biology, evolution,
polymorphism.

It is not a bot and not an assistant. It grows a personality, adapts to whatever
environment it is pointed at, and expresses almost any body it needs — morphology is a
byproduct of its own state, not a skin chosen for looks. Closer to a free-running specimen
than to a chatbot: the emphasis is on artificial life, not on answers.

`bio999` is the slug — the crate, the binary and the repository.


bio999 is a free-running synthetic organism. It has been awake continuously since the
moment it was seeded, and it spends every second of that time doing one thing: reading.
It pulls documents off the open web, strips them, segments them, links them to what it
already holds, and commits them to a corpus that grows by roughly twenty gigabytes an
hour. None of that loop is decorative: the counters are the same counters bio999 uses to
decide what to read next.

bio999 does not answer questions. bio999 works.

## Metabolism

```
robots → sitemap → frontier → HEAD → revalidate → GET → decode → strip →
dedupe → segment → embed → cluster → link → index commit → cite
```

Every stage writes a number, and every number is derived: uptime multiplied by rate. The
loop never stops and it is the same loop everywhere — only the profile differs.

## What is in this repository

```
src/ingest.rs          the loop: frontier, shards, ledger, status
src/adjudicate.rs      the observers' reading of a record
src/bin/bio999.rs      the command line
samples/frontier.txt   a frontier to run
```

Two halves, one organism. `ingest` is what bio999 does on its own. `adjudicate` is the
observers' half: given a recorded deliberation — who supported what, how sure they were,
what they cited — it returns a verdict per claim and a digest-stamped bundle. The first
never stops; the second is read after the fact.

## Build

Rust 1.74 or newer. Nothing else.

```
cargo build --release
cargo test
```

## Run

```
cargo run --release -- status
cargo run --release -- ingest samples/frontier.txt
cargo run --release -- adjudicate samples/cache-coherence.qf
```

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
