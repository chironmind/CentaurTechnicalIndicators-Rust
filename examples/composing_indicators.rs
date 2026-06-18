// Composing indicators: feed the output of one rolling indicator into another.
//
// The library deliberately ships small composable building blocks instead of
// every possible combined indicator. This example shows the recommended
// pattern: pre-smooth prices with a moving average, then run a momentum
// indicator on the smoothed series.
//
// Run with:
//   cargo run --example composing_indicators

use centaur_technical_indicators::{
    momentum_indicators, moving_average, ConstantModelType, MovingAverageType,
};

fn main() {
    // 30 mostly-trending prices.
    let prices: Vec<f64> = (0..30).map(|i| 100.0 + (i as f64) * 0.1).collect();

    // Step 1: smooth the raw prices with a 5-period exponential moving average.
    let smoothed =
        moving_average::bulk::moving_average(&prices, MovingAverageType::Exponential, 5).unwrap();
    println!(
        "Smoothed series ({} values): {:?}",
        smoothed.len(),
        smoothed
    );

    // Step 2: compute RSI on the smoothed series. This is the common "smoothed
    // RSI" recipe — no library-side support is needed because indicators
    // compose by feeding outputs into inputs.
    let rsi = momentum_indicators::single::relative_strength_index(
        &smoothed,
        ConstantModelType::SmoothedMovingAverage,
    )
    .unwrap();
    println!("RSI of smoothed series: {:.4}", rsi);
}
