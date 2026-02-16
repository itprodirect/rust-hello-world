# 01 — Hello CLI + Library Split

## Why This Matters

Most Rust projects should split logic from interface on day one. The pattern: put pure functions in a library crate, put CLI/UI/IO in a binary crate. This gives you testable logic that doesn't depend on stdin/stdout, reusable code that other crates can import, and a thin binary that's easy to replace (swap CLI for a web handler, for example).

This is the same "hexagonal architecture" pattern used in Go, Java, and other ecosystems — Rust just makes it natural because Cargo workspaces give you first-class multi-crate support with zero configuration overhead.

## What's in These Crates

### hello_lib (library)

Pure functions with no I/O:

- `greet(name)` — builds a greeting string
- `parse_name(input)` — validates and trims a name, returns `Result`
- `version()` — returns the crate version from Cargo metadata

Every function has doc comments with examples, and every function has unit tests covering both the happy path and edge cases (empty input, whitespace, unicode).

### hello_cli (binary)

A thin CLI wrapper that:

- Parses `--name <value>` using `clap` (derive API)
- Falls back to the `USER` (Unix) or `USERNAME` (Windows) environment variable
- Calls `hello_lib::greet()` and prints the result
- Exits with code 1 if the name is invalid

## What to Run

```bash
# Run with an explicit name
cargo run -p hello_cli -- --name "Nick"

# Run without a name (uses your system username)
cargo run -p hello_cli

# Run with an empty name (should exit with error)
cargo run -p hello_cli -- --name ""

# Run tests for the library
cargo test -p hello_lib

# Run tests with output visible
cargo test -p hello_lib -- --nocapture

# Check the help text
cargo run -p hello_cli -- --help

# Build a release binary and check its size
cargo build --release -p hello_cli
ls -lh target/release/hello_cli
```

## What to Observe

1. **Crate boundary.** `hello_lib` has zero dependencies on clap or any I/O. You could use it from a web server, a GUI, or another CLI without changing a line.

2. **Testing without I/O.** The unit tests in `hello_lib` don't need to capture stdout or mock anything. They call functions and check return values. This is fast, reliable, and easy to maintain.

3. **Error handling.** `parse_name` returns a `Result`, not a `String`. The CLI decides what to do with the error (print a message, exit with code 1). The library doesn't make that decision — it just reports the problem.

4. **Binary size.** A release build of `hello_cli` should be roughly 1–3 MB depending on your platform. That's a fully static binary with argument parsing, help text, and version info. Compare that to a Node.js or Python equivalent that needs a runtime.

5. **Doc comments.** Run `cargo doc --open -p hello_lib` to see the generated documentation. Rust's doc system compiles and runs the examples in your doc comments as tests — they can't go stale.

## Why clap?

`clap` is the de facto CLI argument parser in the Rust ecosystem. We use it here because it demonstrates how derive macros reduce boilerplate, it gives us `--help`, `--version`, and error messages for free, and it's the crate you'll actually use in production Rust CLIs.

It's the only external dependency in Phase 1. Everything else is standard library.

## Key Concepts

- **Cargo workspace:** One `Cargo.toml` at the root, `members = ["crates/*"]`, shared `target/` directory and lockfile.
- **Path dependencies:** `hello_cli` depends on `hello_lib` via `hello_lib = { path = "../hello_lib" }` in its `Cargo.toml`.
- **`Result` as the default:** Even simple functions should return `Result` if they can fail. This forces callers to handle errors explicitly.
- **Doc tests:** Examples in `///` comments are compiled and executed by `cargo test`. They're documentation that can't lie.

## Next Step

→ [Phase 2: Error Handling](02-error-handling.md)
