/**
 * Primary Node.js FFI example using koffi.
 *
 * This is the recommended default for this repo because it works cleanly on
 * current Node versions and avoids most native build friction.
 *
 * Usage:
 * 1) Build shared library:
 *    cargo build --release -p ffi_demo
 * 2) Install dependency once:
 *    npm install koffi
 * 3) Run:
 *    node crates/ffi_demo/examples/call_from_node_koffi.js
 */

const path = require("path");
const os = require("os");
const koffi = require("koffi");

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

const lib = koffi.load(libPath);
const add = lib.func("int add(int a, int b)");
const fibonacci = lib.func("uint64 fibonacci(uint32 n)");

console.log(`Loaded: ${libPath}`);
console.log(`add(20, 22) = ${add(20, 22)}`);
console.log(`fibonacci(40) = ${fibonacci(40)}`);
