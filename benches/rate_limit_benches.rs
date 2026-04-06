use criterion::{BatchSize, Criterion, criterion_group, criterion_main};
use rate_limit::TokenBucket;
use std::hint::black_box;
use std::time::{Duration, Instant};

fn benchmark_rate_limit(c: &mut Criterion) {
    c.bench_function("new_and_consume_success", |b| {
        b.iter(|| {
            let mut bucket = TokenBucket::new(8, 2);
            bucket.try_consume(black_box(4))
        })
    });

    c.bench_function("consume_success_only", |b| {
        b.iter_batched_ref(
            || TokenBucket::new(100, 2),
            |bucket| bucket.try_consume(black_box(10)),
            BatchSize::SmallInput,
        )
    });
    c.bench_function("consume_at_success_only", |b| {
        b.iter_batched_ref(
            || (TokenBucket::new(100, 2), Instant::now()),
            |(bucket, now)| bucket.try_consume_at(black_box(10), black_box(*now)),
            BatchSize::SmallInput,
        )
    });

    c.bench_function("consume_failed_only", |b| {
        b.iter_batched_ref(
            || TokenBucket::new(8, 2),
            |bucket| bucket.try_consume(black_box(10)),
            BatchSize::SmallInput,
        )
    });

    c.bench_function("consume_at_failed_only", |b| {
        b.iter_batched_ref(
            || (TokenBucket::new(8, 2), Instant::now()),
            |(bucket, now)| bucket.try_consume_at(black_box(10), black_box(*now)),
            BatchSize::SmallInput,
        )
    });

    c.bench_function("consume_after_refill_at", |b| {
        b.iter_batched_ref(
            || {
                let start = Instant::now();
                let mut bucket = TokenBucket::new_at(100, 2, start);
                bucket.try_consume_at(100, start);
                let five_seconds_after = start + Duration::from_secs(5);
                (bucket, five_seconds_after)
            },
            |(bucket, five_seconds_after)| {
                black_box(bucket.try_consume_at(black_box(10), black_box(*five_seconds_after)))
            },
            BatchSize::SmallInput,
        )
    });
}

criterion_group!(benches, benchmark_rate_limit);
criterion_main!(benches);
