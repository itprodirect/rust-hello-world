# rust-hello-world

**A set of small, runnable micro-labs that make Rust's advantages obvious through measurable outcomes.**

This is not a Rust course. It's a hands-on workspace where every lab compiles, runs, and proves something — whether that's memory safety, performance, error handling, or supply-chain security. Each lab is its own crate, runnable in one command, and testable in isolation.

Built for developers coming from Python, JavaScript, Go, or C who want to understand *when and why* Rust is the right tool — and when it's overkill.

---

## Quick Start

```bash
# Install Rust (if you haven't)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Clone and build
git clone https://github.com/itprodirect/rust-hello-world.git
cd rust-hello-world
cargo build --workspace

# Run your first lab
cargo run -p hello_cli -- --name "world"

# Run all tests
cargo test --workspace
```

## Repo Structure

```
rust-hello-world/
├── README.md
├── Cargo.toml                  # workspace root
├── crates/
│   ├── hello_cli/              # CLI args, env vars, exit codes
│   ├── hello_lib/              # pure functions + unit tests
│   ├── error_demo/             # typed errors, Result chains, thiserror
│   ├── concurrency_demo/       # threads, channels, Arc<Mutex<T>>
│   ├── log_parser/             # streaming parser + allocation patterns
│   ├── ffi_demo/               # Rust called from Python/Node via C ABI
│   └── wasm_demo/              # Rust compiled to WebAssembly
├── benches/
│   └── parser_bench/           # criterion benchmarks
├── docs/
│   ├── 00-getting-started.md
│   ├── 01-hello-cli.md
│   ├── 02-error-handling.md
│   ├── 03-concurrency.md
│   ├── 04-performance.md
│   ├── 05-security-tooling.md
│   └── 06-ffi-and-wasm.md
├── scripts/
│   └── size.sh                 # binary size comparison
├── .github/workflows/ci.yml
├── deny.toml
└── SECURITY.md
```

## Labs by Phase

| Phase | Crate | What You'll Learn | What You'll Measure |
|-------|-------|-------------------|---------------------|
| 0 | *(workspace)* | Project scaffolding, CI, linting | fmt/clippy/test pass in CI |
| 1 | `hello_cli` + `hello_lib` | Crate boundaries, CLI design, testing | Binary size, test coverage |
| 2 | `error_demo` | Typed errors, `?` operator, no panics | Error path test coverage |
| 3 | `concurrency_demo` | Threads, channels, compile-time safety | Wall-clock timing: concurrent vs sequential |
| 4 | `log_parser` + bench | Streaming vs allocating, zero-copy | Throughput (lines/sec via criterion) |
| 5 | *(tooling)* | `cargo audit`, `cargo deny`, supply chain | Vulnerability + license scan results |
| 6 | `ffi_demo` + `wasm_demo` | FFI to Python/Node, WASM in browser | FFI call latency, `.wasm` binary size |

## Common Commands

```bash
# Build everything
cargo build --workspace

# Test everything
cargo test --workspace

# Test one crate
cargo test -p hello_lib

# Run one crate
cargo run -p hello_cli -- --name "Nick"

# Check formatting
cargo fmt --check

# Lint
cargo clippy -- -D warnings

# Security audit
cargo audit

# Benchmarks (after Phase 4)
cargo bench -p parser_bench

# Binary sizes (after Phase 4)
bash scripts/size.sh
```

## Where Rust Shines

- **Systems that must not crash:** Network services, embedded, OS-level code. The compiler catches entire classes of bugs (null derefs, data races, use-after-free) before your code ever runs.
- **Performance-critical paths:** Parsers, codecs, crypto, compression, real-time processing. Zero-cost abstractions mean you don't pay for what you don't use.
- **Security-sensitive code:** Memory safety without a garbage collector. Supply-chain tooling (`cargo audit`, `cargo deny`) is built into the ecosystem.
- **Interop / "safe core" strategy:** Write the hot loop or security-critical module in Rust, call it from Python/Node/C via FFI or WASM. Best of both worlds.
- **CLI tools:** Small binaries, fast startup, cross-compilation. Tools like `ripgrep`, `fd`, and `bat` are all Rust.

## Where Rust Is Overkill

- **Quick scripts and automation:** Python or Bash will get you there in 1/10th the time. Rust's compile-time checks add friction that isn't worth it for throwaway code.
- **Rapid prototyping:** If you're still figuring out *what* to build, Rust's strictness slows you down. Prototype in Python/JS, then port the critical path.
- **CRUD web apps:** If your bottleneck is database I/O and developer velocity, Go, Node, or Rails will ship faster. Rust web frameworks (Axum, Actix) are excellent but the ecosystem is smaller.
- **Data science / ML pipelines:** Python's ecosystem (NumPy, pandas, PyTorch) is unmatched. Use Rust for the hot inner loop, not the whole pipeline.
- **Teams with no Rust experience:** The learning curve is real. If the team needs to ship next week and nobody knows Rust, it's the wrong call — for now.

## Documentation

Start here: [Getting Started](docs/00-getting-started.md)

Then follow the phases in order, or jump to whatever interests you. Each doc explains *why* the lab matters, *what* to run, and *what* to look for in the output.

## CI

Every push and PR runs:
1. `cargo fmt --check` — consistent formatting
2. `cargo clippy -- -D warnings` — lint with zero tolerance
3. `cargo test --workspace` — all tests pass
4. `cargo audit` — no known vulnerabilities
5. `cargo deny check` — license and advisory compliance

## License

MIT

## Contributing

This is a learning repo. If you find a bug, have a better example, or want to add a lab — open an issue or PR. Keep labs small, tested, and measurable.
