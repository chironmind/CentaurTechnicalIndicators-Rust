# Centaur Technical Indicators Copilot Instructions

## Repository Overview

**Centaur Technical Indicators** is a Rust technical indicators library for financial data analysis. It provides 70+ configurable indicators across category-focused modules.

**Key Characteristics:**
- **Type**: Library crate
- **Crate name**: `centaur_technical_indicators`
- **Language**: Rust edition 2021
- **Current crate version**: `1.2.0`
- **Authors**: `ChironMind`
- **License**: MIT
- **Dependencies**: No external runtime dependencies in `[dependencies]`

## Build Instructions

### Prerequisites
- Rust toolchain (stable recommended; CI also checks beta and nightly)

### Essential Commands (run in repository root)

```bash
cargo check
cargo build
cargo test
cargo clippy --all-targets --all-features
cargo fmt --check
cargo doc --no-deps
```

### Examples and Optional Benchmark Setup

```bash
cargo run --example reference
cargo bench --no-run
```

### Expected Build Behavior
- `cargo check`, `cargo build`, `cargo test`, and `cargo doc --no-deps` should succeed.
- Current warning output includes deprecated API usage warnings, an `unused_assignments` warning in `src/chart_trends.rs` (`end_index`), and many clippy warnings in tests (including `unused_must_use`).
- Warnings are currently present in the repository baseline; do not assume warning-free output.
- `cargo clippy --all-targets --all-features` is expected to succeed with warnings at the moment.

### Testing Notes
- `cargo test` currently runs both unit/integration-style module tests and doc tests.
- Hand-calculation verification spreadsheet: `assets/centaur_ti_hand_calcs.ods`.
- Do not modify tests only to force unrelated changes to pass.

## Project Architecture and Layout

### Module Structure

```
src/
├── lib.rs                    # Main library entry point with module exports
├── basic_indicators.rs       # Core statistical functions
├── candle_indicators.rs      # Candle/price channel and band indicators
├── chart_trends.rs           # Trend and peak/valley analysis
├── correlation_indicators.rs # Asset correlation metrics
├── error.rs                  # TechnicalIndicatorError and Result alias
├── momentum_indicators.rs    # Momentum and oscillator indicators
├── moving_average.rs         # Moving average models
├── other_indicators.rs       # Miscellaneous indicators/utilities
├── strength_indicators.rs    # Strength/volume-style indicators
├── trend_indicators.rs       # Trend-focused indicators
├── types.rs                  # Shared enums/configuration types
├── validation.rs             # Centralized validation helper functions
└── volatility_indicators.rs  # Volatility indicators
```

### Key Design Patterns
- **Dual calculation APIs**: Many modules expose `single` and `bulk` functions.
- **Shared configuration enums**: Centralized in `types.rs` and re-exported at crate root.
- **Result-based API (preferred)**: Most calculation APIs return `Result<T, TechnicalIndicatorError>` via the crate `Result<T>` alias. Use existing module patterns when extending behavior.
- **Central validation utilities**: Common precondition checks are implemented in `src/validation.rs` and return `TechnicalIndicatorError` variants defined in `src/error.rs`.

### Configuration Files
- **Cargo.toml**: package metadata, example registration, and dependency configuration
- **.github/workflows/rust.yml**: CI build and test workflow
- **.gitignore**: repository ignore rules

### GitHub Actions CI/CD
CI runs on pushes/PRs to main and validates with stable, beta, and nightly Rust toolchains.

## Development Guidelines

### Code Organization
- Use indicator category modules to locate implementations.
- Check `types.rs`, `error.rs`, and `validation.rs` before introducing new parameter validation or error behavior.
- Keep naming consistent with existing module conventions.

### Common File Locations
- **Examples**: `examples/reference.rs`
- **Tests**: inline module tests (`#[cfg(test)]`) in source files
- **Assets**: `assets/`

### Validation Steps
1. `cargo check`
2. `cargo test`
3. `cargo clippy --all-targets --all-features`
4. `cargo run --example reference`
5. `cargo doc --no-deps`

### Performance Notes
- Do not rely on fixed local run-time expectations in this document.
- Command durations vary by machine, profile, and cache state.
- Use dedicated benchmark workflows/repositories for repeatable performance claims.

## Common Tasks

### Adding New Indicators
1. Choose the correct module by indicator category.
2. Implement API behavior consistent with nearby indicators (`single`/`bulk` as applicable).
3. Validate inputs with helper functions in `validation.rs` (for example `assert_non_empty`, `assert_same_len`, `assert_period`) and return `TechnicalIndicatorError` variants from `error.rs`.
4. Add/update tests with hand-calculated expectations.
5. Add or update docs/examples as needed.

### Modifying Existing Indicators
1. Review indicator math and existing tests.
2. Preserve public API compatibility unless intentionally making a breaking change.
3. Keep error behavior consistent with existing `Result<T, TechnicalIndicatorError>` patterns.
4. Re-run the validation commands above.

### Debugging Build Issues
- **Import/export issues**: verify module exports in `src/lib.rs`
- **Type/config errors**: verify enum/type usage in `src/types.rs`
- **Validation errors**: inspect helper usage in `src/validation.rs` and error variants in `src/error.rs`
- **Test failures**: run `cargo test -- --nocapture` for detailed output

## Additional Context
- `README.md` - project overview and usage
- `CONTRIBUTING.md` - contribution workflow
- [Centaur Technical Indicators Tutorials](https://github.com/ChironMind/CentaurTechnicalIndicators-Rust-tutorials)
- [Centaur Technical Indicators Benchmarks](https://github.com/ChironMind/CentaurTechnicalIndicators-Rust-benchmarks)
