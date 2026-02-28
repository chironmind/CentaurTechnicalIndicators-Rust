# Contributing to Centaur Technical Indicators

Thank you for considering contributing—every improvement is appreciated, big or small!

---

## 🙌 How to Contribute

1. **Fork and clone the repository**  
2. **Make your changes**  
3. **Open a Pull Request (PR)** on [GitHub](https://github.com/ChironMind/CentaurTechnicalIndicators-Rust/pulls)
4. **Tag @ChironMind** in your PR for a review
5. **Run benchmarks** from [CentaurTechnicalIndicators-Rust-benchmarks](https://github.com/chironmind/CentaurTechnicalIndicators-Rust-Benchmarks)

See [open issues](https://github.com/ChironMind/CentaurTechnicalIndicators-Rust/issues) if you want to start with something small.

---

## 🛠️ What to Work On?

- Refactor parabolic T/P system: remove single functions; let bulk function determine trend (like `volatility` system)

### New Indicator Ideas

- McGinley dynamic versions of indicators using MA (Donchian, Keltner, Supertrend, PPO, RVI, etc.)?
- Deprecate signal line for MACD. Callers can call the MA functions they want
- Same as above for slow and slowest stochastics
- Order list of indicators in each category alphabetically in docstrings

---

## ➕ Adding a New Indicator

1. **Open an Issue**  
   - Describe the indicator, what it’s used for, and how it’s calculated (with source/reference)
2. **Implement the indicator**  
   - Add documentation and unit tests
3. **Verify results**  
   - Add a tab to `assets/centaur_ti_hand_calcs.ods` with hand calculations to ensure test accuracy
4. **Add to benchmarks and run**
   - Add a bench in [CentaurTechnicalIndicators-Rust-benchmarks](https://github.com/chironmind/CentaurTechnicalIndicators-Rust-Benchmarks), run it, and document the runtime
   - If you do not have the benchmark repo locally, check it out first and run the relevant benchmark suites to confirm no regressions
5. **Update changelog**
   - Add an entry to `CHANGELOG.md` for every user-facing change included in your PR

---

## 🧪 Code Style & Testing

- Format code with `rustfmt` or `cargo fmt`
- Run tests with `cargo test` before submitting your PR

---

## 🤖 AI-Assisted Contributions

If you are using an AI coding assistant (Copilot, Codex, etc.), include a quick note in your PR description about what the assistant changed and what you verified manually.

### Required Checklist (AI-assisted PRs)

- [ ] Reuse validation helpers from `src/validation.rs` for input and argument checks.
- [ ] Return structured `TechnicalIndicatorError` values where applicable instead of ad hoc errors.
- [ ] Document any new public APIs (including examples and parameter behavior).
- [ ] Add or adjust unit tests in the same module as the changed code.
- [ ] Avoid introducing deprecated API usage in new examples and tests.
- [ ] Add or update the `CHANGELOG.md` entry for every user-facing change.

### Required Pre-PR Command Gates

Run all of the following locally before opening a PR:

1. `cargo fmt --all -- --check`  
   **Pass criteria:** no formatting diffs.
2. `cargo clippy --all-targets --all-features -- -D warnings`  
   **Pass criteria:** zero clippy warnings/errors.
3. `cargo test --all-features`  
   **Pass criteria:** all tests pass with no failures.
4. `cargo doc --no-deps`  
   **Pass criteria:** docs build successfully and include new public APIs.
5. Relevant benchmark suite(s) in [CentaurTechnicalIndicators-Rust-benchmarks](https://github.com/chironmind/CentaurTechnicalIndicators-Rust-Benchmarks)  
   **Pass criteria:** benchmark run completes and no unintended regressions are introduced for touched indicators.

For AI-specific guidance, also review:

- [`AGENTS.md`](AGENTS.md)
- [`.github/copilot-instructions.md`](.github/copilot-instructions.md)
- [`AI_FRIENDLY_ROADMAP.md`](AI_FRIENDLY_ROADMAP.md)


---

Thanks again for your interest and contributions!
