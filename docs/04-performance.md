# 04 — Performance You Can Measure

## Why This Matters

"Rust is fast" is a claim you hear constantly. This lab makes it concrete. Instead of taking someone's word for it, you'll run benchmarks that measure throughput in lines per second, compare two implementations of the same parser (allocating vs streaming), and see the difference in a statistical benchmark report.

The deeper lesson: performance in Rust comes from control over memory layout and allocation, not from magic. When you understand *where* allocations happen and *how* to avoid them, you can write code that's 2–10x faster than a naive implementation — in the same language, with the same compiler.

## What's in This Crate

### log_parser (library)

A parser for a simple log format: `TIMESTAMP LEVEL MESSAGE`. Two implementations:

1. **`parse_log(input: &str) -> Vec<LogEntry>`** — Reads the entire input, splits into lines, parses each line, collects into a `Vec`. Simple, correct, and allocates a new `String` for every field of every entry.

2. **`parse_log_streaming<R: BufRead>(reader: R) -> impl Iterator<Item = Result<LogEntry, ParseError>>`** — Reads line by line from any buffered reader. Parses one entry at a time. The caller decides what to do with each entry (filter, count, aggregate) without holding the entire log in memory.

### parser_bench (benchmark)

Uses `criterion` to measure both implementations against a synthetic 10,000-line log. Reports wall-clock time, throughput, and statistical confidence intervals.

## What to Run

```bash
# Run the log_parser tests
cargo test -p log_parser

# Run benchmarks (this takes ~30 seconds per benchmark)
cargo bench -p parser_bench

# Check binary sizes
bash scripts/size.sh

# If you want just the benchmark names without running
cargo bench -p parser_bench -- --list
```

## What to Observe

1. **Throughput numbers.** Criterion reports something like `parse_log_allocating: 5.2 ms` and `parse_log_streaming: 3.1 ms` for 10,000 lines. The streaming version should be measurably faster because it avoids allocating a `Vec` to hold all results.

2. **Memory patterns.** The allocating version creates `n` `LogEntry` structs in a `Vec`, plus `n * 3` `String` allocations (timestamp, level, message). The streaming version reuses a single line buffer and yields entries one at a time. For a 1 GB log file, the difference between "holds everything in memory" and "processes one line at a time" is the difference between running and crashing.

3. **Criterion's statistical rigor.** Criterion runs each benchmark many times, reports the mean with confidence intervals, and detects regressions across runs. This is how real performance-sensitive Rust projects track speed — not with `time cargo run` in the terminal.

4. **Binary size.** `scripts/size.sh` builds in release mode with `strip` and reports the file size. Rust binaries include a small runtime (no GC, no VM) and are typically 1–5 MB for simple programs. Compare to a Go binary (~10 MB typical) or a Python script that needs a 50 MB interpreter.

## Key Concepts

- **Zero-cost abstractions.** Iterators, closures, and generics in Rust compile down to the same machine code as hand-written loops. The streaming parser uses `impl Iterator` — it's an abstraction, but it costs nothing at runtime.
- **Allocation awareness.** Every `String::from()`, `Vec::new()`, or `.to_string()` is a heap allocation. In hot paths, reducing allocations is the single biggest performance lever. Rust makes allocations explicit — there's no hidden boxing or string interning.
- **`&str` vs `String`.** `&str` is a borrowed reference to text that already exists somewhere. `String` is an owned, heap-allocated buffer. Using `&str` in parse results means you're pointing into the original input, not copying it. This is zero-copy parsing.
- **`BufRead` trait.** Any type that implements `BufRead` can be used as input to the streaming parser — files, stdin, network sockets, in-memory buffers. The parser doesn't care where the bytes come from.

## Why criterion?

`criterion` is the standard Rust benchmarking library. It's the only new dependency in this phase, justified by the fact that it provides statistical analysis (mean, median, confidence intervals, regression detection), HTML reports with plots, and stable benchmarks without nightly Rust (unlike the built-in `#[bench]` feature).

It's the tool you'd actually use in a production Rust project to track performance.

## Optimization Ideas to Try

After running the baseline benchmarks, try these modifications and re-bench:

1. **Use `&str` instead of `String` in `LogEntry`.** Change the struct to borrow from the input instead of owning strings. This eliminates per-field allocations. (Requires lifetime annotations — a good exercise.)
2. **Pre-allocate the `Vec`.** If you know the approximate line count, use `Vec::with_capacity()` to avoid reallocations during collection.
3. **Use `memchr` for line splitting.** The `memchr` crate uses SIMD instructions to find newlines faster than `str::lines()`. This is what `ripgrep` does.

Each of these should produce a measurable improvement in the criterion output.

## Next Step

→ [Phase 5: Security Tooling](05-security-tooling.md)
