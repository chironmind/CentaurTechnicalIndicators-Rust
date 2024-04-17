//! # Candle indicators
//!
//! Candle indicators are indicators that are used with candle charts.

/// `single` module holds functions that return a singular values
pub mod single {
    use crate::basic_indicators::single::{median, mode};
    use crate::moving_average::single::moving_average;
    use crate::ConstantModelType;
    use crate::MovingAverageType;
    /// The `moving_constant_envelope` function calculates upper and lower bands based off of the
    /// moving the moving constant of the price. The function returns a tuple with the lower band,
    /// moving constant, upper band (in that order).
    ///
    /// The standard is to use the moving averages to create a moving average envelope but this
    /// function has been extended to allow the caller to use the other constant model types, hence
    /// the name.
    ///
    /// The caller also determines the difference, or the distance between the price and the bands.
    /// The `int` passed in is a percentage, so passing in 3 would mean a 3% difference from the
    /// moving average.
    ///
    /// # Arguments
    ///
    /// * `prices` - An `f64` slice of prices
    /// * `constant_model_type` - A variant of the `ConstantModelType` enum.
    /// * `difference` - The percent difference or distance that the bands will be from the moving
    /// constant
    ///
    /// # Examples
    ///
    /// ```
    /// let prices = vec![100.0, 102.0, 103.0, 101.0, 99.0];
    /// let difference = 3.0;
    ///
    /// let ema_envelope = rust_ti::candle_indicators::single::moving_constant_envelope(&prices,
    /// &rust_ti::ConstantModelType::ExponentialMovingAverage, &difference);
    /// assert_eq!((97.59303317535547, 100.61137440758296, 103.62971563981044), ema_envelope);
    ///
    /// let median_envelope = rust_ti::candle_indicators::single::moving_constant_envelope(&prices,
    /// &rust_ti::ConstantModelType::SimpleMovingMedian, &difference);
    /// assert_eq!((97.97, 101.0, 104.03), median_envelope);
    /// ```
    pub fn moving_constant_envelope(
        prices: &[f64],
        constant_model_type: &crate::ConstantModelType,
        difference: &f64,
    ) -> (f64, f64, f64) {
        if prices.is_empty() {
            panic!("Prices cannot be empty")
        };

        let moving_constant = match constant_model_type {
            ConstantModelType::SimpleMovingAverage => {
                moving_average(&prices, &MovingAverageType::Simple)
            }
            ConstantModelType::SmoothedMovingAverage => {
                moving_average(&prices, &MovingAverageType::Smoothed)
            }
            ConstantModelType::ExponentialMovingAverage => {
                moving_average(&prices, &MovingAverageType::Exponential)
            }
            ConstantModelType::PersonalisedMovingAverage(alpha_nominator, alpha_denominator) => {
                moving_average(
                    &prices,
                    &MovingAverageType::Personalised(alpha_nominator, alpha_denominator),
                )
            }
            ConstantModelType::SimpleMovingMedian => median(&prices),
            ConstantModelType::SimpleMovingMode => mode(&prices),
            _ => panic!("Unsupported ConstantModelType"),
        };

        let upper_envelope = &moving_constant * (1.0 + (difference / 100.0));
        let lower_envelope = &moving_constant * (1.0 - (difference / 100.0));
        return (lower_envelope, moving_constant, upper_envelope);
    }
}

/// `bulk` module holds functions that return multiple valus for `momentum_indicators`
pub mod bulk {}
