# 03 — Concurrency Without Fear

## Why This Matters

Concurrency bugs — data races, deadlocks, torn reads — are some of the hardest bugs to find, reproduce, and fix. In C/C++, they're undefined behavior. In Python, the GIL hides most of them (but limits real parallelism). In Go, the race detector catches some at runtime, but only if you hit the right timing.

Rust eliminates data races at compile time. The ownership and borrowing rules mean that if your code compiles, it's free of data races. Not "probably free" — provably free. This is Rust's most famous selling point, and this lab makes it tangible.

## What's in This Crate

### concurrency_demo

A program that processes a collection of numbers in three ways:

1. **Sequential** — Single-threaded loop. Baseline for comparison.
2. **Threaded with channels** — Spawns worker threads, each processes a chunk, sends results back through an `mpsc` channel.
3. **Shared state with `Arc<Mutex<T>>`** — Multiple threads increment a shared counter protected by a mutex wrapped in an atomic reference count.

The program also includes a **commented-out "bad pattern"** — code that attempts to share mutable state across threads without proper synchronization. This code doesn't compile, and the comments explain exactly which compiler error you'd get and why.

## What to Run

```bash
# Run the demo (prints timing for all approaches)
cargo run -p concurrency_demo

# Run in release mode for more accurate timing
cargo run -p concurrency_demo --release

# Run tests
cargo test -p concurrency_demo

# Try uncommenting the "bad pattern" and see the compiler error
# (instructions are in the source code)
```

## What to Observe

1. **Timing comparison.** The concurrent version should be faster on multi-core machines. The ratio depends on your CPU and the workload. With a heavy enough computation per item, you should see close to linear speedup with the number of cores.

2. **Compiler as safety net.** Look at the commented-out code. In C, that code compiles and runs — sometimes correctly, sometimes not, depending on thread scheduling. In Rust, it's a compile error. The compiler tells you exactly what's wrong: you're trying to move a value into multiple threads, or you're trying to mutate shared state without a lock.

3. **Channel semantics.** The `mpsc` (multi-producer, single-consumer) channel is how threads communicate without sharing memory. Each thread gets its own copy of the data (or a reference-counted handle), does its work, and sends the result back. No locks needed.

4. **`Arc<Mutex<T>>` is explicit.** In Go, you might forget to lock a mutex. In Rust, you literally cannot access the data inside a `Mutex` without calling `.lock()`. The type system makes the locking visible and mandatory.

## Key Concepts

- **Ownership across threads.** When you spawn a thread with `std::thread::spawn`, you must *move* data into the closure. This transfers ownership, ensuring the spawning thread can't modify the data while the new thread uses it.
- **`Send` and `Sync` traits.** Rust has marker traits that determine what can cross thread boundaries. `Send` means a type can be moved to another thread. `Sync` means a type can be shared (by reference) across threads. The compiler checks these automatically.
- **`mpsc` channels.** `std::sync::mpsc` provides a typed channel. The sender can be cloned for multiple producers. The receiver is single-consumer. This is the same message-passing pattern as Go channels, but with compile-time type safety.
- **`Arc<Mutex<T>>`** — `Arc` (atomic reference counting) lets multiple threads hold a handle to the same data. `Mutex` ensures only one thread accesses the inner data at a time. Together, they give you safe shared mutable state.

## The "Bad Pattern" — What the Compiler Catches

The source code includes something like this (commented out):

```rust
// let mut shared_vec = vec![0; 100];
// let handles: Vec<_> = (0..4).map(|i| {
//     std::thread::spawn(|| {
//         shared_vec[i * 25..(i + 1) * 25].fill(1);  // DATA RACE
//     })
// }).collect();
```

This won't compile. The error is something like: "closure may outlive the current function, but it borrows `shared_vec`, which is owned by the current function." Rust requires you to either move ownership (one thread only) or use `Arc<Mutex<T>>` / `Arc<RwLock<T>>` for shared access.

In C, this code compiles and runs. Sometimes it works. Sometimes it corrupts memory. Sometimes it segfaults. The behavior depends on the CPU, the OS scheduler, and the phase of the moon. Rust says no.

## When to Use What

- **Channels** — Best when threads do independent work and send results back. Low coupling, easy to reason about.
- **`Arc<Mutex<T>>`** — Use when you genuinely need shared mutable state (counters, caches, connection pools). Keep the critical section small.
- **`rayon`** (not in this lab) — A data-parallelism library that gives you parallel iterators. Great for "process this collection in parallel" without manual thread management. Worth adding if you continue building.
- **`tokio` / async** (not in this lab) — For I/O-bound concurrency (network servers, lots of connections). Different model from threads; covered partially in the http_server if you extend this repo.

## Next Step

→ [Phase 4: Performance](04-performance.md)
