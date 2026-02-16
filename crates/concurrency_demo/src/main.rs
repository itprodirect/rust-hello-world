use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Instant;

// ---------------------------------------------------------------------------
// Workload: a deliberately expensive per-item computation so timing is visible
// ---------------------------------------------------------------------------

/// Simulate a CPU-bound task: count prime factors (trial division).
fn expensive_work(n: u64) -> u64 {
    if n < 2 {
        return 0;
    }
    let mut count = 0u64;
    let mut remaining = n;
    let mut divisor = 2u64;
    while divisor * divisor <= remaining {
        while remaining.is_multiple_of(divisor) {
            count += 1;
            remaining /= divisor;
        }
        divisor += 1;
    }
    if remaining > 1 {
        count += 1;
    }
    count
}

// ---------------------------------------------------------------------------
// Sequential baseline
// ---------------------------------------------------------------------------

/// Process every item on the current thread and return the sum of results.
pub fn sequential(data: &[u64]) -> u64 {
    data.iter().map(|&n| expensive_work(n)).sum()
}

// ---------------------------------------------------------------------------
// Concurrent: threads + mpsc channels
// ---------------------------------------------------------------------------

/// Split `data` into chunks, process each chunk on its own thread, and
/// collect partial sums through an `mpsc` channel.
pub fn concurrent_channels(data: &[u64], num_threads: usize) -> u64 {
    let chunk_size = data.len().div_ceil(num_threads);
    let (tx, rx) = mpsc::channel();

    let mut handles = Vec::new();
    for chunk in data.chunks(chunk_size) {
        let chunk = chunk.to_vec(); // owned copy moved into the thread
        let tx = tx.clone();
        handles.push(thread::spawn(move || {
            let partial: u64 = chunk.iter().map(|&n| expensive_work(n)).sum();
            tx.send(partial).expect("receiver dropped");
        }));
    }

    // Drop the original sender so `rx` knows when all senders are done.
    drop(tx);

    let total: u64 = rx.iter().sum();

    for h in handles {
        h.join().expect("thread panicked");
    }

    total
}

// ---------------------------------------------------------------------------
// Shared state: Arc<Mutex<T>>
// ---------------------------------------------------------------------------

/// Each thread increments a shared counter. Demonstrates that you cannot
/// access the inner value without calling `.lock()` — the type system
/// makes locking mandatory.
pub fn shared_counter(iterations_per_thread: u64, num_threads: usize) -> u64 {
    let counter = Arc::new(Mutex::new(0u64));
    let mut handles = Vec::new();

    for _ in 0..num_threads {
        let counter = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            for _ in 0..iterations_per_thread {
                let mut lock = counter.lock().expect("mutex poisoned");
                *lock += 1;
            }
        }));
    }

    for h in handles {
        h.join().expect("thread panicked");
    }

    let final_value = *counter.lock().expect("mutex poisoned");
    final_value
}

// ---------------------------------------------------------------------------
// Bad pattern — WOULD NOT COMPILE (uncomment to see the error)
// ---------------------------------------------------------------------------

// The code below attempts to mutate a Vec from multiple threads without
// synchronization. Rust refuses to compile it:
//
//   error[E0373]: closure may outlive the current function, but it borrows
//                 `shared_vec`, which is owned by the current function
//
// Even if you add `move`, each thread would try to take ownership of the
// entire Vec, which is also rejected:
//
//   error[E0382]: use of moved value: `shared_vec`
//
// In C/C++, this compiles fine and produces undefined behavior — the
// result depends on thread scheduling, cache coherence, and luck.
// In Rust, the compiler catches it before a single instruction runs.
//
// fn bad_pattern() {
//     let mut shared_vec = vec![0u64; 100];
//     let handles: Vec<_> = (0..4)
//         .map(|i| {
//             std::thread::spawn(|| {
//                 // DATA RACE: multiple threads writing to the same Vec
//                 // without any synchronization.
//                 shared_vec[i * 25..(i + 1) * 25].fill(1);
//             })
//         })
//         .collect();
//     for h in handles {
//         h.join().unwrap();
//     }
//     println!("shared_vec sum: {}", shared_vec.iter().sum::<u64>());
// }

// ---------------------------------------------------------------------------
// main
// ---------------------------------------------------------------------------

fn main() {
    let num_threads = thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(4);

    // Generate test data: large numbers for meaningful CPU work per item
    let data: Vec<u64> = (1_000_000_000..1_000_002_000).collect();

    println!("Items: {}  |  Threads: {}", data.len(), num_threads);
    println!("{:-<50}", "");

    // --- Sequential ---
    let start = Instant::now();
    let seq_result = sequential(&data);
    let seq_time = start.elapsed();
    println!("Sequential:   sum = {seq_result:>10}  |  {seq_time:>10.2?}");

    // --- Concurrent (channels) ---
    let start = Instant::now();
    let con_result = concurrent_channels(&data, num_threads);
    let con_time = start.elapsed();
    println!("Concurrent:   sum = {con_result:>10}  |  {con_time:>10.2?}");

    assert_eq!(seq_result, con_result, "results must match");

    let speedup = seq_time.as_secs_f64() / con_time.as_secs_f64();
    println!("Speedup:      {speedup:.2}x");

    println!("{:-<50}", "");

    // --- Shared counter ---
    let iters = 10_000u64;
    let start = Instant::now();
    let count = shared_counter(iters, num_threads);
    let counter_time = start.elapsed();
    println!(
        "Shared counter: {count} (expected {})  |  {counter_time:>10.2?}",
        iters * num_threads as u64
    );
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_SIZE: usize = 1000;

    fn test_data() -> Vec<u64> {
        (100_000..100_000 + TEST_SIZE as u64).collect()
    }

    #[test]
    fn sequential_and_concurrent_produce_same_result() {
        let data = test_data();
        let seq = sequential(&data);
        let con = concurrent_channels(&data, 4);
        assert_eq!(seq, con);
    }

    #[test]
    fn concurrent_with_different_thread_counts() {
        let data = test_data();
        let baseline = sequential(&data);
        for threads in [1, 2, 4, 8] {
            assert_eq!(
                concurrent_channels(&data, threads),
                baseline,
                "mismatch with {threads} threads"
            );
        }
    }

    #[test]
    fn shared_counter_has_expected_value() {
        let iters = 1_000u64;
        let threads = 4;
        let result = shared_counter(iters, threads);
        assert_eq!(result, iters * threads as u64);
    }

    #[test]
    fn shared_counter_stress() {
        // Higher contention: many threads, many iterations
        let iters = 5_000u64;
        let threads = 8;
        let result = shared_counter(iters, threads);
        assert_eq!(result, iters * threads as u64);
    }

    #[test]
    fn expensive_work_known_values() {
        // 1 has 0 prime factors
        assert_eq!(expensive_work(1), 0);
        // 2 is prime → 1 factor
        assert_eq!(expensive_work(2), 1);
        // 12 = 2^2 * 3 → 3 factors
        assert_eq!(expensive_work(12), 3);
        // 100_000 = 2^5 * 5^5 → 10 factors
        assert_eq!(expensive_work(100_000), 10);
    }
}
