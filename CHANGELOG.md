# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/), and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [Unreleased]
### Added
- Added repository-level `AGENTS.md` guidance for coding agents, including AI contribution expectations, required quality gates, and PR reporting expectations.
- Added `docs/REPO_MAP.md` with a quick repository map, extension points, and "if changing X, also check Y" guidance.
- Added machine-readable repository policy file `ai-policy.yaml` for required checks, change obligations, and PR section requirements.
- Added default pull request template at `.github/pull_request_template.md` with required sections (`Summary`, `Scope`, `Compatibility`, `Validation`, `Benchmarks`, `Changelog`).
- Added CI policy script `.github/scripts/ai_policy_check.sh` to validate PR policy expectations.

### Changed
- Updated `.github/workflows/rust.yml` to add explicit stable-toolchain CI quality gate steps for `cargo fmt --all -- --check`, `cargo clippy --all-targets --all-features -- -D warnings`, `cargo test --all-features`, and `cargo doc --no-deps`, with each gate logged as its own named step.
- Updated `README.md` contributing guidance to list the exact local quality-gate commands that CI runs.
- Updated `CONTRIBUTING.md` to include a matching `Local quality gates` section with the exact CI command set.
- Added `AI_FRIENDLY_ROADMAP.md` with module API surface, error-handling conventions, validation/testing expectations, and an ML/feature-engineering roadmap.
- Updated `.github/copilot-instructions.md` to reflect the current module layout, remove stale `standard_indicators.rs` references, and avoid outdated warning/timing claims.
- Removed the temporary docs consistency checker (`scripts/check_docs_consistency.py`) and its CI workflow step.
- Updated contributor guidance links in `CONTRIBUTING.md` and `AGENTS.md` so referenced local files exist.
- Expanded `AGENTS.md` with change-scope discipline, backward compatibility rules, performance-sensitive path guidance, and a required PR summary format.
- Reworked `AI_FRIENDLY_ROADMAP.md` into `Now`/`Next`/`Later` priorities, split into contributor-workflow and library-feature roadmaps, and added milestone acceptance criteria and non-goals.
- Updated `.github/workflows/rust.yml` to run a policy job before build/test.
- CI now fails PRs that modify `src/*.rs` without updating `CHANGELOG.md`.
- CI now fails PRs with detected public API (`pub`) changes when the PR body does not include a `Compatibility` section.
- Polished and standardized `CONTRIBUTING.md` style and wording while preserving contribution requirements and aligning language with the CI policy checks.

- Removed the deprecation marker from `volume_price_trend` in both the bulk and single APIs.
- Expanded `CONTRIBUTING.md` with a dedicated AI-assisted contribution checklist, explicit pre-PR validation gates, benchmark regression expectations, and links to AI contribution guidance docs.
- Clarified contributor expectations to update `CHANGELOG.md` for each user-facing change and to validate benchmark impact using the companion benchmark repository.


## [1.2.0] - 2026-02-25
### Added
- Reference URLs to doc strings

### Changed
- Sorted the `## Included Indicators` Bulk and Single lists alphabetically in all module docstrings (`candle_indicators`, `momentum_indicators`, `other_indicators`, `strength_indicators`, `trend_indicators`)

### Deprecated
- Deprecated functions in the `momentum_indicators` module that were just wrappers for moving averages (`signal_line`, slow and slowest stochastics). These functions will be removed in the next major release (2.0.0) to encourage users to call the moving average functions directly for better flexibility and clarity.
- Deprecated the Volume price trend  
- Deprecated the volatility system 


## [1.0.0] - 2026-01-07
### Changed
- **BREAKING:** Rebranded from RustTI to Centaur Technical Indicators
  - Package name changed from `rust_ti` to `centaur_technical_indicators`
  - This is a new package on crates.io with fresh versioning (1.0.0)
  - All functionality remains the same, only branding has changed
  - Updated repository and documentation URLs to reflect Centaur Technologies branding
- **BREAKING:** `panic!` replaced with `Result<>` types in several functions for better error handling

### Removed
- Removed unused `deviation.rs` file

---

## Everything below this line is from RustTI changelog

## [2.2.0] - 2025-10-19
### Added
- Added new deviation indicators:
  - log_standard_deviation
  - student_t_adjusted_std
  - laplace_std_equivalent
  - cauchy_iqr_scale
- AbsDevConfig and DeviationAggregate to allow caller to specify which aggregate to use for absolute deviation calculations

### Changed
- Updated DeviationModel to include new deviation types, and CustomAbsoluteDeviation that allows caller to specify which central point and aggregate to use
- absolute_deviation now uses AbsDevConfig to allow caller to specify which aggregate to use

## [2.1.5] - 2025-10-07
### Added
- Added new indicator: Price distribution

### Changed
- Minor document updates
- `break_down_trends` made more reliable and easier to use 
  - Added a config struct to hold parameters
  - Fixed internal logic to be more robust

## [2.1.4] - 2025-08-07
### Changes
- Minor document updates

### Fixes
- Fixed Welles' Volatility System, in some edge cases it would try to make an immediate pivot after establishing a SaR, which caused a crash. It has been updated to try for an extra period to confirm trend direction

---

## [2.1.3] - 2025-08-04
### Changes
- Minor document updates
- Made directional movement system error message clearer

---

## [2.1.2] - 2025-07-27
### Changed
- Minor document updates

---

## [2.1.1] - 2025-07-22
### Fixed
- Chaikin Oscillator was taking the first Accumulation Distribution instead of the last

### Changed
- Minor doc updates

---

## [2.1.0] - 2025-07-20
### Added
- Added benchmarks to README
- Added tutorials to README

### Changed
- Removed unused loop from valleys
- Inlined functions to improve runtime

---

## [2.0.0] - 2025-07-03
### Added
- Expanded and improved documentation for core modules, including comprehensive doc comments and usage examples for `basic_indicators`, `candle_indicators`, `chart_trends`, and `correlation_indicators`.
- Additional inline documentation and usage instructions in the README.md and CONTRIBUTING.md files, clarifying usage philosophy and adding mascot introduction.
- New doc tests and panic handling for invalid period lengths and other edge cases in indicator functions.

### Changed
- Major refactor of argument signatures: Many functions (especially in `basic_indicators`, `chart_trends`, `correlation_indicators`) now take plain values (e.g., period: usize) instead of references (e.g., &usize).
- Improved error handling and panic messages across all indicator modules for consistency and clarity.
- Numerous functions now use iterators and more idiomatic Rust for windowed calculations and internal logic.
- Refined and clarified module-level and function-level documentation throughout the codebase.
- Refactored custom type handling to use more idiomatic Rust enums and structures.
- Updated tests across modules to cover new error handling and edge cases.

### Removed
- Deprecated legacy argument patterns (e.g., passing reference to period) across most modules for a cleaner API.
- Removed repetitive or redundant docstrings in favor of more centralized, clearer documentation
- Removed main and visa from examples to fall in line with diataxis, clearer tutorials and how tos will be put in another repo

---

## [1.4.2] - 2024-06-27
### Added
- Improved `peaks` and `valleys` function: now avoids producing peaks/valleys when the period shifted and was within a given period of the previous one.

### Changed
- Documentation updates for several indicators.

---

## [1.4.1] - 2024-05-10
### Fixed
- Fixed bug in exponential moving average calculation.
- Minor code formatting improvements.

---

## [1.4.0] - 2024-04-01
### Added
- New indicator: McGinley Dynamic Bands.
- Added configuration options for moving averages.
- Added S&P 500 and Visa usage examples.

### Changed
- Refactored indicator modules for improved organization.

### Fixed
- Calculation bug in RSI fixed.
- Typo corrections in documentation.

---

## [1.3.0] - 2023-12-20
### Added
- Support for more than 70 unique technical indicators.
- Personalised moving average type.
- Bulk and single calculation modes for all indicators.
- Improved error handling for invalid input.

### Changed
- Major refactor of moving average module for flexibility.

---

## [1.2.0] - 2023-07-15
### Added
- Candle indicators: Ichimoku Cloud, McGinley Dynamic Bands/Envelopes, Moving Constant Bands, Donchian Channels, Keltner Channel, Supertrend.
- Chart trend indicators: breakdown, peaks, valleys, trend detection.
- Correlation and momentum indicators (Chaikin Oscillator, MACD, etc).

---

## [1.1.0] - 2023-03-30
### Added
- Standard indicators: Simple, Smoothed, Exponential Moving Averages, Bollinger Bands, MACD, RSI.
- Basic statistical indicators: mean, median, mode, standard deviation, variance, min, max, etc.

---

## [1.0.0] - 2023-01-10
### Added
- Initial release of RustTI.
- Core library structure with modular technical indicator functions.
- Full documentation on docs.rs.
- Unit tests and hand-calculation verification spreadsheets.

---
