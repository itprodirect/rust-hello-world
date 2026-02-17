/**
 * Advanced Node.js FFI example using ffi-napi.
 *
 * Prefer `call_from_node_koffi.js` as the default path.
 * `ffi-napi` can require native build tooling and is typically more reliable
 * on Node 20 than on newer major versions in some Windows environments.
 *
 * Usage:
 * 1) Build shared library:
 *    cargo build --release -p ffi_demo
 * 2) Use Node 20 (recommended for ffi-napi):
 *    nvm install 20.19.0
 *    nvm use 20.19.0
 * 3) Install dependency and run:
 *    npm install ffi-napi
 *    node crates/ffi_demo/examples/call_from_node.js
 */

const ffi = require("ffi-napi");
const path = require("path");
const os = require("os");
const nodeMajor = Number(process.versions.node.split(".")[0]);

if (nodeMajor >= 22) {
  console.warn(
    "Warning: ffi-napi installation may fail on Node >= 22 in some setups. " +
      "If this fails, use call_from_node_koffi.js or switch to Node 20."
  );
}

function sharedLibName() {
  switch (os.platform()) {
    case "win32":
      return "ffi_demo.dll";
    case "darwin":
      return "libffi_demo.dylib";
    default:
      return "libffi_demo.so";
  }
}

const repoRoot = path.resolve(__dirname, "..", "..", "..");
const libPath = path.join(repoRoot, "target", "release", sharedLibName());

const lib = ffi.Library(libPath, {
  add: ["int", ["int", "int"]],
  fibonacci: ["uint64", ["uint32"]],
});

console.log(`Loaded: ${libPath}`);
console.log(`add(20, 22) = ${lib.add(20, 22)}`);
console.log(`fibonacci(40) = ${lib.fibonacci(40)}`);
