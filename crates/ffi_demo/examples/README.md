# Node FFI Examples

This folder includes two Node FFI call paths for `ffi_demo`.

## Recommended Default: `koffi`

Use this first. It is simpler to get running on modern Node versions.

```bash
cargo build --release -p ffi_demo
npm install koffi
node crates/ffi_demo/examples/call_from_node_koffi.js
```

## Advanced Option: `ffi-napi` (Node 20 Recommended)

`ffi-napi` may fail to install on some Node 22+ Windows setups due to
native build toolchain issues.

If you want to keep parity with `ffi-napi`, use Node 20:

```bash
# nvm-windows example
nvm install 20.19.0
nvm use 20.19.0

cargo build --release -p ffi_demo
npm install ffi-napi
node crates/ffi_demo/examples/call_from_node.js
```

You can also use the pinned version in this folder:

```bash
nvm use
```

Alternative version managers:

```bash
# fnm
fnm install 20
fnm use 20

# volta
volta install node@20
```

## Output Check

Both scripts should print:

- `add(20, 22) = 42`
- `fibonacci(40) = 102334155`

Last updated: 2026-02-17
