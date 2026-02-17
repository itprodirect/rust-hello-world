# 03 - Concurrency Without Data Races

## Why This Phase Matters

Rust makes data-race safety a compile-time guarantee, not a runtime hope.

## Crate in This Phase

- `concurrency_demo`

## Run

```bash
cargo run -p concurrency_demo --release
cargo test -p concurrency_demo
```

## Verify

- Sequential and concurrent outputs are identical.
- Different thread counts preserve correctness.
- Shared counter reaches expected value under contention.

## Reusable Patterns

- chunked worker threads + `mpsc` fan-in for CPU-bound reductions
- explicit shared state with `Arc<Mutex<T>>`

Use them in:
- batch transformations
- log/metric aggregation
- parallel offline jobs

## Tests Worth Reading

- `sequential_and_concurrent_produce_same_result`
- `concurrent_with_different_thread_counts`
- `shared_counter_stress`

## Contributor Note

Prioritize correctness assertions before timing comparisons when changing parallel code.

## Next

[04 - Performance](04-performance.md)

Last updated: 2026-02-17
