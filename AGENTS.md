# AGENTS.md

Guidance for coding agents working in this repository. **This file is
self-contained** — everything needed to work here lives in this repo (this file,
`CLAUDE.md`, `CONTRIBUTING.md`, and the README); it depends on no external or
workspace-level file.

## Scope

Applies to the entire repository. This is a public, open-source crate
(`centaur_technical_indicators`, published to crates.io); all contributions must be
suitable for public review and distribution.

## How work arrives

**If your task is a slice brief**, it is self-contained — read it and do exactly
that. You don't need to read the implementation plan or any project spec to execute
it; everything required is in the brief. If you hit a real gap, **stop and ask** —
you'll be pointed at the specific file, never told to read a whole plan. Not every
task is a brief; for ad-hoc work, follow the standing rules below. (Reviewers are
the exception — a plan or PR review legitimately reads the plan and spec.)

## Core contribution requirements

When implementing or modifying Rust code:

1. Use the shared validation helpers in `src/validation.rs` for input/argument checks.
2. Return structured `TechnicalIndicatorError` values; no panics or string errors in
   library code (`unwrap()`/`expect()` only in tests and examples).
3. Document every `pub` item with `///` comments (behavior + parameter expectations).
4. Add or adjust tests in the same module as the change.
5. No deprecated API usage in new examples or tests.
6. Add a `CHANGELOG.md` entry for every user-facing change (see Changelog coupling).
7. Prefer `&[f64]` over `Vec<f64>` for input data; order imports std → external → crate-internal.

## Downstream & cross-repo impact

This crate is the source of truth for indicator behavior; its public API ripples to
the Python and JS bindings, the documentation site, the tutorials and how-to guides,
and the benchmark companion. A change to public API, semantics, output ordering, or
warmup behavior affects all of them — call out the downstream impact in your report.
Don't make cross-repo or architectural decisions from inside this repo; surface them
to the maintainer.

## Change-scope discipline

- Smallest safe diff that solves the task; keep it focused.
- No opportunistic refactors or "while I was here" changes — note unrelated issues
  separately, don't bundle them.
- Preserve existing file organization and naming unless the task requires a structural change.
- **Stage only the files your task names; never `git add .` / `git add -A`** — this
  repo doesn't track `Cargo.lock`, and a blanket add sweeps in generated artifacts.
  The committed decision trail under `docs/` is the deliberate exception: plans, slice
  briefs, `RESUME.md`, and decision records (e.g. under `docs/implementation_decisions/`
  and `docs/technical_decisions/`) are version-controlled on purpose — commit those
  when your task creates or updates them.
- Never hand-edit a generated artifact or a test expectation to make a gate pass. A
  test value that needs changing is a signal to **stop and report**.

## Backward compatibility

Published public APIs — treat every `pub` item as a contract:

1. Do not silently change indicator semantics, output ordering, or warmup behavior.
2. Do not remove or rename public functions, types, enums, or fields without explicit approval.
3. Do not repurpose `TechnicalIndicatorError` variants in ways that break downstream matching.
4. If a behavior change is required, document it in code docs and `CHANGELOG.md` with migration notes.

## Performance-sensitive paths

- Treat hot-path indicator computations as performance-sensitive: avoid unnecessary
  allocations, cloning, and intermediate buffers in computation loops; prefer the
  efficient patterns already used in similar modules.
- If performance could change materially, run the relevant suites in the benchmark
  companion repo (`CentaurTechnicalIndicators-Rust-Benchmarks`) and summarize
  regressions/improvements in your report.

## Pre-PR quality gates (must pass)

Run these before opening a PR; paste the output into the report's Validation section:

1. `cargo fmt --all -- --check` — no formatting diffs.
2. `cargo clippy --all-targets -- -D warnings` — zero warnings/errors.
3. `cargo test` — all tests pass.
4. `cargo doc --no-deps` — docs build, including new public APIs.

## Branch & commit conventions

- Cut every branch off the latest `origin/main`; never commit directly on `main`.
  Name branches `<type>/<kebab-slug>` with a conventional-commit type (`feat`, `fix`,
  `chore`, `docs`, `test`, `refactor`, `perf`, `ci`) — e.g. `feat/rsi-bulk`.
- Commit subjects use a conventional-commit prefix, under 72 characters; reference
  issue numbers where applicable.
- End every agent-authored commit with:

      Co-Authored-By: Claude Opus 4.8 (1M context) <noreply@anthropic.com>

## Stop-and-report

**Never guess or assume.** If information is missing, the task is ambiguous, or two
implementations are plausible, **stop and ask for input** before proceeding — don't
pick one and run. Beyond that, stop and report — never work around, paper over, or
invent a way past — when:

- a pre-PR gate fails for a reason outside your change, or a test expectation shifts
  in a way you can't explain;
- completing the task would require a forbidden or breaking change (public API,
  semantics, output ordering, warmup) without explicit approval;
- the brief or instructions conflict with the repo's actual state.

Surface the blocker; do not invent a way past it.

## Worktree & isolation

For parallel or batched work: one task per git worktree, cut off fresh `origin/main`;
keep concurrent tasks file-disjoint; when two touch the same files, rebase the later
on its predecessor before merging.

## Changelog coupling

Every user-facing change adds a bullet under the existing `## [Unreleased]` heading in
`CHANGELOG.md` (Keep a Changelog categories: Added / Changed / Deprecated / Removed /
Fixed / Security); name the concrete artifact. Don't add a second `[Unreleased]`
heading. Formatting-only or non-user-facing changes (incl. docs like this file) are
exempt — note the exception in the PR.

## CI policy

Keep CI dependency-light: prefer native `rustup`/`cargo` commands; no third-party
GitHub Actions for toolchain setup or caching without explicit approval.

## PR / completion report

Use this structure:

1. **Summary** — what changed and why.
2. **Scope** — files/modules touched, and what was deliberately left untouched.
3. **Compatibility** — user-facing behavior/API/error-handling impact, or "N/A".
4. **Validation** — pasted output of the four pre-PR gates.
5. **Changelog** — the exact `CHANGELOG.md` entry added/updated (or "exempt — non-user-facing").
6. **Benchmarks** — affected suites + regression/improvement summary, or "N/A".

Plus, required: each named acceptance test with its pass output verbatim (incl. the
decisive test), and anything flagged — out-of-scope issues noticed, concerns,
blockers. Justify any deviation from the brief here.
