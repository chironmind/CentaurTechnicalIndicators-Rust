//! # RustTI
//!
//! A Technical Indicators library for Rust
//!
//! What differentiates RustTI from other Technical Indicator packages is that everything can be determined by the caller.
//! Many models were created decades ago when the work weeks were different (such as RSI, SO,
//! Ichimoku cloud...) and the observations were made daily. However if one decides to study common
//! stocks the work week is 5 days, but if one studies cryptocurrencies the work week is 7 days.
//! RustTI allows the caller to determine their own period based on the market being studied.
//! The caller isn't just limited to the period, moving average models, deviation models... are
//! also determined by the caller.
//!
//! While there are a lot of articles online recommending to stick with the defaults because that
//! is what most traders use, I would recommend the opposite. While it is true that most day
//! traders tend to stick to the defaults, large financial institutions such as
//! investment banks and hedge funds have their own quantitative teams who build them custom
//! models. This is what this package allows you to do.
//!
//! If you decide that defaults is the way to go, `standard_indicators` has functions that use the
//! common defaults.
//!
//! Each module is split into two submodules.
//!
//! A `single` submodule that is used to calculate the indicator once, using the entire price slice that is passed in.
//!
//! A `bulk` submodule that is used to iterate over a slice of prices to calculate the indicator for a passed in period.
//!
//! Many of the functions accept parameters that will allow the caller to move away from the technial
//! indicators from its default behaviour. For example, if a function normally uses the mean to calculate
//! the indicator, it can be told to use the median, or mode instead. More information is given in the
//! functions that allow this.

pub mod basic_indicators;
pub mod candle_indicators;
pub mod correlation_indicators;
pub mod momentum_indicators;
pub mod moving_average;
pub mod other_indicators;
pub mod strength_indicators;
pub mod trend_indicators;
pub mod volatility_indicators;

/// The `CentralPoint enum` is used to determine what the central point around
/// which to calculate the absolute deviation around.
pub enum CentralPoint {
    Mean,
    Median,
    Mode,
}

/// The `MovingAverageType` is used when calculating the `moving_average`.
/// The simple caclculates the moving average without interferring, however the smoothed and
/// exponential types assign weights to the more recent prices.
///
/// Personalised allows the caller the influence the weighting calculation, some research should be
/// done in how the alpha is calculated in different types before using this. The first float is
/// the alpha nominator, and the second float is the alpha denominator. The smoothed type
/// uses an alpha nominator of 1, and denominator of 0. The exponential type uses an alpha nominator
/// of 2, and denominator of 1. Probably shouldn't be used...
pub enum MovingAverageType<'a> {
    Simple,
    Smoothed,
    Exponential,
    Personalised(&'a f64, &'a f64),
}

/// The `ConstantModelType` is used by a number of functions to determine the centerpoint around
/// which to do its calculations.
///
/// Most of the time it uses a flavor of the moving average, but more are provided here to give the
/// caller the opportunity to diversify the functions a bit.
///
/// See note in `MovingAverageType` about using the `Personalised` variant.
///
/// The `McGinleyDynamic(f64)` variant takes the previous McGinley Dynamic as an argumet, if none
/// if available use `0.0`. When using it check whether you are able to get the previous one by
/// calculating seperately, it would make little sense to use it but always pass in `0.0`
pub enum ConstantModelType<'a> {
    SimpleMovingAverage,
    SmoothedMovingAverage,
    ExponentialMovingAverage,
    PersonalisedMovingAverage(&'a f64, &'a f64),
    SimpleMovingMedian,
    SimpleMovingMode,
}

/// The `DeviationModel` is used by a number of functions to determine the deviation from a central
/// point.
///
/// A lot of functions use the standard deviation but some also use mean and median absolute
/// deviations.
pub enum DeviationModel {
    StandardDeviation,
    MeanAbsoluteDeviation,
    MedianAbsoluteDeviation,
    ModeAbsoluteDeviation,
    UlcerIndex,
}
