//! Property-based tests for invariants that should hold across many inputs.
//!
//! This crate ships **zero dependencies** — including dev-dependencies — so
//! these property tests do not use `proptest`. Instead they drive each
//! invariant over a fixed grid of input lengths and price scales, spreading
//! inputs within each grid cell with a hand-rolled linear congruential
//! generator (LCG) seeded from a fixed constant. The run is therefore fully
//! deterministic and reproducible: a failing case prints its exact inputs, and
//! re-running reproduces it bit-for-bit (no `--seed` shrinking dance required).
//!
//! Invariants covered (identical to the previous `proptest` version):
//!   * RSI is bounded in `[0, 100]` for positive price series.
//!   * ATR is non-negative for valid OHLC bars.
//!   * `moving_average(vec![c; n], Simple)` is the fixed point `c`.
//!   * bulk moving-average length is `prices.len() - period + 1`.
//!   * RSI, CMO, and mean produce no NaN on constant input.
//!
//! These would have flagged historical regressions like:
//!   * `relative_strength_index` returning values outside `[0, 100]` (the
//!     `previous_gains_loss` zero-drop bug fixed in commit `dc71a22`).
//!   * `chande_momentum_oscillator` returning `NaN` on a flat price series
//!     (fixed in commit `0217469`).

use centaur_technical_indicators::{
    basic_indicators, momentum_indicators, moving_average, other_indicators, ConstantModelType,
    MovingAverageType,
};

/// Minimal linear congruential generator (the MMIX / PCG multiplier and
/// increment). Dependency-free and deterministic; used only to spread inputs
/// within each grid cell, never for cryptographic or statistical purposes.
struct Lcg {
    state: u64,
}

impl Lcg {
    fn new(seed: u64) -> Self {
        Lcg { state: seed }
    }

    fn next_u64(&mut self) -> u64 {
        self.state = self
            .state
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        self.state
    }

    /// Uniform `f64` in `[0.0, 1.0)` from the top 53 bits of the next draw.
    fn unit(&mut self) -> f64 {
        (self.next_u64() >> 11) as f64 / (1u64 << 53) as f64
    }

    /// Uniform `f64` in `[lo, hi)`.
    fn range(&mut self, lo: f64, hi: f64) -> f64 {
        lo + (hi - lo) * self.unit()
    }
}

/// Derive a distinct-but-reproducible per-cell seed from the grid coordinates.
/// The increment in the LCG keeps it well-behaved even for a zero seed, so any
/// mixing that separates the cells is sufficient.
fn seed_for(a: usize, b: usize, salt: u64) -> u64 {
    0x9E37_79B9_7F4A_7C15u64
        .wrapping_add((a as u64).wrapping_mul(0x1_0000_0001))
        .wrapping_add((b as u64).wrapping_mul(0x0001_0000_0001))
        .wrapping_add(salt)
}

/// Fixed grid of input lengths exercised by the length-sensitive invariants.
/// Spans the original `proptest` range (2..=255) at representative sizes.
const LENGTHS: &[usize] = &[2, 3, 4, 5, 8, 13, 21, 34, 55, 89, 128, 200, 255];

/// Fixed grid of price magnitudes; probes floating-point behaviour across
/// several orders of magnitude up to the original `1.0e6` ceiling.
const SCALES: &[f64] = &[1.0, 100.0, 10_000.0, 1_000_000.0];

/// Trials per (length, scale) cell. Kept small so the whole suite stays fast.
const TRIALS: usize = 8;

/// RSI is bounded in `[0.0, 100.0]` (and never NaN) for positive price series.
#[test]
fn rsi_bounded_on_positive_series() {
    for (li, &len) in LENGTHS.iter().enumerate() {
        for (si, &scale) in SCALES.iter().enumerate() {
            let mut rng = Lcg::new(seed_for(li, si, 1));
            for _ in 0..TRIALS {
                let prices: Vec<f64> = (0..len).map(|_| rng.range(0.01, scale)).collect();
                let rsi = momentum_indicators::single::relative_strength_index(
                    &prices,
                    ConstantModelType::SmoothedMovingAverage,
                )
                .unwrap();
                assert!(
                    (0.0..=100.0).contains(&rsi),
                    "rsi = {rsi} out of bounds for prices {prices:?}"
                );
                assert!(!rsi.is_nan(), "rsi was NaN for prices {prices:?}");
            }
        }
    }
}

/// ATR is non-negative (and never NaN) for valid OHLC bars.
///
/// The ATR signature takes `(close, high, low)` only, so we generate those
/// three bracketing `high >= close >= low > 0` — the validity condition the
/// invariant relies on. (`open` is not part of the ATR input and is omitted.)
#[test]
fn atr_non_negative_on_valid_ohlc() {
    for (li, &len) in LENGTHS.iter().enumerate() {
        for (si, &scale) in SCALES.iter().enumerate() {
            let mut rng = Lcg::new(seed_for(li, si, 2));
            for _ in 0..TRIALS {
                let mut close = Vec::with_capacity(len);
                let mut high = Vec::with_capacity(len);
                let mut low = Vec::with_capacity(len);
                for _ in 0..len {
                    let c = rng.range(0.02, scale);
                    let h = c + rng.range(0.0, scale); // high >= close
                    let l = (c - rng.range(0.0, scale)).max(0.01); // low <= close, > 0
                    close.push(c);
                    high.push(h);
                    low.push(l);
                }
                let atr = other_indicators::single::average_true_range(
                    &close,
                    &high,
                    &low,
                    ConstantModelType::SimpleMovingAverage,
                )
                .unwrap();
                assert!(
                    atr >= 0.0,
                    "atr = {atr} was negative (close {close:?}, high {high:?}, low {low:?})"
                );
                assert!(!atr.is_nan(), "atr was NaN");
            }
        }
    }
}

/// `moving_average(vec![c; n], Simple)` is the fixed point `c`.
#[test]
fn ma_fixed_point_on_constant() {
    for (li, &n) in LENGTHS.iter().enumerate() {
        for (si, &scale) in SCALES.iter().enumerate() {
            let mut rng = Lcg::new(seed_for(li, si, 3));
            for _ in 0..TRIALS {
                let value = rng.range(0.01, scale);
                let prices = vec![value; n];
                let ma = moving_average::single::moving_average(&prices, MovingAverageType::Simple)
                    .unwrap();
                // Relative tolerance accommodates floating-point summation error
                // at large input magnitudes. Absolute precision is ~ulp(value)*n.
                assert!(
                    (ma - value).abs() < value.abs() * 1e-12 + 1e-12,
                    "moving_average of constant {value} (n={n}) was {ma} (delta {})",
                    (ma - value).abs()
                );
            }
        }
    }
}

/// Bulk moving-average length invariant: `len(prices) - period + 1`.
#[test]
fn bulk_ma_length_invariant() {
    const PERIODS: &[usize] = &[2, 3, 5, 8, 13, 21, 34];
    for (pi, &period) in PERIODS.iter().enumerate() {
        // Lengths from `period` up to `period * 4`, mirroring the old strategy.
        for mult in 1..=4 {
            let len = period * mult;
            let mut rng = Lcg::new(seed_for(pi, mult, 4));
            let prices: Vec<f64> = (0..len).map(|_| rng.range(1.0, 1.0e6)).collect();
            let mas =
                moving_average::bulk::moving_average(&prices, MovingAverageType::Simple, period)
                    .unwrap();
            assert_eq!(
                mas.len(),
                prices.len() - period + 1,
                "bulk MA length mismatch (len={len}, period={period})"
            );
        }
    }
}

/// No NaN on flat (constant) input. Regression test for the CMO bug (commit
/// `0217469`) and the RSI zero-drop bug (commit `dc71a22`). The invariant is
/// about *constant* inputs, so a deterministic grid over `(value, n)` covers
/// the space directly — no LCG needed here.
#[test]
fn flat_input_no_nan() {
    const VALUES: &[f64] = &[0.01, 0.5, 1.0, 2.0, 100.0, 1234.5, 1.0e4, 1.0e6];
    const NS: &[usize] = &[2, 3, 5, 8, 13, 21, 55];
    for &value in VALUES {
        for &n in NS {
            let prices = vec![value; n];

            // RSI on flat input has no gains and no losses -> 0.0, not NaN.
            let rsi = momentum_indicators::single::relative_strength_index(
                &prices,
                ConstantModelType::SmoothedMovingAverage,
            )
            .unwrap();
            assert!(
                !rsi.is_nan(),
                "rsi was NaN on flat input (value={value}, n={n})"
            );

            // CMO on flat input should return 0.0 (the bug returned NaN).
            let cmo = momentum_indicators::single::chande_momentum_oscillator(&prices).unwrap();
            assert!(
                !cmo.is_nan(),
                "cmo was NaN on flat input (value={value}, n={n})"
            );

            // Mean of a constant must equal the constant (relative tolerance for
            // FP summation drift at large magnitudes).
            let mean = basic_indicators::single::mean(&prices).unwrap();
            assert!(
                (mean - value).abs() < value.abs() * 1e-12 + 1e-12,
                "mean of constant {value} was {mean} (delta {})",
                (mean - value).abs()
            );
        }
    }
}
