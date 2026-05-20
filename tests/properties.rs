//! Property-based tests for invariants that should hold across many inputs.
//!
//! These complement the per-module unit tests and would have flagged historical
//! regressions like:
//!   * `relative_strength_index` returning values outside [0, 100] (the
//!     `previous_gains_loss` zero-drop bug fixed in commit `dc71a22`).
//!   * `chande_momentum_oscillator` returning `NaN` on a flat price series
//!     (fixed in commit `0217469`).

use centaur_technical_indicators::{
    basic_indicators, momentum_indicators, moving_average, other_indicators, ConstantModelType,
    MovingAverageType,
};
use proptest::prelude::*;

/// Strategy: vector of finite, modestly-bounded prices with at least 2 entries.
/// Prices are constrained to `(0.0, 1.0e6]` so the natural-log / division-based
/// indicators stay in their valid domain.
fn prices() -> impl Strategy<Value = Vec<f64>> {
    prop::collection::vec(1.0f64..1.0e6, 2..256)
}

/// Strategy: a vector long enough for windowed `bulk::` operations with a
/// realistic period range.
fn prices_with_period() -> impl Strategy<Value = (Vec<f64>, usize)> {
    (3usize..64).prop_flat_map(|period| {
        prop::collection::vec(1.0f64..1.0e6, period..period * 4).prop_map(move |v| (v, period))
    })
}

/// Strategy: aligned (close, high, low) triples where `low <= close <= high`.
fn ohlc() -> impl Strategy<Value = (Vec<f64>, Vec<f64>, Vec<f64>)> {
    prop::collection::vec((1.0f64..1.0e5, 0.0f64..1.0e5, 0.0f64..1.0e5), 2..128).prop_map(|rows| {
        let mut close = Vec::with_capacity(rows.len());
        let mut high = Vec::with_capacity(rows.len());
        let mut low = Vec::with_capacity(rows.len());
        for (c, h_offset, l_offset) in rows {
            let h = c + h_offset;
            let l = (c - l_offset).max(0.01);
            close.push(c);
            high.push(h);
            low.push(l);
        }
        (close, high, low)
    })
}

proptest! {
    /// RSI is bounded in `[0.0, 100.0]`.
    #[test]
    fn rsi_bounded(prices in prices()) {
        let rsi = momentum_indicators::single::relative_strength_index(
            &prices,
            ConstantModelType::SmoothedMovingAverage,
        )
        .unwrap();
        prop_assert!((0.0..=100.0).contains(&rsi), "rsi = {} out of bounds", rsi);
        prop_assert!(!rsi.is_nan(), "rsi was NaN");
    }

    /// ATR is non-negative for valid OHLC.
    #[test]
    fn atr_non_negative((close, high, low) in ohlc()) {
        let atr = other_indicators::single::average_true_range(
            &close,
            &high,
            &low,
            ConstantModelType::SimpleMovingAverage,
        )
        .unwrap();
        prop_assert!(atr >= 0.0, "atr = {} was negative", atr);
        prop_assert!(!atr.is_nan(), "atr was NaN");
    }

    /// `moving_average(vec![c; n], Simple)` is the fixed point `c`.
    #[test]
    fn ma_fixed_point_on_constant(value in 0.01f64..1.0e6, n in 2usize..128) {
        let prices = vec![value; n];
        let ma = moving_average::single::moving_average(
            &prices,
            MovingAverageType::Simple,
        )
        .unwrap();
        // Relative tolerance accommodates floating-point summation error at
        // large input magnitudes. Absolute precision is ~ulp(value) * n.
        prop_assert!(
            (ma - value).abs() < value.abs() * 1e-12 + 1e-12,
            "moving_average of constant {} was {} (delta {})",
            value,
            ma,
            (ma - value).abs()
        );
    }

    /// Bulk moving average length invariant: `len(prices) - period + 1`.
    #[test]
    fn bulk_ma_length((prices, period) in prices_with_period()) {
        let mas = moving_average::bulk::moving_average(
            &prices,
            MovingAverageType::Simple,
            period,
        )
        .unwrap();
        prop_assert_eq!(mas.len(), prices.len() - period + 1);
    }

    /// No NaN on flat input. Regression test for the CMO bug (commit 0217469)
    /// and the RSI zero-drop bug (commit dc71a22). Any constant input series
    /// must produce finite, non-NaN outputs for these indicators.
    #[test]
    fn flat_input_no_nan(value in 0.01f64..1.0e6, n in 2usize..64) {
        let prices = vec![value; n];

        // RSI on flat input should return 0.0 (no gains, no losses).
        let rsi = momentum_indicators::single::relative_strength_index(
            &prices,
            ConstantModelType::SmoothedMovingAverage,
        )
        .unwrap();
        prop_assert!(!rsi.is_nan(), "rsi was NaN on flat input");

        // CMO on flat input should return 0.0 (the bug returned NaN).
        let cmo = momentum_indicators::single::chande_momentum_oscillator(&prices).unwrap();
        prop_assert!(!cmo.is_nan(), "cmo was NaN on flat input");

        // Mean of a constant must equal the constant (relative tolerance for
        // FP summation drift at large magnitudes).
        let mean = basic_indicators::single::mean(&prices).unwrap();
        prop_assert!(
            (mean - value).abs() < value.abs() * 1e-12 + 1e-12,
            "mean of constant {} was {} (delta {})",
            value,
            mean,
            (mean - value).abs()
        );
    }
}
