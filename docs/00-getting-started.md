# 00 — Getting Started

## What This Repo Is

A Cargo workspace containing small, independent Rust labs ("micro-labs"). Each lab is its own crate — you can build, test, and run them independently. The goal is to make Rust's strengths visible through code you can actually execute and measure.

## Prerequisites

You need three things: Rust, Git, and a terminal.

### Install Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# Verify
rustc --version    # should be 1.78+
cargo --version
```

On Windows, download and run [rustup-init.exe](https://rustup.rs/) instead. If you're using WSL2, the `curl` command above works fine.

### Install Supporting Tools

```bash
# Linting and formatting (usually included with rustup)
rustup component add clippy
rustup component add rustfmt

# Security auditing
cargo install cargo-audit

# License + advisory checking (optional, used in Phase 5)
cargo install cargo-deny
```

### Clone the Repo

```bash
git clone https://github.com/itprodirect/rust-hello-world.git
cd rust-hello-world
```

## How the Workspace Is Organized

The root `Cargo.toml` defines a workspace. Each crate lives in `crates/` and has its own `Cargo.toml`, `src/`, and tests. You never need to `cd` into a crate — Cargo handles it from the root:

```bash
# Build everything
cargo build --workspace

# Test everything
cargo test --workspace

# Run a specific crate
cargo run -p hello_cli -- --name "world"

# Test a specific crate
cargo test -p hello_lib
```

## How to Use the Labs

Each lab has a corresponding doc in `docs/` numbered by phase. Read the doc first — it explains *why* the lab exists and *what* you should observe. Then run the code and look at the output.

The recommended order is phases 0 through 6, but you can jump ahead if a topic interests you. The only dependency chain is that `hello_cli` depends on `hello_lib`.

## Interpreting Outputs

Every lab produces at least one measurable outcome. Here's what to look for:

- **Tests:** `cargo test -p <crate>` should print all green. Read the test names — they describe the behavior being verified.
- **Timing:** Some labs print wall-clock comparisons (e.g., concurrent vs sequential). Look for the ratio, not the absolute numbers — your machine's speed will vary.
- **Benchmarks:** `cargo bench` (Phase 4+) uses criterion, which produces statistical output. Focus on the "time" column and the throughput (elements/sec or lines/sec).
- **Binary size:** `bash scripts/size.sh` reports stripped release binary sizes. Rust binaries are typically 1–5 MB for simple programs.
- **Security scans:** `cargo audit` and `cargo deny check` either pass clean or list advisories. A clean pass means your dependency tree has no known vulnerabilities.

## Troubleshooting

**`cargo build` fails with "could not find `Cargo.toml` in ... or any parent directory"**
You're not in the repo root. `cd rust-hello-world` first.

**`cargo clippy` warns about something**
Good — that's the point. Read the warning, fix it, and re-run. Clippy catches real bugs.

**`cargo audit` reports a vulnerability**
Run `cargo update` to pull patched versions. If the advisory is in a transitive dependency, check if a newer version of your direct dependency fixes it.

**Tests fail on Windows**
Some labs use Unix-style paths or env vars. If you're on Windows without WSL, check the test for platform-specific assumptions.

## Next Step

→ [Phase 1: Hello CLI](01-hello-cli.md)
