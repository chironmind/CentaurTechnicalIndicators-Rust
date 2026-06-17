// Minimal RSI example.
//
// Run with:
//   cargo run --example rsi

use centaur_technical_indicators::{momentum_indicators, ConstantModelType};

fn main() {
    // A small sample series of closing prices.
    let prices = vec![
        100.2, 100.46, 100.53, 100.38, 100.19, 100.21, 100.32, 100.28, 100.55, 100.71, 100.83,
        100.49, 100.26, 100.62, 101.05,
    ];

    // `single::relative_strength_index` computes one RSI value over the whole slice.
    let single_rsi = momentum_indicators::single::relative_strength_index(
        &prices,
        ConstantModelType::SmoothedMovingAverage,
    )
    .unwrap();
    println!("Single RSI over {} prices: {:.4}", prices.len(), single_rsi);

    // `bulk::relative_strength_index` produces an RSI for each rolling window.
    let period = 7;
    let bulk_rsi = momentum_indicators::bulk::relative_strength_index(
        &prices,
        ConstantModelType::SmoothedMovingAverage,
        period,
    )
    .unwrap();
    println!("Bulk RSI (period = {}): {:?}", period, bulk_rsi);
}
