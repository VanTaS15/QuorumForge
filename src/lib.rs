//! # bio999
//!
//! bio999 is a free-running synthetic organism. It has been awake since the
//! moment it was seeded, and it spends every second of that time doing one
//! thing: reading. It pulls documents off the open web, strips them, segments
//! them, links them to what it already holds, and commits them to a corpus that
//! grows by roughly twenty gigabytes an hour.
//!
//! Nothing in the loop is decorative. The counters the observers watch are the
//! same counters bio999 uses to decide what to read next.
//!
//! ## Metabolism
//!
//! ```text
//! robots → sitemap → frontier → HEAD → revalidate → GET → decode → strip →
//! dedupe → segment → embed → cluster → link → index commit → cite
//! ```
//!
//! Every stage writes a number, and every number is derived: uptime multiplied
//! by rate. The same frontier always produces the same log bytes and the same
//! shard digests.
//!
//! ## Two halves
//!
//! `ingest` is the organism's own loop: the frontier, the shards, the ledger.
//! `adjudicate` is the observers' reading of a record: weighted positions, a
//! verdict per claim, a digest-stamped bundle. The first is what bio999 does;
//! the second is how a council of observers is read after the fact.
//!
//! ## Example
//!
//! ```
//! let frontier = vec![
//!     "https://arxiv.org/abs/2409.02142|https://arxiv.org/".to_string(),
//! ];
//! let (log, ledger) = bio999::ingest::cycle(&frontier);
//! assert_eq!(ledger.len(), 1);
//! assert!(!log.is_empty());
//! ```
//!
//! ```
//! use bio999::{adjudicate, normalize, parse};
//!
//! let src = "\
//! delib | d1 | Is the cache coherent under concurrent writes?
//! agent | a | Ada | 1.5 | systems
//! agent | b | Boro | 1.0 | skeptic
//! claim | c1 | cache | Writes are linearizable.
//! pos   | a  | c1 | support     | 0.9 | verified with a model checker
//! pos   | b  | c1 | contradict  | 0.4 | one race remains under retry
//! ";
//! let mut delib = parse::parse_lines(src).unwrap();
//! normalize::normalize_deliberation(&mut delib);
//! let adj = adjudicate::adjudicate(&delib, &adjudicate::Policy::default());
//! let verdict = &adj.verdicts["c1"];
//! assert!(verdict.support_mass > verdict.contradiction_mass);
//! ```

pub mod adjudicate;
pub mod bundle;
pub mod ingest;
pub mod json;
pub mod model;
pub mod normalize;
pub mod parse;
pub mod report;

pub use adjudicate::{adjudicate, Adjudication, Outcome, Policy, Verdict};
pub use bundle::{build as build_bundle, Bundle};
pub use ingest::{cycle, Document, Ledger};
pub use model::{Agent, Citation, Claim, Deliberation, Position, Stance};

/// The full end-to-end run: parse, normalize, and adjudicate a source string
/// whose format is inferred from `path`. Returns the deliberation together with
/// its adjudication so callers can render whichever report they need.
pub fn run(
    path: &str,
    contents: &str,
    policy: &Policy,
) -> Result<(Deliberation, Adjudication), Box<dyn std::error::Error>> {
    policy.validate()?;
    let mut delib = parse::parse_auto(path, contents)?;
    normalize::normalize_deliberation(&mut delib);
    let adj = adjudicate::adjudicate(&delib, policy);
    Ok((delib, adj))
}
