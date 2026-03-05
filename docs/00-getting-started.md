# 00 - Getting Started

## Goal

Get the workspace running locally, then pick a phase and move with a tight feedback loop.

## Prerequisites

- Rust stable (`rustup`, `cargo`, `rustc`)
- Git
- Terminal (PowerShell, Bash, or zsh)

Recommended tooling:

```bash
rustup component add clippy rustfmt
cargo install cargo-audit
cargo install cargo-deny
```

## First Run

```bash
cargo build --workspace
cargo test --workspace
cargo run -p hello_cli -- --name "world"
```

## Learning Workflow

1. Read one phase doc.
2. Run tests for only that crate.
3. Run the binary/bench/example.
4. Copy one reusable pattern.
5. Make one change and re-run tests.

## Crate Map

- `hello_lib`: pure functions and doc tests.
- `hello_cli`: thin CLI over library code.
- `error_demo`: typed error flow.
- `concurrency_demo`: std-thread concurrency patterns.
- `log_parser`: robust parsing and streaming.
- `ffi_demo`: native ABI interop exports.
- `wasm_demo`: web-target interop exports.
- `power_blocks`: reusable production snippets.

## Useful Commands

```bash
# Focused test runs
cargo test -p hello_lib
cargo test -p log_parser
cargo test -p power_blocks

# Full quality gates
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace

# Security and benchmarking
cargo audit
cargo deny check
cargo bench -p parser_bench

# Interop regression checks
bash scripts/check_phase6.sh
powershell -NoProfile -File scripts/check_phase6.ps1
```

## Release Process

For repeatable tagged releases, use:

- [09 - Release Process](09-release-process.md)

## Interop Troubleshooting

1. `Cannot find module 'ffi-napi'`
- Install or use recommended path:
```bash
npm install koffi
node crates/ffi_demo/examples/call_from_node_koffi.js
```

2. `npm install ffi-napi` fails with `node-gyp` / `MSBuild` / `libffi`
- Use Node 20 for the advanced ffi-napi path:
```bash
nvm install 20.19.0
nvm use 20.19.0
npm install ffi-napi
```

3. `wasm-pack: command not found`
- Install wasm-pack:
```bash
cargo install wasm-pack
```

4. `failed to create temp dir for cargo install wasm-bindgen` / `Access is denied`
- Run in a terminal with write access to temp/cache directories.
- On Windows, use:
```bash
powershell -NoProfile -File scripts/check_phase6.ps1
```

5. FFI library not found (`ffi_demo.dll` / `libffi_demo.so` / `libffi_demo.dylib`)
- Build release artifact first:
```bash
cargo build --release -p ffi_demo
```

6. WASM page loads but prints nothing
- Serve over HTTP, do not open `index.html` directly:
```bash
cd crates/wasm_demo/examples
python -m http.server 8080
```

## Next

[01 - Hello CLI](01-hello-cli.md)

Last updated: 2026-03-05


