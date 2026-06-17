# 2.0 Plan

> Status: planning artifact. No removal, no `#[non_exhaustive]`, no NaN/inf
> validation change happens until each item below is explicitly approved by
> the maintainer and the binding-repo PRs are queued up. This document is
> the place where each item is classified, sized, and stitched to its
> downstream impact before the 2.0 cut.

## Scope

The 2.0 release exists to absorb the compatibility-sensitive changes the 1.x
hygiene cycle deferred:

1. Remove the seven deprecated functions inventoried below.
2. Add `#[non_exhaustive]` to `TechnicalIndicatorError`.
3. Resolve the NaN/inf audit: every site classified as a structured-error
   change lands as a breaking validation tightening.

Nothing else (new indicators, refactors, new public types) goes in 2.0
unless explicitly requested.

## Deprecated function inventory

Each function has been `#[deprecated(since = "1.0.0", note = "...")]` since
2026-01. The "Path in source" column gives the file and line of the
`#[deprecated]` attribute.

### Momentum indicators (six functions, three concepts)

| Function | Path in source | Deprecated `note` text |
|---|---|---|
| `single::signal_line` | `src/momentum_indicators.rs:982` | Users can call the moving average functions directly on the MACD values to get the signal line. |
| `bulk::signal_line` | `src/momentum_indicators.rs:2173` | Same — apply a moving average to the bulk MACD output. |
| `single::slow_stochastic` | `src/momentum_indicators.rs:268` | Users can call the moving average functions directly on the SO. |
| `bulk::slow_stochastic` | `src/momentum_indicators.rs:1580` | Same — apply a moving average to the bulk stochastic output. |
| `single::slowest_stochastic` | `src/momentum_indicators.rs:347` | Same — apply a moving average to the slow stochastic output. |
| `bulk::slowest_stochastic` | `src/momentum_indicators.rs:1650` | Same — apply a moving average to the bulk slow stochastic output. |

### Volatility indicators (one function)

| Function | Path in source | Deprecated `note` text |
|---|---|---|
| `bulk::volatility_system` | `src/volatility_indicators.rs:209` | This function is deprecated as it is not commonly used. |

Note: there is **no `single::volatility_system`** — the `single` module of
`volatility_indicators` exposes only `ulcer_index`. Removal only touches
`bulk::volatility_system`.

### Migration story for users

The deprecation `note` text is the canonical migration recipe. In short:
the deprecated wrappers all reduce to "apply a moving average over the
output of the underlying indicator." Library users can write:

```rust
use centaur_technical_indicators::{
    momentum_indicators, moving_average, ConstantModelType, MovingAverageType,
};

// Was: bulk::signal_line(&prices, ConstantModelType::SimpleMovingAverage, 9)
// Now: compute the MACD line, then apply a moving average to it.
let macd_line = momentum_indicators::bulk::macd_line(
    &prices,
    9,
    ConstantModelType::ExponentialMovingAverage,
    26,
    ConstantModelType::ExponentialMovingAverage,
).unwrap();
let signal = moving_average::bulk::moving_average(
    &macd_line,
    MovingAverageType::Exponential,
    9,
).unwrap();
```

The same shape applies to the stochastic and volatility-system replacements.
The 2.0 changelog will include one worked example per function.

## Cross-repo impact analysis

Confirmed by grepping the binding repos in `~/Projects/` for the deprecated
function names on 2026-05-20:

### `CentaurTechnicalIndicators-Python` — coordinated PR required

All four deprecated concepts are surfaced through PyO3:

- `src/momentum_indicators.rs:204-247` — `single`/`bulk` `slow_stochastic`,
  `slowest_stochastic`, `signal_line`. Each already emits a Python
  `DeprecationWarning` referencing "deprecated upstream and will be removed
  in the next major version."
- `src/volatility_indicators.rs` — `bulk::volatility_system`.
- `tests/test_deprecations.py`, `tests/test_volatility_indicators.py`,
  `tests/test_momentum_indicators.py` — test coverage for the deprecation
  warnings.

**Action for 2.0:** open a coordinated PR in
`CentaurTechnicalIndicators-Python` that removes the PyO3 wrappers, drops
the deprecation tests, and bumps the dependency on
`centaur_technical_indicators` to `2.0`.

### `CentaurTechnicalIndicators-JS` — coordinated PR required

All four deprecated concepts are surfaced through `#[wasm_bindgen]`:

- `src/momentum_indicators.rs:26-280` — `momentum_single_slow_stochastic`,
  `momentum_single_slowest_stochastic`, `momentum_single_signal_line`, and
  their `_bulk_` counterparts.
- `src/volatility_indicators.rs` — `volatility_bulk_volatility_system`.

**Action for 2.0:** open a coordinated PR in
`CentaurTechnicalIndicators-JS` that removes the wasm-bindgen wrappers and
bumps the core dependency to `2.0`.

### `CentaurTechnicalIndicators-Rust-Cuda` — no action

The CUDA port has its own kernel implementations (`signal_line_bulk_rolling_f64`,
`slow_stochastic_bulk_rolling_f64`, `slowest_stochastic_bulk_rolling_f64`,
`volatility_system_bulk_recursive_f64` in `src/ptx.rs` and `build.rs`).
These do not depend on the Rust core's deprecated wrappers. Removing the
Rust-core wrappers has zero effect on the CUDA port.

If the maintainer also wants to retire these CUDA kernels at the same time,
that is a separate decision tracked in the CUDA repo, not gated on the 2.0
core cut.

### `crt-research-engine` — no action

The workspace was consolidated (2026-05-01) into three crates —
`gpu-primitives`, `gpu-indicators`, and `engine`; the former
`indicator-optimizer` and `gpu-scoring` crates were deleted. The core crate
`centaur_technical_indicators` (pinned `=1.2.2`) is now a **dev-dependency of
`gpu-indicators` only**, used solely by its parity tests; no production crate
(`engine`, `gpu-primitives`, or the `gpu-indicators` library itself) links it.
A repo-wide grep (re-run 2026-06-17) finds **zero** uses of any deprecated
function (`slow_stochastic`, `slowest_stochastic`, `signal_line`,
`volatility_system`) across the workspace: the parity tests call only
non-deprecated base functions (e.g. `stochastic_oscillator`), and the only
`signal_line` hit is the engine's own GPU kernel `signal_line_f32` (an
independent MA-over-MACD implementation), not a call into the core wrapper.
Peak/valley target detection is currently out of scope in `engine` (targets are
supplied as inputs — see `crates/engine/src/types.rs`), so the engine does not
borrow it from the core either. Removing the seven wrappers is therefore
invisible to `crt-research-engine`.

## `#[non_exhaustive]` migration note

When 2.0 ships, `TechnicalIndicatorError` will gain
`#[non_exhaustive]`. The migration cost for downstream code is:

- Any `match err { ... }` against `TechnicalIndicatorError` that does **not**
  already include a `_ => ...` arm becomes a compile error. Adding the arm
  is the fix; no semantic change is required.
- Code that constructs error values directly (e.g., for tests) is
  unaffected — `#[non_exhaustive]` only constrains pattern matching and
  struct-literal construction of foreign types.

**Suggested CHANGELOG text:**

> ### Changed (breaking)
> - `TechnicalIndicatorError` is now `#[non_exhaustive]`. Downstream code
>   that exhaustively `match`es against the enum must add a `_ => ...` arm.
>   No semantic change. This enables future error-variant additions as
>   non-breaking minor versions.

Cross-repo: the Python and JS bindings construct error values from the
core's `Err` returns; they do not pattern-match against them, so the
`#[non_exhaustive]` change has no impact on the binding repos.

## NaN/inf audit

The Codex review of the 2026-04-29 roadmap surfaced public APIs that can
currently produce `NaN` or `inf` for plausible-but-pathological inputs.
Each site is classified below per the taxonomy:

- **bug-fix** — emit a structured `Err`; the current NaN/inf return is
  undocumented and surprising.
- **documented current behavior** — keep returning NaN/inf, but add a
  `# Errors` or `# Returns` note in rustdoc to make the behavior explicit.
- **2.0-breaking validation change** — promote the silent NaN/inf path to
  a structured validation error; user code that relied on the legacy
  behavior would need migration.

The classification recommendations below are the agent's reading of the
codebase. Each site needs the maintainer's explicit sign-off before the
2.0 cut.

### Sites

Line numbers below are accurate as of the PR3 cut (after PR1 and PR2 land);
they will shift again as 2.0 is implemented, so use them as anchors, not
absolute references.

| # | Site | Trigger input | Current behavior | Recommended class |
|---|---|---|---|---|
| 1 | `src/momentum_indicators.rs:219` `single::stochastic_oscillator` (division at the `(max - min)` line) | flat input (`max == min`) | returns NaN | **bug-fix** — return `0.0` (matches CMO precedent in commit `0217469`). |
| 2 | `src/momentum_indicators.rs:429` `single::williams_percent_r` (division at `(max_high - min_low)`) | flat OHLC (`max_high == min_low`) | returns NaN | **bug-fix** — return `0.0`. |
| 3 | `src/momentum_indicators.rs:534` `single::rate_of_change` | `previous_price == 0.0` | returns inf or NaN | **2.0-breaking validation change** — `Err(InvalidValue { name: "previous_price", value: 0.0, reason: "must be non-zero" })`. A zero previous price is nonsensical for ROC and should be rejected loudly. |
| 4 | `src/other_indicators.rs:93` `single::return_on_investment` (and its bulk caller at line 334) | `investment == 0.0` | returns inf or NaN | **2.0-breaking validation change** — `Err(InvalidValue { name: "investment", value: 0.0, reason: "must be non-zero" })`. Note: `single::return_on_investment` currently has no `Result` return — promoting to `Result` is itself a breaking signature change. Bundle both. |
| 5 | `src/momentum_indicators.rs:1332` `single::percentage_price_oscillator` (and the bulk wrapper at line 2454) | long-period MA evaluates to `0.0` (all-zero input) | returns NaN/inf | **documented current behavior** — the trigger requires a degenerate all-zero series. Add a rustdoc note rather than a validation arm. |
| 6 | `src/strength_indicators.rs:148` `single::volume_index` (the `change = (current_close - previous_close) / previous_close` line) | `previous_close == 0.0` | returns inf/NaN | **2.0-breaking validation change** — `Err(InvalidValue { name: "previous_close", value: 0.0, reason: "must be non-zero" })`. Like site 4, `single::volume_index` returns `f64` today (no `Result`), so promoting it is itself a breaking signature change. |
| 7 | `src/trend_indicators.rs:354` `single::volume_price_trend` (and bulk at line 1174) | `previous_price == 0.0` | returns inf/NaN | **2.0-breaking validation change** — same shape as site 3. |
| 8 | `src/volatility_indicators.rs:67` `single::ulcer_index` (division by `period_max`) | `period_max == 0.0` (only possible with zero-only input) | returns NaN | **documented current behavior** — ulcer index over an all-zero series is mathematically undefined; rustdoc should state this; no validation change. |
| 9 | `src/basic_indicators.rs:838` `bulk::log` | non-positive entries in `prices` | returns NaN (silent `.ln()` of `<= 0`); only `assert_non_empty` runs before the `.ln()` map | **bug-fix** — `single::log_difference` already validates positivity. Add an equivalent per-element positivity check to `bulk::log` before the `.ln()` map. This brings `bulk::log` into parity with the rest of the log family. |

### Sequencing within 2.0

1. Land the bug-fix items (sites 1, 2, 9) first — they restore intended
   behavior and won't surprise anyone who reads the rustdoc.
2. Bundle the validation-change items (sites 3-7) into a single PR titled
   "`feat!: validate non-zero divisors in ROC / ROI / volume_index / VPT`"
   with the `Compatibility` section enumerating every now-rejected input
   shape.
3. Add the rustdoc note for site 8 as part of the same PR as site 9 (both
   touch boundary documentation).

## Open questions for the maintainer

1. **Bug-fix vs. validation for sites 1 and 2.** Returning `0.0` on flat
   input is the CMO precedent (the 2026-04 fix). It treats "no movement"
   as "no momentum signal." An alternative is to return an `Err(EmptyData)`
   (technically the data is non-empty but contains no information). The
   recommendation is `0.0` to match precedent; flag if you prefer Err.
2. **`single::ulcer_index` on all-zero input.** Confirm "document the
   NaN" is acceptable rather than rejecting at the boundary.
3. **2.0 release window.** Whether to bundle 2.0 with a downstream
   `centaur-optimizer` consumer commitment (`FromStr` in core, Python
   wrapper consolidation, etc.) or ship 2.0 first and follow up.

## Cross-repo coordination checklist (when 2.0 actually ships)

- [ ] Open the core 2.0 PR (this repo): removes the seven functions, adds
      `#[non_exhaustive]`, lands NaN/inf changes per the table above.
- [ ] Open coordinated `CentaurTechnicalIndicators-Python` PR: removes the
      four PyO3 deprecation wrappers, drops the `test_deprecations.py`
      coverage, bumps core dep to `2.0`.
- [ ] Open coordinated `CentaurTechnicalIndicators-JS` PR: removes the
      four wasm_bindgen wrappers, bumps core dep to `2.0`.
- [ ] Decide whether the CUDA kernels for the four removed concepts should
      also be retired; if yes, open a separate PR in
      `CentaurTechnicalIndicators-Rust-Cuda`.
- [ ] Tag `centaur-v2.0.0` only after the binding-repo PRs are queued and
      pass CI against the 2.0-rc tag.
