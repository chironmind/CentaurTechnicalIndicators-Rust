// Bollinger Bands via the generalised `moving_constant_bands`.
//
// Bollinger Bands are the special case of `moving_constant_bands` that uses
// the simple moving average as the centre line and the standard deviation as
// the dispersion measure. Swap either model to roll your own variant.
//
// Run with:
//   cargo run --example bollinger

use centaur_technical_indicators::{candle_indicators, ConstantModelType, DeviationModel};

fn main() {
    let prices = vec![
        100.2, 100.46, 100.53, 100.38, 100.19, 100.21, 100.32, 100.28, 100.55, 100.71,
    ];

    // Classical Bollinger Bands: SMA centre, 2 standard deviations either side.
    let (lower, mid, upper) = candle_indicators::single::moving_constant_bands(
        &prices,
        ConstantModelType::SimpleMovingAverage,
        DeviationModel::StandardDeviation,
        2.0,
    )
    .unwrap();
    println!(
        "Bollinger (SMA / StdDev / 2.0): lower={:.4} mid={:.4} upper={:.4}",
        lower, mid, upper
    );

    // Robust variant: median centre, median absolute deviation, same multiplier.
    let (lower2, mid2, upper2) = candle_indicators::single::moving_constant_bands(
        &prices,
        ConstantModelType::SimpleMovingMedian,
        DeviationModel::MedianAbsoluteDeviation,
        2.0,
    )
    .unwrap();
    println!(
        "Robust bands (Median / MedianAD / 2.0): lower={:.4} mid={:.4} upper={:.4}",
        lower2, mid2, upper2
    );
}
