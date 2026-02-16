# 02 — Error Handling That Doesn't Rot

## Why This Matters

In Python, you get exceptions that can surprise you at runtime. In Go, you get `if err != nil` repeated hundreds of times. In C, you get integer return codes and hope someone checks them.

Rust takes a different approach: errors are values, encoded in the type system, and the compiler forces you to handle them. You can't accidentally ignore an error — the code won't compile. This means error-handling logic doesn't "rot" over time. When you add a new error variant, the compiler tells you every place that needs updating.

This lab shows the pattern you'll use in real Rust projects: custom error types with `thiserror`, the `?` operator for clean propagation, and tests that verify every failure mode.

## What's in This Crate

### error_demo

A small program that reads a config file (`key=value` format), parses it, and validates the values. The interesting part is how errors flow:

1. **Custom error enum** — `AppError` with variants for I/O errors, parse errors, and validation errors. Each variant carries context (which field failed, what the message is).

2. **Error propagation with `?`** — Functions return `Result<T, AppError>`. The `?` operator converts and propagates errors up the call stack with zero boilerplate.

3. **No panics** — There is no `.unwrap()` or `.expect()` in non-test code. Every fallible operation returns a `Result`.

## What to Run

```bash
# Run with a valid config file
echo -e "host=localhost\nport=8080" > /tmp/test.conf
cargo run -p error_demo -- /tmp/test.conf

# Run with a missing file (should show IoError)
cargo run -p error_demo -- /tmp/nonexistent.conf

# Run with an invalid port (should show ValidationError)
echo -e "host=localhost\nport=notanumber" > /tmp/bad.conf
cargo run -p error_demo -- /tmp/bad.conf

# Run with a missing required field (should show ValidationError)
echo -e "port=8080" > /tmp/incomplete.conf
cargo run -p error_demo -- /tmp/incomplete.conf

# Run all tests
cargo test -p error_demo

# Run tests with output
cargo test -p error_demo -- --nocapture
```

## What to Observe

1. **Error messages are useful.** Each error variant produces a human-readable message via the `Display` impl that `thiserror` generates. Compare this to Go's `fmt.Errorf("something went wrong: %w", err)` — Rust makes the structure explicit.

2. **The `?` operator is clean.** Look at `read_config()` — it calls functions that can fail with different error types, and `?` converts them automatically via `From` impls. No `if err != nil` chains.

3. **Tests cover failure modes.** The test suite doesn't just test the happy path. It tests each error variant explicitly using `assert!(matches!(...))`. This is how you prove your error handling works.

4. **No `.unwrap()` outside tests.** In test code, `.unwrap()` is fine because a panic *is* a test failure. In library/binary code, every `Result` is handled. Clippy enforces this.

## thiserror vs anyhow

You'll see two popular error crates in the Rust ecosystem:

- **`thiserror`** — Generates `Display` and `From` impls for your error enums. Best for libraries and code that other people will call, because the error types are explicit and matchable.
- **`anyhow`** — Provides a catch-all `anyhow::Error` type that can hold any error. Best for application-level code (like `main()`) where you just want to print the error and exit.

This lab uses `thiserror` because it teaches the pattern you need for real libraries. In a production app, your `main()` might use `anyhow` at the top level while your internal crates use `thiserror`.

## Key Concepts

- **Errors as values.** `Result<T, E>` is just an enum: `Ok(T)` or `Err(E)`. No exceptions, no hidden control flow.
- **The `?` operator.** Syntactic sugar for "if this is an error, convert it and return it." Equivalent to a try/catch that only catches to re-throw, but enforced at compile time.
- **`From` conversions.** `thiserror`'s `#[from]` attribute generates `From<std::io::Error> for AppError`, which is what makes `?` work across error types.
- **Exhaustive matching.** If you `match` on an error enum and add a new variant later, the compiler tells you every match that needs updating. Errors can't silently fall through.

## Common Mistakes to Avoid

- **Using `String` as your error type.** It compiles, but callers can't match on it or handle specific errors differently. Use an enum.
- **`.unwrap()` in library code.** It panics, which is a crash. Return `Result` instead and let the caller decide.
- **Catching errors too early.** Let errors propagate with `?` until you reach a point where you can actually do something useful (log it, show it to the user, retry).
- **Giant error enums.** Each crate should have its own error type. Don't create one global error enum for the whole project.

## Next Step

→ [Phase 3: Concurrency](03-concurrency.md)
