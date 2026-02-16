use std::io::Cursor;

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use log_parser::{generate_log, parse_log, parse_log_streaming};

const NUM_LINES: usize = 10_000;

fn bench_parsers(c: &mut Criterion) {
    let input = generate_log(NUM_LINES);
    let input_bytes = input.len() as u64;

    let mut group = c.benchmark_group("log_parser");
    group.throughput(Throughput::Elements(NUM_LINES as u64));

    group.bench_with_input(
        BenchmarkId::new("allocating", NUM_LINES),
        &input,
        |b, input| {
            b.iter(|| {
                let entries = parse_log(input);
                assert_eq!(entries.len(), NUM_LINES);
            });
        },
    );

    group.bench_with_input(
        BenchmarkId::new("streaming", NUM_LINES),
        &input,
        |b, input| {
            b.iter(|| {
                let count = parse_log_streaming(Cursor::new(input.as_bytes()))
                    .filter(|r| r.is_ok())
                    .count();
                assert_eq!(count, NUM_LINES);
            });
        },
    );

    group.finish();

    // Also report throughput in bytes/sec
    let mut bytes_group = c.benchmark_group("log_parser_bytes");
    bytes_group.throughput(Throughput::Bytes(input_bytes));

    bytes_group.bench_with_input(
        BenchmarkId::new("allocating", input_bytes),
        &input,
        |b, input| {
            b.iter(|| parse_log(input));
        },
    );

    bytes_group.bench_with_input(
        BenchmarkId::new("streaming", input_bytes),
        &input,
        |b, input| {
            b.iter(|| {
                parse_log_streaming(Cursor::new(input.as_bytes())).for_each(drop);
            });
        },
    );

    bytes_group.finish();
}

criterion_group!(benches, bench_parsers);
criterion_main!(benches);
