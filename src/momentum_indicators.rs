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
                mcginley_dynamic(&previous_gains, previous_mcginley_dynamic),
                mcginley_dynamic(&previous_loss, previous_mcginley_dynamic),
            ),
            ConstantModelType::SimpleMovingMedian => (median(&previous_gains), median(&previous_loss)),
            ConstantModelType::SimpleMovingMode => (mode(&previous_gains), mode(&previous_loss)),
            _ => panic!("Unsupported ConstantModelType"),
        };

        return 100.0_f64 - (100.0_f64 / (1.0_f64 + (previous_average_gains / previous_average_loss)));
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
    fn test_median_rsi() {
        let prices = vec![100.2, 100.46, 100.53, 100.38, 100.19];
        assert_eq!(49.2537313432832, single::relative_strength_index(&prices, &crate::ConstantModelType::SimpleMovingMedian));
    }

    //#[test]
    //fn test_mode_rsi() {
    //    let prices = vec![100.2, 100.46, 100.53, 100.38, 100.19];
    //    assert_eq!(49.2537313432832, single::relative_strength_index(&prices, &crate::ConstantModelType::SimpleMovingMode));
    //}

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
    // TODO personalised MA, McGinley t and t+1, fix broken mode
}
