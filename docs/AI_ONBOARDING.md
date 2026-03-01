# AI Onboarding

Start here when an AI agent begins work in `CentaurTechnicalIndicators-Rust`.

## Goal

Provide one deterministic startup flow so agents can orient quickly, avoid policy misses, and make minimal, safe changes without altering public APIs unintentionally.

## Startup Flow (in order)

1. Read repository rules:
   - `AGENTS.md`
   - `.github/copilot-instructions.md`
   - `CONTRIBUTING.md`
2. Read project orientation:
   - `docs/REPO_MAP.md`
   - `AI_FRIENDLY_ROADMAP.md`
3. Use machine-readable discovery/policy sources:
   - `docs/indicator_registry.json` (canonical indicator registry)
   - `docs/indicator_registry.schema.json` (registry schema)
   - `ai-policy.yaml` (machine-readable contribution policy)
4. Confirm affected modules in `src/` and keep scope focused.

## Non-Negotiable Rules

- Reuse validation helpers from `src/validation.rs`.
- Return structured `TechnicalIndicatorError` variants for fallible operations.
- Keep public API behavior stable unless explicitly asked to introduce a breaking change.
- Add/adjust tests in the same module as the implementation change.
- Add a `CHANGELOG.md` entry for each user-facing change.

## Agent-Friendly Change Strategy

1. Identify the smallest module/file that can satisfy the task.
2. Prefer additive or internal-only edits over broad refactors.
3. If changing math/output semantics, update tests and document compatibility impact.
4. If touching public `pub` items, include clear compatibility notes.
5. If touching indicator logic in `src/*_indicators.rs`, run relevant benchmark suites in the companion benchmark repo.

## Required Local Validation Gates

Run from repository root:

```bash
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-features
cargo doc --no-deps
```

Also run the machine checks:

```bash
python3 .github/scripts/validate_indicator_registry.py
python3 .github/scripts/check_indicator_registry_coverage.py
python3 .github/scripts/docs_consistency_check.py
```

## PR/Report Output Format

Use this structure:

1. `Summary`
2. `Scope`
3. `Compatibility`
4. `Validation`
5. `Benchmarks`
6. `Changelog`

## Quick Pointers

- Crate entry/export map: `src/lib.rs`
- Error types: `src/error.rs`
- Shared validation helpers: `src/validation.rs`
- Shared enums/types: `src/types.rs`
- API usage sample: `examples/reference.rs`

