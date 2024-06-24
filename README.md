# RustTI

A configurable Technical Indicators library for Rust

Everything is configurable in the RustTI functions.

Many of the functions accept parameters that allow the caller to move the technical
indicators away from their default behaviour. For example, if a TI uses the mean to calculate
the indicator, it can be told to use the median, or mode instead.

For this reason, RustTI is a more advanced Technical Inidcators package, and users should
have some knowledge of the indicators they plan on using.

* [Install](#install)
* [Documentation](#documentation)
* [Examples](#examples)
* [Available indicators](#available-indicators)

## Install

Run the following Cargo command in your project directory:

```shell
cargo add rust_ti
```

Or add the following line to your Cargo.toml:

```
rust_ti = "1.0.1" 
```

## Documentation

Documentation can be found here: [rust_ti](https://docs.rs/rust_ti/latest/rust_ti/)

## Examples

### Simple Example

Single example, where the moving average needs to be calculated for the entire vector

```rust
use rust_ti;

let prices = vec![100.2, 100.46, 100.53, 100.38, 100.19];

let ma = rust_ti::moving_average::single::moving_average(
    &prices,
    &rust_ti::MovingAverageType::Simple
);
assert_eq!(100.352, ma);

let sma = rust_ti::moving_average::single::moving_average(
    &prices,
    &rust_ti::MovingAverageType::Smoothed
);
assert_eq!(100.34228938600666, sma);

let ema = rust_ti::moving_average::single::moving_average(
    &prices,
    &rust_ti::MovingAverageType::Exponential
);
assert_eq!(100.32810426540287, ema);

// The values used in the example for the personalised moving average are random.
// If using the PMA, it is recommended to look into how the moving averages are calculated before using values.
let pma = rust_ti::moving_average::single::moving_average(
    &prices,
    &rust_ti::MovingAverageType::Personalised(&5.0, &3.0)
);
assert_eq!(100.27405995388162, personalised_ma);
```

Bulk example, where the moving average is calculated for a period.

Behind the scenes, the function will be calculating the moving average for a period of 3. The slices 
`[100.2, 100.46, 100.53]`, `[100.46, 100.53, 100.38]`, and `[100.53, 100.38, 100.19]` will be derived from
`prices`. The function will return the moving averages for the 3 periods as a vector.

```rust
let prices = vec![100.2, 100.46, 100.53, 100.38, 100.19];
let period: usize = 3;

let ma = rust_ti::moving_average::bulk::moving_average(
    &prices,
    &rust_ti::MovingAverageType::Simple,
    &period
);
assert_eq!(
    vec![100.39666666666666, 100.456666666666666, 100.36666666666667],
    ma
);

let sma = rust_ti::moving_average::bulk::moving_average(
    &prices,
    &rust_ti::MovingAverageType::Smoothed,
    &period
);
assert_eq!(
    vec![100.43842105263158, 100.4442105263158, 100.32157894736842],
    sma
);

let ema = rust_ti::moving_average::bulk::moving_average(
    &prices,
    &rust_ti::MovingAverageType::Exponential,
    &period
);
assert_eq!(
    vec![100.46285714285715, 100.4342857142857, 100.29285714285713],
    pma
);

let pma = rust_ti::moving_average::bulk::moving_average(
    &prices,
    &rust_ti::MovingAverageType::Personalised(&5.0, &3.0),
    &period
);
assert_eq!(
    vec![100.5125581395349, 100.40279069767443, 100.22441860465118],
    pma
);
```

### S&P 500 Example

An example using the Rust TI for the S&P 500 can be found in [GitHub](https://github.com/0100101001010000/rustTI/blob/main/examples/main.rs)

and under `examples/main.rs`

The code in `examples/main.rs` can be run by cloning the repo, and running:
```shell
cargo build
cargo run --example sp500
```

## Available indicators:

The indicators a split into two modules. 

`Bulk` to calculate indicators over a vector of prices for a given period.

`Single` to calculate indicators once 

### Standard Indicators

#### Bulk

* [simple_moving_average]()
* [smoothed_moving_average]()
* [exponential_moving_average]()
* [bollinger_bands]()
* [macd]()
* [rsi]()

#### Single 

* [simple_moving_average]()
* [smoothed_moving_average]()
* [exponential_moving_average]()
* [bollinger_bands]()
* [macd]()
* [rsi]()

#### Single

### Basic Indicators

#### Bulk
         
* [absolute_deviation](https://docs.rs/rust_ti/latest/rust_ti/basic_indicators/bulk/fn.absolute_deviation.html)
* [log](https://docs.rs/rust_ti/latest/rust_ti/basic_indicators/bulk/fn.log.html)
* [log_difference](https://docs.rs/rust_ti/latest/rust_ti/basic_indicators/bulk/fn.log_difference.html)
* [mean](https://docs.rs/rust_ti/latest/rust_ti/basic_indicators/bulk/fn.mean.html) 
* [median](https://docs.rs/rust_ti/latest/rust_ti/basic_indicators/bulk/fn.median.html) 
* [mode](https://docs.rs/rust_ti/latest/rust_ti/basic_indicators/bulk/fn.mode.html) 
* [standard_deviation](https://docs.rs/rust_ti/latest/rust_ti/basic_indicators/bulk/fn.standard_deviation.html) 
* [variance](https://docs.rs/rust_ti/latest/rust_ti/basic_indicators/bulk/fn.variance.html)

#### Single

* [absolute_deviation](https://docs.rs/rust_ti/latest/rust_ti/basic_indicators/single/fn.absolute_deviation.html)
* [log_difference](https://docs.rs/rust_ti/latest/rust_ti/basic_indicators/single/fn.log_difference.html)
* [max](https://docs.rs/rust_ti/latest/rust_ti/basic_indicators/single/fn.max.html)
* [mean](https://docs.rs/rust_ti/latest/rust_ti/basic_indicators/single/fn.mean.html)
* [median](https://docs.rs/rust_ti/latest/rust_ti/basic_indicators/single/fn.median.html) 
* [min](https://docs.rs/rust_ti/latest/rust_ti/basic_indicators/single/fn.min.html) 
* [mode](https://docs.rs/rust_ti/latest/rust_ti/basic_indicators/single/fn.mode.html) 
* [standard_deviation](https://docs.rs/rust_ti/latest/rust_ti/basic_indicators/single/fn.standard_deviation.html)
* [variance](https://docs.rs/rust_ti/latest/rust_ti/basic_indicators/single/fn.variance.html)  

### Candle Indicators

#### Bulk

* [ichimoku_cloud](https://docs.rs/rust_ti/latest/rust_ti/candle_indicators/bulk/fn.ichimoku_cloud.html) 
* [mcginley_dynamic_bands](https://docs.rs/rust_ti/latest/rust_ti/candle_indicators/bulk/fn.mcginley_dynamic_bands.html) 
* [mcginley_dynamic_envelopes](https://docs.rs/rust_ti/latest/rust_ti/candle_indicators/bulk/fn.mcginley_dynamic_envelopes.html) 
* [moving_constant_bands](https://docs.rs/rust_ti/latest/rust_ti/candle_indicators/bulk/fn.moving_constant_bands.html) 
* [moving_constant_envelopes](https://docs.rs/rust_ti/latest/rust_ti/candle_indicators/bulk/fn.moving_constant_envelopes.html) 

#### Single

* [ichimoku_cloud](https://docs.rs/rust_ti/latest/rust_ti/candle_indicators/single/fn.ichimoku_cloud.html) 
* [mcginley_dynamic_bands](https://docs.rs/rust_ti/latest/rust_ti/candle_indicators/single/fn.mcginley_dynamic_bands.html) 
* [mcginley_dynamic_envelopes](https://docs.rs/rust_ti/latest/rust_ti/candle_indicators/single/fn.mcginley_dynamic_envelopes.html) 
* [moving_constant_bands](https://docs.rs/rust_ti/latest/rust_ti/candle_indicators/single/fn.moving_constant_bands.html) 
* [moving_constant_envelopes](https://docs.rs/rust_ti/latest/rust_ti/candle_indicators/single/fn.moving_constant_envelopes.html) 

### Chart Trends

* [break_down_trends](https://docs.rs/rust_ti/latest/rust_ti/chart_trends/fn.break_down_trends.html) 
* [overall_trend](https://docs.rs/rust_ti/latest/rust_ti/chart_trends/fn.overall_trend.html) 
* [peak_trend](https://docs.rs/rust_ti/latest/rust_ti/chart_trends/fn.peak_trend.html) 
* [peaks](https://docs.rs/rust_ti/latest/rust_ti/chart_trends/fn.peaks.html) 
* [valley_trend](https://docs.rs/rust_ti/latest/rust_ti/chart_trends/fn.valley_trend.html) 
* [valleys](https://docs.rs/rust_ti/latest/rust_ti/chart_trends/fn.valleys.html) 

### Correlation indicators

#### Bulk

* [correlate_asset_prices](https://docs.rs/rust_ti/latest/rust_ti/correlation_indicators/bulk/fn.correlate_asset_prices.html) 

#### Single

* [correlate_asset_prices](https://docs.rs/rust_ti/latest/rust_ti/correlation_indicators/single/fn.correlate_asset_prices.html) 

### Momentum Indicators

#### Bulk

* [chaikin_oscillator](https://docs.rs/rust_ti/latest/rust_ti/momentum_indicators/bulk/fn.chaikin_oscillator.html)
* [commodity_channel_index](https://docs.rs/rust_ti/latest/rust_ti/momentum_indicators/bulk/fn.commodity_channel_index.html) 
* [macd_line](https://docs.rs/rust_ti/latest/rust_ti/momentum_indicators/bulk/fn.macd_line.html) 
* [mcginley_dynamic_chaikin_oscillator](https://docs.rs/rust_ti/latest/rust_ti/momentum_indicators/bulk/fn.mcginley_dynamic_chaikin_oscillator.html) 
* [mcginley_dynamic_commodity_channel_index](https://docs.rs/rust_ti/latest/rust_ti/momentum_indicators/bulk/fn.mcginley_dynamic_commodity_channel_index.html) 
* [mcginley_dynamic_macd_line](https://docs.rs/rust_ti/latest/rust_ti/momentum_indicators/bulk/fn.mcginley_dynamic_macd_line.html) 
* [mcginley_dynamic_rsi](https://docs.rs/rust_ti/latest/rust_ti/momentum_indicators/bulk/fn.mcginley_dynamic_rsi.html) 
* [money_flow_index](https://docs.rs/rust_ti/latest/rust_ti/momentum_indicators/bulk/fn.money_flow_index.html)
* [on_balance_volume](https://docs.rs/rust_ti/latest/rust_ti/momentum_indicators/bulk/fn.on_balance_volume.html) 
* [rate_of_change](https://docs.rs/rust_ti/latest/rust_ti/momentum_indicators/bulk/fn.rate_of_change.html) 
* [relative_strength_index](https://docs.rs/rust_ti/latest/rust_ti/momentum_indicators/bulk/fn.relative_strength_index.html) 
* [signal_line](https://docs.rs/rust_ti/latest/rust_ti/momentum_indicators/bulk/fn.signal_line.html) 
* [slow_stochastic](https://docs.rs/rust_ti/latest/rust_ti/momentum_indicators/bulk/fn.slow_stochastic.html) 
* [slowest_stochastic](https://docs.rs/rust_ti/latest/rust_ti/momentum_indicators/bulk/fn.slowest_stochastic.html)
* [stochastic_oscillator](https://docs.rs/rust_ti/latest/rust_ti/momentum_indicators/bulk/fn.stochastic_oscillator.html) 
* [williams_percent_r](https://docs.rs/rust_ti/latest/rust_ti/momentum_indicators/bulk/fn.williams_percent_r.html)

#### Single

* [chaikin_oscillator](https://docs.rs/rust_ti/latest/rust_ti/momentum_indicators/single/fn.chaikin_oscillator.html) 
* [commodity_channel_index](https://docs.rs/rust_ti/latest/rust_ti/momentum_indicators/single/fn.commodity_channel_index.html) 
* [macd_line](https://docs.rs/rust_ti/latest/rust_ti/momentum_indicators/single/fn.macd_line.html)
* [mcginley_dynamic_chaikin_oscillator](https://docs.rs/rust_ti/latest/rust_ti/momentum_indicators/single/fn.mcginley_dynamic_chaikin_oscillator.html)
* [mcginley_dynamic_commodity_channel_index](https://docs.rs/rust_ti/latest/rust_ti/momentum_indicators/single/fn.mcginley_dynamic_commodity_channel_index.html)
* [mcginley_dynamic_macd_line](https://docs.rs/rust_ti/latest/rust_ti/momentum_indicators/single/fn.mcginley_dynamic_macd_line.html) 
* [mcginley_dynamic_rsi](https://docs.rs/rust_ti/latest/rust_ti/momentum_indicators/single/fn.mcginley_dynamic_rsi.html) 
* [money_flow_index](https://docs.rs/rust_ti/latest/rust_ti/momentum_indicators/single/fn.money_flow_index.html)
* [on_balance_volume](https://docs.rs/rust_ti/latest/rust_ti/momentum_indicators/single/fn.on_balance_volume.html)
* [rate_of_change](https://docs.rs/rust_ti/latest/rust_ti/momentum_indicators/single/fn.rate_of_change.html) 
* [relative_strength_index](https://docs.rs/rust_ti/latest/rust_ti/momentum_indicators/single/fn.relative_strength_index.html)
* [signal_line](https://docs.rs/rust_ti/latest/rust_ti/momentum_indicators/single/fn.signal_line.html)
* [slow_stochastic](https://docs.rs/rust_ti/latest/rust_ti/momentum_indicators/single/fn.slow_stochastic.html)
* [slowest_stochastic](https://docs.rs/rust_ti/latest/rust_ti/momentum_indicators/single/fn.slowest_stochastic.html) 
* [stochastic_oscillator](https://docs.rs/rust_ti/latest/rust_ti/momentum_indicators/single/fn.stochastic_oscillator.html)
* [williams_percent_r](https://docs.rs/rust_ti/latest/rust_ti/momentum_indicators/single/fn.williams_percent_r.html)

### Moving Averages

#### Bulk

* [mcginley_dynamic](https://docs.rs/rust_ti/latest/rust_ti/moving_average/bulk/fn.mcginley_dynamic.html)
* [moving_average](https://docs.rs/rust_ti/latest/rust_ti/moving_average/bulk/fn.moving_average.html)

#### Single

* [mcginley_dynamic](https://docs.rs/rust_ti/latest/rust_ti/moving_average/bulk/fn.mcginley_dynamic.html)
* [moving_average](https://docs.rs/rust_ti/latest/rust_ti/moving_average/bulk/fn.moving_average.html)

### Other Indicators

#### Bulk

* [return_on_investment](https://docs.rs/rust_ti/latest/rust_ti/other_indicators/bulk/fn.return_on_investment.html)

#### Single

* [return_on_investment](https://docs.rs/rust_ti/latest/rust_ti/other_indicators/single/fn.return_on_investment.html) 

### Strength Indicators

#### Bulk

* [accumulation_distribution](https://docs.rs/rust_ti/latest/rust_ti/strength_indicators/bulk/fn.accumulation_distribution.html)

#### Single

* [accumulation_distribution](https://docs.rs/rust_ti/latest/rust_ti/strength_indicators/single/fn.accumulation_distribution.html) 

### Trend Indicators

#### Bulk

* [aroon_down](https://docs.rs/rust_ti/latest/rust_ti/trend_indicators/bulk/fn.aroon_down.html)
* [aroon_indicator](https://docs.rs/rust_ti/latest/rust_ti/trend_indicators/bulk/fn.aroon_indicator.html) 
* [aroon_oscillator](https://docs.rs/rust_ti/latest/rust_ti/trend_indicators/bulk/fn.aroon_oscillator.html)
* [aroon_up](https://docs.rs/rust_ti/latest/rust_ti/trend_indicators/bulk/fn.aroon_up.html)
* [parabolic_time_price_system](https://docs.rs/rust_ti/latest/rust_ti/trend_indicators/bulk/fn.parabolic_time_price_system.html) 

#### Single

* [aroon_down](https://docs.rs/rust_ti/latest/rust_ti/trend_indicators/single/fn.aroon_down.html) 
* [aroon_indicator](https://docs.rs/rust_ti/latest/rust_ti/trend_indicators/single/fn.aroon_indicator.html)
* [aroon_oscillator](https://docs.rs/rust_ti/latest/rust_ti/trend_indicators/single/fn.aroon_oscillator.html)
* [aroon_up](https://docs.rs/rust_ti/latest/rust_ti/trend_indicators/single/fn.aroon_up.html)
* [long_parabolic_time_price_system](https://docs.rs/rust_ti/latest/rust_ti/trend_indicators/single/fn.long_parabolic_time_price_system.html)
* [short_parabolic_time_price_system](https://docs.rs/rust_ti/latest/rust_ti/trend_indicators/single/fn.short_parabolic_time_price_system.html) 

### Volatility Indicators

#### Bulk

* [ulcer_index](https://docs.rs/rust_ti/latest/rust_ti/volatility_indicators/bulk/fn.ulcer_index.html)

#### Single

* [ulcer_index](https://docs.rs/rust_ti/latest/rust_ti/volatility_indicators/single/fn.ulcer_index.html)

