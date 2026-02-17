# 04 - Performance You Can Measure

## Why This Phase Matters

Performance work is easier when APIs make allocation behavior obvious.

## Crates in This Phase

- `log_parser`
- `parser_bench`

## Run

```bash
cargo test -p log_parser
cargo bench -p parser_bench
```

## Verify

- `parse_line` handles realistic whitespace edge cases correctly.
- streaming parser and allocating parser produce equivalent results.
- criterion output shows time/throughput differences you can reason about.

## Reusable Patterns

- robust token parsing with typed errors (`parse_line`)
- streaming iterator parsing (`parse_log_streaming`)
- deterministic synthetic input generation (`generate_log`)

Use them in:
- ingestion services
- ETL pipelines
- CLI analytics tools

## Tests Worth Reading

- `parse_line_handles_multiple_spaces_between_fields`
- `parse_line_handles_tabs_between_fields`
- `streaming_matches_allocating`

## Contributor Note

If parser behavior changes, update both allocating and streaming tests to keep contracts aligned.

## Next

[05 - Security Tooling](05-security-tooling.md)

Last updated: 2026-02-17
