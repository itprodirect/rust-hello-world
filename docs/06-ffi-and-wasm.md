# 06 — Interop Superpower: FFI & WASM

## Why This Matters

You don't have to rewrite everything in Rust. The most practical way to adopt Rust is as a **safe, fast module** inside an existing ecosystem. Write the performance-critical or security-sensitive piece in Rust, then call it from Python, Node.js, or the browser.

This is how Rust gets adopted in the real world: a Python data pipeline calls a Rust parser for the hot loop. A Node.js backend delegates crypto to a Rust library. A web app loads a Rust WASM module for client-side computation. The Rust piece is small, fast, and safe — the rest stays in whatever language the team already knows.

This lab demonstrates both paths: FFI (Foreign Function Interface) for calling Rust from native code, and WASM (WebAssembly) for running Rust in the browser.

## What's in These Crates

### ffi_demo (shared library)

A Rust library compiled to a C-compatible shared library (`.so` on Linux, `.dylib` on macOS, `.dll` on Windows). Exposes functions through the C ABI:

- `add(a: i32, b: i32) -> i32` — Simple function to verify the FFI bridge works.
- `fibonacci(n: u32) -> u64` — A non-trivial computation to benchmark against Python's native implementation.

Includes example scripts:
- `examples/call_from_python.py` — Uses Python's `ctypes` to load and call the Rust library. No pip packages needed.
- `examples/call_from_node.js` — Uses `ffi-napi` to call the same library from Node.js.

### wasm_demo (WebAssembly module)

A Rust library compiled to WebAssembly using `wasm-bindgen`. Exposes:

- `greet(name: &str) -> String` — Returns a greeting, demonstrating string passing across the WASM boundary.
- `fibonacci(n: u32) -> u64` — Same computation as the FFI demo, now running in the browser.

Includes `examples/index.html` that loads the WASM module and calls both functions with results displayed on the page.

## What to Run

### FFI Demo

```bash
# Build the shared library
cargo build --release -p ffi_demo

# Find the output (platform-dependent)
# Linux:  target/release/libffi_demo.so
# macOS:  target/release/libffi_demo.dylib
# Windows: target/release/ffi_demo.dll

# Run from Python
python3 examples/call_from_python.py

# Run from Node.js (requires ffi-napi: npm install ffi-napi)
node examples/call_from_node.js
```

### WASM Demo

```bash
# Install wasm-pack (one-time)
cargo install wasm-pack

# Build the WASM module
cd crates/wasm_demo
wasm-pack build --target web

# Check the output size
ls -lh pkg/wasm_demo_bg.wasm

# Serve the example page (any static server works)
cd examples
python3 -m http.server 8080
# Open http://localhost:8080 in your browser
```

## What to Observe

1. **FFI is zero-overhead.** The Rust function is compiled to native machine code. Calling it from Python via `ctypes` has minimal overhead — just the cost of crossing the language boundary (nanoseconds). Compare `fibonacci(40)` in pure Python vs the Rust FFI call: expect 10–100x speedup.

2. **WASM binary size.** The `.wasm` file for simple functions should be 20–200 KB. That's a complete, sandboxed execution environment delivered to the browser. Compare to shipping a JavaScript bundle with equivalent functionality.

3. **String passing is the hard part.** Simple types (integers, floats) cross the FFI/WASM boundary trivially. Strings require marshaling — the Rust side and the calling side need to agree on memory layout, encoding, and ownership. `wasm-bindgen` handles this automatically for WASM. For C FFI, you need `CStr`/`CString` conversions.

4. **No runtime needed.** The Rust shared library doesn't need a Rust runtime installed on the target machine. The WASM module runs in any browser. This is what makes Rust a practical choice for distributing performance-critical code.

## Key Concepts

- **`extern "C"`** — Tells Rust to use the C calling convention. This makes the function callable from any language that can call C functions (which is almost every language).
- **`#[no_mangle]`** — Prevents Rust from renaming the function in the compiled output. Without this, Python/Node wouldn't be able to find the function by name.
- **`crate-type = ["cdylib"]`** — Tells Cargo to produce a C-compatible dynamic library instead of a Rust-native `.rlib`.
- **`wasm-bindgen`** — A Rust crate and CLI tool that generates the JavaScript glue code for calling Rust WASM functions. It handles type conversion, memory management, and module loading.
- **`wasm-pack`** — A build tool that wraps `cargo build --target wasm32-unknown-unknown` and runs `wasm-bindgen` to produce a ready-to-use npm package or ES module.

## Safety at the Boundary

FFI is one of the few places where Rust's safety guarantees weaken. The `extern "C"` functions are `unsafe` from the caller's perspective — Rust can't verify what Python or Node does with the returned values. Keep FFI surfaces small and simple, validate inputs on the Rust side, and avoid exposing raw pointers unless absolutely necessary.

WASM is safer because the runtime (browser or wasmtime) enforces memory isolation. The WASM module can't access the host's memory, and the host can't corrupt the module's memory. This sandboxing is a security feature.

## When to Use Which

- **FFI** — When you need maximum performance and the caller is a native process (Python, Ruby, C, Go). Best for compute-heavy functions, crypto, parsing, compression.
- **WASM** — When you need to run in the browser, or when you want sandboxed execution on the server (e.g., plugin systems). Best for client-side computation, portable modules, edge computing.
- **Both** — Some projects ship the same Rust core as both a native library (for server-side Python) and a WASM module (for the browser). The core logic is identical; only the build target changes.

## What Comes Next

This is the final phase of the structured labs. From here, you might extend the repo with an `http_server` crate (Axum or Hyper for async Rust networking), add `rayon` for data-parallel benchmarks in Phase 4, explore `tokio` for async concurrency as a complement to Phase 3, or add `cargo fuzz` to the log_parser from Phase 4.

The workspace structure supports all of this — just add a new crate under `crates/` and wire it into the workspace `Cargo.toml`.
