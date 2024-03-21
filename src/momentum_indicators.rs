//! # Momentum Indicators
//!
//! Momentum indicators show how much the price is rising or falling

/// `single` module holds functions that return a singular values
pub mod single {
    use crate::basic_indicators::single::{median, mode};
    use crate::moving_average::single::{mcginley_dynamic, moving_average};
    use crate::ConstantModelType;
    use crate::MovingAverageType;
    /// The `relative_strenght_index` measures the speed and magnitude of price changes
    ///
    /// The period is determined based on length of `prices`. Based on the 7 day work week when the
    /// RSI was developed the default length is 14 (2 weeks) but the caller can determine their own
    /// period.
    ///
    /// # Arguments
    ///
    /// * `prices` - An `f64` slice of prices
    /// * `constant_model_type` - A variant of the `ConstantModelType` enum. The default model for
    /// the RSI as per Welles is the `ConstantModelType::SmoothedMovingAverage`
    ///
    /// # Examples
    ///
    /// ```
    /// // Short example
    /// let prices = vec![100.0, 102.0, 103.0, 101.0, 99.0];
    ///
    /// let defaut_rsi = rust_ti::momentum_indicators::single::relative_strength_index(&prices, &rust_ti::ConstantModelType::SmoothedMovingAverage);
    /// assert_eq!(39.99999999999999, defaut_rsi);
    ///
    /// let ema_rsi = rust_ti::momentum_indicators::single::relative_strength_index(&prices, &rust_ti::ConstantModelType::ExponentialMovingAverage);
    /// assert_eq!(38.46153846153846, ema_rsi);
    ///
    /// let moving_median_rsi = rust_ti::momentum_indicators::single::relative_strength_index(&prices, &rust_ti::ConstantModelType::SimpleMovingMedian);
    /// assert_eq!(42.857142857142854, moving_median_rsi);
    /// ```
    pub fn relative_strength_index(
        prices: &[f64],
        constant_model_type: &crate::ConstantModelType,
    ) -> f64 {
        if prices.is_empty() {
            panic!("Prices is empty");
        };

        let mut previous_gains = Vec::new();
        let mut previous_loss = Vec::new();

        for (i, value) in prices.iter().enumerate() {
            // TODO: must be a better way to do this
            if i == 0 {
                continue
            };
            if value > &prices[i - 1] {
                previous_gains.push(value - prices[i - 1]);
            } else if value < &prices[i - 1] {
                previous_loss.push(prices[i - 1] - value);
            };
        }

        if previous_gains.is_empty() {
            return 0.0;
        }
        if previous_loss.is_empty() {
            return 100.0;
        }

        let (previous_average_gains, previous_average_loss) = match constant_model_type {
            ConstantModelType::SimpleMovingAverage => (
                moving_average(&previous_gains, &MovingAverageType::Simple),
                moving_average(&previous_loss, &MovingAverageType::Simple),
            ),
            ConstantModelType::SmoothedMovingAverage => (
                moving_average(&previous_gains, &MovingAverageType::Smoothed),
                moving_average(&previous_loss, &MovingAverageType::Smoothed),
            ),
            ConstantModelType::ExponentialMovingAverage => (
                moving_average(&previous_gains, &MovingAverageType::Exponential),
                moving_average(&previous_loss, &MovingAverageType::Exponential),
            ),
            ConstantModelType::PersonalisedMovingAverage(alpha_nominator, alpha_denominator) => (
                moving_average(
                    &previous_gains,
                    &MovingAverageType::Personalised(alpha_nominator, alpha_denominator),
                ),
                moving_average(
                    &previous_loss,
                    &MovingAverageType::Personalised(alpha_nominator, alpha_denominator),
                ),
            ),
            ConstantModelType::McGinleyDynamic(previous_mcginley_dynamic) => (
                // TODO: Panic here and tell it to use the other RSI just for McGinley
                mcginley_dynamic(&previous_gains, previous_mcginley_dynamic),
                mcginley_dynamic(&previous_loss, previous_mcginley_dynamic),
            ),
            ConstantModelType::SimpleMovingMedian => (median(&previous_gains), median(&previous_loss)),
            ConstantModelType::SimpleMovingMode => (mode(&previous_gains), mode(&previous_loss)),
            _ => panic!("Unsupported ConstantModelType"),
        };
        if previous_average_loss == 0.0 {
            return 0.0
        };
        return 100.0_f64 - (100.0_f64 / (1.0_f64 + (previous_average_gains / previous_average_loss)));
    }
}

/// `bulk` module holds functions that return multiple valus for `momentum_indicators`
pub mod bulk {
    use crate::momentum_indicators::single;
    /// The `relative_strenght_index` measures the speed and magnitude of price changes
    ///
    /// Based on the 7 day work week when the RSI was developed the default length is 14 (2 weeks) but the caller can determine their own
    /// period.
    ///
    /// # Arguments
    ///
    /// * `prices` - An `f64` slice of prices
    /// * `constant_model_type` - A variant of the `ConstantModelType` enum. The default model for
    /// the RSI as per Welles is the `ConstantModelType::SmoothedMovingAverage`
    /// * `period` - A `usize` period over which to calculate the RSI
    ///
    /// # Examples
    ///
    /// ```
    /// let prices = vec![100.0, 102.0, 103.0, 101.0, 99.0];
    /// let period: usize = 3;
    /// let defaut_rsi = rust_ti::momentum_indicators::bulk::relative_strength_index(&prices, &rust_ti::ConstantModelType::SmoothedMovingAverage, &period);
    /// assert_eq!(vec![100.0, 33.33333333333333, 0.0], defaut_rsi);
    ///
    /// let ema_rsi = rust_ti::momentum_indicators::bulk::relative_strength_index(&prices, &rust_ti::ConstantModelType::ExponentialMovingAverage, &period);
    /// assert_eq!(vec![100.0, 33.33333333333333, 0.0], ema_rsi);
    ///
    /// let moving_median_rsi = rust_ti::momentum_indicators::bulk::relative_strength_index(&prices, &rust_ti::ConstantModelType::SimpleMovingMedian, &period);
    /// assert_eq!(vec![100.0, 33.33333333333333, 0.0], moving_median_rsi);
    ///
    /// // Long example
    /// let prices = vec![100.2, 100.46, 100.53, 100.38, 100.19, 100.21, 100.32, 100.28];
    /// let period: usize = 5;
    ///
    /// let personalised_rsi = rust_ti::momentum_indicators::bulk::relative_strength_index(&prices, &rust_ti::ConstantModelType::PersonalisedMovingAverage(&5.0, &4.0), &period); 
    /// //TODO: assert_eq!(vec![
    /// ```
    pub fn relative_strength_index(prices: &[f64], constant_model_type: &crate::ConstantModelType, period: &usize) -> Vec<f64> {
        let length = prices.len();
        if period > &length {
            panic!("Period ({}) is longer than length of prices ({})", period, length);
        };

        let mut rsis = Vec::new();
        for i in 0..length {
            let end_index = period + i;
            if end_index > length {
                break;
            };
            rsis.push(single::relative_strength_index(&prices[i..end_index], constant_model_type));
        }
        return rsis;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_ma_rsi() {
        let prices = vec![100.2, 100.46, 100.53, 100.38, 100.19];
        assert_eq!(49.2537313432832, single::relative_strength_index(&prices, &crate::ConstantModelType::SimpleMovingAverage));
    }

    #[test]
    fn test_short_median_rsi() {
        // Because there are too few values, ends up being the means
        let prices = vec![100.2, 100.46, 100.53, 100.38, 100.19];
        assert_eq!(49.2537313432832, single::relative_strength_index(&prices, &crate::ConstantModelType::SimpleMovingMedian));
    }

    #[test]
    fn test_long_median_rsi() {
        let prices = vec![100.2, 100.46, 100.53, 100.38, 100.19, 100.21, 100.32, 100.28];
        assert_eq!(37.5, single::relative_strength_index(&prices, &crate::ConstantModelType::SimpleMovingMedian));
    }

    #[test]
    fn test_small_mode_rsi() {
        // Mode rounds the values, the difference being so small all rounds down to 0.0
        let prices = vec![100.2, 100.46, 100.53, 100.38, 100.19];
        assert_eq!(0.0, single::relative_strength_index(&prices, &crate::ConstantModelType::SimpleMovingMode));
    }

    #[test]
    fn test_large_mode_rsi() {
        let prices = vec![100.0, 103.0, 106.0, 107.0, 108.0, 105.0, 102.0];
        assert_eq!(39.99999999999999, single::relative_strength_index(&prices, &crate::ConstantModelType::SimpleMovingMode));
    }

    #[test]
    fn test_smoothed_rsi() {
        let prices = vec![100.2, 100.46, 100.53, 100.38, 100.19];
        assert_eq!(43.01075268817234, single::relative_strength_index(&prices, &crate::ConstantModelType::SmoothedMovingAverage));
    }

    #[test]
    fn test_exponential_rsi() {
        let prices = vec![100.2, 100.46, 100.53, 100.38, 100.19];
        assert_eq!(39.495798319328436, single::relative_strength_index(&prices, &crate::ConstantModelType::ExponentialMovingAverage));
    }

    #[test]
    fn test_personalised_rsi() {
        let prices = vec![100.2, 100.46, 100.53, 100.38, 100.19];
        assert_eq!(35.6725146198842, single::relative_strength_index(&prices, &crate::ConstantModelType::PersonalisedMovingAverage(&4.0, &3.0)));
    }

    #[test]
    fn test_only_price_rise_rsi() {
        let prices = vec![100.0, 101.0, 102.0, 103.0];
        assert_eq!(100.0, single::relative_strength_index(&prices, &crate::ConstantModelType::SimpleMovingAverage) );
    }

    #[test]
    fn test_only_price_fall_rsi() {
        let prices = vec![103.0, 102.0, 101.0, 100.0];
        assert_eq!(0.0, single::relative_strength_index(&prices, &crate::ConstantModelType::SimpleMovingAverage));
    }

    #[test]
    #[should_panic]
    fn test_rsi_panic() {
        let prices = Vec::new();
        single::relative_strength_index(&prices, &crate::ConstantModelType::SimpleMovingAverage);
    }
    //#[test]
    // TODO: McGinley needs its own RSI flavor because it needs to return the previous RSIs...
    //fn test_mcginley_dynamic_rsi() {
    //    let prices = vec![100.2, 100.46, 100.53, 100.38, 100.19];
    //    assert_eq!(
}
