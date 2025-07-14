![RustTI Banner](./assets/banner.webp)

[![Crates.io](https://img.shields.io/crates/v/rust_ti.svg)](https://crates.io/crates/rust_ti)
[![Docs.rs](https://docs.rs/rust_ti/badge.svg)](https://docs.rs/rust_ti/)
[![CI](https://github.com/0100101001010000/RustTI/actions/workflows/rust.yml/badge.svg)](https://github.com/0100101001010000/RustTI/actions)
[![License](https://img.shields.io/github/license/0100101001010000/RustTI)](LICENSE)

# 🦀 Meet RusTI

Say hello to RusTI, the clawed crusader of candlesticks and the battle-hardened cousin of Ferris! 🦀📈

Forged from rusted metal and born in the depths of the financial abyss, RusTI doesn't just ride sideways markets — he lives for them. With a stack of notebooks, a thousand-yard stare, and more indicators on his screen than legs on his body, RusTI is the ultimate trading bro. He reads charts, calculates MACD in his sleep, and isn’t afraid to pinch your code into shape.

Welcome to RustTI — powered by precision, performance, and one extremely serious crustacean.

# RustTI

A highly configurable and high-performance technical indicators library written in pure Rust. 

Designed for flexibility, speed, and advanced use cases in quantitative and algorithmic trading.

---

## 🚀 Getting Started (Tutorial)

> The fastest way to get up and running with RustTI.

**1. Add RustTI to your project:**

```shell
cargo add rust_ti
```
Or, manually in your `Cargo.toml`:
```toml
rust_ti = "2.0.0"
```

**2. Calculate your first indicator:**

```rust
use rust_ti;

let prices = vec![100.2, 100.46, 100.53, 100.38, 100.19];

let ma = rust_ti::moving_average::single::moving_average(
    &prices,
    rust_ti::MovingAverageType::Simple
);
println!("Simple Moving Average: {}", ma);
```
Expected output:
```
Simple Moving Average: 100.352
```

**3. Explore more examples**

- [Getting started tutorial](https://github.com/0100101001010000/RustTI-tutorials/blob/main/getting_started.md)
- [Choosing the right model](https://github.com/0100101001010000/RustTI-tutorials/blob/main/choose_right_model.md)
- [Building your first strategy](https://github.com/0100101001010000/RustTI-tutorials/blob/main/first_strategy.md)
- [Backtesting tutorial](https://github.com/0100101001010000/RustTI-tutorials/blob/main/backtest.md)
- [Visualization tutorial](https://github.com/0100101001010000/RustTI-tutorials/blob/main/visualization.md)

---

## 🛠️ How-To Guides

> Task-oriented guides for common problems and advanced scenarios.

- **COMING SOON** Being developed [here](https://github.com/0100101001010000/RustTI-how-to-guides)

*(Contributions welcome! Submit your favorite how-to guide as a PR.)*

---


## 📚 Reference

> For complete API details, see [docs.rs/rust_ti](https://docs.rs/rust_ti/).

### Example

A reference of how to call each function can be found 

- [Reference Example](https://github.com/0100101001010000/RustTI/blob/main/examples/reference.rs)

Clone and run:
```shell
cargo build
cargo run --example reference
```


### Library Structure

- Modules based on their analysis areas (**`moving_average`**, **`momentum_indicators`**, **`strength_indicators`**...)
- **`bulk` & `single` submodules**  
  - `bulk`: Compute indicator over rolling periods, returns a vector.
  - `single`: Compute indicator for the entire vector, returns a single value.
- Types used to personalise the technical indicators (**`MovingAverageType`**, **`DeviationModel`**, **`Position`**...)

---

## 🧠 Explanation & Design

### Why RustTI?

- **Performance:** Pure Rust implementation for maximal speed, safety, and zero dependencies.
- **Configurability:** Most indicators are highly customizable—tweak calculation methods, periods, or even use medians instead of means.
- **Breadth:** Covers a wide range of technical indicators out of the box.
- **Advanced Use:** Designed for users who understand technical analysis and want deep control.

**Note:** Some features may require background in technical analysis. See [Investopedia: Technical Analysis](https://www.investopedia.com/terms/t/technicalanalysis.asp) for a primer.

---

## 📈 Available Indicators

All indicators are grouped and split into modules based on their analysis area.  
Each module has `bulk` (vector output) and `single` (scalar output) submodules.

### Standard Indicators
- Simple, Smoothed, Exponential Moving Average
- Bollinger Bands, MACD, RSI

### Basic Indicators
- Absolute Deviation, Log, Mean, Median, Mode, Std. Deviation, Variance, Max/Min

### Candle Indicators
- Ichimoku Cloud, McGinley Bands/Envelopes, Donchian Channels, Keltner, Supertrend

### Chart Trends
- Trend break down, overall trends, peak/valley trends

### Correlation Indicators
- Correlate asset prices

### Momentum Indicators
- Chaikin Oscillator, CCI, MACD, Money Flow Index, On Balance Volume, ROC, RSI, etc.

### Moving Averages
- McGinley Dynamic, Moving Average

### Other Indicators
- ROI, True Range, ATR, Internal Bar Strength

### Strength Indicators
- Accumulation/Distribution, PVI, NVI, RVI

### Trend Indicators
- Aroon (Up/Down/Oscillator), Parabolic, DM, Volume-Price Trend, TSI

### Volatility Indicators
- Ulcer Index, Volatility System

---

<<<<<<< Updated upstream
=======
## 📊 Performance Benchmarks

Want to know how fast RustTI runs in real-world scenarios?  
We provide detailed, reproducible benchmarks using realistic OHLCV data and a variety of indicators.

### Momentum Indicators

| Function                                      | Time per Operation |
|-----------------------------------------------|--------------------|
| `relative_strength_index`                     | 573.86 µs          |
| `stochastic_oscillator`                       | 784.13 µs          |
| `slow_stochastic`                             | 28.866 µs          |
| `slowest_stochastic`                          | 28.866 µs          |
| `williams_percent_r`                          | 76.256 µs          |
| `money_flow_index`                            | 150.69 µs          |
| `rate_of_change`                              | 5.3984 µs          |
| `on_balance_volume`                           | 17.405 µs          |
| `commodity_channel_index`                     | 103.19 µs          |
| `mcginley_dynamic_commodity_channel_index`    | 66.044 µs          |
| `macd_line`                                   | 51.482 µs          |
| `mcginley_dynamic_macd_line`                  | 44.461 µs          |
| `chaikin_oscillator`                          | 258.33 µs          |
| `percentage_price_oscillator`                 | 58.060 µs          |
| `chande_momentum_oscillator`                  | 370.14 µs          |

### Candle Indicators

| Function                                      | Time per Operation |
|-----------------------------------------------|--------------------|
| `moving_constant_envelopes`                   | 37.572 µs          |
| `mcginley_dynamic_envelopes`                  | 39.264 µs          |
| `moving_constant_bands`                       | 119.70 µs          |
| `mcginley_dynamic_bands`                      | 43.219 µs          |
| `ichimoku_cloud`                              | 192.93 µs          |
| `donchian_channel`                            | 28.481 µs          |
| `keltner_channel`                             | 318.05 µs          |
| `supertrend`                                  | 148.80 µs          |

### Trend Indicators

| Function                                      | Time per Operation |
|-----------------------------------------------|--------------------|
| `aroon_up`                                    | 16.531 µs          |
| `aroon_down`                                  | 16.592 µs          |
| `aroon_indicator`                             | 66.468 µs          |
| `parabolic_time_price_system`                 | 43.939 µs          |
| `directional_movement_system`                 | 88.965 µs          |
| `volume_price_trend`                          | 6.2801 µs          |
| `true_strength_indx`                          | 705.25 µs          |

### Strength Indicators

| Function                                      | Time per Operation |
|-----------------------------------------------|--------------------|
| `accumulation_distribution`                   | 8.2935 µs          |
| `positive_volume_index`                       | 7.6977 µs          |
| `negative_volume_index`                       | 7.6167 µs          |
| `relative_vigor_index`                        | 505.34 µs          |

### Other Indicators

| Function                                      | Time per Operation |
|-----------------------------------------------|--------------------|
| `return_on_investment`                        | 40.962 µs          |
| `true_range`                                  | 3.4663 µs          |
| `average_true_range`                          | 122.08 µs          |
| `internal_bar_strength`                       | 5.3943 µs          |
| `positivity_indicator`                        | 20.683 µs          |

### Basic Indicators

| Function                                      | Time per Operation |
|-----------------------------------------------|--------------------|
| `mean`                                        | 5.7432 µs          |
| `median`                                      | 333.68 µs          |
| `mode`                                        | 931.09 µs          |
| `log`                                         | 20.335 µs          |
| `log_difference`                              | 42.223 µs          |
| `variance`                                    | 20.921 µs          |
| `standard_deviation`                          | 24.095 µs          |
| `absolute_deviation(Mean)`                    | 26.991 µs          |
| `absolute_deviation(Median)`                  | 345.14 µs          |
| `absoluite_deviation(Mode)`                   | 956.83 µs          |

### Chart Trends

| Function                                      | Time per Operation |
|-----------------------------------------------|--------------------|
| `peaks`                                       | 93.094 µs          |
| `valleys`                                     | 92.119 µs          |
| `peak_trend`                                  | 188.14 µs          |
| `valley_trend`                                | 188.81 µs          |
| `overall_trend`                               | 10.337 µs          |
| `break_down_trends`                           | 14.655 ms          |

### Correlation Indicators

| Function                                      | Time per Operation |
|-----------------------------------------------|--------------------|
| `correlate_asset_prices`                      | 231.14 µs          |

### Moving Average

| Function                                      | Time per Operation |
|-----------------------------------------------|--------------------|
| `moving_average(Simple)`                      | 17.575 µs          |
| `moving_average(Smoothed)`                    | 76.601 µs          |
| `moving_average(Exponential)`                 | 78.505 µs          |
| `mcginley_dynamic`                            | 39.653 µs          |

### Volatility Indicators

| Function                                      | Time per Operation |
|-----------------------------------------------|--------------------|
| `ulcer_index`                                 | 65.959 µs          |
| `volatility_system`                           | 137.25 µs          |


*These results are from a Raspberry Pi 5 8GB, your machine will likely be faster!*

👉 [See all benchmarks and how to run your own](https://github.com/0100101001010000/RustTI-benchmarks/blob/main/README.md)

---

>>>>>>> Stashed changes
## 🤝 Contributing

Contributions, bug reports, and feature requests are welcome!
- [Open an issue](https://github.com/0100101001010000/RustTI/issues)
- [Submit a pull request](https://github.com/0100101001010000/RustTI/pulls)
- See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines

---

## 💬 Community & Support

- Start a [discussion](https://github.com/0100101001010000/RustTI/discussions)
- File [issues](https://github.com/0100101001010000/RustTI/issues)
- Add your project to the [Showcase](https://github.com/0100101001010000/RustTI/discussions/categories/show-and-tell)

---

## 📰 Release Notes

**Latest (v2.1.0):**
- Improved runtime of some indicators
- Added links to benchmark, tutorials, and how to documents

[Changlelog →](https://github.com/0100101001010000/RustTI/blob/main/CONTRIBUTING.md)

[Full changelog →](https://github.com/0100101001010000/RustTI/releases)

---

## 📄 License

MIT License. See [LICENSE](LICENSE).

