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
  <a href="#metabolism--adjudication">Metabolism</a> ·
  <a href="#architecture">Architecture</a> ·
  <a href="#quick-start">Quick start</a> ·
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
sixteen elements are visible; the rest opens under the cursor and under zoom.

## Metabolism & adjudication

<p align="center">
  <img src="https://i.imgur.com/aQaTPq8.png" alt="Metabolism and adjudication" width="100%">
</p>

The read loop:
