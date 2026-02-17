#!/usr/bin/env bash
# Build release artifacts and report their sizes.
set -euo pipefail

echo "Building release artifacts..."
cargo build --release -p hello_cli -p ffi_demo 2>&1

echo ""
echo "Artifact sizes (release):"
echo "-------------------------"

report_size() {
    local path="$1"
    if [[ -f "$path" ]]; then
        ls -lh "$path" | awk '{print $5 "\t" $NF}'
    fi
}

if [[ -f target/release/hello_cli.exe ]]; then
    report_size target/release/hello_cli.exe
elif [[ -f target/release/hello_cli ]]; then
    strip target/release/hello_cli 2>/dev/null || true
    report_size target/release/hello_cli
fi

report_size target/release/ffi_demo.dll
report_size target/release/libffi_demo.so
report_size target/release/libffi_demo.dylib

if [[ -f crates/wasm_demo/pkg/wasm_demo_bg.wasm ]]; then
    report_size crates/wasm_demo/pkg/wasm_demo_bg.wasm
fi

echo ""
echo "Done."
