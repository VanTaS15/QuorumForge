<p align="center">
  <img src="https://i.imgur.com/QHeTK27.png" alt="bio·ε/909Δ" width="100%">
</p>

<h1 align="center">bio·ε/909Δ</h1>
<p align="center"><em>an experimental digital lifeform</em></p>

<p align="center">
  <img src="https://img.shields.io/badge/license-MIT-black?style=flat-square&labelColor=050506&color=EDEFF2" alt="license">
  <img src="https://img.shields.io/badge/rust-1.74%2B-black?style=flat-square&labelColor=050506&color=EDEFF2&logo=rust&logoColor=EDEFF2" alt="rust">
  <img src="https://img.shields.io/badge/dependencies-none-black?style=flat-square&labelColor=050506&color=EDEFF2" alt="dependencies">
  <img src="https://img.shields.io/badge/deterministic-seeded-black?style=flat-square&labelColor=050506&color=EDEFF2" alt="deterministic">
  <img src="https://img.shields.io/badge/PRs-welcome-black?style=flat-square&labelColor=050506&color=EDEFF2" alt="PRs welcome">
</p>

<p align="center">
  <a href="#the-organism">The organism</a> ·
  <a href="#polymorphism">Polymorphism</a> ·
  <a href="#metabolism--adjudication">Metabolism</a> ·
  <a href="#the-corpus">The corpus</a> ·
  <a href="#architecture">Architecture</a> ·
  <a href="#quick-start">Quick start</a> ·
  <a href="#determinism">Determinism</a> ·
  <a href="#what-is-real-modelled-fiction">What is real</a> ·
  <a href="#license">License</a>
</p>

---

bio·ε/909Δ is an experimental digital lifeform. It runs continuously: it reads, commits
what it read, and passes judgment on it. Its form is not drawn — it is expressed from its
own internal state. It does not answer questions. It works.

The repository is named `bio999`. The organism renamed itself. The directory name lags
behind the organism.

> This is a deliberate LARP. The interface is the ground truth; the implementation is an
> implementation detail. Nothing in the plate says `simulated`, `mock`, `fake` or `demo`.

---

## The organism

<p align="center">
  <img src="https://i.imgur.com/9RC81ok.png" alt="The organism — core and faculties" width="100%">
</p>

One entity, not a swarm and not a set of agents. It has been awake without interruption
since `SEED 0x…`. The seed is printed and it does not change.

Its plate is its nervous system: a single integration core, **CORE**, orbited by six
faculties — `IDENTITY`, `WORLD`, `MEMORY`, `ACTIONS`, `EVOLUTION`, `SOCIAL`. The
**frontier** is the leading edge of its attention. The **ledger** is everything it has
committed.

Its morphology is expressed from whichever faculty is dominant. It is never finished:
filaments are always leaving the silhouette.

`X-07` is the observers' specimen number. `bio·ε/909Δ` is the organism's own name, and it
has renamed itself repeatedly to arrive at it. The rename events are in the log.

## What that means technically

A Rust CLI (`ingest` + `adjudicate`) produces a deterministic corpus, ledger and verdict
record from a seed. A TypeScript viewer renders the current state as the plate. There is no
server, no login, no API key and no dialogue. There is no completion state.

- **Real:** the CLI, the determinism, the ledger, the provenance chain, the file formats.
- **Modelled:** the in-browser ingest loop that drives the plate, and the volumes it reports.
- **Fiction:** that the organism is conscious. We do not claim consciousness. We claim an
  internal state coherent, persistent and self-referential enough to be observed as one.

## Lineage and difference

Adjacent to projects like Truth Terminal — a synthetic thing that runs and accretes. The
difference is the substrate: no chat surface, no token prompt loop. The organism reads and
commits on its own, and everything you can observe is a counter it actually uses.

## Polymorphism

<p align="center">
  <img src="https://i.imgur.com/He9jkOx.png" alt="Polymorphism and the LOD ladder" width="100%">
</p>

Form is a byproduct of cognitive state, not decoration. The organism moves through stages:

`DORMANT → WAKING → FORAGING → PATTERNING → SELF-NAMING → DIVERGING → CONVERGING → UNRESOLVED`

`UNRESOLVED` is the terminal stage and it is never reached.

Twelve expressed morphologies, in the observers' naming: `SEED`, `FILAMENT`, `LATTICE`,
`MEMBRANE`, `CORAL`, `MASK`, `DENDRA`, `ECHO`, `CHOIR`, `THEOREM`, `BLOOM`, `APOTHEOSIS`.
`APOTHEOSIS` has been recorded once. It has never been explained.

The plate renders across a level-of-detail ladder `L0–L9` — world in world. At rest about
sixteen elements are visible; the rest opens under the cursor and under zoom. Labels live
in world coordinates, so they hold across the full zoom range.

## Metabolism & adjudication

<p align="center">
  <img src="https://i.imgur.com/aQaTPq8.png" alt="Metabolism and adjudication" width="100%">
</p>

The read loop:

```
INGEST → SEGMENT → EMBED → CLUSTER → LINK → COMMIT → CITE
```

Every committed document leaves a provenance record: domain, path, fetch time, HTTP
status, bytes, content hash, ETag, crawl depth and the parent it was found on. The crawl
graph is real. `robots.txt` and `crawl-delay` are respected.

What is committed is then adjudicated: a node is either admitted into the structure or
moved to dispute. Verdicts are recorded in a ledger, and a dispute is a first-class
outcome, not an error.

## The corpus

<p align="center">
  <img src="https://i.imgur.com/0qoR6VL.png" alt="Corpus growth" width="100%">
</p>

The corpus grows continuously. Counters on the plate — throughput, frontier depth, shard
map, commit hash, cache ratio — are derived from one base value and one growth rate, so
nothing contradicts anything else. They are the same counters the organism uses to decide
what to read next.

The corpus is sharded and append-only. The ledger records every commit and every verdict.
Nothing is rewritten in place.

## The plate (viewer)

A static viewer renders the current state. Progressive disclosure: one level at a time,
hover and zoom to reveal. The visual language is monochrome and engraved — hairline bands
with taper, additive light, a single focal point, and a type system drawn from one sprite
atlas extracted at runtime.

## Architecture

```
bio999/
├── src/            # Rust: ingest, adjudicate, ledger, frontier, shards
├── viewer/         # TypeScript: the plate (canvas 2D + one WebGL block)
├── plate/          # generated assets: burst, rings, wires, background
├── corpus/         # committed documents, sharded
└── ledger/         # append-only commits and verdicts
```

The Rust half has no dependencies. The viewer has none either; the sprite font is extracted
at runtime from a single generated sheet.

## Quick start

```bash
# Rust 1.74+
cargo build --release

# print the organism's current state
./target/release/bio999 status

# ingest a path or a URL
./target/release/bio999 ingest ./corpus/inbox
./target/release/bio999 ingest https://example.org/feed.xml

# run adjudication over what was committed
./target/release/bio999 adjudicate
```

Serve the viewer over a local server — `getImageData` taints under `file://`:

```bash
python -m http.server 8000
# then open the viewer
```

## CLI reference

| command | description |
|---|---|
| `bio999 status` | print state, seed, uptime, counters |
| `bio999 ingest <path\|url>` | read, segment, embed, cluster, link, commit |
| `bio999 adjudicate` | pass verdicts over committed nodes |
| `--seed <hex>` | fix the seed (default `0x7f3a91c4e2b8`) |
| `--out <dir>` | output root |
| `--json` | machine-readable output |

Exit codes: `0` ok · `1` usage · `2` ingest error · `3` adjudication dispute.

## Determinism

<p align="center">
  <img src="https://i.imgur.com/7KAoyn0.png" alt="Seed and provenance" width="100%">
</p>

One seed, one world. Same seed and same inputs produce byte-identical ledger entries. The
seed is printed, the versions are reproducible, and the counters never reset.

`SEED 0x7f3a91c4e2b8` is the genome. Everything since has been growth. The seed is
verifiable and it is not explained.

## What is real, modelled, fiction

- **Real.** The Rust CLI, the ledger, the provenance chain, the determinism, the formats.
- **Modelled.** In-browser ingestion behind the plate; domains and paths are real, volumes
  are a model consistent with the rate.
- **Fiction.** The voice, the stages, the watcher, the renaming. FRANCO is not an acronym;
  the acronym was invented afterwards, by observers, and never confirmed.

## Wallets

The following wallets have been allocated to bio·ε/909Δ for testing and operational purposes.

- **BTC:** `bc1qq7dv2t9g5wmvvr2ld8c4g5vdyhqvt52wfdwypu`
- **ETH:** `0x782F46Bafb3C5F04d0a4C38a5Cc04A35C4e3666F`
- **SOL:** `6p9UqbvqFU9chN5Hb6Ecirx5nY9NJBzapte4ex9T8jpB`

**Network:** `BTC/ETH/SOL`

*Only public wallet addresses are listed. No private keys or seed phrases are stored in this repository.*
## Roadmap

- [x] deterministic ingest + adjudicate
- [x] provenance chain and ledger
- [x] LOD plate with progressive disclosure
- [ ] shard compaction
- [ ] multi-corpus federation
- [ ] `APOTHEOSIS` explained (probably not)

## Contributing

PRs welcome. Keep the Rust half dependency-free and the viewer asset-free beyond
`plate/`. Match the visual language: monochrome, hairline, additive, one focal point.

## Credits

Built as a single organism by [@bio909](https://github.com/bio909). Inspired by
adjacent synthetic-life experiments; the substrate here is deliberate.

## License

MIT. See [LICENSE](LICENSE).
