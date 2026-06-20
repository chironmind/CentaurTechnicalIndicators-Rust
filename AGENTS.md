# AGENTS.md

Guidance for coding agents working in this repository. This file is the
authoritative, self-contained source for this repo's standing conventions — pre-PR
gates, branch/commit format, PR shape, scope discipline, stop-and-report, and
changelog coupling. Task briefs defer to it and record only what is specific to
the task.

## Scope
This file applies to the entire repository.
This is an open source repository; all contributions should be suitable for public review and distribution.

## Core contribution requirements
When implementing or modifying Rust code:

1. Use shared validation helpers from `src/validation.rs` for input/argument checks.
2. Return structured `TechnicalIndicatorError` values where applicable.
3. No `unwrap()` or `expect()` in library code (`src/`) — acceptable only in tests and examples.
4. Document new public APIs with `///` doc comments on all `pub` items (behavior and parameter expectations).
5. Prefer `&[f64]` over `Vec<f64>` for input-data parameters; order imports std → external crates → crate-internal.
6. Add or adjust tests in the same module as the change.
7. Do not introduce deprecated API usage in new examples or tests.
8. Add a `CHANGELOG.md` entry under the `## [Unreleased]` heading for every user-facing change, using Keep a Changelog categories (`Added` / `Changed` / `Deprecated` / `Removed` / `Fixed` / `Security`) and naming concrete artifacts (module, function). Exceptions that may skip the entry: internal-only refactors with no behavior change, test-only changes, and CI/tooling changes — note in the PR why no entry was needed.
9. MSRV is `Cargo.toml`'s `rust-version` (currently 1.81), edition 2021 — do not use APIs stabilized after the MSRV.

## Change scope discipline
- Keep changes minimal and focused on the requested task.
- Do not include opportunistic refactors unless explicitly requested.
- If you identify unrelated issues, note them separately instead of bundling them into the same change.
- Preserve existing file organization and naming conventions unless the task requires a structural change.
- Only modify the files the task requires; do not touch unrelated files in the same change.
- Do not commit `Cargo.lock` — it is git-ignored and not committed for this library crate.
- Do not hand-edit generated or golden artifacts (e.g. test fixtures, `assets/`) to make a check pass; a value that needs changing is a signal to **stop and report**.
- Do not run a global or whole-file reformat; keep formatting to the lines you change (CI runs `cargo fmt --all -- --check`).
- Treat governing docs as read-only for unrelated tasks — see **Stop and report**.

## Backward compatibility rules
When changing public APIs (`pub` items), preserve compatibility unless the task explicitly allows a breaking change:

1. Do not silently change indicator semantics, output ordering, or warmup behavior.
2. Do not remove or rename public functions, types, enums, or fields without explicit approval.
3. Do not repurpose existing `TechnicalIndicatorError` variants in ways that break downstream matching.
4. If behavior changes are required, document them in code docs and `CHANGELOG.md` with clear migration notes.

## Performance-sensitive paths
- Treat hot-path indicator computations as performance-sensitive.
- Avoid unnecessary allocations, cloning, and intermediate buffers in computation loops.
- Prefer existing efficient utilities/patterns already used in similar modules.
- If performance could change materially, run and report benchmark results for affected indicators.

## Branch and worktree workflow
- Work on a topic branch, never directly on `main`; PRs target `main`.
- Branch name = `<type>/<slug>`, where `<type>` is one of `feat`, `fix`, `chore`, `docs`, `test`, `refactor`, `perf`, `ci` (e.g. `feat/display-and-property-tests`, `fix/all-nan-guard`). Reserved variants: `release/<x.y.z>` for release prep and `batch/<n>-<slug>` for numbered cleanup batches. Tool-generated branches (e.g. `dependabot/...`) are not part of the human convention.
- One task/batch per branch — and per `git worktree` when working in parallel, so concurrent builds don't collide.
- Rebase or merge `main` into the branch before requesting merge so the PR is current.

## Commit conventions
- Use conventional-commit subject prefixes: `feat:`, `fix:`, `chore:`, `docs:`, `refactor:`, `test:`, `perf:`, `ci:` (scoped form allowed, e.g. `chore(deps):`). Keep the subject under 72 characters; reference issue numbers where applicable.
- When an AI agent authors or co-authors a commit, append this trailer on its own final line:
  ```text
  Co-Authored-By: Claude Opus 4.8 (1M context) <noreply@anthropic.com>
  ```
  This is the canonical trailer for Claude-authored commits; existing lowercase `Co-authored-by:` lines for human or bot co-authors are unaffected.

## Pre-PR quality gates (must pass)
Run these before opening a PR:

1. `cargo fmt --all -- --check` (no formatting diffs)
2. `cargo clippy --all-targets -- -D warnings` (zero warnings/errors). This crate defines no Cargo features, so `--all-features` is intentionally omitted.
3. `cargo test` (all tests pass)
4. `cargo doc --no-deps` (docs build successfully, including new public APIs)
5. Benchmark checks in the companion repo (no performance regressions for affected indicators):
   - Clone/check out `https://github.com/chironmind/CentaurTechnicalIndicators-Rust-Benchmarks`
   - Run the relevant benchmark suite(s) and summarize regressions/improvements in the PR
   - Required only when one of the indicator modules listed in `ai-policy.yaml` `benchmark.required_for_paths` changes.

CI (`.github/workflows/rust.yml`) also enforces these gates, which you should reproduce when relevant:

6. **MSRV build** (`msrv` job): `rustup toolchain install 1.81 --profile minimal`, then `cargo build` and `cargo test --no-run` under Rust 1.81 (matches `Cargo.toml` `rust-version`).
7. **Security audit** (`audit` job): `cargo install cargo-audit --locked`, then `cargo generate-lockfile` (the lockfile isn't committed), then `cargo audit`.
8. **Beta/nightly matrix** (`build-matrix` job): `cargo build --verbose` and `cargo test --verbose` on `beta` and `nightly`.

CI additionally runs an **AI-policy check** (`policy` job: `bash .github/scripts/ai_policy_check.sh`). It fails any PR that changes `src/*.rs` without a `CHANGELOG.md` update, or that changes a public API without a `Compatibility` section in the PR body. See the script for the exact rules.

## Stop and report
- If you hit a blocker, an ambiguous or contradictory instruction, or an unexpected obstacle, STOP and report it — do not work around it or guess a path forward.
- If you notice drift between the governance docs (`AGENTS.md`, `CONTRIBUTING.md`, `ai-policy.yaml`, `.github/pull_request_template.md`, `.github/workflows/rust.yml`), surface it in your report rather than silently reconciling it.
- Treat these governing docs as read-only for unrelated tasks — change them only when that is the explicit task (and CI workflow files need maintainer approval): `AGENTS.md`, `CLAUDE.md`, `CONTRIBUTING.md`, `ai-policy.yaml`, `ROADMAP.md`, and `.github/workflows/`.

## CI implementation policy
- Keep CI implementation dependency-light, consistent with the crate philosophy.
- Prefer native `rustup` and `cargo` commands in workflows.
- Do not introduce third-party GitHub Actions for Rust toolchain setup or Cargo caching unless explicitly approved by maintainers.

## PR expectations for agents
- Keep PRs focused and minimal.
- Summarize what the agent changed and what was manually verified.
- Include command output for the required quality gates.
- Explicitly note the `CHANGELOG.md` entry and benchmark results from the companion benchmark repo.

### Required PR summary format
Use this structure in PR descriptions/comments:

1. `Summary`: what changed and why.
2. `Scope`: files/modules touched and what was intentionally not changed.
3. `Compatibility`: any user-facing behavior/API/error-handling impact (required when a public API changes — the AI-policy check enforces this).
4. `Validation`: paste the verbatim command output (or the explicit pass line) for `fmt`, `clippy`, `test`, and `doc`; note any CI-only gates (MSRV, audit, beta/nightly) you could not run locally.
5. `Benchmarks`: affected suites and regression/improvement summary (required only when an `ai-policy.yaml` `benchmark.required_for_paths` module changes).
6. `Changelog`: exact `CHANGELOG.md` entry added/updated (or note why exempt).
7. `Flagged items`: any unrelated issues, drift between governance docs, or blockers encountered — listed separately, not bundled into the change.
