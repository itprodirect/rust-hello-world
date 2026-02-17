//! C ABI-friendly functions that can be called from Python/Node/C.
//!
//! Build this crate in release mode to produce a shared library:
//! - Linux: `target/release/libffi_demo.so`
//! - macOS: `target/release/libffi_demo.dylib`
//! - Windows: `target/release/ffi_demo.dll`

/// Adds two integers with saturating behavior.
///
/// Saturation keeps behavior deterministic across debug/release builds and
/// across callers from other languages.
#[no_mangle]
pub extern "C" fn add(a: i32, b: i32) -> i32 {
    a.saturating_add(b)
}

/// Computes the `n`th Fibonacci number using an iterative loop.
///
/// The implementation is `O(n)` time and `O(1)` space, and uses saturating
/// arithmetic to avoid panic-on-overflow in debug builds.
#[no_mangle]
pub extern "C" fn fibonacci(n: u32) -> u64 {
    fibonacci_impl(n)
}

fn fibonacci_impl(n: u32) -> u64 {
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }

    let mut prev = 0u64;
    let mut curr = 1u64;
    for _ in 2..=n {
        let next = prev.saturating_add(curr);
        prev = curr;
        curr = next;
    }
    curr
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_basic() {
        assert_eq!(add(2, 3), 5);
        assert_eq!(add(-10, 4), -6);
    }

    #[test]
    fn add_saturates() {
        assert_eq!(add(i32::MAX, 1), i32::MAX);
        assert_eq!(add(i32::MIN, -1), i32::MIN);
    }

    #[test]
    fn fibonacci_known_values() {
        let expected = [0u64, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55];
        for (n, value) in expected.into_iter().enumerate() {
            assert_eq!(fibonacci(n as u32), value, "failed at n={n}");
        }
    }

    #[test]
    fn fibonacci_monotonic_after_one() {
        let mut last = fibonacci(1);
        for n in 2..50 {
            let current = fibonacci(n);
            assert!(current >= last, "sequence decreased at n={n}");
            last = current;
        }
    }

    #[test]
    fn fibonacci_saturates_after_u64_limit() {
        assert_eq!(fibonacci(93), 12_200_160_415_121_876_738);
        assert_eq!(fibonacci(94), u64::MAX);
        assert_eq!(fibonacci(120), u64::MAX);
    }
}
