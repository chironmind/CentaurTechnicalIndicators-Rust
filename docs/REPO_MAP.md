# Repository Map

Quick orientation for contributors and coding agents working in `CentaurTechnicalIndicators-Rust`.

## Top-level layout

- `src/`: crate source files (all indicator and shared support modules).
- `examples/`: usage examples.
- `assets/`: supporting artifacts (including hand-calculation spreadsheet guidance from `CONTRIBUTING.md`).
- `.github/workflows/rust.yml`: baseline CI build/test workflow.
- `CHANGELOG.md`: required entry point for every user-facing change.
- `AGENTS.md`: agent operating rules and PR/reporting expectations.
- `docs/AI_ONBOARDING.md`: canonical start-here checklist for coding agents.
- `AI_FRIENDLY_ROADMAP.md`: contributor-workflow and feature roadmap.
- `CONTRIBUTING.md`: contributor expectations and required validation gates.

## Source module map (`src/`)

- `lib.rs`: crate exports and module wiring.
- `error.rs`: `TechnicalIndicatorError` and crate-level `Result<T>`.
- `types.rs`: shared enums/config types re-exported by crate root.
- `validation.rs`: shared input/argument validation helpers.
- `basic_indicators.rs`: descriptive statistics and utility math helpers.
- `candle_indicators.rs`: candle/band/channel/envelope-style indicators.
- `chart_trends.rs`: peak/valley and trend-structure analysis.
- `correlation_indicators.rs`: pairwise/statistical relationship indicators.
- `momentum_indicators.rs`: momentum/oscillator indicators.
- `moving_average.rs`: moving average implementations used across indicators.
- `other_indicators.rs`: miscellaneous indicators.
- `strength_indicators.rs`: strength/participation indicators.
- `trend_indicators.rs`: trend direction/strength indicators.
- `volatility_indicators.rs`: volatility/range-expansion indicators.

## Extension points

- New indicator implementation: add to the appropriate `src/*_indicators.rs` file (or create a new module only when categorization requires it).
- Shared validation logic: add/reuse helpers in `src/validation.rs`.
- Shared error semantics: use/extend `TechnicalIndicatorError` in `src/error.rs` only when needed and with compatibility care.
- Public exports: wire through `src/lib.rs`.
- Public type/config additions: place in `src/types.rs` when shared across modules.

## If changing X, also check Y

- If changing indicator inputs/parameter checks:
  - Also check `src/validation.rs` for helper reuse/consistency.
- If changing error behavior:
  - Also check `src/error.rs` and existing module tests for variant consistency.
- If adding/changing public APIs (`pub` items):
  - Also update docs/comments and confirm crate exports in `src/lib.rs`.
  - Also add a user-facing entry in `CHANGELOG.md`.
- If changing indicator math/outputs:
  - Also update/add tests in the same source module.
  - Also run relevant benchmark suites from the companion benchmark repo.
- If adding new user-visible behavior:
  - Also update `README.md` and/or examples when appropriate.
  - Also update `CHANGELOG.md`.

## Required local validation gates

Run these before opening a PR:

1. `cargo fmt --all -- --check`
2. `cargo clippy --all-targets --all-features -- -D warnings`
3. `cargo test --all-features`
4. `cargo doc --no-deps`
5. Relevant benchmark suite(s) in `CentaurTechnicalIndicators-Rust-Benchmarks`

## Minimal PR content checklist

- What changed and why.
- Compatibility/user-impact notes.
- Validation command summary.
- Benchmark summary for touched indicators.
- Explicit `CHANGELOG.md` entry note.
