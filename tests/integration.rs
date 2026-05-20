//! Smoke tests for the public re-export surface.
//!
//! These tests only use `centaur_technical_indicators::...` paths (the same
//! surface a downstream crate sees). They catch breakage in the crate-root
//! `pub use` wiring even when per-module unit tests still pass.

use centaur_technical_indicators::{
    basic_indicators, momentum_indicators, moving_average, AbsDevConfig, CentralPoint,
    ConstantModelType, DeviationAggregate, DeviationModel, MovingAverageType, Position,
    TechnicalIndicatorError,
};

#[test]
fn crate_root_reexports_compile() {
    // Just constructing each re-exported type proves the path resolves.
    let _ma: MovingAverageType = MovingAverageType::Exponential;
    let _cm: ConstantModelType = ConstantModelType::SimpleMovingAverage;
    let _dm: DeviationModel = DeviationModel::StandardDeviation;
    let _cp: CentralPoint = CentralPoint::Mean;
    let _da: DeviationAggregate = DeviationAggregate::Median;
    let _pos: Position = Position::Long;
    let _cfg = AbsDevConfig {
        center: CentralPoint::Median,
        aggregate: DeviationAggregate::Median,
    };
}

#[test]
fn moving_average_roundtrip_via_public_api() {
    let prices = vec![100.2, 100.46, 100.53, 100.38, 100.19];

    let single_ma =
        moving_average::single::moving_average(&prices, MovingAverageType::Simple).unwrap();
    let bulk_ma =
        moving_average::bulk::moving_average(&prices, MovingAverageType::Simple, prices.len())
            .unwrap();

    // bulk over the full window should match the single value.
    assert_eq!(bulk_ma.len(), 1);
    assert_eq!(bulk_ma[0], single_ma);
}

#[test]
fn chained_pipeline_bulk_ma_then_rsi() {
    // Verify that a downstream caller can chain bulk -> single across modules.
    let prices: Vec<f64> = (0..30).map(|i| 100.0 + (i as f64) * 0.1).collect();

    let smoothed_prices =
        moving_average::bulk::moving_average(&prices, MovingAverageType::Smoothed, 3).unwrap();

    let rsi = momentum_indicators::single::relative_strength_index(
        &smoothed_prices,
        ConstantModelType::SmoothedMovingAverage,
    )
    .unwrap();
    assert!((0.0..=100.0).contains(&rsi));
}

#[test]
fn error_variant_visible_at_crate_root() {
    // The `TechnicalIndicatorError` type is re-exported and a returned `Err`
    // can be inspected by a downstream caller.
    let empty: Vec<f64> = Vec::new();
    let err = basic_indicators::single::mean(&empty).unwrap_err();
    assert!(matches!(err, TechnicalIndicatorError::EmptyData { .. }));
}

#[test]
fn display_impls_roundtrip_via_public_api() {
    // Display is part of the API surface (added this cycle). Test via the
    // crate-root re-export to confirm the impl is reachable.
    assert_eq!(MovingAverageType::Simple.to_string(), "simple");
    assert_eq!(
        ConstantModelType::ExponentialMovingAverage.to_string(),
        "exponential_moving_average"
    );
    assert_eq!(Position::Long.to_string(), "long");
}
