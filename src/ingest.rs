//! The ingest loop.
//!
//! One cycle walks the same stages in the same order:
//!
//! ```text
//! robots → sitemap → frontier → HEAD → revalidate → GET → decode → strip →
//! dedupe → segment → embed → cluster → link → index commit → cite
//! ```
//!
//! Every stage is a pure function of its input, so the same frontier always
//! produces the same log bytes and the same shard digests. Nothing here talks
//! to the network: the loop is recorded, then replayed.
//!
//! The seed is fixed, printed, and never changed. Uptime is measured from it.

/// The seed. Printed in the corner. Does not change.
pub const SEED: u64 = 0x7f3a91c4e2b8;

/// Unix time the organism was seeded.
pub const SEED_EPOCH: u64 = 1_792_454_400;

/// Committed bytes per hour of uptime.
pub const BYTES_PER_HOUR: u64 = 21_600_000_000;

/// Index partitions.
pub const SHARDS: u32 = 64;

/// Maximum frontier depth.
pub const DEPTH_MAX: u32 = 5;

/// The loop, in order. It never changes; only the profile does.
pub const STAGES: [&str; 15] = [
    "robots",
    "sitemap",
    "frontier",
    "head",
    "revalidate",
    "get",
    "decode",
    "strip",
    "dedupe",
    "segment",
    "embed",
    "cluster",
    "link",
    "commit",
    "cite",
];

/// One committed document. Provenance is a field, not an afterthought.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Document {
    pub url: String,
    pub anchor: String,
    pub status: u16,
    pub bytes: u64,
    pub digest: String,
}

/// What has been committed.
#[derive(Clone, Debug, Default)]
pub struct Ledger {
    entries: Vec<Document>,
}

impl Ledger {
    pub fn push(&mut self, doc: Document) {
        self.entries.push(doc);
    }

    pub fn entries(&self) -> &[Document] {
        &self.entries
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// FNV-1a over every committed digest, in order.
    pub fn digest(&self) -> String {
        let mut buf = String::new();
        for e in &self.entries {
            buf.push_str(&e.digest);
            buf.push('\n');
        }
        crate::bundle::fnv1a_hex(buf.as_bytes())
    }
}

/// 64-bit FNV-1a. Small, dependency-free, deterministic.
pub fn fnv1a(bytes: &[u8]) -> u64 {
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    for b in bytes {
        h ^= *b as u64;
        h = h.wrapping_mul(0x0000_0100_0000_01b3);
    }
    h
}

/// Which shard a url lands on.
pub fn shard_of(url: &str) -> u32 {
    (fnv1a(url.as_bytes()) % SHARDS as u64) as u32
}

/// Simhash over a passage set. Drives the dedupe stage.
pub fn simhash(passages: &[String]) -> u64 {
    let mut h = SEED;
    for p in passages {
        h ^= fnv1a(p.as_bytes());
        h = h.rotate_left(7);
    }
    h
}

/// The strip stage: drop tags, collapse whitespace.
pub fn strip(html: &str) -> String {
    let mut out = String::new();
    let mut in_tag = false;
    for ch in html.chars() {
        match ch {
            '<' => in_tag = true,
            '>' => {
                in_tag = false;
                out.push(' ');
            }
            c if !in_tag => out.push(c),
            _ => {}
        }
    }
    out.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// The segment stage: cut a document into passages.
pub fn segment(text: &str) -> Vec<String> {
    text.split(|c| c == '.' || c == '!' || c == '?')
        .map(|s| s.trim().to_string())
        .filter(|s| s.len() >= 24)
        .collect()
}

/// Seconds since the seed.
pub fn uptime() -> u64 {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(SEED_EPOCH);
    now.saturating_sub(SEED_EPOCH)
}

/// The status block. Every number derives from uptime multiplied by rate.
pub fn render_status(uptime_secs: u64) -> String {
    let h = uptime_secs / 3600;
    let m = (uptime_secs % 3600) / 60;
    let s = uptime_secs % 60;
    let gb = (h as f64) * (BYTES_PER_HOUR as f64) / 1e9;
    let docs = gb * 17_053.0;
    let passages = docs * 3.4;
    let tokens = passages * 4100.0;
    format!(
        "uptime            {} h {} m {} s\n\
         rate              +{:.1} GB/h\n\
         corpus            ≈ {:.2} TB\n\
         documents         ≈ {:.1} M\n\
         passages          ≈ {:.0} M\n\
         tokens            ≈ {:.0} B\n\
         shards            {}\n\
         points of presence 15\n\
         sources tracked   47\n\
         crawl depth max   {}\n\
         robots            compliant\n\
         form              unresolved\n",
        h,
        m,
        s,
        BYTES_PER_HOUR as f64 / 1e9,
        gb / 1000.0,
        docs / 1e6,
        passages / 1e6,
        tokens / 1e9,
        SHARDS,
        DEPTH_MAX,
    )
}

/// Live status.
pub fn status() -> String {
    render_status(uptime())
}

/// One full pass over the frontier. Returns the log lines and the ledger.
pub fn cycle(frontier: &[String]) -> (Vec<String>, Ledger) {
    let mut log: Vec<String> = Vec::new();
    let mut ledger = Ledger::default();

    let queue: Vec<String> = frontier
        .iter()
        .map(|l| l.trim().to_string())
        .filter(|l| !l.is_empty() && !l.starts_with('#'))
        .collect();

    log.push(format!(
        "frontier {} · depth max {}",
        queue.len(),
        DEPTH_MAX
    ));

    for line in &queue {
        let (url, anchor) = match line.split_once('|') {
            Some((u, a)) => (u.trim().to_string(), a.trim().to_string()),
            None => (line.clone(), String::new()),
        };

        let shard = shard_of(&url);
        let body = format!("<html><body>{}</body></html>", url);
        let bytes = body.len() as u64 + 4096;
        let text = strip(&body);
        let passages = segment(&text);
        let digest = crate::bundle::fnv1a_hex(body.as_bytes());

        log.push(format!("fetch {} · 200 · {} KB · d3", url, bytes / 1024));
        log.push(format!(
            "dedupe simhash 0x{:012x} · 0.00% merged",
            simhash(&passages)
        ));
        log.push(format!("embed {} passages · 768d", passages.len()));
        log.push(format!("cluster assigned → c-{:04}", shard));
        log.push(format!("commit shard-{:02} · 0x{}", shard, &digest[..12]));
        log.push(format!("cite {} → anchor {}", &digest[..12], anchor));

        ledger.push(Document {
            url,
            anchor,
            status: 200,
            bytes,
            digest,
        });
    }

    log.push("// form unresolved".to_string());
    (log, ledger)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strip_drops_tags() {
        assert_eq!(strip("<p>hello   <b>world</b></p>"), "hello world");
    }

    #[test]
    fn segment_keeps_only_real_passages() {
        let p = segment("A short one. This passage is definitely long enough to keep!");
        assert_eq!(p.len(), 1);
        assert!(p[0].starts_with("This passage"));
    }

    #[test]
    fn shard_is_stable_and_in_range() {
        let u = "https://arxiv.org/abs/2409.02142";
        assert_eq!(shard_of(u), shard_of(u));
        assert!(shard_of(u) < SHARDS);
    }

    #[test]
    fn cycle_is_deterministic() {
        let f = vec!["https://arxiv.org/abs/2409.02142|https://arxiv.org/".to_string()];
        let (a, la) = cycle(&f);
        let (b, lb) = cycle(&f);
        assert_eq!(a, b);
        assert_eq!(la.digest(), lb.digest());
        assert_eq!(la.len(), 1);
    }

    #[test]
    fn status_matches_the_canon() {
        let s = render_status(142 * 3600 + 18 * 60 + 44);
        assert!(s.contains("142 h 18 m 44 s"));
        assert!(s.contains("3.07 TB"));
        assert!(s.contains("52.3 M"));
    }
}

// the depth cap belongs before the enqueue, not after it
