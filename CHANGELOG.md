# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/), and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [Unreleased]
### Added
- Added machine-readable indicator discovery files: `docs/indicator_registry.json` (canonical registry) and `docs/indicator_registry.schema.json` (JSON schema).
- Added repository-level `AGENTS.md` guidance for coding agents, including AI contribution expectations, required quality gates, and PR reporting expectations.
- Added `docs/REPO_MAP.md` with a quick repository map, extension points, and "if changing X, also check Y" guidance.
- Added machine-readable repository policy file `ai-policy.yaml` for required checks, change obligations, and PR section requirements.
- Added default pull request template at `.github/pull_request_template.md` with required sections (`Summary`, `Scope`, `Compatibility`, `Validation`, `Benchmarks`, `Changelog`).
- Added CI policy script `.github/scripts/ai_policy_check.sh` to validate PR policy expectations.

### Changed
- Moved pre-rebrand RustTI release history into `CHANGELOG_RUSTTI_LEGACY.md` and added a historical note with explicit legacy release-tag links.
- Updated CI (`.github/workflows/rust.yml`) to run lightweight indicator registry schema validation via `.github/scripts/validate_indicator_registry.py`.
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

### Historical note
Pre-rebrand RustTI release history is documented in [`CHANGELOG_RUSTTI_LEGACY.md`](CHANGELOG_RUSTTI_LEGACY.md). Legacy entries use explicit `rustti-v*` tag links to avoid ambiguity with Centaur releases.

[Unreleased]: https://github.com/chironmind/CentaurTechnicalIndicators-Rust/compare/centaur-v1.2.0...HEAD
[1.2.0]: https://github.com/chironmind/CentaurTechnicalIndicators-Rust/releases/tag/centaur-v1.2.0
[1.0.0]: https://github.com/chironmind/CentaurTechnicalIndicators-Rust/releases/tag/centaur-v1.0.0
