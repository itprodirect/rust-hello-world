# 05 - Security Tooling in the Default Workflow

## Why This Phase Matters

Memory safety is foundational, but production security also requires dependency and license controls.

## Workspace Assets

- `deny.toml`
- `SECURITY.md`
- `.github/workflows/ci.yml`

## Run

```bash
cargo audit
cargo deny check
cargo deny check advisories
cargo deny check licenses
```

## Verify

- known vulnerabilities are blocked
- license policy is enforced
- checks run automatically in CI

## Reusable Baseline Gate

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo audit
cargo deny check
```

Use this sequence in every Rust repo by default.

## Contributor Note

Any new dependency should be justified in docs and validated with `cargo audit` and `cargo deny`.

## Next

[06 - FFI and WASM](06-ffi-and-wasm.md)

Last updated: 2026-02-17
