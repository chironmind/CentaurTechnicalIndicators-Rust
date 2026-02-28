# Centaur Technical Indicators Copilot Instructions

## Repository Overview

Centaur Technical Indicators is a Rust library crate for technical analysis indicators. It is dependency-light and organized by indicator families.

## Build and validation commands

Run commands from repository root:

```bash
cargo check
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-features
cargo doc --no-deps
```

Notes:

- Do not assume fixed command timings; runtime varies by machine and cache state.
- Do not assume warnings are acceptable in CI; clippy is run with `-D warnings`.

## Module layout

```text
src/
├── lib.rs
├── types.rs
├── error.rs
├── validation.rs
├── basic_indicators.rs
├── candle_indicators.rs
├── chart_trends.rs
├── correlation_indicators.rs
├── momentum_indicators.rs
├── moving_average.rs
├── other_indicators.rs
├── strength_indicators.rs
├── trend_indicators.rs
└── volatility_indicators.rs
```

## Development expectations

- Reuse validation helpers in `src/validation.rs`.
- Prefer structured `TechnicalIndicatorError` returns for invalid inputs.
- Keep tests close to changed implementation modules.
- Update `CHANGELOG.md` for user-facing changes.

## References

- `README.md`
- `CONTRIBUTING.md`
- `AGENTS.md`
- `AI_FRIENDLY_ROADMAP.md`
- [Centaur Technical Indicators Tutorials](https://github.com/ChironMind/CentaurTechnicalIndicators-Rust-tutorials)
- [Centaur Technical Indicators Benchmarks](https://github.com/chironmind/CentaurTechnicalIndicators-Rust-Benchmarks)
