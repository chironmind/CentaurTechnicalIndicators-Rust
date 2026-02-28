# AI-Friendly Roadmap

This document is a practical map for contributors and coding agents working in `CentaurTechnicalIndicators-Rust`.

## Current API surface by module

The crate currently exports the following indicator modules from `src/lib.rs`, each with `single` and/or `bulk` submodules where applicable:

- `basic_indicators`: descriptive statistics and utility math helpers.
- `candle_indicators`: candle-derived indicators (bands/channels/envelopes and related tools).
- `chart_trends`: peak/valley and trend-structure analysis.
- `correlation_indicators`: pairwise/statistical relationship indicators.
- `momentum_indicators`: momentum and oscillator families.
- `moving_average`: core moving-average implementations used across indicators.
- `other_indicators`: miscellaneous indicators that do not fit other categories.
- `strength_indicators`: strength/volume participation indicators.
- `trend_indicators`: trend direction/strength systems.
- `volatility_indicators`: volatility and range-expansion indicators.

Shared crate-level support modules:

- `error`: defines `TechnicalIndicatorError` and crate-wide `Result<T>`.
- `types`: shared enums/config types re-exported at crate root.
- `validation`: shared argument/input validation helpers.

## Error-handling conventions (`TechnicalIndicatorError`)

Contributors should follow these conventions for public and internal APIs:

1. **Prefer `Result<T, TechnicalIndicatorError>` for fallible operations** rather than panicking.
2. **Use shared validation helpers from `src/validation.rs`** (`assert_non_empty`, `assert_same_len`, `assert_period`, etc.) to keep error behavior consistent.
3. **Return structured variants** with meaningful fields:
   - `EmptyData { name }`
   - `MismatchedLength { names }`
   - `InvalidPeriod { period, data_len, reason }`
   - `InvalidValue { name, value, reason }`
   - `UnsupportedType { type_name }`
   - `Custom { message }`
4. **Surface user-meaningful reasons** in `reason`/`message` strings (for debugging and downstream tooling).
5. **Keep behavior stable across modules** so equal classes of invalid inputs return equal error variants.

## Testing/benchmark expectations

Before opening a PR, contributors should run and report:

1. `cargo fmt --all -- --check`
2. `cargo clippy --all-targets --all-features -- -D warnings`
3. `cargo test --all-features`
4. `cargo doc --no-deps`

Benchmark expectation for touched indicators:

- Clone/check out `https://github.com/chironmind/CentaurTechnicalIndicators-Rust-Benchmarks`.
- Run relevant benchmark suites for impacted indicators.
- Summarize regressions/improvements in the PR description.

Testing guidance:

- Add or adjust tests in the same module as the implementation change.
- Prefer deterministic tests with explicit expected values.
- Do not weaken or remove unrelated assertions to make a change pass.

## ML/feature-engineering roadmap

Potential roadmap items for ML-oriented users of this library:

1. **Feature extraction profiles**
   - Curated presets (trend/momentum/volatility/volume mixes) to reduce manual wiring.
2. **Window-safe feature generation APIs**
   - Helpers that align rolling indicators into model-ready matrices while avoiding look-ahead bias.
3. **Normalization and transform helpers**
   - Optional z-score/rank/robust scaling utilities for indicator outputs.
4. **Regime/context features**
   - Composite features (e.g., trend regime + volatility regime labels) for model conditioning.
5. **Cross-asset feature blocks**
   - Higher-level wrappers for correlation/covariance style signals across multiple instruments.
6. **Feature metadata and provenance**
   - Standardized descriptors for feature name, source inputs, lookback, and warmup requirements.

Near-term implementation preference:

- Start with non-breaking additions in utility/helper layers.
- Maintain strong error semantics and test coverage for each new feature API.
