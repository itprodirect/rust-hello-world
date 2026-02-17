# 01 - Hello CLI + Library Split

## Why This Phase Matters

Rust projects scale better when core logic is separate from interfaces.

- Library crate: pure, testable behavior.
- Binary crate: argument parsing, environment access, output, exit codes.

## Crates in This Phase

- `hello_lib`
- `hello_cli`

## Run

```bash
cargo run -p hello_cli -- --name "Nick"
cargo run -p hello_cli
cargo run -p hello_cli -- --name ""
cargo run -p hello_cli -- --help
cargo test -p hello_lib
```

## Verify

- `hello_cli` exits non-zero on invalid names.
- `hello_lib` tests pass quickly and independently.
- Doc tests in `hello_lib` remain executable documentation.

## Reusable Pattern

`parse_name` is a strong boundary function for any user-provided identifier.

Use it in:
- CLI flags
- API payload validation
- configuration parsing

## Tests Worth Reading

- `greet_unicode_name`
- `parse_name_rejects_whitespace_only`
- `version_is_semver`

## Contributor Note

When extending this phase, keep I/O out of `hello_lib` and add tests before CLI changes.

## Next

[02 - Error Handling](02-error-handling.md)

Last updated: 2026-02-17
