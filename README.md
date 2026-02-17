# rust-hello-world

[![CI](https://github.com/itprodirect/rust-hello-world/actions/workflows/ci.yml/badge.svg)](https://github.com/itprodirect/rust-hello-world/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-green.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-stable-orange.svg)](https://www.rust-lang.org/)

A practical Rust workspace of runnable micro-labs and reusable building blocks.

This repo is designed for developers who want to learn Rust by shipping useful code, not by collecting syntax trivia.

## New Contributor Checklist (5 Minutes)

1. `cargo build --workspace`
2. `cargo test --workspace`
3. `cargo run -p hello_cli -- --name "world"`
4. Read `docs/README.md` and pick one phase.
5. Run that phase crate tests only (`cargo test -p <crate>`).

For interop work (Phase 6), use:

```bash
powershell -NoProfile -File scripts/check_phase6.ps1
```

## Why This Repo Works

- Every phase has executable code and tests.
- Concepts are taught through measurable outcomes.
- Reusable modules are organized as copyable "lego blocks".
- Interop paths (FFI and WASM) are first-class, not afterthoughts.

## Quick Start

```bash
# Build and test everything
cargo build --workspace
cargo test --workspace

# First runnable lab
cargo run -p hello_cli -- --name "world"

# Explore reusable blocks
cargo test -p power_blocks
```

## 5-Step Learning Loop

1. Read one phase doc in `docs/`.
2. Run that crate's tests only (`cargo test -p <crate>`).
3. Run the crate binary/bench/example and inspect output.
4. Copy one pattern into a scratch project.
5. Modify behavior and re-run tests.

This loop keeps learning practical and transferable.

## Workspace Layout

```text
rust-hello-world/
|- README.md
|- LICENSE
|- Cargo.toml
|- crates/
|  |- hello_cli/          # thin CLI over tested library logic
|  |- hello_lib/          # pure functions + unit/doc tests
|  |- error_demo/         # typed errors + propagation
|  |- concurrency_demo/   # threads, channels, shared state
|  |- log_parser/         # robust parsing + streaming iterator
|  |- ffi_demo/           # C ABI exports for Python/Node/C
|  |- wasm_demo/          # wasm-bindgen exports for browser use
|  `- power_blocks/       # reusable patterns for real projects
|- benches/parser_bench/  # criterion benchmarks
|- docs/                  # phase-by-phase guidance
`- scripts/               # utility and focused check scripts
```

## Phases at a Glance

| Phase | Crate(s) | Core Skill | Fast Validation |
| --- | --- | --- | --- |
| 1 | `hello_cli`, `hello_lib` | crate boundaries and pure functions | `cargo test -p hello_lib` |
| 2 | `error_demo` | typed error design with `?` | `cargo test -p error_demo` |
| 3 | `concurrency_demo` | safe parallelism with std | `cargo test -p concurrency_demo` |
| 4 | `log_parser`, `parser_bench` | allocation-aware performance | `cargo bench -p parser_bench` |
| 5 | workspace tooling | CI-level security checks | `cargo audit && cargo deny check` |
| 6 | `ffi_demo`, `wasm_demo` | interop with native and web targets | `powershell -NoProfile -File scripts/check_phase6.ps1` |
| 7 | `power_blocks` | reusable production-grade patterns | `cargo test -p power_blocks` |

## Best Blocks to Copy

- `crates/power_blocks/src/typed_id.rs`: domain-safe IDs.
- `crates/power_blocks/src/typestate_builder.rs`: required fields at compile time.
- `crates/power_blocks/src/retry.rs`: policy-driven retries.
- `crates/power_blocks/src/parallel_map.rs`: ordered parallel mapping with std threads.
- `crates/power_blocks/src/zero_copy.rs`: borrowed parser design.
- `crates/log_parser/src/lib.rs`: robust whitespace-delimited parsing.
- `crates/ffi_demo/src/lib.rs`: stable C ABI exports.

## Common Commands

```bash
# Quality gates
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace

# Security
cargo audit
cargo deny check

# Benchmarks
cargo bench -p parser_bench

# Interop checks (Phase 6)
bash scripts/check_phase6.sh
powershell -NoProfile -File scripts/check_phase6.ps1
```

## Node FFI Paths

- Recommended default: `koffi`
- Advanced parity path: `ffi-napi` on Node `20.19.0`

See `crates/ffi_demo/examples/README.md` for exact setup and troubleshooting.

## Session Notes

- [Session Wrap-Up (2026-02-17)](docs/08-session-wrap-up-2026-02-17.md)

## Documentation

- [Docs Index](docs/README.md)
- [00 - Getting Started](docs/00-getting-started.md)
- [01 - Hello CLI](docs/01-hello-cli.md)
- [02 - Error Handling](docs/02-error-handling.md)
- [03 - Concurrency](docs/03-concurrency.md)
- [04 - Performance](docs/04-performance.md)
- [05 - Security Tooling](docs/05-security-tooling.md)
- [06 - FFI and WASM](docs/06-ffi-and-wasm.md)
- [07 - Reusable Power Blocks](docs/07-reusable-power-blocks.md)
- [08 - Session Wrap-Up (2026-02-17)](docs/08-session-wrap-up-2026-02-17.md)

## License

MIT. See [LICENSE](LICENSE).
