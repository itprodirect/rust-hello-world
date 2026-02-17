# 02 - Error Handling That Stays Maintainable

## Why This Phase Matters

Rust treats errors as typed values, which makes failure paths explicit and testable.

## Crate in This Phase

- `error_demo`

## Run

```bash
cargo run -p error_demo -- path/to/config.conf
cargo test -p error_demo
```

## Verify

- Error variants are specific (`IoError`, `ParseError`, `ValidationError`).
- `?` propagation stays readable across function boundaries.
- Non-test code avoids panic-driven control flow.

## Reusable Pattern

Typed error enums with `thiserror` and `#[from]` for conversion-based propagation.

Use it in:
- file parsing tools
- service configuration loading
- API adapters where callers need structured failure handling

## Tests Worth Reading

- `file_not_found_propagates_as_io_error`
- `malformed_line_is_parse_error`
- `missing_host_is_validation_error`

## Contributor Note

If you add a new `AppError` variant, add at least one test that matches on it directly.

## Next

[03 - Concurrency](03-concurrency.md)

Last updated: 2026-02-17
