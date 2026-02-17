use std::sync::{mpsc, Arc};
use std::thread;

/// Maps values in parallel and preserves original order.
///
/// This uses a simple worker-bucket strategy and only relies on `std`.
pub fn parallel_map<T, U, F>(items: Vec<T>, num_threads: usize, map_fn: F) -> Vec<U>
where
    T: Send + 'static,
    U: Send + 'static,
    F: Fn(T) -> U + Send + Sync + 'static,
{
    let len = items.len();
    if len == 0 {
        return Vec::new();
    }

    let worker_count = num_threads.max(1).min(len);
    let mut buckets: Vec<Vec<(usize, T)>> = (0..worker_count).map(|_| Vec::new()).collect();
    for (index, item) in items.into_iter().enumerate() {
        buckets[index % worker_count].push((index, item));
    }

    let map_fn = Arc::new(map_fn);
    let (tx, rx) = mpsc::channel::<(usize, U)>();
    let mut handles = Vec::with_capacity(worker_count);

    for bucket in buckets {
        if bucket.is_empty() {
            continue;
        }
        let tx = tx.clone();
        let map_fn = Arc::clone(&map_fn);
        handles.push(thread::spawn(move || {
            for (index, item) in bucket {
                let mapped = map_fn(item);
                tx.send((index, mapped)).expect("result receiver dropped");
            }
        }));
    }
    drop(tx);

    let mut slots: Vec<Option<U>> = std::iter::repeat_with(|| None).take(len).collect();
    for (index, value) in rx {
        slots[index] = Some(value);
    }

    for handle in handles {
        handle.join().expect("worker thread panicked");
    }

    slots
        .into_iter()
        .map(|slot| slot.expect("worker did not produce value"))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn preserves_input_order() {
        let input = vec![5, 1, 3, 9, 2];
        let output = parallel_map(input, 3, |n| n * 10);
        assert_eq!(output, vec![50, 10, 30, 90, 20]);
    }

    #[test]
    fn supports_owned_non_copy_types() {
        let input = vec!["a".to_string(), "bb".to_string(), "ccc".to_string()];
        let output = parallel_map(input, 2, |s| s.len());
        assert_eq!(output, vec![1, 2, 3]);
    }

    #[test]
    fn handles_empty_input() {
        let input: Vec<u64> = Vec::new();
        let output = parallel_map(input, 4, |n| n * n);
        assert!(output.is_empty());
    }

    #[test]
    fn matches_sequential_for_multiple_thread_counts() {
        let input: Vec<u64> = (0..2_000).collect();
        let expected: Vec<u64> = input.iter().map(|n| n * 2).collect();

        for threads in [0, 1, 2, 4, 16] {
            let actual = parallel_map(input.clone(), threads, |n| n * 2);
            assert_eq!(actual, expected, "thread count mismatch: {threads}");
        }
    }
}
