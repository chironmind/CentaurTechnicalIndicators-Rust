//! # Momentum Indicators
//!
//! Momentum indicators show how much the price is rising or falling

/// `single` module holds functions that return a singular values
pub mod single {
    use crate::basic_indicators::single::{median, mode};
    use crate::moving_average::single::{mcginley_dynamic, moving_average};
    use crate::ConstantModelType;
    use crate::MovingAverageType;
    use std::cmp::Ordering;
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
        let (previous_gains, previous_loss) = previous_gains_loss(prices);
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
            ConstantModelType::SimpleMovingMedian => {
                (median(&previous_gains), median(&previous_loss))
            }
            ConstantModelType::SimpleMovingMode => (mode(&previous_gains), mode(&previous_loss)),
            _ => panic!("Unsupported ConstantModelType"),
        };
        return rsi(&previous_average_gains, &previous_average_loss);
    }

    /// The `mcginley_dynamic_rsi` is a variation of the `relative_strength_index` that accepts
    /// previous dynamics for the gains and loss calculations, but also returns the new ones so
    /// that they can be used going forward
    ///
    /// # Arguments
    ///  
    /// * `prices` - An `f64` slice of prices
    /// * `previous_gain_mcginley_dynamic` - The previous McGinley dynamic used for the gains
    /// calculation. Use 0.0 if it hasn't yet been calculated.
    /// * `previous_loss_mcginley_dynamic` - The previous McGinley dynamic used for the loss
    /// caclulation. Use 0.0 if it hasn't yet been calculated.
    ///
    /// # Examples
    ///
    /// ```
    /// let prices = vec![100.0, 102.0, 103.0, 101.0, 99.0];
    /// let previous_gain_dynamic = 0.0;
    /// let previous_loss_dynamic = 0.0;
    /// let mcginley_rsi = rust_ti::momentum_indicators::single::mcginley_dynamic_rsi(&prices,
    /// &previous_gain_dynamic, &previous_loss_dynamic);
    /// assert_eq!((33.33333333333333, 1.0, 2.0), mcginley_rsi);
    ///
    /// let prices = vec![102.0, 103.0, 101.0, 99.0, 97.0];
    /// let previous_gain_dynamic = mcginley_rsi.1;
    /// let previous_loss_dynamic = mcginley_rsi.2;
    /// let mcginley_rsi = rust_ti::momentum_indicators::single::mcginley_dynamic_rsi(&prices,
    /// &previous_gain_dynamic, &previous_loss_dynamic);
    /// assert_eq!((33.33333333333333, 1.0, 2.0), mcginley_rsi);
    /// ```
    pub fn mcginley_dynamic_rsi(
        prices: &[f64],
        previous_gain_mcginley_dynamic: &f64,
        previous_loss_mcginley_dynamic: &f64,
    ) -> (f64, f64, f64) {
        let (previous_gains, previous_loss) = previous_gains_loss(prices);
        if previous_gains.is_empty() {
            return (
                0.0,
                *previous_gain_mcginley_dynamic,
                mcginley_dynamic(
                    &previous_loss.last().unwrap(),
                    previous_loss_mcginley_dynamic,
                    &previous_loss.len(),
                ),
            );
        };
        if previous_loss.is_empty() {
            return (
                100.0,
                mcginley_dynamic(
                    &previous_gains.last().unwrap(),
                    previous_gain_mcginley_dynamic,
                    &previous_gains.len(),
                ),
                *previous_loss_mcginley_dynamic,
            );
        }
        let previous_gain_dynamic = mcginley_dynamic(
            &previous_gains.last().unwrap(),
            previous_gain_mcginley_dynamic,
            &previous_gains.len(),
        );
        let previous_loss_dynamic = mcginley_dynamic(
            &previous_loss.last().unwrap(),
            previous_loss_mcginley_dynamic,
            &previous_loss.len(),
        );
        println!(
            "{:?}, {}, {}, {}",
            previous_gains,
            previous_gain_mcginley_dynamic,
            previous_gains.len(),
            previous_gain_dynamic
        );
        return (
            rsi(&previous_gain_dynamic, &previous_loss_dynamic),
            previous_gain_dynamic,
            previous_loss_dynamic,
        );
    }

    /// The `stochastic_oscillator` is a momentum indicator that is calculated by using support and
    /// resistance levels
    ///
    /// When developed by George Lane the stochastic oscillator looked at a period of 14 days as
    /// the work week had 7 days back then, however this implementation allows for prices of any
    /// length to be used
    ///
    /// # Arguments
    ///
    /// * `prices` - An `f64` slice of prices
    ///
    /// # Examples
    ///
    /// ```
    /// let prices = vec![100.0, 102.0, 103.0, 101.0, 99.0];
    /// let stochastic_oscillator = rust_ti::momentum_indicators::single::stochastic_oscillator(&prices);
    /// assert_eq!(0.0, stochastic_oscillator);
    /// ```
    pub fn stochastic_oscillator(prices: &[f64]) -> f64 {
        if prices.is_empty() {
            panic!("Prices is empty");
        };
        let mut ordered_prices = prices
            .iter()
            .filter_map(|f| if f.is_nan() { None } else { Some(*f) })
            .collect::<Vec<f64>>();
        ordered_prices.sort_by(cmp_f64);
        let min = ordered_prices[0];
        let max = ordered_prices.last().unwrap();
        return 100.0 * ((prices.last().unwrap() - min) / (max - min));
    }

    /// The `slow_stochastic` is a momentum indicator that takes the moving average of passed in
    /// stochastic oscillators
    ///
    /// The standard length of stochastic oscillators is 3, and the constant model is a simple moving average
    ///
    /// # Arguments
    ///
    /// * `stochastics` - An `f64` slice of stochastics
    /// * `constant_model_type` - A variant of `ConstantModelType`
    ///
    /// # Examples
    ///
    /// ```
    /// let stochstic_oscillators = [0.0, 50.0, 100.0];
    ///
    /// let simple_ma_slow_stochastic = rust_ti::momentum_indicators::single::slow_stochastic(&stochstic_oscillators, &rust_ti::ConstantModelType::SimpleMovingAverage);
    /// assert_eq!(50.0, simple_ma_slow_stochastic);
    ///
    /// let sma_slow_stochastic =
    /// rust_ti::momentum_indicators::single::slow_stochastic(&stochstic_oscillators,
    /// &rust_ti::ConstantModelType::SmoothedMovingAverage);
    /// assert_eq!(63.15789473684211, sma_slow_stochastic);
    ///
    /// let median_slow_stochastic = rust_ti::momentum_indicators::single::slow_stochastic(&stochstic_oscillators, &rust_ti::ConstantModelType::SimpleMovingMedian);
    /// assert_eq!(50.0, median_slow_stochastic);
    /// ```
    pub fn slow_stochastic(
        stochastics: &[f64],
        constant_model_type: &crate::ConstantModelType,
    ) -> f64 {
        if stochastics.is_empty() {
            panic!("stochastics cannot be empty");
        };

        let slow_stochastic = match constant_model_type {
            ConstantModelType::SimpleMovingAverage => {
                moving_average(&stochastics, &MovingAverageType::Simple)
            }
            ConstantModelType::SmoothedMovingAverage => {
                moving_average(&stochastics, &MovingAverageType::Smoothed)
            }
            ConstantModelType::ExponentialMovingAverage => {
                moving_average(&stochastics, &MovingAverageType::Exponential)
            }
            ConstantModelType::PersonalisedMovingAverage(alpha_nominator, alpha_denominator) => {
                moving_average(
                    &stochastics,
                    &MovingAverageType::Personalised(alpha_nominator, alpha_denominator),
                )
            }
            ConstantModelType::SimpleMovingMedian => median(&stochastics),
            ConstantModelType::SimpleMovingMode => mode(&stochastics),
            _ => panic!("Unsupported ConstantModelType"),
        };
        return slow_stochastic;
    }

    /// The `slowest_stochastic` is a momentum indicator that takes the moving average of passed in
    /// slow stochastic oscillators
    ///
    /// The standard length of slow stochastics is 3, and the constant model is a simple moving average
    ///
    /// # Arguments
    ///
    /// * `slow_stochastics` - An `f64` slice of slow stochastics
    /// * `constant_model_type` - A variant of `ConstantModelType`
    ///
    /// # Examples
    ///
    /// ```
    /// let slow_stochstic = [30.0, 20.0, 10.0];
    ///
    /// let simple_ma_slowest_stochastic = rust_ti::momentum_indicators::single::slowest_stochastic(&slow_stochstic, &rust_ti::ConstantModelType::SimpleMovingAverage);
    /// assert_eq!(20.0, simple_ma_slowest_stochastic);
    ///
    /// let sma_slowest_stochastic =
    /// rust_ti::momentum_indicators::single::slowest_stochastic(&slow_stochstic,
    /// &rust_ti::ConstantModelType::SmoothedMovingAverage);
    /// assert_eq!(17.368421052631582, sma_slowest_stochastic);
    ///
    /// let median_slowest_stochastic = rust_ti::momentum_indicators::single::slowest_stochastic(&slow_stochstic, &rust_ti::ConstantModelType::SimpleMovingMedian);
    /// assert_eq!(20.0, median_slowest_stochastic);
    /// ```
    pub fn slowest_stochastic(
        slow_stochastics: &[f64],
        constant_model_type: &crate::ConstantModelType,
    ) -> f64 {
        if slow_stochastics.is_empty() {
            panic!("stochastics cannot be empty");
        };

        let slowest_stochastic = match constant_model_type {
            ConstantModelType::SimpleMovingAverage => {
                moving_average(&slow_stochastics, &MovingAverageType::Simple)
            }
            ConstantModelType::SmoothedMovingAverage => {
                moving_average(&slow_stochastics, &MovingAverageType::Smoothed)
            }
            ConstantModelType::ExponentialMovingAverage => {
                moving_average(&slow_stochastics, &MovingAverageType::Exponential)
            }
            ConstantModelType::PersonalisedMovingAverage(alpha_nominator, alpha_denominator) => {
                moving_average(
                    &slow_stochastics,
                    &MovingAverageType::Personalised(alpha_nominator, alpha_denominator),
                )
            }
            ConstantModelType::SimpleMovingMedian => median(&slow_stochastics),
            ConstantModelType::SimpleMovingMode => mode(&slow_stochastics),
            _ => panic!("Unsupported ConstantModelType"),
        };
        return slowest_stochastic;
    }

    /// `williams_percent_r` determines momentum by 
    ///
    /// The standard period is 14 days, so the high would be the max of the past 14 days
    ///
    /// # Arguments
    /// 
    /// * `high` - High price for the observed period
    /// * `low` - Low price for the observed period
    /// * `close` - Close price for the observed period
    ///
    /// # Examples
    ///
    /// ```
    /// let high = 200.0;
    /// let low = 175.0;
    /// let close = 192.0;
    /// let williams_percent_r = rust_ti::momentum_indicators::single::williams_percent_r(&high, &low,
    /// &close);
    /// assert_eq!(-32.0, williams_percent_r);
    /// ```
    pub fn williams_percent_r(high: &f64, low: &f64, close: &f64) -> f64 {
        return -100.0_f64 * ((high - close) / (high - low))
    }

    fn previous_gains_loss(prices: &[f64]) -> (Vec<f64>, Vec<f64>) {
        if prices.is_empty() {
            panic!("Prices is empty");
        };
        let mut previous_gains = Vec::new();
        let mut previous_loss = Vec::new();
        for (i, value) in prices.iter().enumerate() {
            // TODO: must be a better way to do this
            if i == 0 {
                continue;
            };
            if value > &prices[i - 1] {
                previous_gains.push(value - prices[i - 1]);
            } else if value < &prices[i - 1] {
                previous_loss.push(prices[i - 1] - value);
            };
        }
        return (previous_gains, previous_loss);
    }

    fn rsi(previous_average_gains: &f64, previous_average_loss: &f64) -> f64 {
        if previous_average_loss == &0.0_f64 {
            return 0.0;
        };
        return 100.0_f64
            - (100.0_f64 / (1.0_f64 + (previous_average_gains / previous_average_loss)));
    }

    fn cmp_f64(a: &f64, b: &f64) -> Ordering {
        if a < b {
            return Ordering::Less;
        } else if a > b {
            return Ordering::Greater;
        }
        return Ordering::Equal;
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
    /// assert_eq!(vec![34.51776649746324, 12.837837837836929, 34.51776649746182, 61.26126126126377], personalised_rsi);
    /// ```
    pub fn relative_strength_index(
        prices: &[f64],
        constant_model_type: &crate::ConstantModelType,
        period: &usize,
    ) -> Vec<f64> {
        let length = prices.len();
        if period > &length {
            panic!(
                "Period ({}) is longer than length of prices ({})",
                period, length
            );
        };

        let mut rsis = Vec::new();
        for i in 0..length {
            let end_index = period + i;
            if end_index > length {
                break;
            };
            rsis.push(single::relative_strength_index(
                &prices[i..end_index],
                constant_model_type,
            ));
        }
        return rsis;
    }
    /// The `mcginley_dynamic_rsi` is a variation of the `relative_strength_index` that accepts
    /// previous dynamics for the gains and loss calculations, but also returns the new ones so
    /// that they can be used going forward
    ///
    /// # Arguments
    ///  
    /// * `prices` - An `f64` slice of prices
    /// * `previous_gain_mcginley_dynamic` - The previous McGinley dynamic used for the gains
    /// calculation. Use 0.0 if it hasn't yet been calculated.
    /// * `previous_loss_mcginley_dynamic` - The previous McGinley dynamic used for the loss
    /// caclulation. Use 0.0 if it hasn't yet been calculated.
    /// * `period` - Period over which to calculate the McGinley dynamic RSI
    ///
    /// # Examples
    ///
    /// ```
    /// let prices = vec![100.0, 102.0, 103.0, 101.0, 100.0, 97.0, 102.0, 103.0];
    /// let period: usize = 5;
    /// let previous_gain_dynamic = 0.0;
    /// let previous_loss_dynamic = 0.0;
    /// let mcginley_rsi = rust_ti::momentum_indicators::bulk::mcginley_dynamic_rsi(&prices,
    /// &previous_gain_dynamic, &previous_loss_dynamic, &period);
    /// assert_eq!(vec![(50.0, 1.0, 1.0), (49.795081967213115, 1.0, 1.008230452674897),
    /// (49.745434479572864, 1.0064, 1.0167002312649644), (49.344186087882605, 1.003117290207188, 1.0297813544692562)], mcginley_rsi);
    /// ```
    pub fn mcginley_dynamic_rsi(
        prices: &[f64],
        previous_gain_mcginley_dynamic: &f64,
        previous_loss_mcginley_dynamic: &f64,
        period: &usize,
    ) -> Vec<(f64, f64, f64)> {
        let length = prices.len();
        if &length < period {
            panic!(
                "Period ({}) is longer than length ({}) of prices",
                period, length
            );
        };
        let mut rsis = Vec::new();
        for i in 0..length {
            let end_index = period + i;
            if end_index > length {
                break;
            };
            if i == 0 {
                rsis.push(single::mcginley_dynamic_rsi(
                    &prices[i..end_index],
                    previous_gain_mcginley_dynamic,
                    previous_loss_mcginley_dynamic,
                ));
            } else {
                rsis.push(single::mcginley_dynamic_rsi(
                    &prices[i..end_index],
                    &rsis[i - 1].1,
                    &rsis[i - 1].2,
                ));
            };
        }
        return rsis;
    }

    /// The `stochastic_oscillator` is a momentum indicator that is calculated by using support and
    /// resistance levels
    ///
    /// When developed by George Lane the stochastic oscillator looked at a period of 14 days as
    /// the work week had 7 days back then, however this implementation allows for prices of any
    /// length to be used
    ///
    /// # Arguments
    ///
    /// * `prices` - An `f64` slice of prices
    /// * `period` - Period over which to calculate the stochastic oscillator
    ///
    /// # Examples
    ///
    /// ```
    /// let prices = vec![100.0, 102.0, 103.0, 101.0, 99.0];
    /// let period: usize = 3;
    /// let stochastic_oscillator = rust_ti::momentum_indicators::bulk::stochastic_oscillator(&prices, &period);
    /// assert_eq!(vec![100.0, 0.0, 0.0], stochastic_oscillator);
    /// ```
    pub fn stochastic_oscillator(prices: &[f64], period: &usize) -> Vec<f64> {
        let length = prices.len();
        if period > &length {
            panic!(
                "Period ({}) cannot be greater than length ({}) of prices",
                period, length
            );
        };

        let mut so = Vec::new();
        for i in 0..length {
            let end_index = period + i;
            if end_index > length {
                break;
            };
            so.push(single::stochastic_oscillator(&prices[i..end_index]));
        }
        return so;
    }

    /// The `slow_stochastic` is a momentum indicator that takes the moving average of passed in
    /// stochastic oscillators
    ///
    /// The standard length of prices is 3, and the constant model is a simple moving average
    ///
    /// # Arguments
    ///
    /// * `stochastics` - An `f64` slice of prices
    /// * `constant_model_type` - A variant of `ConstantModelType`
    /// * `period` - Period over which to calculate the stochastic oscillator
    /// 
    /// # Examples
    ///
    /// ```
    /// let stochstic_oscillators = [0.0, 25.0, 50.0, 33.0, 73.0];
    /// let period: usize = 3;
    ///
    /// let simple_ma_slow_stochastic = rust_ti::momentum_indicators::bulk::slow_stochastic(&stochstic_oscillators, &rust_ti::ConstantModelType::SimpleMovingAverage, &period);
    /// assert_eq!(vec![25.0, 36.0, 52.0], simple_ma_slow_stochastic);
    ///
    /// let sma_slow_stochastic =
    /// rust_ti::momentum_indicators::bulk::slow_stochastic(&stochstic_oscillators,
    /// &rust_ti::ConstantModelType::SmoothedMovingAverage, &period);
    /// assert_eq!(vec![31.578947368421055, 36.684210526315795, 55.526315789473685], sma_slow_stochastic);
    ///
    /// let median_slow_stochastic = rust_ti::momentum_indicators::bulk::slow_stochastic(&stochstic_oscillators, &rust_ti::ConstantModelType::SimpleMovingMedian, &period);
    /// assert_eq!(vec![25.0, 33.0, 50.0], median_slow_stochastic);
    /// ```
    pub fn slow_stochastic(
        stochastics: &[f64],
        constant_model_type: &crate::ConstantModelType,
        period: &usize,
    ) -> Vec<f64> {
        let length = stochastics.len();
        if period > &length {
            panic!(
                "Period ({}) cannot be greater than length ({}) of stochastics",
                period, length
            );
        };
        let mut sso = Vec::new();
        for i in 0..length {
            let end_index = period + i;
            if end_index > length {
                break;
            };
            sso.push(single::slow_stochastic(
                &stochastics[i..end_index],
                constant_model_type,
            ));
        }
        return sso;
    }

    /// The `slowest_stochastic` is a momentum indicator that takes the moving average of passed in
    /// slow stochastics
    ///
    /// The standard length of prices is 3, and the constant model is a simple moving average
    ///
    /// # Arguments
    ///
    /// * `slow_stochastics` - An `f64` slice of prices
    /// * `constant_model_type` - A variant of `ConstantModelType`
    /// * `period` - Period over which to calculate the stochastic oscillator
    /// 
    /// # Examples
    ///
    /// ```
    /// let slow_stochstics = [75.0, 60.0, 73.0, 58.0];
    /// let period: usize = 3;
    ///
    /// let ma_slowest_stochastic = rust_ti::momentum_indicators::bulk::slowest_stochastic(&slow_stochstics, &rust_ti::ConstantModelType::SimpleMovingAverage, &period);
    /// assert_eq!(vec![69.33333333333333, 63.666666666666664], ma_slowest_stochastic);
    ///
    /// let sma_slowest_stochastic =
    /// rust_ti::momentum_indicators::bulk::slow_stochastic(&slow_stochstics,
    /// &rust_ti::ConstantModelType::SmoothedMovingAverage, &period);
    /// assert_eq!(vec![69.31578947368422, 63.15789473684211], sma_slowest_stochastic);
    ///
    /// let median_slowest_stochastic = rust_ti::momentum_indicators::bulk::slow_stochastic(&slow_stochstics, &rust_ti::ConstantModelType::SimpleMovingMedian, &period);
    /// assert_eq!(vec![73.0, 60.0], median_slowest_stochastic);
    /// ```
    pub fn slowest_stochastic(
        slow_stochastics: &[f64],
        constant_model_type: &crate::ConstantModelType,
        period: &usize,
    ) -> Vec<f64> {
        let length = slow_stochastics.len();
        if period > &length {
            panic!(
                "Period ({}) cannot be greater than length ({}) of stochastics",
                period, length
            );
        };
        let mut sso = Vec::new();
        for i in 0..length {
            let end_index = period + i;
            if end_index > length {
                break;
            };
            sso.push(single::slowest_stochastic(
                &slow_stochastics[i..end_index],
                constant_model_type,
            ));
        }
        return sso;
    }

    /// 
    pub fn williams_percent_r(
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_ma_rsi() {
        let prices = vec![100.2, 100.46, 100.53, 100.38, 100.19];
        assert_eq!(
            49.2537313432832,
            single::relative_strength_index(
                &prices,
                &crate::ConstantModelType::SimpleMovingAverage
            )
        );
    }

    #[test]
    fn test_single_short_median_rsi() {
        // Because there are too few values, ends up being the means
        let prices = vec![100.2, 100.46, 100.53, 100.38, 100.19];
        assert_eq!(
            49.2537313432832,
            single::relative_strength_index(&prices, &crate::ConstantModelType::SimpleMovingMedian)
        );
    }

    #[test]
    fn test_single_long_median_rsi() {
        let prices = vec![
            100.2, 100.46, 100.53, 100.38, 100.19, 100.21, 100.32, 100.28,
        ];
        assert_eq!(
            37.5,
            single::relative_strength_index(&prices, &crate::ConstantModelType::SimpleMovingMedian)
        );
    }

    #[test]
    fn test_single_small_mode_rsi() {
        // Mode rounds the values, the difference being so small all rounds down to 0.0
        let prices = vec![100.2, 100.46, 100.53, 100.38, 100.19];
        assert_eq!(
            0.0,
            single::relative_strength_index(&prices, &crate::ConstantModelType::SimpleMovingMode)
        );
    }

    #[test]
    fn test_single_large_mode_rsi() {
        let prices = vec![100.0, 103.0, 106.0, 107.0, 108.0, 105.0, 102.0];
        assert_eq!(
            39.99999999999999,
            single::relative_strength_index(&prices, &crate::ConstantModelType::SimpleMovingMode)
        );
    }

    #[test]
    fn test_single_smoothed_rsi() {
        let prices = vec![100.2, 100.46, 100.53, 100.38, 100.19];
        assert_eq!(
            43.01075268817234,
            single::relative_strength_index(
                &prices,
                &crate::ConstantModelType::SmoothedMovingAverage
            )
        );
    }

    #[test]
    fn test_single_exponential_rsi() {
        let prices = vec![100.2, 100.46, 100.53, 100.38, 100.19];
        assert_eq!(
            39.495798319328436,
            single::relative_strength_index(
                &prices,
                &crate::ConstantModelType::ExponentialMovingAverage
            )
        );
    }

    #[test]
    fn test_single_personalised_rsi() {
        let prices = vec![100.2, 100.46, 100.53, 100.38, 100.19];
        assert_eq!(
            35.6725146198842,
            single::relative_strength_index(
                &prices,
                &crate::ConstantModelType::PersonalisedMovingAverage(&4.0, &3.0)
            )
        );
    }

    #[test]
    fn test_single_only_price_rise_rsi() {
        let prices = vec![100.0, 101.0, 102.0, 103.0];
        assert_eq!(
            100.0,
            single::relative_strength_index(
                &prices,
                &crate::ConstantModelType::SimpleMovingAverage
            )
        );
    }

    #[test]
    fn test_single_only_price_fall_rsi() {
        let prices = vec![103.0, 102.0, 101.0, 100.0];
        assert_eq!(
            0.0,
            single::relative_strength_index(
                &prices,
                &crate::ConstantModelType::SimpleMovingAverage
            )
        );
    }

    #[test]
    #[should_panic]
    fn test_single_rsi_panic() {
        let prices = Vec::new();
        single::relative_strength_index(&prices, &crate::ConstantModelType::SimpleMovingAverage);
    }

    #[test]
    fn test_bulk_simple_ma_rsi() {
        let prices = vec![
            100.2, 100.46, 100.53, 100.38, 100.19, 100.21, 100.32, 100.28,
        ];
        let period: usize = 5;
        assert_eq!(
            vec![
                49.2537313432832,
                20.930232558140005,
                27.6595744680842,
                36.111111111111335
            ],
            bulk::relative_strength_index(
                &prices,
                &crate::ConstantModelType::SimpleMovingAverage,
                &period
            )
        );
    }

    #[test]
    fn test_bulk_smoothed_ma_rsi() {
        let prices = vec![
            100.2, 100.46, 100.53, 100.38, 100.19, 100.21, 100.32, 100.28,
        ];
        let period: usize = 5;
        assert_eq!(
            vec![
                43.01075268817234,
                17.187499999999886,
                31.168831168830664,
                47.05882352941291
            ],
            bulk::relative_strength_index(
                &prices,
                &crate::ConstantModelType::SmoothedMovingAverage,
                &period
            )
        );
    }

    #[test]
    fn test_bulk_exponential_ma_rsi() {
        let prices = vec![
            100.2, 100.46, 100.53, 100.38, 100.19, 100.21, 100.32, 100.28,
        ];
        let period: usize = 5;
        assert_eq!(
            vec![
                39.495798319328436,
                15.2941176470584,
                32.71028037383145,
                53.03030303030472
            ],
            bulk::relative_strength_index(
                &prices,
                &crate::ConstantModelType::ExponentialMovingAverage,
                &period
            )
        );
    }

    #[test]
    fn test_bulk_personalised_ma_rsi() {
        let prices = vec![
            100.2, 100.46, 100.53, 100.38, 100.19, 100.21, 100.32, 100.28,
        ];
        let period: usize = 5;
        assert_eq!(
            vec![
                35.6725146198842,
                13.385826771652745,
                34.13173652694594,
                59.375000000002316
            ],
            bulk::relative_strength_index(
                &prices,
                &crate::ConstantModelType::PersonalisedMovingAverage(&4.0, &3.0),
                &period
            )
        );
    }

    #[test]
    fn test_bulk_simple_median_rsi() {
        let prices = vec![
            100.2, 100.46, 100.53, 100.38, 100.19, 100.21, 100.32, 100.28,
        ];
        let period: usize = 5;
        assert_eq!(
            vec![
                49.2537313432832,
                20.930232558140005,
                27.6595744680842,
                36.111111111111335
            ],
            bulk::relative_strength_index(
                &prices,
                &crate::ConstantModelType::SimpleMovingMedian,
                &period
            )
        );
    }

    #[test]
    fn test_bulk_simple_mode_rsi() {
        let prices = vec![
            100.2, 100.46, 100.53, 100.38, 100.19, 100.21, 100.32, 100.28,
        ];
        let period: usize = 5;
        assert_eq!(
            vec![0.0, 0.0, 0.0, 0.0],
            bulk::relative_strength_index(
                &prices,
                &crate::ConstantModelType::SimpleMovingMode,
                &period
            )
        );
    }

    #[test]
    #[should_panic]
    fn test_bulk_rsi_panic() {
        let prices = vec![
            100.2, 100.46, 100.53, 100.38, 100.19, 100.21, 100.32, 100.28,
        ];
        let period: usize = 50;
        bulk::relative_strength_index(
            &prices,
            &crate::ConstantModelType::SimpleMovingAverage,
            &period,
        );
    }

    #[test]
    fn test_mcginley_dynamic_rsi_no_previous() {
        let prices = vec![100.2, 100.46, 100.53, 100.38, 100.19];
        assert_eq!(
            (26.923076923079236, 0.07000000000000739, 0.18999999999999773),
            single::mcginley_dynamic_rsi(&prices, &0.0, &0.0)
        );
    }

    #[test]
    fn test_mcginley_dynamic_rsi_previous() {
        let prices = vec![100.46, 100.53, 100.38, 100.19, 100.21];
        assert_eq!(
            (105.44168978787299, -3.6815625000054153, 0.18999999999999773),
            single::mcginley_dynamic_rsi(&prices, &0.07000000000000739, &0.18999999999999773)
        );
    }

    #[test]
    #[should_panic]
    fn test_mcginley_dynamic_rsi_panic() {
        let prices = Vec::new();
        single::mcginley_dynamic_rsi(&prices, &0.0, &0.0);
    }

    #[test]
    fn test_bulk_mcginley_dynamic_rsi_no_previous() {
        let prices = vec![
            100.2, 100.46, 100.53, 100.38, 100.19, 100.21, 100.32, 100.28,
        ];
        let period: usize = 5;
        assert_eq!(
            vec![
                (26.923076923079236, 0.07000000000000739, 0.18999999999999773),
                (105.44168978787299, -3.6815625000054153, 0.18999999999999773),
                (99.99999201255198, 2378732.0357780023, 0.18999999999999773),
                (100.0, -2.6009191432509403e35, -37.989980468780004)
            ],
            bulk::mcginley_dynamic_rsi(&prices, &0.0, &0.0, &period)
        );
    }

    #[test]
    fn test_bulk_mcginley_dynamic_rsi_previous() {
        let prices = vec![100.46, 100.53, 100.38, 100.19, 100.21, 100.32, 100.28];
        let period: usize = 5;
        assert_eq!(
            vec![
                (105.44168978787299, -3.6815625000054153, 0.18999999999999773),
                (99.99999201255198, 2378732.0357780023, 0.18999999999999773),
                (100.0, -2.6009191432509403e35, -37.989980468780004)
            ],
            bulk::mcginley_dynamic_rsi(
                &prices,
                &0.07000000000000739,
                &0.18999999999999773,
                &period
            )
        );
    }

    #[test]
    #[should_panic]
    fn test_bulk_mcginley_dynamic_rsi_panic() {
        let prices = vec![
            100.2, 100.46, 100.53, 100.38, 100.19, 100.21, 100.32, 100.28,
        ];
        let period: usize = 50;
        bulk::mcginley_dynamic_rsi(&prices, &0.0, &0.0, &period);
    }

    #[test]
    fn test_single_stochastic_oscillator_min() {
        let prices = vec![100.2, 100.46, 100.53, 100.38, 100.19];
        assert_eq!(0.0, single::stochastic_oscillator(&prices));
    }

    #[test]
    fn test_single_stochastic_oscillator_max() {
        let prices = vec![100.2, 100.46, 100.53, 100.38, 100.66];
        assert_eq!(100.0, single::stochastic_oscillator(&prices));
    }

    #[test]
    fn test_single_stochastic_oscillator() {
        let prices = vec![100.2, 100.46, 100.53, 100.38, 100.34];
        assert_eq!(42.42424242424281, single::stochastic_oscillator(&prices));
    }

    #[test]
    #[should_panic]
    fn test_single_stochastic_oscillator_panic() {
        let prices = Vec::new();
        single::stochastic_oscillator(&prices);
    }

    #[test]
    fn test_bulk_stochastic_oscillator() {
        let prices = vec![
            100.2, 100.46, 100.53, 100.38, 100.19, 100.21, 100.32, 100.28,
        ];
        let period: usize = 5;
        assert_eq!(
            vec![0.0, 5.882352941175241, 38.23529411764534, 47.36842105263394],
            bulk::stochastic_oscillator(&prices, &period)
        );
    }

    #[test]
    #[should_panic]
    fn test_bulk_stochastic_oscillator_bulk() {
        let prices = vec![
            100.2, 100.46, 100.53, 100.38, 100.19, 100.21, 100.32, 100.28,
        ];
        let period: usize = 50;
        bulk::stochastic_oscillator(&prices, &period);
    }

    #[test]
    fn test_single_ma_slow_stochastic() {
        let stochastics = vec![0.0, 5.882352941175241, 38.23529411764534, 47.36842105263394];
        assert_eq!(
            22.871517027863632,
            single::slow_stochastic(&stochastics, &crate::ConstantModelType::SimpleMovingAverage)
        );
    }

    #[test]
    fn test_single_sma_slow_stochastic() {
        let stochastics = vec![0.0, 5.882352941175241, 38.23529411764534, 47.36842105263394];
        assert_eq!(
            29.02078726227347,
            single::slow_stochastic(
                &stochastics,
                &crate::ConstantModelType::SmoothedMovingAverage
            )
        );
    }

    #[test]
    fn test_single_ema_slow_stochastic() {
        let stochastics = vec![0.0, 5.882352941175241, 38.23529411764534, 47.36842105263394];
        assert_eq!(
            33.284579311601206,
            single::slow_stochastic(
                &stochastics,
                &crate::ConstantModelType::ExponentialMovingAverage
            )
        );
    }

    #[test]
    fn test_single_pma_slow_stochastic() {
        let stochastics = vec![0.0, 5.882352941175241, 38.23529411764534, 47.36842105263394];
        assert_eq!(
            39.872151259403616,
            single::slow_stochastic(
                &stochastics,
                &crate::ConstantModelType::PersonalisedMovingAverage(&5.0, &4.0)
            )
        );
    }

    #[test]
    fn test_single_median_slow_stochastic() {
        let stochastics = vec![0.0, 5.882352941175241, 38.23529411764534, 47.36842105263394];
        assert_eq!(
            22.05882352941029,
            single::slow_stochastic(&stochastics, &crate::ConstantModelType::SimpleMovingMedian)
        );
    }

    #[test]
    fn test_single_mode_slow_stochastic() {
        let stochastics = vec![0.0, 5.882352941175241, 38.23529411764534, 47.36842105263394];
        assert_eq!(
            22.75,
            single::slow_stochastic(&stochastics, &crate::ConstantModelType::SimpleMovingMode)
        );
    }

    #[test]
    #[should_panic]
    fn test_single_slow_stochastic_panic() {
        let stochastics = Vec::new();
        single::slow_stochastic(&stochastics, &crate::ConstantModelType::SimpleMovingMode);
    }

    #[test]
    fn test_bulk_ma_slow_stochastic() {
        let stochastics = vec![0.0, 5.882352941175241, 38.23529411764534, 47.36842105263394];
        let period: usize = 3;
        assert_eq!(
            vec![14.705882352940193, 30.49535603715151],
            bulk::slow_stochastic(&stochastics, &crate::ConstantModelType::SimpleMovingAverage, &period)
        );
    }

    #[test]
    fn test_bulk_sma_slow_stochastic() {
        let stochastics = vec![0.0, 5.882352941175241, 38.23529411764534, 47.36842105263394];
        let period: usize = 3;
        assert_eq!(
            vec![19.969040247676816, 35.75036662864623],
            bulk::slow_stochastic(
                &stochastics,
                &crate::ConstantModelType::SmoothedMovingAverage,
                &period
            )
        );
    }

    #[test]
    fn test_bulk_ema_slow_stochastic() {
        let stochastics = vec![0.0, 5.882352941175241, 38.23529411764534, 47.36842105263394];
        let period: usize = 3;
        assert_eq!(
            vec![23.529411764704548, 38.832375055285944],
            bulk::slow_stochastic(
                &stochastics,
                &crate::ConstantModelType::ExponentialMovingAverage,
                &period
            )
        );
    }

    #[test]
    fn test_bulk_pma_slow_stochastic() {
        let stochastics = vec![0.0, 5.882352941175241, 38.23529411764534, 47.36842105263394];
        let period: usize = 3;
        assert_eq!(
            vec![29.192273924493655, 42.98322628344476],
            bulk::slow_stochastic(
                &stochastics,
                &crate::ConstantModelType::PersonalisedMovingAverage(&5.0, &4.0),
                &period
            )
        );
    }

    #[test]
    fn test_bulk_median_slow_stochastic() {
        let stochastics = vec![0.0, 5.882352941175241, 38.23529411764534, 47.36842105263394];
        let period: usize = 3;
        assert_eq!(
            vec![5.882352941175241, 38.23529411764534],
            bulk::slow_stochastic(&stochastics, &crate::ConstantModelType::SimpleMovingMedian, &period)
        );
    }

    #[test]
    fn test_bulk_mode_slow_stochastic() {
        let stochastics = vec![0.0, 5.882352941175241, 38.23529411764534, 47.36842105263394];
        let period: usize = 3;
        assert_eq!(
            vec![14.666666666666666, 30.3333333333333332],
            bulk::slow_stochastic(&stochastics, &crate::ConstantModelType::SimpleMovingMode, &period)
        );
    }

    #[test]
    #[should_panic]
    fn test_bulk_slow_stochastic_panic() {
        let stochastics = vec![0.0, 5.882352941175241, 38.23529411764534, 47.36842105263394];
        let period: usize = 30;
        bulk::slow_stochastic(&stochastics, &crate::ConstantModelType::SimpleMovingMode, &period);
    }
    
    #[test]
    fn test_single_ma_slowest_stochastic() {
        let stochastics = vec![0.0, 5.882352941175241, 38.23529411764534, 47.36842105263394];
        assert_eq!(
            22.871517027863632,
            single::slowest_stochastic(&stochastics, &crate::ConstantModelType::SimpleMovingAverage)
        );
    }

    #[test]
    fn test_single_sma_slowest_stochastic() {
        let stochastics = vec![0.0, 5.882352941175241, 38.23529411764534, 47.36842105263394];
        assert_eq!(
            29.02078726227347,
            single::slowest_stochastic(
                &stochastics,
                &crate::ConstantModelType::SmoothedMovingAverage
            )
        );
    }

    #[test]
    fn test_single_ema_slowest_stochastic() {
        let stochastics = vec![0.0, 5.882352941175241, 38.23529411764534, 47.36842105263394];
        assert_eq!(
            33.284579311601206,
            single::slowest_stochastic(
                &stochastics,
                &crate::ConstantModelType::ExponentialMovingAverage
            )
        );
    }

    #[test]
    fn test_single_pma_slowest_stochastic() {
        let stochastics = vec![0.0, 5.882352941175241, 38.23529411764534, 47.36842105263394];
        assert_eq!(
            39.872151259403616,
            single::slowest_stochastic(
                &stochastics,
                &crate::ConstantModelType::PersonalisedMovingAverage(&5.0, &4.0)
            )
        );
    }

    #[test]
    fn test_single_median_slowest_stochastic() {
        let stochastics = vec![0.0, 5.882352941175241, 38.23529411764534, 47.36842105263394];
        assert_eq!(
            22.05882352941029,
            single::slowest_stochastic(&stochastics, &crate::ConstantModelType::SimpleMovingMedian)
        );
    }

    #[test]
    fn test_single_mode_slowest_stochastic() {
        let stochastics = vec![0.0, 5.882352941175241, 38.23529411764534, 47.36842105263394];
        assert_eq!(
            22.75,
            single::slowest_stochastic(&stochastics, &crate::ConstantModelType::SimpleMovingMode)
        );
    }

    #[test]
    #[should_panic]
    fn test_single_slowest_stochastic_panic() {
        let stochastics = Vec::new();
        single::slowest_stochastic(&stochastics, &crate::ConstantModelType::SimpleMovingMode);
    }

    #[test]
    fn test_bulk_ma_slowest_stochastic() {
        let stochastics = vec![0.0, 5.882352941175241, 38.23529411764534, 47.36842105263394];
        let period: usize = 3;
        assert_eq!(
            vec![14.705882352940193, 30.49535603715151],
            bulk::slowest_stochastic(&stochastics, &crate::ConstantModelType::SimpleMovingAverage, &period)
        );
    }

    #[test]
    fn test_bulk_sma_slowest_stochastic() {
        let stochastics = vec![0.0, 5.882352941175241, 38.23529411764534, 47.36842105263394];
        let period: usize = 3;
        assert_eq!(
            vec![19.969040247676816, 35.75036662864623],
            bulk::slowest_stochastic(
                &stochastics,
                &crate::ConstantModelType::SmoothedMovingAverage,
                &period
            )
        );
    }

    #[test]
    fn test_bulk_ema_slowest_stochastic() {
        let stochastics = vec![0.0, 5.882352941175241, 38.23529411764534, 47.36842105263394];
        let period: usize = 3;
        assert_eq!(
            vec![23.529411764704548, 38.832375055285944],
            bulk::slowest_stochastic(
                &stochastics,
                &crate::ConstantModelType::ExponentialMovingAverage,
                &period
            )
        );
    }

    #[test]
    fn test_bulk_pma_slowest_stochastic() {
        let stochastics = vec![0.0, 5.882352941175241, 38.23529411764534, 47.36842105263394];
        let period: usize = 3;
        assert_eq!(
            vec![29.192273924493655, 42.98322628344476],
            bulk::slowest_stochastic(
                &stochastics,
                &crate::ConstantModelType::PersonalisedMovingAverage(&5.0, &4.0),
                &period
            )
        );
    }

    #[test]
    fn test_bulk_median_slowest_stochastic() {
        let stochastics = vec![0.0, 5.882352941175241, 38.23529411764534, 47.36842105263394];
        let period: usize = 3;
        assert_eq!(
            vec![5.882352941175241, 38.23529411764534],
            bulk::slowest_stochastic(&stochastics, &crate::ConstantModelType::SimpleMovingMedian, &period)
        );
    }

    #[test]
    fn test_bulk_mode_slowest_stochastic() {
        let stochastics = vec![0.0, 5.882352941175241, 38.23529411764534, 47.36842105263394];
        let period: usize = 3;
        assert_eq!(
            vec![14.666666666666666, 30.3333333333333332],
            bulk::slowest_stochastic(&stochastics, &crate::ConstantModelType::SimpleMovingMode, &period)
        );
    }

    #[test]
    #[should_panic]
    fn test_bulk_slowest_stochastic_panic() {
        let stochastics = vec![0.0, 5.882352941175241, 38.23529411764534, 47.36842105263394];
        let period: usize = 30;
        bulk::slowest_stochastic(&stochastics, &crate::ConstantModelType::SimpleMovingMode, &period);
    }

    #[test]
    fn test_single_williams_percent_r() {
        let high = 100.93;
        let low = 100.37;
        let close = 100.49;
        assert_eq!(-78.57142857143037, single::williams_percent_r(&high, &low, &close));
    }
}
