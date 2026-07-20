# Contributing to QuorumForge

QuorumForge is deliberately scoped: normalize claims, weigh support and
contradiction, render deterministic verdicts - no network, no dependencies,
no hidden state. Contributions that respect that scope are welcome.

## Development setup

```bash
git clone https://github.com/VanTaS15/QuorumForge.git
cd QuorumForge
cargo build
cargo test
```

The TypeScript viewer lives in `viewer/` (`npm ci && npm test`). Verdicts
must reproduce on any toolchain; CI fails on output drift.

## Ground rules

- **Deterministic verdicts.** The same evidence bundle and roster must
  always produce the same verdict bytes. No wall-clock, no randomness, no
  map-iteration order in output.
- **Std-only.** The engine, parser, JSON codec, and CLI stay on the Rust
  standard library. A new dependency needs a very good reason.
- **Provenance in every report.** Each rendered verdict cites the claims
  and agents that produced it; an uncited number is a bug.
- **Tests on behaviour changes.** `tests/adjudication.rs`,
  `tests/parsing.rs`, and `tests/bundle_and_reports.rs` are the contract;
  threshold changes need boundary coverage.

## Commit style

Short imperative subjects (`feat: ...`, `fix: ...`, `docs: ...`). Body only
when the "why" is not obvious from the diff.

## Reporting issues

Include a minimal `.qf` session (see `samples/`), the agent roster, and the
verdict you expected versus the one rendered. Sanitize names and paths.