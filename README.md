# RustTI

A Technical Indicators library for Rust

What differentiates RustTI from other Technical Indicator packages is how configurable the
functions are.

Many models were created decades ago when the work weeks were different (such as RSI, SO,
Ichimoku cloud...) and the observations were made daily. For example, if one decides to study
 stocks the work week is 5 days, but if one studies cryptocurrencies the work week is 7 days.

RustTI allows the caller to determine their own period based on the market being studied.
Everything is configurable in the RustTI functions, from period to moving average models.

A lot of online articles recommend sticking with the TI defaults as these are used by
traders. I would recommend the opposite. While it is true that most day
traders tend to stick to the defaults, large financial institutions such as
investment banks and hedge funds have their own quantitative teams who build them custom
models. This is what this package allows you to do.

Many of the functions accept parameters that allow the caller to move the technial
indicators away from its default behaviour. For example, if a TI uses the mean to calculate
the indicator, it can be told to use the median, or mode instead.

For this reason, RustTI is a more advanced Technical Inidcators package, and the users should
have some base knowledge of the indicators they plan on using.

RustTI is split into different modules, organised by common TI areas. Each module is then split
into a `single` and a `bulk` submodule.

The `single` submodule is used to calculate the indicator once, using the entire price slice
that is passed in.

The `bulk` submodule is used to iterate over a slice of prices to calculate the indicator for a period.

Many of the functions accept parameters that will allow the caller to move away from the technial
indicators from its default behaviour. For example, if a function normally uses the mean to calculate
the indicator, it can be told to use the median, or mode instead. More information is given in the
functions that allow this.

## Documentation

Documentation can be found on `DOCS.RS` [rust_ti](https://docs.rs/rust_ti/latest/rust_ti/)

## S&P 500 Example

An example using the Rust TI for the S&P 500 can be found in `examples/main.rs`

The code in `examples/main.rs` can be run by cloning this repo, then running the following commands:
```shell
cargo build
cargo run --example sp500
```

## Available indicators:

### Basic Indicators
[`basic_indicators`](https://docs.rs/rust_ti/latest/rust_ti/basic_indicators/index.html)
 are basic functions that are often the foundation of more complex indicators.

#### Bulk
         
* [`absolute_deviation`](https://docs.rs/rust_ti/latest/rust_ti/basic_indicators/bulk/fn.absolute_deviation.html) - Calculates the absolute deviation
* [`log`](https://docs.rs/rust_ti/latest/rust_ti/basic_indicators/bulk/fn.log.html) - 
Calculates the natural logrithm of slice of prices
* [`log_difference`](https://docs.rs/rust_ti/latest/rust_ti/basic_indicators/bulk/fn.log_difference.html) - 
Calculates the difference between the natural logarithm at t and t-1
* [`mean`](https://docs.rs/rust_ti/latest/rust_ti/basic_indicators/bulk/fn.mean.html) - 
Calculates the mean (average) of a slice of prices
* [`median`](https://docs.rs/rust_ti/latest/rust_ti/basic_indicators/bulk/fn.median.html) - 
Calculates the median (middle value) of a slice of prices
* [`mode`](https://docs.rs/rust_ti/latest/rust_ti/basic_indicators/bulk/fn.mode.html) - 
Calculates the mode (most common price) of a slice of prices
* [`standard_deviation`](https://docs.rs/rust_ti/latest/rust_ti/basic_indicators/bulk/fn.standard_deviation.html) - Calculates the standard
deviation of a slice of prices
* [`variance`](https://docs.rs/rust_ti/latest/rust_ti/basic_indicators/bulk/fn.variance.html) - 
Calculates the variance of slice of prices

#### Single

* [`absolute_deviation`](https://docs.rs/rust_ti/latest/rust_ti/basic_indicators/single/fn.absolute_deviation.html) - Calculates the absolute deviation
* [`log_difference`](https://docs.rs/rust_ti/latest/rust_ti/basic_indicators/single/fn.log_difference.html) - 
Calculates the difference between the natural logarithm at t and t-1
* [`max`](https://docs.rs/rust_ti/latest/rust_ti/basic_indicators/single/fn.max.html) - 
Calculates the maximum of a slice of prices
* [`mean`](https://docs.rs/rust_ti/latest/rust_ti/basic_indicators/single/fn.mean.html) - 
Calculates the mean (average) of a slice of prices
* [`median`](https://docs.rs/rust_ti/latest/rust_ti/basic_indicators/single/fn.median.html) - 
Calculates the median (middle value) of a slice of prices
* [`min`](https://docs.rs/rust_ti/latest/rust_ti/basic_indicators/single/fn.min.html) - 
Calculates the minimum of a slice of prices
* [`mode`](https://docs.rs/rust_ti/latest/rust_ti/basic_indicators/single/fn.mode.html) - 
Calculates the mode (most common price) of a slice of prices
* [`standard_deviation`](https://docs.rs/rust_ti/latest/rust_ti/basic_indicators/single/fn.standard_deviation.html) - Calculates the
 standard deviation of a slice of prices
* [`variance`](https://docs.rs/rust_ti/latest/rust_ti/basic_indicators/single/fn.variance.html) - 
Calculates the variance of slice of prices

### Candle Indicators
[`candle_indicators`](https://docs.rs/rust_ti/latest/rust_ti/candle_indicators/index.html) 
 are indicators that are used with candle charts.

#### Bulk

* [`ichimoku_cloud`](https://docs.rs/rust_ti/latest/rust_ti/candle_indicators/bulk/fn.ichimoku_cloud.html) - 
Calculates the Ichimoku Cloud
* [`mcginley_dynamic_bands`](https://docs.rs/rust_ti/latest/rust_ti/candle_indicators/bulk/fn.mcginley_dynamic_bands.html) - The McGinley Dynamic
version of the `moving_constant_bands`
* [`mcginley_dynamic_envelopes`](https://docs.rs/rust_ti/latest/rust_ti/candle_indicators/bulk/fn.mcginley_dynamic_envelopes.html) - The McGinley 
 Dynamic version of the
* [`moving_constant_bands`](https://docs.rs/rust_ti/latest/rust_ti/candle_indicators/bulk/fn.moving_constant_bands.html) - Calculates the moving 
 constant bands
* [`moving_constant_envelopes`](https://docs.rs/rust_ti/latest/rust_ti/candle_indicators/bulk/fn.moving_constant_envelopes.html) - Calculates the moving
 constant envelopes

#### Single

* [`ichimoku_cloud`](https://docs.rs/rust_ti/latest/rust_ti/candle_indicators/single/fn.ichimoku_cloud.html) - Calculates the Ichimoku Cloud
* [`mcginley_dynamic_bands`](https://docs.rs/rust_ti/latest/rust_ti/candle_indicators/single/fn.mcginley_dynamic_bands.html) - The McGinley Dynamic
 version of the `moving_constant_bands`
* [`mcginley_dynamic_envelopes`](https://docs.rs/rust_ti/latest/rust_ti/candle_indicators/single/fn.mcginley_dynamic_envelopes.html) - The McGinley 
 Dynamic version of the
* [`moving_constant_bands`](https://docs.rs/rust_ti/latest/rust_ti/candle_indicators/single/fn.moving_constant_bands.html) - Calculates 
the moving constant bands
* [`moving_constant_envelopes`](https://docs.rs/rust_ti/latest/rust_ti/candle_indicators/single/fn.moving_constant_envelopes.html) - Calculates the moving

### Chart Trends
[`chart_trends`](https://docs.rs/rust_ti/latest/rust_ti/chart_trends/index.html) shows trends on charts. 
Unlike the other modules it has no bulk or single.

* [`break_down_trends`](https://docs.rs/rust_ti/latest/rust_ti/chart_trends/fn.break_down_trends.html) - Breaks down the chart into different
price trends
* [`overall_trend`](https://docs.rs/rust_ti/latest/rust_ti/chart_trends/fn.overall_trend.html) - Calculates the overall trend for all price
points
* [`peak_trend`](https://docs.rs/rust_ti/latest/rust_ti/chart_trends/fn.peak_trend.html) - Calculates the trend of the peaks
* [`peaks`](https://docs.rs/rust_ti/latest/rust_ti/chart_trends/fn.peaks.html) - Returns all the peaks 
* [`valley_trend`](https://docs.rs/rust_ti/latest/rust_ti/chart_trends/fn.valley_trend.html) - Calculates trend of the valleys
* [`valleys`](https://docs.rs/rust_ti/latest/rust_ti/chart_trends/fn.valleys.html) - Returns all the valleys

### Correlation indicators

[`correlation_indicators`](https://docs.rs/rust_ti/latest/rust_ti/correlation_indicators/index.html)
 show how closely the prices of two different assets move together.

#### Bulk

* [`correlate_asset_prices`](https://docs.rs/rust_ti/latest/rust_ti/correlation_indicators/bulk/fn.correlate_asset_prices.html) - Calculates 
the correlation between two assets

#### Single

* [`correlate_asset_prices`](https://docs.rs/rust_ti/latest/rust_ti/correlation_indicators/single/fn.correlate_asset_prices.html) - Calculates 
the correlation between two assets

### Momentum Indicators

[`momentum_indicators`](https://docs.rs/rust_ti/latest/rust_ti/momentum_indicators/index.html) 
show how much the price is rising or falling

#### Bulk

* [`chaikin_oscillator`](https://docs.rs/rust_ti/latest/rust_ti/momentum_indicators/bulk/fn.chaikin_oscillator.html) - Calculates the Chaikin Oscillator
* [`commodity_channel_index`](https://docs.rs/rust_ti/latest/rust_ti/momentum_indicators/bulk/fn.commodity_channel_index.html) - Calculate 
the Commodity Channel Index
* [`macd_line`](https://docs.rs/rust_ti/latest/rust_ti/momentum_indicators/bulk/fn.macd_line.html) - Calculates the Moving Average Convergence 
Divergence line
[`mcginley_dynamic_chaikin_oscillator`](https://docs.rs/rust_ti/latest/rust_ti/momentum_indicators/bulk/fn.mcginley_dynamic_chaikin_oscillator.html) - 
Calculate the McGinley dynanic version of the Chaikin Oscillator
* [`mcginley_dynamic_commodity_channel_index`](https://docs.rs/rust_ti/latest/rust_ti/momentum_indicators/bulk/fn.mcginley_dynamic_commodity_channel_index.html) -
 Calculates the McGinley dynamic version of the Commodity Channel Index
* [`mcginley_dynamic_macd_line`](https://docs.rs/rust_ti/latest/rust_ti/momentum_indicators/bulk/fn.mcginley_dynamic_macd_line.html) - 
 Calculates the McGinley dynamic version of the Moving Average Convergence Divergence line
* [`mcginley_dynamic_rsi`](https://docs.rs/rust_ti/latest/rust_ti/momentum_indicators/bulk/fn.mcginley_dynamic_rsi.html) - Calculates 
the McGinley dynamic version of the Relative Strength Index
* [`money_flow_index`](https://docs.rs/rust_ti/latest/rust_ti/momentum_indicators/bulk/fn.money_flow_index.html) - Calculates the Money Flow Index
* [`on_balance_volume`](https://docs.rs/rust_ti/latest/rust_ti/momentum_indicators/bulk/fn.on_balance_volume.html) - Calculates the On-balance Volume
* [`rate_of_change`](https://docs.rs/rust_ti/latest/rust_ti/momentum_indicators/bulk/fn.rate_of_change.html) - Calculates the Rate of Change
* [`relative_strength_index`](https://docs.rs/rust_ti/latest/rust_ti/momentum_indicators/bulk/fn.relative_strength_index.html) - Calculates 
the Relative Strength Index
* [`signal_line`](https://docs.rs/rust_ti/latest/rust_ti/momentum_indicators/bulk/fn.signal_line.html) - Calculates the Signal line to be 
 used with the MACD line
* [`slow_stochastic`](https://docs.rs/rust_ti/latest/rust_ti/momentum_indicators/bulk/fn.slow_stochastic.html) - Calculates the slow stochastic 
to be used with the stochastic oscillator
* [`slowest_stochastic`](https://docs.rs/rust_ti/latest/rust_ti/momentum_indicators/bulk/fn.slowest_stochastic.html) - Calculates the 
 slowest stochastic to be used with the stochastic oscillator
* [`stochastic_oscillator`](https://docs.rs/rust_ti/latest/rust_ti/momentum_indicators/bulk/fn.stochastic_oscillator.html) - Calculates the 
Stochastic Oscillator
* [`williams_percent_r`](https://docs.rs/rust_ti/latest/rust_ti/momentum_indicators/bulk/fn.williams_percent_r.html) - Calcualtes the Williams %R

#### Single

* [`chaikin_oscillator`](https://docs.rs/rust_ti/latest/rust_ti/momentum_indicators/single/fn.chaikin_oscillator.html) - Calculates the Chaikin Oscillator
* [`commodity_channel_index`](https://docs.rs/rust_ti/latest/rust_ti/momentum_indicators/single/fn.commodity_channel_index.html) - Calculate the 
 Commodity Channel Index
* [`macd_line`](https://docs.rs/rust_ti/latest/rust_ti/momentum_indicators/single/fn.macd_line.html) - Calculates the Moving Average 
 Convergence Divergence line
* [`mcginley_dynamic_chaikin_oscillator`](https://docs.rs/rust_ti/latest/rust_ti/momentum_indicators/single/fn.mcginley_dynamic_chaikin_oscillator.html) -
 Calculate the McGinley dynanic version of the Chaikin Oscillator
* [`mcginley_dynamic_commodity_channel_index`](https://docs.rs/rust_ti/latest/rust_ti/momentum_indicators/single/fn.mcginley_dynamic_commodity_channel_index.html) -
 Calculates the McGinley dynamic version of the Commodity Channel Index
* [`mcginley_dynamic_macd_line`](https://docs.rs/rust_ti/latest/rust_ti/momentum_indicators/single/fn.mcginley_dynamic_macd_line.html) - 
 Calculates the McGinley dynamic version of the Moving Average Convergence Divergence line
* [`mcginley_dynamic_rsi`](https://docs.rs/rust_ti/latest/rust_ti/momentum_indicators/single/fn.mcginley_dynamic_rsi.html) - Calculates 
 the McGinley dynamic version of the Relative Strength Index
* [`money_flow_index`](https://docs.rs/rust_ti/latest/rust_ti/momentum_indicators/single/fn.money_flow_index.html) - Calculates the Money Flow Index
* [`on_balance_volume`](https://docs.rs/rust_ti/latest/rust_ti/momentum_indicators/single/fn.on_balance_volume.html) - Calculates the On-balance Volume
* [`rate_of_change`](https://docs.rs/rust_ti/latest/rust_ti/momentum_indicators/single/fn.rate_of_change.html) - Calculates the Rate of Change
* [`relative_strength_index`](https://docs.rs/rust_ti/latest/rust_ti/momentum_indicators/single/fn.relative_strength_index.html) - Calculates 
 the Relative Strength Index
* [`signal_line`](https://docs.rs/rust_ti/latest/rust_ti/momentum_indicators/single/fn.signal_line.html) - Calculates the Signal line 
 to be used with the MACD line
* [`slow_stochastic`](https://docs.rs/rust_ti/latest/rust_ti/momentum_indicators/single/fn.slow_stochastic.html) - Calculates the 
 slow stochastic to be used with the stochastic oscillator
* [`slowest_stochastic`](https://docs.rs/rust_ti/latest/rust_ti/momentum_indicators/single/fn.slowest_stochastic.html) - Calculates the 
 slowest stochastic to be used with the stochastic oscillator
* [`stochastic_oscillator`](https://docs.rs/rust_ti/latest/rust_ti/momentum_indicators/single/fn.stochastic_oscillator.html) - Calculates 
the Stochastic Oscillator
* [`williams_percent_r`](https://docs.rs/rust_ti/latest/rust_ti/momentum_indicators/single/fn.williams_percent_r.html) - Calcualtes the Williams %R

### Moving Averages

The [`moving_average`](https://docs.rs/rust_ti/latest/rust_ti/moving_average/index.html)
 module has functions used to calculate the moving average

#### Bulk

* [`mcginley_dynamic`](https://docs.rs/rust_ti/latest/rust_ti/moving_average/bulk/fn.mcginley_dynamic.html) - Calculates the McGinley Dynamic
* [`moving_average`](https://docs.rs/rust_ti/latest/rust_ti/moving_average/bulk/fn.moving_average.html) - Calculates different types Moving Averages

#### Single

* [`mcginley_dynamic`](https://docs.rs/rust_ti/latest/rust_ti/moving_average/bulk/fn.mcginley_dynamic.html) - Calculates the McGinley Dynamic
* [`moving_average`](https://docs.rs/rust_ti/latest/rust_ti/moving_average/bulk/fn.moving_average.html) - Calculates different types Moving Averages

### Other Indicators

[`other_indicators`](https://docs.rs/rust_ti/latest/rust_ti/other_indicators/index.html) 
don't really fit in anywhere else

#### Bulk

* [`return_on_investment`](https://docs.rs/rust_ti/latest/rust_ti/other_indicators/bulk/fn.return_on_investment.html) - Calculates the return 
on investment

#### Single

* [`return_on_investment`](https://docs.rs/rust_ti/latest/rust_ti/other_indicators/single/fn.return_on_investment.html) - Calculates the return
on investment

### Strength Indicators

[`strength_indicators`](https://docs.rs/rust_ti/latest/rust_ti/strength_indicators/index.html) 
show the strength of a trend

#### Bulk

* [`accumulation_distribution`](https://docs.rs/rust_ti/latest/rust_ti/strength_indicators/bulk/fn.accumulation_distribution.html) - Calculates 
the Accumulation Distribution

#### Single

* [`accumulation_distribution`](https://docs.rs/rust_ti/latest/rust_ti/strength_indicators/single/fn.accumulation_distribution.html) - Calculates 
the Accumulation Distribution

### Trend Indicators

[`trend_indicators`](https://docs.rs/rust_ti/latest/rust_ti/trend_indicators/index.html) show the trend direction of an asset

#### Bulk

* [`aroon_down`](https://docs.rs/rust_ti/latest/rust_ti/trend_indicators/bulk/fn.aroon_down.html) - Calculates the Aroon down
* [`aroon_indicator`](https://docs.rs/rust_ti/latest/rust_ti/trend_indicators/bulk/fn.aroon_indicator.html) - Calculates the Aroon indicator
* [`aroon_oscillator`](https://docs.rs/rust_ti/latest/rust_ti/trend_indicators/bulk/fn.aroon_oscillator.html) - Calculates the Aroon Oscillator
* [`aroon_up`](https://docs.rs/rust_ti/latest/rust_ti/trend_indicators/bulk/fn.aroon_up.html) - Calculates the Aroon up
* [`parabolic_time_price_system`](https://docs.rs/rust_ti/latest/rust_ti/trend_indicators/bulk/fn.parabolic_time_price_system.html) - Calculates 
the parabolic time price system

#### Single

* [`aroon_down`](https://docs.rs/rust_ti/latest/rust_ti/trend_indicators/single/fn.aroon_down.html) - Calculates the Aroon down
* [`aroon_indicator`](https://docs.rs/rust_ti/latest/rust_ti/trend_indicators/single/fn.aroon_indicator.html) - Calculates the Aroon indicator
* [`aroon_oscillator`](https://docs.rs/rust_ti/latest/rust_ti/trend_indicators/single/fn.aroon_oscillator.html) - Calculates the Aroon Oscillator
* [`aroon_up`](https://docs.rs/rust_ti/latest/rust_ti/trend_indicators/single/fn.aroon_up.html) - Calculates the Aroon up
* [`long_parabolic_time_price_system`](https://docs.rs/rust_ti/latest/rust_ti/trend_indicators/single/fn.long_parabolic_time_price_system.html) - 
 Calculates the parabolic time price system for long positions
* [`short_parabolic_time_price_system`](https://docs.rs/rust_ti/latest/rust_ti/trend_indicators/single/fn.short_parabolic_time_price_system.html) - 
 Calculates the parabolic time price system for short positions

### Volatility Indicators

[`volatility_indicators`](https://docs.rs/rust_ti/latest/rust_ti/volatility_indicators/index.html) 
show how volatile an asset are.

#### Bulk

* [`ulcer_index`](https://docs.rs/rust_ti/latest/rust_ti/volatility_indicators/bulk/fn.ulcer_index.html) - Calculates the Ulcer Index

#### Single

* [`ulcer_index`](https://docs.rs/rust_ti/latest/rust_ti/volatility_indicators/single/fn.ulcer_index.html) - Calculates the Ulcer Index

