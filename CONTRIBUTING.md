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

