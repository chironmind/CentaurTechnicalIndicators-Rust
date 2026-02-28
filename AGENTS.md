# AGENTS.md

Guidance for coding agents working in this repository.

## Scope
This file applies to the entire repository.
This is an open source repository; all contributions should be suitable for public review and distribution.

## Core contribution requirements
When implementing or modifying Rust code:

1. Use shared validation helpers from `src/validation.rs` for input/argument checks.
2. Return structured `TechnicalIndicatorError` values where applicable.
3. Document new public APIs (including behavior and parameter expectations).
4. Add or adjust tests in the same module as the change.
5. Do not introduce deprecated API usage in new examples or tests.
6. Add an entry for every user-facing change in `CHANGELOG.md`.

## Pre-PR quality gates (must pass)
Run these before opening a PR:

1. `cargo fmt --all -- --check` (no formatting diffs)
2. `cargo clippy --all-targets --all-features -- -D warnings` (zero warnings/errors)
3. `cargo test --all-features` (all tests pass)
4. `cargo doc --no-deps` (docs build successfully, including new public APIs)
5. Benchmark checks in the companion repo (no performance regressions for affected indicators):
   - Clone/check out `https://github.com/chironmind/CentaurTechnicalIndicators-Rust-Benchmarks`
   - Run the relevant benchmark suite(s) and summarize regressions/improvements in the PR

## Docs to review before coding
- `.github/copilot-instructions.md`
- `AI_FRIENDLY_ROADMAP.md`
- `CONTRIBUTING.md`

## PR expectations for agents
- Keep PRs focused and minimal.
- Summarize what the agent changed and what was manually verified.
- Include command output summary for the required quality gates.
- Explicitly note the `CHANGELOG.md` entry and benchmark results from the companion benchmark repo.
