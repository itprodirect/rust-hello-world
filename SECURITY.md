# Security Policy

## Reporting a Vulnerability

If you discover a security vulnerability in this repository, please report it responsibly.

**For vulnerabilities in this repo's code:** Open a GitHub issue or email the maintainer directly. This is a learning repo, so public disclosure is acceptable.

**For vulnerabilities in dependencies:** Check if the issue is already tracked in the [RustSec Advisory Database](https://rustsec.org/advisories/). If not, report it to the affected crate's maintainers.

## Tools Used

This repository uses the following tools to maintain supply-chain security:

- **`cargo audit`** — Scans `Cargo.lock` against the RustSec Advisory Database for known vulnerabilities in dependencies.
- **`cargo deny`** — Enforces policies on dependency advisories, licenses, and sources. Configuration is in `deny.toml`.
- **`cargo clippy`** — Static analysis that catches potential security issues (unchecked arithmetic, suspicious `unsafe` usage, missing error handling). Warnings are treated as errors in CI.
- **`cargo fmt`** — Consistent formatting reduces the chance of logic errors hiding in unusual code layout.

## CI Enforcement

Every push and pull request runs:
1. `cargo fmt --check`
2. `cargo clippy -- -D warnings`
3. `cargo test --workspace`
4. `cargo audit`
5. `cargo deny check`

A failure in any step blocks the merge.

## Dependency Policy

- Prefer the Rust standard library over external crates when practical.
- Every external dependency must be justified in a code comment.
- No wildcard version requirements (`*`).
- Copyleft licenses (GPL, AGPL, LGPL) are flagged by `cargo deny`.
- `cargo update` should be run periodically to pull patched versions.
