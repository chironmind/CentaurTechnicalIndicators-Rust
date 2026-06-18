# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/), and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## Changelog Conventions (Machine/Agent Friendly)
- Version headers use `## [x.y.z] - YYYY-MM-DD`; only `Unreleased` omits a date.
- Change type headers use Keep a Changelog categories: `Added`, `Changed`, `Deprecated`, `Removed`, `Fixed`, `Security`.
- Each bullet should identify concrete artifacts (`module`, `file`, `function`, or `workflow`) and avoid ambiguous wording.
- Compatibility-impact details are captured under `### Agent Notes` for each version.

---

## [Unreleased]

### Added
- Added `docs/2_0_PLAN.md` capturing the deprecated-function inventory (six momentum + one volatility function), the cross-repo impact analysis for the Python/JS bindings, the `#[non_exhaustive]` migration plan, and a nine-site NaN/inf audit classified into bug-fix / documented-behavior / 2.0-breaking-validation-change. Nothing in the plan is executed yet — it is the staging area for the 2.0 cut.
- Added a crate-level `## Errors` section to `src/lib.rs` listing every `TechnicalIndicatorError` variant with a one-line meaning. Renders in `cargo doc` and docs.rs.
- Added per-category example files: `examples/rsi.rs` (minimal RSI), `examples/bollinger.rs` (Bollinger bands via `moving_constant_bands`, plus a robust median variant), and `examples/composing_indicators.rs` (smoothed-RSI recipe showing the library's compose-don't-bundle philosophy). Wired through `[[example]]` blocks in `Cargo.toml`. The kitchen-sink `examples/reference.rs` is unchanged.
- Implemented `Display` on all public types in `src/types.rs` (`CentralPoint`, `DeviationAggregate`, `AbsDevConfig`, `MovingAverageType`, `ConstantModelType`, `DeviationModel`, `Position`). Unit variants render as `snake_case`; parameterized variants render as `name(field=value, ...)`. Lock-in tests cover every variant and pin the exact string output for any future `FromStr` to roundtrip against.
- Added `tests/properties.rs` with five property invariants, exercised over a fixed grid of input lengths and price scales by a dependency-free, deterministic LCG harness (no `proptest`; the crate keeps **zero** dependencies and dev-dependencies): RSI is bounded in `[0, 100]`; ATR is non-negative for valid OHLC; `moving_average(vec![c; n], Simple)` is the fixed point `c`; bulk MA length is `prices.len() - period + 1`; constant-input series produce no NaN for RSI, CMO, or mean (regression coverage for the bugs fixed in commits `0217469` and `dc71a22`).
- Added `tests/golden.rs` with eleven golden-value tests on the public re-export surface. Covers MA, variance, RSI, CMO, MFI, ATR, Aroon, Ichimoku, Donchian, Supertrend, and moving constant envelopes. Each value is sourced from `assets/centaur_ti_hand_calcs.ods` (mirrored in per-module unit tests).
- Added `tests/integration.rs` with five smoke tests that exercise only the crate-root `pub use` surface (`use centaur_technical_indicators::...`), catching breakage in re-export wiring even when per-module unit tests still pass.
- Declared MSRV `rust-version = "1.81"` in `Cargo.toml`.
- Added MSRV CI job (`.github/workflows/rust.yml`) that builds against pinned 1.81 toolchain via native `rustup`. Job runs `cargo build` (library only) and `cargo test --no-run` (so the test targets are also compiled against the MSRV toolchain). Replaced two `usize::is_multiple_of` calls in `basic_indicators` with `% 2 == 0` (the helper was stabilised in 1.87, incompatible with the declared MSRV).
- Workflow `pull_request` trigger no longer filters on `branches: [ "main" ]`. PRs against any base branch (e.g. stacked PRs) now get CI coverage.
- Added `cargo audit` CI job using a native `cargo install cargo-audit --locked` step (no third-party Action). The job runs `cargo generate-lockfile` first since `Cargo.lock` is gitignored.
- Added `.github/dependabot.yml` for monthly `github-actions` ecosystem updates.
- Added `#![forbid(unsafe_code)]` at the crate root in `src/lib.rs`.
- Added `chart_trends::peak_favorable_move` and `chart_trends::valley_favorable_move`: the per-target maximum favorable excursion (MFE) over a forward window of `period` bars after a reference index. Peak measures the entry-to-window-minimum drop; valley measures the window-maximum-minus-entry rise. Returns the raw signed `f64` (not floored at zero) and is generic over the series — the library stays policy-free. Reuses `basic_indicators::single::{min, max}` and validates via a shared underflow-safe window guard reusing `EmptyData` / `InvalidPeriod`.

### Fixed
- Fixed `chart_trends::peaks` and `chart_trends::valleys` emitting a spurious adjacent extremum (and silently bypassing the `closest_neighbor` proximity/dedup rules) when a real peak/valley occurs at absolute index 0. The `last_peak_idx`/`last_valley_idx` trackers used the `usize` value `0` as both the "none seen yet" sentinel and a valid index 0; they are now `Option<usize>`, so an extremum at index 0 is distinguished from "none seen yet". Affected output now matches the index-shifted case (e.g. `peaks(&[110.0, 109.0, 108.0, 107.0], 2, 1)` returns `[(110.0, 0)]` instead of `[(110.0, 0), (109.0, 1)]`).
- Hardened `chart_trends::peaks` and `chart_trends::valleys` against all-NaN input: the internal `rposition(...).unwrap()` panicked when `single::max`/`single::min` returned `NaN` for an all-NaN window. Such windows are now skipped instead of panicking.
- README Quick Start example now compiles. Previously, `let ma = moving_average::single::moving_average(...)` followed by `println!("...: {}", ma)` was invalid because `ma: Result<f64, _>` does not implement `Display`. The example now `.unwrap()`s the `Result` and the `use` line imports `MovingAverageType` directly.
- `bulk::volatility_system` rustdoc at `src/volatility_indicators.rs:150` now references `TechnicalIndicatorError::MismatchedLength` (the actual variant) instead of the non-existent `InvalidLength`.
- Corrected the 2.0 migration guidance so `volatility_system` is not described as a moving-average composition wrapper; it now has an explicit "no direct replacement" note.

### Changed
- Demoted the RustTI rebrand banner. The README now leads with the project name and badges; the rebrand information moved into a small `## 🔀 Migrating from RustTI` section near the end with a link to `docs/CHANGELOG_RUSTTI_LEGACY.md`. `Cargo.toml` `description` no longer leads with "A rebrand of RustTI".
- Reorganised the README "Available Indicators" section. Removed the "Standard Indicators" subsection; each indicator now appears in exactly one category. Section order now matches the `src/` module layout (basic, candle, chart_trends, correlation, momentum, moving_average, other, strength, trend, volatility). Bollinger Bands moved into Candle Indicators (the home of `moving_constant_bands`); RSI and MACD live only under Momentum.
- Moved `CHANGELOG_RUSTTI_LEGACY.md` to `docs/CHANGELOG_RUSTTI_LEGACY.md`. The historical-note link in `CHANGELOG.md` and the README migration section were updated to the new path.
- Replaced the `panic!` in the private `empirical_quantile_from_distribution` helper (in `basic_indicators`'s `single` module, `src/basic_indicators.rs:620`) with a structured `TechnicalIndicatorError::InvalidValue` return. The path is unreachable from public callers, which validate `low`/`high` in `(0, 1)` before forwarding them as `q`.
- Removed crate-wide `#![allow(unreachable_patterns)]` from `src/lib.rs`. Each defensive `_ => Err(unsupported_type(...))` wildcard arm now carries a scoped `#[allow(unreachable_patterns)]` on the arm itself, preserving the future-proofing pattern (new enum variants hit the wildcard instead of breaking the match) without hiding drift across the rest of the crate.
- Narrowed the `#[cfg_attr(test, allow(...))]` list in `src/lib.rs` by removing `unused_must_use` (kept `deprecated` and `clippy::excessive_precision`). Tests now must handle `Result`s explicitly. Thirteen previously-silent error-path tests in `basic_indicators` and `candle_indicators` were updated to bind the result and assert `.is_err()`, closing a real test rigour gap.
- Dropped `--all-features` from CI workflows, `AGENTS.md`, `CONTRIBUTING.md`, `README.md`, `docs/REPO_MAP.md`, `ai-policy.yaml`, and `.github/pull_request_template.md`. The crate has no `[features]` so the flag was a no-op.
- Build matrix in `.github/workflows/rust.yml` scoped to `[beta, nightly]` with `fail-fast: false`. Stable is covered by the dedicated `test` job; one toolchain failure no longer cancels the others.
- Removed deprecated `[badges]` block from `Cargo.toml`. README badges are unaffected.
- `Cargo.lock` is now gitignored, matching the Cargo convention for library crates. Policy documented in `CONTRIBUTING.md`.
- `.github/ISSUE_TEMPLATE/bug_report.md` and `feature_request.md` now assign to `@ChironMind`.

## [1.2.2] - 2026-04-01

### Fixed
- Fixed `relative_strength_index` (single and bulk) producing incorrect values. The internal `previous_gains_loss` helper was only collecting non-zero gains/losses, discarding zero entries and causing misaligned averages. Both vectors now maintain the same length as the price change series by pushing `0.0` for unchanged periods. All doc examples and tests updated to reflect corrected output.

### Removed
- Removed `docs/indicator_registry.json`, `docs/indicator_registry.schema.json`, and their CI validation scripts (`validate_indicator_registry.py`, `check_indicator_registry_coverage.py`). The registry duplicated information already available in source code and docs.rs, and its manual line-number tracking was fragile.
- Removed `docs/AI_ONBOARDING.md`, `.github/copilot-instructions.md`, and `.github/scripts/docs_consistency_check.py` — redundant with `AGENTS.md`.
- Removed dangling references to the missing `AI_FRIENDLY_ROADMAP.md` from `AGENTS.md`, `CONTRIBUTING.md`, `docs/REPO_MAP.md`, and `ai-policy.yaml`.

### Changed
- CI jobs (`fmt`, `clippy`, `test`, `doc`) now run in parallel instead of waiting on the `policy` job.
- Removed the no-op `build` CI job that only echoed a success message.
- Simplified PR template from 8 sections to 4 (`Summary`, `Compatibility`, `Validation`, `Changelog`).

## [1.2.1] - 2026-03-01
### Added
- Added machine-readable indicator discovery files: `docs/indicator_registry.json` (canonical registry) and `docs/indicator_registry.schema.json` (JSON schema).
- Added repository-level `AGENTS.md` guidance for coding agents, including AI contribution expectations, required quality gates, and PR reporting expectations.
- Added `docs/REPO_MAP.md` with a quick repository map, extension points, and "if changing X, also check Y" guidance.
- Added `docs/AI_ONBOARDING.md` as a canonical start-here onboarding flow for coding agents.
- Added machine-readable repository policy file `ai-policy.yaml` for required checks, change obligations, and PR section requirements.
- Added default pull request template at `.github/pull_request_template.md` with required sections (`Summary`, `Scope`, `Compatibility`, `Validation`, `Benchmarks`, `Changelog`).
- Added CI policy script `.github/scripts/ai_policy_check.sh` to validate PR policy expectations.
- Added `.github/scripts/check_indicator_registry_coverage.py` to enforce parity between the Rust public indicator function surface and `docs/indicator_registry.json`.
- Added `.github/scripts/docs_consistency_check.py` to catch common docs naming drifts/typos and unknown README benchmark function names.

### Changed
- Moved pre-rebrand RustTI release history into `CHANGELOG_RUSTTI_LEGACY.md` and added a historical note with explicit legacy release-tag links.
- Updated CI (`.github/workflows/rust.yml`) to run lightweight indicator registry schema validation via `.github/scripts/validate_indicator_registry.py`.
- Expanded `docs/indicator_registry.schema.json` and `docs/indicator_registry.json` metadata with per-indicator `source` locations, `docs_url`, optional parameter `constraints`, warmup descriptions, and structured deprecation guidance.
- Expanded `.github/scripts/validate_indicator_registry.py` to validate enriched registry metadata, including source line mapping, docs URL shape, and deprecation consistency/replacement references.
- Updated CI (`.github/workflows/rust.yml`) policy job to run indicator registry coverage checks and docs consistency checks.
- Linked the indicator registry in `README.md` and `AI_FRIENDLY_ROADMAP.md` as the canonical discovery source for tools/agents.
- Normalized `docs/indicator_registry.json` entries so `id` and `function_path` are canonical and unique, replaced `supports_bulk`/`supports_single` with `mode` (`single`/`bulk`/`module`), renamed schema field names to avoid keyword-collisions (`parameters[].param_type`, `returns.return_type`, `is_deprecated`), standardized fallible return types to `centaur_technical_indicators::Result<...>`, and expanded `.github/scripts/validate_indicator_registry.py` to enforce uniqueness, mode/path consistency, and return-type/fallibility consistency.
- Updated `.github/workflows/rust.yml` to add explicit stable-toolchain CI quality gate steps for `cargo fmt --all -- --check`, `cargo clippy --all-targets --all-features -- -D warnings`, `cargo test --all-features`, and `cargo doc --no-deps`, with each gate logged as its own named step.
- Updated `.github/workflows/rust.yml` so each required quality gate (`fmt`, `clippy`, `test`, `doc`) runs as its own blocking CI job on PRs and pushes to `main`; the toolchain matrix build now depends on all gate jobs.
- Updated `.github/workflows/rust.yml` to remove third-party Rust CI actions and use native `rustup`/`cargo` commands for toolchain setup and checks.
- Updated `README.md` contributing guidance to list the exact local quality-gate commands that CI runs.
- Updated `CONTRIBUTING.md` to include a matching `Local quality gates` section with the exact CI command set.
- Updated `AGENTS.md` and `CONTRIBUTING.md` with CI policy guidance to avoid third-party GitHub Actions for Rust toolchain setup/caching unless explicitly approved by maintainers.
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
- Removed deprecated wrapper references from module overview docs and the reference example to keep docs focused on supported APIs.
- Corrected minor naming/typo drifts in docs and Rust doc comments (for example `default_rsi`, `donchian_channels`, and `true_strength_index`) to improve deterministic parsing for tools/agents.

### Agent Notes
- `breaking_change`: no
- `public_api_change`: no intentional `pub` removal/rename; one deprecation marker removal (`volume_price_trend`)
- `error_variant_change`: none
- `indicator_semantics_change`: none
- `warmup_or_output_order_change`: none

## [1.2.0] - 2026-02-25
### Added
- Reference URLs in docstrings.

### Changed
- Sorted the `## Included Indicators` Bulk and Single lists alphabetically in all module docstrings (`candle_indicators`, `momentum_indicators`, `other_indicators`, `strength_indicators`, `trend_indicators`)

### Deprecated
- Functions in `momentum_indicators` that were wrappers for moving averages (`signal_line`, slow stochastic, slowest stochastic). These functions are planned for removal in `2.0.0`; call moving average functions directly for composability.
- `volume_price_trend`.
- Volatility system wrappers.

### Agent Notes
- `breaking_change`: no
- `public_api_change`: deprecations only
- `error_variant_change`: none
- `indicator_semantics_change`: none
- `warmup_or_output_order_change`: none


## [1.0.0] - 2026-01-07
### Changed
- **BREAKING:** Rebranded from RustTI to Centaur Technical Indicators.
- **BREAKING:** Package name changed from `rust_ti` to `centaur_technical_indicators`.
- **BREAKING:** New package on crates.io with fresh versioning (`1.0.0`).
- **BREAKING:** Repository and documentation URLs updated to reflect Centaur Technologies branding.
- **BREAKING:** All functionality remained the same; this change was branding/package identity.
- **BREAKING:** `panic!` replaced with `Result<>` types in several functions for better error handling

### Removed
- Removed unused `deviation.rs` file

### Agent Notes
- `breaking_change`: yes
- `public_api_change`: package/crate identity changed (`rust_ti` -> `centaur_technical_indicators`)
- `error_variant_change`: fallible APIs now return `Result` in places that previously panicked
- `indicator_semantics_change`: no
- `warmup_or_output_order_change`: none documented

---

### Historical note
Pre-rebrand RustTI release history is documented in [`docs/CHANGELOG_RUSTTI_LEGACY.md`](docs/CHANGELOG_RUSTTI_LEGACY.md). Legacy entries use explicit `rustti-v*` tag links to avoid ambiguity with Centaur releases.

[Unreleased]: https://github.com/chironmind/CentaurTechnicalIndicators-Rust/compare/centaur-v1.2.2...HEAD
[1.2.2]: https://github.com/chironmind/CentaurTechnicalIndicators-Rust/compare/centaur-v1.2.1...centaur-v1.2.2
[1.2.1]: https://github.com/chironmind/CentaurTechnicalIndicators-Rust/compare/centaur-v1.2.0...centaur-v1.2.1
[1.2.0]: https://github.com/chironmind/CentaurTechnicalIndicators-Rust/releases/tag/centaur-v1.2.0
[1.0.0]: https://github.com/chironmind/CentaurTechnicalIndicators-Rust/releases/tag/centaur-v1.0.0
