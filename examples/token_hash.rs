//! Reads JSON lines with a "text" field on stdin, tokenizes each with the
//! standard tokenizer and writes one xxh3 hash per document (over every
//! token and its position) to stdout. Reports tokenize-only time on stderr.
use std::io::{BufRead, BufWriter, Write};

use infino::superfile::fts::tokenize::StandardTokenizer;
use xxhash_rust::xxh3::Xxh3;

fn main() {
    let stdin = std::io::stdin().lock();
    let mut out = BufWriter::new(std::io::stdout().lock());
    let mut tok_ns: u128 = 0;
    let (mut docs, mut tokens, mut bytes) = (0u64, 0u64, 0u64);
    for line in stdin.lines() {
        let line = line.expect("line");
        let v: serde_json::Value = serde_json::from_str(&line).expect("json");
        let text = v["text"].as_str().unwrap_or("");
        // Timed pass with a callback that only touches the token, so the
        // time is the tokenizer's; the hashing pass below is untimed.
        let mut sink = 0usize;
        let t0 = std::time::Instant::now();
        StandardTokenizer.tokenize_each_inline(text, |t| {
            sink = sink.wrapping_add(t.len()).wrapping_add(t.as_bytes()[0] as usize);
        });
        tok_ns += t0.elapsed().as_nanos();
        std::hint::black_box(sink);
        let mut h = Xxh3::new();
        let mut pos = 0u64;
        StandardTokenizer.tokenize_each_inline(text, |t| {
            h.update(t.as_bytes());
            h.update(&[0xff]);
            h.update(&pos.to_le_bytes());
            pos += 1;
        });
        docs += 1;
        tokens += pos;
        bytes += text.len() as u64;
        writeln!(out, "{:016x}", h.digest()).expect("write");
    }
    eprintln!(
        "docs={docs} tokens={tokens} bytes={bytes} tokenize_s={:.3}",
        tok_ns as f64 / 1e9
    );
}
