#!/usr/bin/env bash
# Build release binaries and report their stripped sizes.
set -euo pipefail

echo "Building release binaries..."
cargo build --release -p hello_cli 2>&1

echo ""
echo "Binary sizes (release, not stripped):"
echo "--------------------------------------"

if [[ -f target/release/hello_cli.exe ]]; then
    # Windows
    ls -lh target/release/hello_cli.exe | awk '{print $5 "\t" $NF}'
elif [[ -f target/release/hello_cli ]]; then
    # Unix — strip and report
    strip target/release/hello_cli 2>/dev/null || true
    ls -lh target/release/hello_cli | awk '{print $5 "\t" $NF}'
fi

echo ""
echo "Done."
