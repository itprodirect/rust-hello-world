# 07 - Reusable Power Blocks

## Why This Phase Matters

Reusable patterns compound your Rust velocity faster than one-off examples.

Crate: `crates/power_blocks`

Run all tests:

```bash
cargo test -p power_blocks
```

## Block: Typed IDs (`typed_id`)

Use newtype wrappers to prevent domain ID mixups at compile time.

Use for:
- API/database entity IDs
- functions with multiple ID parameters

Key APIs:
- `UserId::new`
- `OrderId::new`
- `attach_order`

## Block: Typestate Builder (`typestate_builder`)

Use type-state to enforce required builder fields before `build()` can exist.

Use for:
- client configuration
- SDK setup where incomplete config should not compile

Key APIs:
- `HttpClientConfigBuilder::new`
- `.base_url(...)`
- `.timeout_ms(...)`
- `.build()`

## Block: Retry Policy (`retry`)

Use explicit retry policy + retry predicate instead of hidden retry loops.

Use for:
- transient network failures
- flaky external dependencies

Key APIs:
- `RetryPolicy::with_fixed_backoff`
- `retry(policy, operation, should_retry)`

## Block: Ordered Parallel Map (`parallel_map`)

Use parallel processing while preserving input order.

Use for:
- CPU-heavy collection transforms
- deterministic batch pipelines

Key API:
- `parallel_map(items, num_threads, map_fn)`

## Block: Zero-Copy Parsing (`zero_copy`)

Use borrowed slices when parsing high-volume text.

Use for:
- metrics/log ingestion
- memory-sensitive parsing workloads

Key APIs:
- `parse_metric_row`
- `parse_metric_rows`

## Practical Extraction Workflow

1. Copy one module and its tests.
2. Rename domain-specific types/errors.
3. Keep public API shape stable.
4. Extend behavior only after tests pass.

This keeps copied code dependable while you adapt it.

## Contributor Note

When adding a new block, include at least one edge-case test and one usage example.

Last updated: 2026-02-17
