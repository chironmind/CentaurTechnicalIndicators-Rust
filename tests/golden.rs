//! Golden-value tests on the public API surface.
//!
//! Each test pins one representative indicator to a hand-calculated expected
//! value sourced from `assets/centaur_ti_hand_calcs.ods` (and mirrored in the
//! per-module unit tests). They serve two purposes:
//!
//!   1. Exercise the public re-export surface — if a future refactor breaks
//!      the crate-root `pub use`, these fail before any user does.
//!   2. Lock in indicator semantics. Any numerical drift surfaces as a
//!      failing assertion, independent of internal restructuring.

use centaur_technical_indicators::{
    basic_indicators, candle_indicators, momentum_indicators, moving_average, other_indicators,
    trend_indicators, ConstantModelType, MovingAverageType,
};

#[test]
fn moving_average_simple_5_prices() {
    let prices = vec![100.2, 100.46, 100.53, 100.38, 100.19];
    let ma = moving_average::single::moving_average(&prices, MovingAverageType::Simple).unwrap();
    assert_eq!(ma, 100.352);
}

#[test]
fn variance_5_prices() {
    let prices = vec![100.2, 100.46, 100.53, 100.38, 100.19];
    let variance = basic_indicators::single::variance(&prices).unwrap();
    assert_eq!(variance, 0.018695999999999734);
}

#[test]
fn rsi_smoothed_5_prices() {
    let prices = vec![100.2, 100.46, 100.53, 100.38, 100.19];
    let rsi = momentum_indicators::single::relative_strength_index(
        &prices,
        ConstantModelType::SmoothedMovingAverage,
    )
    .unwrap();
    assert_eq!(rsi, 33.010380622837275);
}

#[test]
fn chande_momentum_oscillator_5_prices() {
    let prices = vec![100.01, 100.44, 100.39, 100.63, 100.71];
    let cmo = momentum_indicators::single::chande_momentum_oscillator(&prices).unwrap();
    assert_eq!(cmo, 87.50000000000044);
}

#[test]
fn money_flow_index_8_prices() {
    let prices = vec![
        100.2, 100.46, 100.53, 100.38, 100.19, 100.21, 100.32, 100.28,
    ];
    let volume = vec![1200.0, 1400.0, 1450.0, 1100.0, 900.0, 875.0, 1025.0, 1100.0];
    let mfi = momentum_indicators::single::money_flow_index(&prices, &volume).unwrap();
    assert_eq!(mfi, 63.40886336843541);
}

#[test]
fn average_true_range_simple_3_bars() {
    let close = vec![100.46, 100.53, 100.38];
    let high = vec![101.12, 101.3, 100.11];
    let low = vec![100.29, 100.87, 99.94];
    let atr = other_indicators::single::average_true_range(
        &close,
        &high,
        &low,
        ConstantModelType::SimpleMovingAverage,
    )
    .unwrap();
    assert_eq!(atr, 0.6799999999999974);
}

#[test]
fn aroon_up_4_highs() {
    let highs = vec![101.26, 102.57, 102.32, 100.69];
    let aroon_up = trend_indicators::single::aroon_up(&highs).unwrap();
    assert_eq!(aroon_up, 33.33333333333333);
}

#[test]
fn ichimoku_cloud_7_bars() {
    let highs = vec![101.26, 102.57, 102.32, 100.69, 100.83, 101.73, 102.01];
    let lows = vec![100.08, 98.75, 100.14, 98.98, 99.07, 100.1, 99.96];
    let close = vec![100.46, 100.53, 100.38, 100.19, 100.21, 100.32, 100.28];
    let cloud = candle_indicators::single::ichimoku_cloud(&highs, &lows, &close, 3, 5, 7).unwrap();
    assert_eq!(cloud, (100.595, 100.66, 100.65, 100.53999999999999, 100.38));
}

#[test]
fn donchian_channels_5_bars() {
    let highs = vec![101.26, 102.57, 102.32, 100.69, 100.83];
    let lows = vec![100.08, 98.75, 100.14, 98.98, 99.07];
    let dc = candle_indicators::single::donchian_channels(&highs, &lows).unwrap();
    assert_eq!(dc, (98.75, 100.66, 102.57));
}

#[test]
fn supertrend_5_bars() {
    let highs = vec![101.26, 102.57, 102.32, 100.69, 100.83];
    let lows = vec![100.08, 98.75, 100.14, 98.98, 99.07];
    let close = vec![100.94, 101.27, 100.55, 99.01, 100.43];
    let st = candle_indicators::single::supertrend(
        &highs,
        &lows,
        &close,
        ConstantModelType::SimpleMovingAverage,
        2.0,
    )
    .unwrap();
    assert_eq!(st, 104.91999999999999);
}

#[test]
fn moving_constant_envelopes_simple_ma_5_prices() {
    let prices = vec![100.46, 100.53, 100.38, 100.19, 100.21];
    let env = candle_indicators::single::moving_constant_envelopes(
        &prices,
        ConstantModelType::SimpleMovingAverage,
        3.0,
    )
    .unwrap();
    assert_eq!(env, (97.34338, 100.354, 103.36462));
}
