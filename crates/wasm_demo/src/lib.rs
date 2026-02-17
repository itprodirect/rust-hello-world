use wasm_bindgen::prelude::*;

/// Greets the provided name from Rust/WASM.
#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    format!("Hello from WASM, {name}!")
}

/// Computes the `n`th Fibonacci number.
#[wasm_bindgen]
pub fn fibonacci(n: u32) -> u64 {
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
    fn greet_returns_expected_string() {
        assert_eq!(greet("Rust"), "Hello from WASM, Rust!");
    }

    #[test]
    fn fibonacci_known_values() {
        assert_eq!(fibonacci(0), 0);
        assert_eq!(fibonacci(1), 1);
        assert_eq!(fibonacci(10), 55);
        assert_eq!(fibonacci(20), 6765);
    }

    #[test]
    fn fibonacci_monotonic_after_one() {
        let mut last = fibonacci(1);
        for n in 2..40 {
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
