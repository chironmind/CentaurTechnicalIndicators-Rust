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
rust_ti = "1.4.2"
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

**COMING SOON**

---

## 🛠️ How-To Guides

> Task-oriented guides for common problems and advanced scenarios.

- **COMING SOON**

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

**Latest (v2.0.0):**
- Full refactor of the code and docstrings to fall in line with Rust standards (obvious, constrained, unsurprising, and flexible)

[Changlelog →](https://github.com/0100101001010000/RustTI/blob/main/CONTRIBUTING.md)
[Full changelog →](https://github.com/0100101001010000/RustTI/releases)

---

## 📄 License

MIT License. See [LICENSE](LICENSE).

