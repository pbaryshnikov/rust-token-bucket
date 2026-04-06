# Rust Token Bucket Rate Limiter

Small Rust learning project implementing a single-threaded in-memory token bucket rate limiter.

## Features
- configurable capacity
- tokens-per-second refill
- deterministic time-based testing helpers
- Criterion benchmarks for public and deterministic paths

## Benchmark highlights
- public consume: ~24 ns
- deterministic consume_at: ~4.5 ns
- refill path: ~5.5 ns

## Key takeaway
The biggest cost in the public API path is `Instant::now()`, while the refill logic itself adds only a small overhead.

## Run
cargo test
cargo bench