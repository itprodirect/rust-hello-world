# Security Policy

## Reporting Vulnerabilities

If you discover a security issue in this repository, report it responsibly.

For issues in this repository's code:
- Open a GitHub issue, or
- contact the maintainer directly.

For issues in dependencies:
- Check the [RustSec Advisory Database](https://rustsec.org/advisories/).
- If not listed, report to the affected crate maintainers.

## Security Tooling

This repository uses:

- `cargo audit`: vulnerability scanning against RustSec.
- `cargo deny`: policy checks for advisories, licenses, and sources.
- `cargo clippy`: static analysis with warnings treated as errors in CI.
- `cargo fmt`: consistent formatting to improve code review clarity.

## CI Enforcement

Every push and pull request runs:

1. `cargo fmt --all -- --check`
2. `cargo clippy --workspace --all-targets -- -D warnings`
3. `cargo test --workspace`
4. `cargo audit`
5. `cargo deny check`
6. `bash scripts/check_phase6.sh`

A failure in any step blocks the merge.

## Dependency Policy

- Prefer standard library features over external crates when practical.
- Justify each external dependency in code or docs.
- Avoid wildcard dependency versions (`*`).
- Keep advisories and license policies enforced through `deny.toml`.
- Run `cargo update` periodically to pick up patched dependencies.

Last updated: 2026-02-17
