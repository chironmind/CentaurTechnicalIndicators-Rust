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

## Change scope discipline
- Keep changes minimal and focused on the requested task.
- Do not include opportunistic refactors unless explicitly requested.
- If you identify unrelated issues, note them separately instead of bundling them into the same change.
- Preserve existing file organization and naming conventions unless the task requires a structural change.

## Backward compatibility rules
When changing public APIs (`pub` items), preserve compatibility unless the task explicitly allows a breaking change:

1. Do not silently change indicator semantics, output ordering, or warmup behavior.
2. Do not remove or rename public functions, types, enums, or fields without explicit approval.
3. Do not repurpose existing `TechnicalIndicatorError` variants in ways that break downstream matching.
4. If behavior changes are required, document them in code docs and `CHANGELOG.md` with clear migration notes.

## Performance-sensitive paths
- Treat hot-path indicator computations as performance-sensitive.
- Avoid unnecessary allocations, cloning, and intermediate buffers in computation loops.
- Prefer existing efficient utilities/patterns already used in similar modules.
- If performance could change materially, run and report benchmark results for affected indicators.

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
- `docs/REPO_MAP.md`
- `CONTRIBUTING.md`

## PR expectations for agents
- Keep PRs focused and minimal.
- Summarize what the agent changed and what was manually verified.
- Include command output summary for the required quality gates.
- Explicitly note the `CHANGELOG.md` entry and benchmark results from the companion benchmark repo.

### Required PR summary format
Use this structure in PR descriptions/comments:

1. `Summary`: what changed and why.
2. `Scope`: files/modules touched and what was intentionally not changed.
3. `Compatibility`: any user-facing behavior/API/error-handling impact.
4. `Validation`: results summary for `fmt`, `clippy`, `test`, and `doc`.
5. `Benchmarks`: affected suites and regression/improvement summary.
6. `Changelog`: exact `CHANGELOG.md` entry added/updated.
