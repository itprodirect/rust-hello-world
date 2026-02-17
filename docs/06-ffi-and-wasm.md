# 06 - Interop: FFI and WASM

## Why This Phase Matters

Rust adoption is most practical when you move high-value modules first.

This phase demonstrates:
- native-process interop (C ABI)
- browser/runtime interop (WebAssembly)

## Crates in This Phase

- `ffi_demo`: exports `add` and `fibonacci` through `extern "C"`.
- `wasm_demo`: exports `greet` and `fibonacci` through `wasm-bindgen`.

## Run: FFI

```bash
cargo build --release -p ffi_demo

# Python path
python crates/ffi_demo/examples/call_from_python.py

# Node primary path (recommended)
npm install koffi
node crates/ffi_demo/examples/call_from_node_koffi.js

# Node advanced parity path (Node 20 recommended)
nvm install 20.19.0
nvm use 20.19.0
npm install ffi-napi
node crates/ffi_demo/examples/call_from_node.js
```

Native library output:
- Linux: `target/release/libffi_demo.so`
- macOS: `target/release/libffi_demo.dylib`
- Windows: `target/release/ffi_demo.dll`

## Run: WASM

```bash
cargo install wasm-pack
cd crates/wasm_demo
wasm-pack build --target web

cd examples
python -m http.server 8080
# open http://localhost:8080
```

## Fast Regression Check

```bash
bash scripts/check_phase6.sh
powershell -NoProfile -File scripts/check_phase6.ps1
```

## Verify

- FFI function outputs match expectations (`add(20,22)=42`, `fibonacci(40)=102334155`).
- WASM package output exists at `crates/wasm_demo/pkg/`.
- Phase 6 scripts pass after interop edits.

## Reusable Assets

- `crates/ffi_demo/src/lib.rs`
- `crates/ffi_demo/examples/call_from_python.py`
- `crates/ffi_demo/examples/call_from_node_koffi.js`
- `crates/ffi_demo/examples/call_from_node.js`
- `crates/ffi_demo/examples/README.md`
- `crates/wasm_demo/src/lib.rs`

## Contributor Note

Run `scripts/check_phase6` after any interop change to catch regressions early.

## Next

[07 - Reusable Power Blocks](07-reusable-power-blocks.md)

Last updated: 2026-02-17
