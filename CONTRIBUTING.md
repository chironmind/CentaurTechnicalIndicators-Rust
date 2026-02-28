# Contributing to Centaur Technical Indicators

Thank you for contributing to `CentaurTechnicalIndicators-Rust`.

## Contribution Workflow

1. Fork and clone the repository.
2. Make focused changes.
3. Open a pull request (PR) on [GitHub](https://github.com/ChironMind/CentaurTechnicalIndicators-Rust/pulls).
   - Use the default PR template: `.github/pull_request_template.md`.
   - Complete all required sections in the template.
   - CI policy checks may fail PRs that change `src/*.rs` without a `CHANGELOG.md` update.
   - CI policy checks may fail PRs with public API changes that omit a `Compatibility` section.
4. Tag `@ChironMind` in your PR for review.
5. Run relevant benchmarks from [CentaurTechnicalIndicators-Rust-Benchmarks](https://github.com/chironmind/CentaurTechnicalIndicators-Rust-Benchmarks).

If you want a smaller starter task, see [open issues](https://github.com/ChironMind/CentaurTechnicalIndicators-Rust/issues).

## Areas to Help

- Refactor parabolic T/P system: remove single functions and let bulk functions determine trend (similar to the `volatility` system).

### New Indicator Ideas

- McGinley dynamic versions of indicators using moving averages (Donchian, Keltner, Supertrend, PPO, RVI, and related indicators).
- Deprecate MACD signal line where callers can compose their preferred moving average.
- Apply the same composability approach to slow and slowest stochastics.
- Order indicator lists alphabetically in category docstrings.

## Adding a New Indicator

1. Open an issue.
   - Describe the indicator, use case, and calculation method with a source/reference.
2. Implement the indicator.
   - Add documentation and unit tests.
3. Verify results.
   - Add a tab to `assets/centaur_ti_hand_calcs.ods` with hand calculations.
4. Add and run benchmarks.
   - Add a benchmark in [CentaurTechnicalIndicators-Rust-Benchmarks](https://github.com/chironmind/CentaurTechnicalIndicators-Rust-Benchmarks).
   - Run relevant benchmark suites and document runtime/regression details.
5. Update changelog.
   - Add an entry to `CHANGELOG.md` for every user-facing change in your PR.

## Code Style and Testing

- Format code with `cargo fmt`.
- Run tests with `cargo test` before submitting your PR.

### Local quality gates

Run the same quality gates locally that CI runs:

```bash
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-features
cargo doc --no-deps
```

### CI dependency policy

- Keep CI aligned with the project dependency philosophy.
- Prefer native `rustup`/`cargo` commands in GitHub workflows.
- Avoid third-party GitHub Actions for Rust toolchain setup and Cargo caching unless maintainers explicitly approve them.

## AI-Assisted Contributions

If AI tools were used (Copilot, Codex, etc.), include what the assistant changed and what you verified manually in the PR.

### Required Checklist (AI-Assisted PRs)

- [ ] Reuse validation helpers from `src/validation.rs` for input and argument checks.
- [ ] Return structured `TechnicalIndicatorError` values where applicable.
- [ ] Document new public APIs, including behavior and parameter expectations.
- [ ] Add or adjust unit tests in the same module as the changed code.
- [ ] Avoid deprecated API usage in new examples and tests.
- [ ] Add or update the `CHANGELOG.md` entry for every user-facing change.

### Required Pre-PR Command Gates

Run all commands below before opening a PR:

1. `cargo fmt --all -- --check`  
   Pass criteria: no formatting diffs.
2. `cargo clippy --all-targets --all-features -- -D warnings`  
   Pass criteria: zero clippy warnings/errors.
3. `cargo test --all-features`  
   Pass criteria: all tests pass.
4. `cargo doc --no-deps`  
   Pass criteria: docs build successfully, including new public APIs.
5. Relevant benchmark suite(s) in [CentaurTechnicalIndicators-Rust-Benchmarks](https://github.com/chironmind/CentaurTechnicalIndicators-Rust-Benchmarks)  
   Pass criteria: benchmark run completes with no unintended regressions for touched indicators.

## Related Documentation

- [`AGENTS.md`](AGENTS.md)
- [`.github/copilot-instructions.md`](.github/copilot-instructions.md)
- [`AI_FRIENDLY_ROADMAP.md`](AI_FRIENDLY_ROADMAP.md)
- [`docs/REPO_MAP.md`](docs/REPO_MAP.md)

Thanks again for contributing.
