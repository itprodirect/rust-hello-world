# 05 — Security Toolchain as Part of Dev

## Why This Matters

Memory safety is Rust's headline security feature, but it's not the whole story. Modern software security also means knowing what's in your dependency tree, checking for known vulnerabilities before they hit production, and enforcing license compliance so your legal team doesn't get surprised.

Rust's ecosystem has first-class tooling for all of this. Unlike npm's `audit` (which reports thousands of transitive issues you can't easily fix) or Python's fragmented landscape, Rust's supply-chain tools are designed to be part of your CI pipeline, not an afterthought.

This phase adds security checks to the repo and makes them part of every build.

## What's in This Phase

No new crate — this phase adds tooling and configuration at the workspace level.

### cargo audit

Checks your `Cargo.lock` against the RustSec Advisory Database. If any dependency (direct or transitive) has a known vulnerability, it reports the advisory ID, severity, and affected versions.

### cargo deny

A more comprehensive policy tool that checks advisories (same database as `cargo audit`, plus custom policies), licenses (flag copyleft, require approval for specific licenses), bans (block specific crates or versions), and sources (ensure crates only come from crates.io, not git repos or unknown registries).

### deny.toml

The configuration file for `cargo deny`. It lives at the workspace root and defines your policies. Each section has comments explaining what it does and why.

### CI Integration

Both tools run in the GitHub Actions pipeline. A failure in either one blocks the PR.

## What to Run

```bash
# Run a vulnerability scan
cargo audit

# Run the full policy check
cargo deny check

# Check just advisories
cargo deny check advisories

# Check just licenses
cargo deny check licenses

# See what licenses are in your dependency tree
cargo deny list
```

## What to Observe

1. **Clean audit results.** If `cargo audit` reports nothing, your dependency tree has no known vulnerabilities. That's the goal — and it should stay that way because CI runs this on every push.

2. **License inventory.** `cargo deny list` shows every license in your dependency tree. For this repo, you should see mostly MIT and Apache-2.0. If a copyleft license (GPL, AGPL) appears, `cargo deny check licenses` will flag it.

3. **Policy as code.** The `deny.toml` file is version-controlled. Anyone reviewing the repo can see exactly what security and license policies are enforced. This is auditable and reproducible — no "I think we checked that once" conversations.

4. **CI enforcement.** Check the GitHub Actions tab after pushing. The security steps should be green. If someone adds a vulnerable dependency, the build breaks before it merges.

## Key Concepts

- **Supply-chain security.** Your code is only as secure as your dependencies. A single compromised crate can inject malicious code into your binary. `cargo audit` and `cargo deny` are your first line of defense.
- **Advisory databases.** RustSec (rustsec.org) is a curated database of security advisories for Rust crates. It's community-maintained and integrated into Cargo tooling.
- **License compliance.** Open-source licenses have real legal implications. Copyleft licenses (GPL) can require you to open-source your own code. `cargo deny` catches these before they become a problem.
- **Clippy as security tool.** `cargo clippy -- -D warnings` catches more than style issues. It flags potential security problems: unchecked arithmetic, suspicious use of `unsafe`, missing error handling, and more. Treating warnings as errors means these issues block the build.

## SECURITY.md

The repo includes a `SECURITY.md` at the root. This is a standard file that tells users and contributors how to report vulnerabilities, what tools the project uses for security, and what the expected response process is.

GitHub recognizes this file and surfaces it in the "Security" tab of the repository.

## What If a Vulnerability Appears?

1. `cargo audit` identifies the advisory, affected crate, and patched version.
2. Run `cargo update -p <affected-crate>` to pull the fix.
3. If no fix exists yet, check if the vulnerability affects your usage. `cargo deny` can be configured to allow specific advisories temporarily with a note explaining why.
4. Re-run `cargo audit` to confirm the fix.

## Optional: Fuzzing

If you want to go further, `cargo fuzz` lets you fuzz-test your parsers. Fuzzing throws random/malformed input at your code to find panics, crashes, and unexpected behavior. It's particularly valuable for the `log_parser` crate from Phase 4.

```bash
# Install cargo-fuzz (requires nightly)
cargo install cargo-fuzz

# Initialize fuzzing for log_parser
cd crates/log_parser
cargo fuzz init
cargo fuzz add parse_line

# Run the fuzzer (ctrl+c to stop)
cargo +nightly fuzz run parse_line
```

This is optional because it requires nightly Rust and runs indefinitely, but it's how production Rust projects find edge cases that unit tests miss.

## Next Step

→ [Phase 6: FFI & WASM](06-ffi-and-wasm.md)
