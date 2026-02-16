# Claude Code Session Plan — rust-hello-world

> **Purpose:** Step-by-step prompts and checkpoints for building this repo with Claude Code.
> Each session targets one phase. Finish one before starting the next.
> Estimated total: 6–8 sessions of 30–60 minutes each.

---

## Pre-Session Setup

Before your first session, make sure these are in place:

```bash
# 1. Rust toolchain
rustup update stable
rustc --version          # confirm 1.78+
cargo --version

# 2. Clone the repo
git clone https://github.com/itprodirect/rust-hello-world.git
cd rust-hello-world

# 3. Install optional tooling (used in later phases)
cargo install cargo-audit
cargo install cargo-deny    # optional, Phase 5
rustup component add clippy
rustup component add rustfmt
```

---

## Session 0 — Bootstrap & Guardrails (30 min)

**Goal:** Empty repo → working Cargo workspace with CI, formatting, and linting enforced.

### Claude Code Prompts (in order)

```
1. "Initialize a Cargo workspace in the current directory. The workspace
    should have a `crates/` directory. Add two placeholder crates:
    `hello_cli` (binary) and `hello_lib` (library). Make sure
    `cargo build` succeeds with no warnings."

2. "Create a `rustfmt.toml` with: max_width = 100, edition = 2021,
    use_field_init_shorthand = true. Create a `clippy.toml` if needed.
    Add a `.gitignore` for Rust projects."

3. "Create `.github/workflows/ci.yml` that runs on push and PR to main.
    Steps: checkout, install stable Rust, cargo fmt --check, cargo clippy
    -- -D warnings, cargo test --workspace, cargo audit. Use ubuntu-latest."

4. "Create `docs/00-getting-started.md` from the file I'll provide."
   (Then paste or reference the file from this repo's docs/)

5. "Add a top-level README.md from the file I'll provide."
```

### Definition of Done — Phase 0

- [ ] `cargo build --workspace` compiles with zero warnings
- [ ] `cargo fmt --check` passes
- [ ] `cargo clippy -- -D warnings` passes
- [ ] `cargo test --workspace` passes (even if no tests yet)
- [ ] CI workflow file exists and is valid YAML
- [ ] `docs/00-getting-started.md` exists
- [ ] `README.md` exists with repo purpose and quick-start
- [ ] `.gitignore` covers `/target`, `Cargo.lock` (for libs), IDE files
- [ ] Commit and push: `git commit -m "phase-0: workspace bootstrap + CI"`

---

## Session 1 — Hello CLI + Library Split (45 min)

**Goal:** Two crates that demonstrate clean boundaries. Library = pure logic + tests. CLI = thin wrapper.

### Claude Code Prompts

```
1. "In `crates/hello_lib/src/lib.rs`, create these public functions:
    - `greet(name: &str) -> String` — returns a greeting
    - `parse_name(input: &str) -> Result<&str, &'static str>` — trims
      whitespace, rejects empty strings
    - `version() -> &'static str` — returns env!('CARGO_PKG_VERSION')
    Add unit tests for all three, including edge cases (empty string,
    whitespace-only, unicode names)."

2. "In `crates/hello_cli/src/main.rs`, build a CLI that:
    - Accepts a `--name` flag (use `clap` derive API, latest version)
    - Falls back to the `USER` / `USERNAME` env var if no flag given
    - Calls `hello_lib::greet()` and prints the result
    - Exits with code 1 if the name is invalid
    - Has `--version` and `--help` flags automatically via clap
    Add `hello_lib` as a path dependency and `clap` with features =
    ['derive']. Justify clap in a code comment."

3. "Add a doc comment to every public function in hello_lib explaining
    what it does and showing a usage example with `///` doc comments."

4. "Create `docs/01-hello-cli.md` from the file I'll provide."
```

### What to Measure

- `cargo test -p hello_lib` — all tests pass
- `cargo run -p hello_cli -- --name "Nick"` — prints greeting
- `cargo run -p hello_cli` — falls back to env var
- Binary size: `ls -lh target/release/hello_cli` after `cargo build --release -p hello_cli`

### Definition of Done — Phase 1

- [ ] `hello_lib` has 3+ public functions with doc comments
- [ ] `hello_lib` has 5+ unit tests covering happy path + edge cases
- [ ] `hello_cli` parses args with clap, falls back to env var
- [ ] `hello_cli` exits with code 1 on invalid input
- [ ] `cargo run -p hello_cli -- --help` works
- [ ] `cargo test --workspace` all green
- [ ] `docs/01-hello-cli.md` exists
- [ ] Commit: `git commit -m "phase-1: hello_cli + hello_lib with tests"`

---

## Session 2 — Error Handling That Doesn't Rot (45 min)

**Goal:** Replace string errors with typed errors. Show `Result` chains, `?` operator, and `thiserror`.

### Claude Code Prompts

```
1. "Create a new crate `crates/error_demo` (binary). Add it to the
    workspace. This crate should demonstrate Rust error handling by:
    - Defining a custom error enum using `thiserror` with variants:
      IoError (wraps std::io::Error), ParseError(String),
      ValidationError { field: String, message: String }
    - A function `read_config(path: &str) -> Result<Config, AppError>`
      that reads a file, parses key=value pairs, validates them
    - A function `validate_port(s: &str) -> Result<u16, AppError>`
    - Show the `?` operator propagating errors cleanly through the chain
    Justify `thiserror` in a comment. Keep `anyhow` out — explain why
    in a comment (it's better for applications, thiserror for libraries)."

2. "Add tests for error_demo that:
    - Test the happy path (valid config file)
    - Test file-not-found propagation
    - Test invalid port values
    - Test missing required fields
    - Use `assert!(matches!(result, Err(AppError::ValidationError{..})))`
    Create a `tests/` directory or inline tests, your choice."

3. "Create `docs/02-error-handling.md` from the file I'll provide."
```

### What to Measure

- All error paths tested — `cargo test -p error_demo`
- No `.unwrap()` in non-test code (clippy should catch this)
- Error messages are human-readable via `Display` impl from thiserror

### Definition of Done — Phase 2

- [ ] Custom error enum with 3+ variants
- [ ] `?` operator used throughout (no manual match-and-return)
- [ ] Zero `.unwrap()` in library/binary code
- [ ] 4+ tests covering success and each error variant
- [ ] `cargo clippy -- -D warnings` still clean
- [ ] `docs/02-error-handling.md` exists
- [ ] Commit: `git commit -m "phase-2: typed error handling with thiserror"`

---

## Session 3 — Concurrency Without Fear (45 min)

**Goal:** Show Rust's ownership model preventing data races at compile time.

### Claude Code Prompts

```
1. "Create `crates/concurrency_demo` (binary). Demonstrate:
    - Spawning threads that each process a chunk of data
    - Using `std::sync::mpsc` channels to collect results
    - Using `Arc<Mutex<T>>` for shared mutable state (a counter)
    - A commented-out block showing code that WOULD be a data race in
      C/C++ but fails to compile in Rust, with an explanation of the
      compiler error
    - Print wall-clock timing for the concurrent vs sequential version
    No external crates needed — std only."

2. "Add tests:
    - Verify the concurrent and sequential versions produce the same result
    - Verify the shared counter has the expected final value
    - Test with at least 1000 items to make timing meaningful"

3. "Create `docs/03-concurrency.md` from the file I'll provide."
```

### What to Measure

- `cargo run -p concurrency_demo` — prints timing comparison
- `cargo test -p concurrency_demo` — correctness verified
- The commented-out code should actually fail if uncommented (verify manually)

### Definition of Done — Phase 3

- [ ] Thread spawning + channel collection working
- [ ] `Arc<Mutex<T>>` shared state demo working
- [ ] Commented-out "bad pattern" with compiler error explanation
- [ ] Timing output shows concurrent vs sequential
- [ ] 3+ tests verifying correctness
- [ ] `docs/03-concurrency.md` exists
- [ ] Commit: `git commit -m "phase-3: concurrency demos with timing"`

---

## Session 4 — Performance You Can Measure (60 min)

**Goal:** Add a log parser crate with benchmarks showing optimization impact.

### Claude Code Prompts

```
1. "Create `crates/log_parser` (library). It should:
    - Define a `LogEntry` struct (timestamp, level, message)
    - Implement `parse_line(line: &str) -> Result<LogEntry, ParseError>`
    - Implement `parse_log(input: &str) -> Vec<LogEntry>` (allocating)
    - Implement `parse_log_streaming<R: BufRead>(reader: R) ->
      impl Iterator<Item = Result<LogEntry, ParseError>>` (streaming)
    - Include unit tests for both parse paths
    Keep it std-only."

2. "Create `benches/parser_bench/` as a separate crate or use the
    `[[bench]]` section. Use `criterion` for benchmarking:
    - Bench `parse_log` (allocating) vs `parse_log_streaming` (streaming)
    - Generate a 10,000-line synthetic log in the benchmark setup
    - Report throughput in lines/second
    Add `criterion` as a dev-dependency. Justify it in a comment."

3. "Create `scripts/size.sh` that builds hello_cli and log_parser in
    release mode, strips the binary, and prints the file sizes."

4. "Create `docs/04-performance.md` from the file I'll provide."
```

### What to Measure

- `cargo bench -p parser_bench` (or however the bench is wired)
- `bash scripts/size.sh` — binary sizes
- Throughput numbers from criterion output

### Definition of Done — Phase 4

- [ ] `log_parser` crate with allocating + streaming parse
- [ ] 5+ unit tests in `log_parser`
- [ ] Criterion benchmark comparing both approaches
- [ ] `scripts/size.sh` runs and prints sizes
- [ ] Benchmark output shows measurable throughput difference
- [ ] `docs/04-performance.md` exists
- [ ] Commit: `git commit -m "phase-4: log_parser + criterion benchmarks"`

---

## Session 5 — Security Toolchain as Part of Dev (30 min)

**Goal:** Bake security into the workflow, not as an afterthought.

### Claude Code Prompts

```
1. "Add a `deny.toml` at the workspace root configured for:
    - Banning known-vulnerable crates (advisories)
    - Flagging copyleft licenses (license checking)
    - Banning wildcard dependencies
    Show a comment explaining each section."

2. "Update `.github/workflows/ci.yml` to add:
    - A `cargo audit` step
    - A `cargo deny check` step (advisory + license)
    - Make clippy warnings fail the build (-D warnings)"

3. "Add a `SECURITY.md` at the repo root with:
    - How to report vulnerabilities
    - What tools this repo uses for supply-chain security
    - A note about `cargo audit` and `cargo deny`"

4. "Create `docs/05-security-tooling.md` from the file I'll provide."
```

### Definition of Done — Phase 5

- [ ] `deny.toml` exists and `cargo deny check` passes
- [ ] `cargo audit` passes (no known vulnerabilities)
- [ ] CI runs both audit and deny
- [ ] `SECURITY.md` exists
- [ ] `docs/05-security-tooling.md` exists
- [ ] Commit: `git commit -m "phase-5: security toolchain (audit + deny)"`

---

## Session 6 — Interop Superpower: FFI + WASM (60 min)

**Goal:** Show Rust as the safe, fast module inside other ecosystems.

### Claude Code Prompts

```
1. "Create `crates/ffi_demo` as a library crate with `crate-type =
    ['cdylib']`. Expose a C-compatible function:
    - `pub extern 'C' fn add(a: i32, b: i32) -> i32`
    - `pub extern 'C' fn fibonacci(n: u32) -> u64`
    Add a `cbindgen.toml` or header comment showing the C header.
    Include a Python example using `ctypes` in `examples/call_from_python.py`.
    Include a Node.js example using `ffi-napi` in `examples/call_from_node.js`."

2. "Create `crates/wasm_demo` as a library crate with `crate-type =
    ['cdylib']`. Use `wasm-bindgen` to expose:
    - A `greet(name: &str) -> String` function
    - A `fibonacci(n: u32) -> u64` function
    Add a minimal `index.html` in `examples/` that loads the WASM
    and calls both functions. Include build instructions in comments."

3. "Create `docs/06-ffi-and-wasm.md` from the file I'll provide."
```

### What to Measure

- FFI: `python3 examples/call_from_python.py` works
- WASM: `wasm-pack build --target web` produces a `.wasm` file; check size
- Compare fibonacci performance: Rust FFI vs pure Python

### Definition of Done — Phase 6

- [ ] `ffi_demo` compiles to a shared library
- [ ] Python and Node examples run successfully
- [ ] `wasm_demo` builds with `wasm-pack`
- [ ] Example HTML page loads and runs WASM functions
- [ ] `docs/06-ffi-and-wasm.md` exists
- [ ] Commit: `git commit -m "phase-6: FFI + WASM interop demos"`

---

## Post-Build Checklist

After all phases are complete:

- [ ] `cargo build --workspace` — zero warnings
- [ ] `cargo test --workspace` — all green
- [ ] `cargo fmt --check` — clean
- [ ] `cargo clippy -- -D warnings` — clean
- [ ] `cargo audit` — no vulnerabilities
- [ ] All 7 docs exist in `docs/`
- [ ] README.md has table of contents linking to all docs
- [ ] Every crate has a one-line description in its `Cargo.toml`
- [ ] Push to main and verify CI passes on GitHub Actions
- [ ] Tag release: `git tag v0.1.0 && git push --tags`

---

## Tips for Working with Claude Code

1. **One prompt per logical unit.** Don't ask for a whole phase in one prompt — break it into the numbered prompts above.
2. **Review before committing.** After each Claude Code output, read the code. Ask "explain this function" if anything is unclear.
3. **Run tests after every prompt.** Don't accumulate untested code.
4. **Use `@filename` references.** Point Claude Code at specific files when asking for edits.
5. **Commit per phase.** Each phase = one atomic commit with a descriptive message.
6. **If something breaks,** paste the error into Claude Code: `"I'm getting this error: [paste]. Fix it."`
