//! # Standard Indicators
//!
//! Standard indicators are a collection of indicators that aren't configurable and use the
//! standard, industry agreed defaults.
//!
//! These functions are wrappers for the configurable functions but with the configurations
//! hardcoded to be the standard defaults
//!
//! ## Bulk
//!
//! * [`simple_moving_average`](bulk::simple_moving_average)
//! * [`smoothed_moving_average`](bulk::smoothed_moving_average)
//! * [`exponential_moving_average`](bulk::exponential_moving_average)
//! * [`bollinger_bands`](bulk::bollinger_bands)
//! * [`macd`](bulk::macd)
//! * [`rsi`](bulk::rsi)
//!
//! ## Single
//!
//! * [`simple_moving_average`](single::simple_moving_average)
//! * [`smoothed_moving_average`](single::smoothed_moving_average)
//! * [`exponential_moving_average`](single::exponential_moving_average)
//! * [`bollinger_bands`](single::bollinger_bands)
//! * [`macd`](single::macd)
//! * [`rsi`](single::rsi)

/// `single` module holds functions that return a singular value
pub mod single {
    use crate::candle_indicators::single::moving_constant_bands;
    use crate::momentum_indicators::single::{macd_line, relative_strength_index, signal_line};
    use crate::moving_average::single::moving_average;
    use crate::{ConstantModelType, DeviationModel, MovingAverageType};

    /// The `simple_moving_average` is essentially just the mean
    ///
    /// # Arguments
    ///
    /// * `prices` - Slice of prices
    ///
    /// # Panics
    ///
    /// `simple_moving_average` will panic if prices is empty
    ///
    /// # Examples
    ///
    /// ```rust
    /// let prices = vec![100.0, 102.0, 103.0, 101.0, 100.0];
    ///
    /// let simple_moving_average =
    /// rust_ti::standard_indicators::single::simple_moving_average(&prices);
    /// assert_eq!(101.2, simple_moving_average);
    /// ```
    pub fn simple_moving_average(prices: &[f64]) -> f64 {
        if prices.is_empty() {
            panic!("Prices cannot be empty")
        };

        return moving_average(&prices, &MovingAverageType::Simple);
    }

    /// The `smoothed_moving_average` puts more weight on recent prices
    ///
    /// # Arguments
    ///
    /// * `prices` - Slice of prices
    ///
    /// # Panics
    ///
    /// `smoothed_moving_average` will panic if prices is empty
    ///
    /// # Examples
    ///
    /// ```rust
    /// let prices = vec![100.0, 102.0, 103.0, 101.0, 100.0];
    ///
    /// let smoothed_moving_average =
    /// rust_ti::standard_indicators::single::smoothed_moving_average(&prices);
    /// assert_eq!(101.11375535459305, smoothed_moving_average);
    /// ```
    pub fn smoothed_moving_average(prices: &[f64]) -> f64 {
        if prices.is_empty() {
            panic!("Prices cannot be empty")
        };

        return moving_average(&prices, &MovingAverageType::Smoothed);
    }

    /// The `exponential_moving_average` puts exponentially more weight on recent prices
    ///
    /// # Arguments
    ///
    /// * `prices` - Slice of prices
    ///
    /// # Panics
    ///
    /// `exponential_moving_average` will panic if prices is empty
    ///
    /// # Examples
    ///
    /// ```rust
    /// let prices = vec![100.0, 102.0, 103.0, 101.0, 100.0];
    ///
    /// let exponential_moving_average =
    /// rust_ti::standard_indicators::single::exponential_moving_average(&prices);
    /// assert_eq!(100.99526066350714, exponential_moving_average);
    /// ```
    pub fn exponential_moving_average(prices: &[f64]) -> f64 {
        if prices.is_empty() {
            panic!("Prices cannot be empty")
        };

        return moving_average(&prices, &MovingAverageType::Exponential);
    }

    /// `bollinger_bands` are an indicator created by John Bollinger in the 80s.
    ///
    /// The function returns a tulpe with the lower band, moving average, and upper band in that order.
    ///
    /// The `bollinger_bands` function is a wrapper for the
    /// [`moving_constant_bands`] using a
    /// simple moving average, standard deviation, and a deviation multiplier of 2.
    ///
    /// The standard period for Bollinger Bands is 20 periods, as such the function will panic if
    /// there aren't exactly 20 periods.
    ///
    /// # Arguments
    ///
    /// * `prices` - Slice of prices
    ///
    /// # Panics
    ///
    /// `bollinger_bands` will panic if:
    ///     * `prices` is empty
    ///     * length of `prices` isn't equal to 20
    ///
    /// # Example
    ///
    /// ```rust
    /// let prices = vec![
    ///     5238.34, 5294.39, 5306.26, 5297.43, 5311.95, 5314.53, 5305.4, 5288.88, 5298.25,
    ///     5300.94, 5270.64, 5239.26, 5249.84, 5273.27, 5282.59, 5335.27, 5350.22, 5351.13,
    ///     5352.7, 5359.50
    /// ];
    /// let bbands = rust_ti::standard_indicators::single::bollinger_bands(&prices);
    /// assert_eq!((5230.115435120723, 5301.039500000001, 5371.963564879278), bbands);
    /// ```
    pub fn bollinger_bands(prices: &[f64]) -> (f64, f64, f64) {
        if prices.is_empty() {
            panic!("Prices cannot be empty");
        };
        if prices.len() != 20 {
            panic!(
                "Prices must be 20 periods long not {} periods",
                prices.len()
            )
        };

        return moving_constant_bands(
            prices,
            ConstantModelType::SimpleMovingAverage,
            DeviationModel::StandardDeviation,
            2.0,
        );
    }

    /// The `macd`, short for the Moving Average Convergence/Divergencei,
    /// was created by Gerald Appel in the late 1970s when the working week was 6 days, and daily
    /// charts were primarily used to study stocks.
    ///
    /// The standard MACD uses a long period of 26, a short period of 12, and an exponential moving
    /// average model for both. The MACD will be returned first
    ///
    /// This function will return the signal line second. It uses a period of 9 and an exponential
    /// moving average.
    ///
    /// Finally the MACD histogram will be returned, which is simply the difference between MACD
    /// and signal line.
    ///
    /// # Arguments
    ///
    /// * `prices` - Slice of prices
    ///
    /// # Panics
    ///
    /// `macd` will panic if length of `prices` isn't equal to 35.
    ///
    /// 26 periods for the MACD, and as the signal does the exponential moving average of the MACD
    /// over periods, these need to be added to the total length.
    ///
    /// # Examples
    ///
    /// ```rust
    /// let prices = vec![
    ///     5069.38, 5032.25, 5095.91, 5109.44, 5060.61, 5042.65, 5049.49, 5122.71,
    ///     5168.05, 5188.96, 5181.83, 5203.26, 5224.01, 5223.28, 5238.34, 5294.39,
    ///     5306.26, 5297.44, 5311.95, 5314.53, 5305.4, 5288.88, 5298.25, 5300.95,
    ///     5270.64, 5239.26, 5249.84, 5273.28, 5282.59, 5335.28, 5350.22, 5351.13,
    ///     5352.7, 5359.51,
    /// ];
    ///
    /// let macd = rust_ti::standard_indicators::single::macd(&prices);
    /// assert_eq!((23.98848088685554, 24.707439348177488, -0.7189584613219466), macd);
    /// ```
    pub fn macd(prices: &[f64]) -> (f64, f64, f64) {
        if prices.len() != 34 {
            panic!(
                "Prices must be 34 periods long, not {} periods",
                prices.len()
            )
        };
        let mut macds = Vec::new();
        let model = crate::ConstantModelType::ExponentialMovingAverage;
        for i in 0..9 {
            macds.push(macd_line(&prices[i..i + 26], &12_usize, &model, &model));
        }
        let signal = signal_line(&macds, &model);
        return (macds[8], signal, macds[8] - signal);
    }

    /// The `rsi` or Relative Strength Index, is a momentum indicator that measures velocity and
    /// magnitude of price movements.
    ///
    /// The standard period used is 14 days, and the model is a smoothed moving average.
    ///
    /// # Arguments
    ///
    /// * `prices` - Slice of prices
    ///
    /// # Panics
    ///
    /// `rsi` will panic if the length of prices isn't equal to 14
    ///
    /// # Examples
    ///
    /// ```rust
    /// let prices = vec![
    ///     5288.88, 5298.25, 5300.95, 5270.64, 5239.26, 5249.84, 5273.28, 5282.59,
    ///     5335.28, 5350.22, 5351.13, 5352.7, 5359.51, 5425.8
    /// ];
    /// let rsi = rust_ti::standard_indicators::single::rsi(&prices);
    /// assert_eq!(39.44166748365885, rsi);
    /// ```
    pub fn rsi(prices: &[f64]) -> f64 {
        if prices.len() != 14 {
            panic!("RSI must have a period of 14 not {}", prices.len())
        };
        return relative_strength_index(prices, &ConstantModelType::SmoothedMovingAverage);
    }
}

/// `bulk` module holds functions that return multiple values
pub mod bulk {
    use crate::standard_indicators::single;

    /// The `simple_moving_average` is essentially just the mean
    ///
    /// # Arguments
    ///
    /// * `prices` - A slice of prices
    /// * `period` - Period over which to calculate the moving average
    ///
    /// # Panics
    ///
    /// `simple_moving_average` will panic if:
    ///     * `prices` is empty
    ///     * `period` is greater than length of `prices`
    ///
    /// # Examples
    ///
    /// ```rust
    /// let prices = vec![100.0, 102.0, 103.0, 101.0, 100.0];
    /// let period: usize = 3;
    ///
    /// let simple_moving_average =
    /// rust_ti::standard_indicators::bulk::simple_moving_average(&prices, &period);
    /// assert_eq!(vec![101.66666666666667, 102.0, 101.33333333333333], simple_moving_average);
    /// ```
    pub fn simple_moving_average(prices: &[f64], period: &usize) -> Vec<f64> {
        let length = prices.len();
        if period > &length {
            panic!(
                "Period ({}) cannot be greater than length of prices ({})",
                period, length
            )
        };

        let mut mas = Vec::new();
        let loop_max = length - period + 1;
        for i in 0..loop_max {
            mas.push(single::simple_moving_average(&prices[i..i + period]));
        }
        return mas;
    }

    /// The `smoothed_moving_average` puts more weight on recent prices
    ///
    /// # Arguments
    ///
    /// * `prices` - A slice of prices
    /// * `period` - Period over which to calculate the moving average
    ///
    /// # Panics
    ///
    /// `smoothed_moving_average` will panic if:
    ///     * `prices` is empty
    ///     * `period` is greater than length of `prices`
    ///
    /// # Examples
    ///
    /// ```rust
    /// let prices = vec![100.0, 102.0, 103.0, 101.0, 100.0];
    /// let period: usize = 3;
    ///
    /// let smoothed_moving_average =
    /// rust_ti::standard_indicators::bulk::smoothed_moving_average(&prices, &period);
    /// assert_eq!(vec![102.05263157894737, 101.8421052631579, 100.94736842105264], smoothed_moving_average);
    /// ```
    pub fn smoothed_moving_average(prices: &[f64], period: &usize) -> Vec<f64> {
        let length = prices.len();
        if period > &length {
            panic!(
                "Period ({}) cannot be greater than length of prices ({})",
                period, length
            )
        };

        let mut mas = Vec::new();
        let loop_max = length - period + 1;
        for i in 0..loop_max {
            mas.push(single::smoothed_moving_average(&prices[i..i + period]));
        }
        return mas;
    }

    /// The `exponential_moving_average` puts exponentially more weight on recent prices
    ///
    /// # Arguments
    ///
    /// * `prices` - A slice of prices
    /// * `period` - Period over which to calculate the moving average
    ///
    /// # Panics
    ///
    /// `exponential_moving_average` will panic if:
    ///     * `prices` is empty
    ///     * `period` is greater than length of `prices`
    ///
    /// # Examples
    ///
    /// ```rust
    /// let prices = vec![100.0, 102.0, 103.0, 101.0, 100.0];
    /// let period: usize = 3;
    ///
    /// let exponential_moving_average =
    /// rust_ti::standard_indicators::bulk::exponential_moving_average(&prices, &period);
    /// assert_eq!(vec![102.28571428571429, 101.71428571428571, 100.71428571428571], exponential_moving_average);
    /// ```
    pub fn exponential_moving_average(prices: &[f64], period: &usize) -> Vec<f64> {
        let length = prices.len();
        if period > &length {
            panic!(
                "Period ({}) cannot be greater than length of prices ({})",
                period, length
            )
        };

        let mut mas = Vec::new();
        let loop_max = length - period + 1;
        for i in 0..loop_max {
            mas.push(single::exponential_moving_average(&prices[i..i + period]));
        }
        return mas;
    }

    /// `bollinger_bands` are an indicator created by John Bollinger in the 80s.
    ///
    /// The function returns a vector of tulpes with the lower band, moving average,
    /// and upper band in that order.
    ///
    /// The `bollinger_bands` function is a wrapper for the
    /// [`moving_constant_bands`](crate::candle_indicators::bulk::moving_constant_bands) using a
    /// simple moving average, standard deviation, and a deviation multiplier of 2.
    ///
    /// The standard period for Bollinger Bands is 20 periods, as such the function will panic if
    /// there are less than 20 periods.
    ///
    /// # Arguments
    ///
    /// * `prices` - Slice of prices
    ///
    /// # Panics
    ///
    /// `bollinger_bands` will panic if:
    ///     * `prices` is empty
    ///     * length of `prices` is less than 20
    ///
    /// # Example
    ///
    /// ```rust
    /// let prices = vec![
    ///     5224.0, 5223.28, 5238.34, 5294.39, 5306.26, 5297.43, 5311.95, 5314.53,
    ///     5305.4, 5288.88, 5298.25, 5300.94, 5270.64, 5239.26, 5249.84, 5273.27,
    ///     5282.59, 5335.27, 5350.22, 5351.13,5352.7, 5359.50
    /// ];
    /// let bbands = rust_ti::standard_indicators::bulk::bollinger_bands(&prices);
    /// assert_eq!(
    ///     vec![
    ///         (5213.581409805746, 5287.7935, 5362.005590194253),
    ///         (5220.94517276249, 5294.2285, 5367.51182723751),
    ///         (5230.115435120723, 5301.039500000001, 5371.963564879278)
    ///     ], bbands);
    /// ```
    pub fn bollinger_bands(prices: &[f64]) -> Vec<(f64, f64, f64)> {
        let length = prices.len();
        if length < 20 {
            panic!(
                "Prices must be at least 20 periods long not {} periods",
                length
            )
        };

        let mut bbands = Vec::new();
        let loop_max = length - 19;
        for i in 0..loop_max {
            bbands.push(single::bollinger_bands(&prices[i..i + 20]));
        }

        return bbands;
    }

    /// The `macd`, short for the Moving Average Convergence/Divergence,
    /// was created by Gerald Appel in the late 1970s when the working week was 6 days, and daily
    /// charts were primarily used to study stocks.
    ///
    /// The standard MACD uses a long period of 26, a short period of 12, and an exponential moving
    /// average model for both. The MACD will be returned first
    ///
    /// This function will return the signal line second. It uses a period of 9 and an exponential
    /// moving average.
    ///
    /// Finally the MACD histogram will be returned, which is simply the difference between MACD
    /// and signal line.
    ///
    /// # Arguments
    ///
    /// * `prices` - Slice of prices
    ///
    /// # Panics
    ///
    /// `macd` will panic if length of `prices` is less than 34.
    ///
    /// 26 periods for the MACD, and as the signal does the exponential moving average of the MACD
    /// over periods, these need to be added to the total length.
    ///
    /// # Examples
    ///
    /// ```rust
    /// let prices = vec![
    ///     5006.28, 5058.21, 5069.38, 5032.25, 5095.91, 5109.44, 5060.61, 5042.65, 5049.49,
    ///     5122.71, 5168.05, 5188.96, 5181.83, 5203.26, 5224.01, 5223.28, 5238.34, 5294.39,
    ///     5306.26, 5297.44, 5311.95, 5314.53, 5305.4, 5288.88, 5298.25, 5300.95, 5270.64,
    ///     5239.26, 5249.84, 5273.28, 5282.59, 5335.28, 5350.22, 5351.13, 5352.7, 5359.51,
    ///     5425.8
    /// ];
    ///
    /// let macd = rust_ti::standard_indicators::bulk::macd(&prices);
    /// assert_eq!(vec![
    ///         (23.732307403474806, 27.84886885937626, -4.116561455901454),
    ///         (23.231108818822577, 25.86755810362568, -2.6364492848031027),
    ///         (23.98848088685554, 24.707439348177488, -0.7189584613219466),
    ///         (30.420645672222236, 25.56096652679902, 4.859679145423215)],
    ///     macd);
    /// ```
    pub fn macd(prices: &[f64]) -> Vec<(f64, f64, f64)> {
        let length = prices.len();
        if length < 34 {
            panic!(
                "Prices must be at least 35 periods long, not {} periods",
                length
            )
        };
        let mut macds = Vec::new();
        let loop_max = length - 33;
        // TODO: This shouldn't repeat calcs that are already being done, the function should
        // return the enough so that it can be passed to other functions rather than recalc the
        // macds to pass to signal function
        for i in 0..loop_max {
            macds.push(single::macd(&prices[i..i + 34]));
        }
        return macds;
    }

    /// The `rsi` or Relative Strength Index, is a momentum indicator that measures velocity and
    /// magnitude of price movements.
    ///
    /// The standard period used is 14 days, and the model is a smoothed moving average.
    ///
    /// # Arguments
    ///
    /// * `prices` - Slice of prices
    ///
    /// # Panics
    ///
    /// `rsi` will panic if the length of prices is less than 14
    ///
    /// # Examples
    ///
    /// ```rust
    /// let prices = vec![
    ///     5311.95, 5314.53, 5305.4, 5288.88, 5298.25, 5300.95, 5270.64, 5239.26, 5249.84,
    ///     5273.28, 5282.59, 5335.28, 5350.22, 5351.13, 5352.7, 5359.51, 5425.8
    /// ];
    /// let rsi = rust_ti::standard_indicators::bulk::rsi(&prices);
    /// assert_eq!(vec![38.168621439659084, 35.624517227910545, 31.14286676169411, 39.44166748365885], rsi);
    /// ```
    pub fn rsi(prices: &[f64]) -> Vec<f64> {
        let length = prices.len();
        if length < 14 {
            panic!("RSI must have a period of at least 14 not {}", length)
        };
        let mut rsis = Vec::new();
        let loop_max = length - 13;
        for i in 0..loop_max {
            rsis.push(single::rsi(&prices[i..i + 14]));
        }
        return rsis;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_simple_moving_average() {
        let prices = vec![100.2, 100.46, 100.53, 100.38, 100.19];
        assert_eq!(100.352, single::simple_moving_average(&prices));
    }

    #[test]
    #[should_panic]
    fn single_simple_moving_average_panic() {
        let prices = Vec::new();
        single::simple_moving_average(&prices);
    }

    #[test]
    fn bulk_simple_moving_average() {
        let prices = vec![100.2, 100.46, 100.53, 100.38, 100.19];
        let period: usize = 4;
        assert_eq!(
            vec![100.3925, 100.39],
            bulk::simple_moving_average(&prices, &period)
        );
    }

    #[test]
    #[should_panic]
    fn bulk_simple_moving_average_panic() {
        let prices = vec![100.2, 100.46, 100.53, 100.38, 100.19];
        let period: usize = 40;
        bulk::simple_moving_average(&prices, &period);
    }

    #[test]
    fn single_smoothed_moving_average() {
        let prices = vec![100.2, 100.46, 100.53, 100.38, 100.19];
        assert_eq!(100.34228938600666, single::smoothed_moving_average(&prices));
    }

    #[test]
    #[should_panic]
    fn single_smoothed_moving_average_panic() {
        let prices = Vec::new();
        single::smoothed_moving_average(&prices);
    }

    #[test]
    fn bulk_smoothed_moving_average() {
        let prices = vec![100.2, 100.46, 100.53, 100.38, 100.19];
        let period: usize = 4;
        assert_eq!(
            vec![100.40982857142858, 100.35371428571428],
            bulk::smoothed_moving_average(&prices, &period)
        );
    }

    #[test]
    #[should_panic]
    fn bulk_smoothed_moving_average_panic() {
        let prices = vec![100.2, 100.46, 100.53, 100.38, 100.19];
        let period: usize = 40;
        bulk::smoothed_moving_average(&prices, &period);
    }

    #[test]
    fn single_exponential_moving_average() {
        let prices = vec![100.2, 100.46, 100.53, 100.38, 100.19];
        assert_eq!(
            100.32810426540287,
            single::exponential_moving_average(&prices)
        );
    }

    #[test]
    #[should_panic]
    fn single_exponential_moving_average_panic() {
        let prices = Vec::new();
        single::exponential_moving_average(&prices);
    }

    #[test]
    fn bulk_exponential_moving_average() {
        let prices = vec![100.2, 100.46, 100.53, 100.38, 100.19];
        let period: usize = 4;
        assert_eq!(
            vec![100.41672794117645, 100.32544117647058],
            bulk::exponential_moving_average(&prices, &period)
        );
    }

    #[test]
    #[should_panic]
    fn bulk_exponential_moving_average_panic() {
        let prices = vec![100.2, 100.46, 100.53, 100.38, 100.19];
        let period: usize = 40;
        bulk::exponential_moving_average(&prices, &period);
    }

    #[test]
    fn single_bollinger_bands() {
        let prices = vec![
            99.39, 99.59, 99.68, 99.98, 99.06, 98.39, 99.23, 98.66, 98.88, 98.31, 98.16, 97.87,
            98.74, 99.47, 98.86, 99.73, 100.06, 100.66, 99.69, 100.63,
        ];
        assert_eq!(
            (97.73388801467088, 99.25200000000002, 100.77011198532917),
            single::bollinger_bands(&prices)
        );
    }

    #[test]
    #[should_panic]
    fn single_bollinger_band_panic_empty_prices() {
        let prices = Vec::new();
        single::bollinger_bands(&prices);
    }

    #[test]
    #[should_panic]
    fn single_bollinger_band_panic_wrong_period() {
        let prices = vec![
            99.39, 99.59, 99.68, 99.98, 99.06, 98.39, 99.23, 98.66, 98.88, 98.31,
        ];
        single::bollinger_bands(&prices);
    }

    #[test]
    fn bulk_bollinger_bands() {
        let prices = vec![
            99.39, 99.59, 99.68, 99.98, 99.06, 98.39, 99.23, 98.66, 98.88, 98.31, 98.16, 97.87,
            98.74, 99.47, 98.86, 99.73, 100.06, 100.66, 99.69, 100.63, 99.75, 99.55, 98.8,
        ];
        assert_eq!(
            vec![
                (97.73388801467088, 99.25200000000002, 100.77011198532917),
                (97.7373030306026, 99.27000000000001, 100.80269696939742),
                (97.73687492346315, 99.268, 100.79912507653685),
                (97.69218538980725, 99.224, 100.75581461019276)
            ],
            bulk::bollinger_bands(&prices)
        );
    }

    #[test]
    #[should_panic]
    fn bulk_bollinger_band_panic_wrong_period() {
        let prices = vec![
            99.39, 99.59, 99.68, 99.98, 99.06, 98.39, 99.23, 98.66, 98.88, 98.31,
        ];
        bulk::bollinger_bands(&prices);
    }

    #[test]
    fn single_macd() {
        let prices = vec![
            99.39, 99.59, 99.68, 99.98, 99.06, 98.39, 99.23, 98.66, 98.88, 98.31, 98.16, 97.87,
            98.74, 99.47, 98.86, 99.73, 100.06, 100.66, 99.69, 100.63, 99.75, 99.55, 98.8, 98.97,
            98.83, 98.15, 97.42, 96.94, 96.51, 96.71, 96.5, 97.22, 98.03, 98.21,
        ];
        assert_eq!(
            (
                -0.6285719796983358,
                -0.6158898367280627,
                -0.012682142970273036
            ),
            single::macd(&prices)
        );
    }

    #[test]
    #[should_panic]
    fn single_macd_panic() {
        let prices = vec![
            99.39, 99.59, 99.68, 99.98, 99.06, 98.39, 99.23, 98.66, 98.88, 98.31, 98.16, 97.87,
            98.74, 99.47, 98.86, 99.73, 100.06, 100.66, 99.69, 100.63, 99.75, 99.55, 98.8,
        ];
        single::macd(&prices);
    }

    #[test]
    fn bulk_macd() {
        let prices = vec![
            99.39, 99.59, 99.68, 99.98, 99.06, 98.39, 99.23, 98.66, 98.88, 98.31, 98.16, 97.87,
            98.74, 99.47, 98.86, 99.73, 100.06, 100.66, 99.69, 100.63, 99.75, 99.55, 98.8, 98.97,
            98.83, 98.15, 97.42, 96.94, 96.51, 96.71, 96.5, 97.22, 98.03, 98.21, 98.05, 98.24,
        ];
        assert_eq!(
            vec![
                (
                    -0.6285719796983358,
                    -0.6158898367280627,
                    -0.012682142970273036
                ),
                (-0.54985540794776, -0.61936157195289, 0.06950616400512999),
                (
                    -0.4749341506892648,
                    -0.6001186622390476,
                    0.12518451154978283
                )
            ],
            bulk::macd(&prices)
        );
    }

    #[test]
    #[should_panic]
    fn bulk_macd_panic() {
        let prices = vec![
            99.39, 99.59, 99.68, 99.98, 99.06, 98.39, 99.23, 98.66, 98.88, 98.31, 98.16, 97.87,
            98.74, 99.47, 98.86, 99.73, 100.06, 100.66, 99.69, 100.63, 99.75, 99.55, 98.8,
        ];
        bulk::macd(&prices);
    }

    #[test]
    fn single_rsi() {
        let prices = vec![
            99.75, 99.55, 98.8, 98.97, 98.83, 98.15, 97.42, 96.94, 96.51, 96.71, 96.5, 97.22,
            98.03, 98.21,
        ];
        assert_eq!(49.49693728728258, single::rsi(&prices));
    }

    #[test]
    #[should_panic]
    fn single_rsi_panic() {
        let prices = vec![
            99.75, 99.55, 98.8, 98.97, 98.83, 98.15, 97.42, 96.94, 96.51, 96.71, 96.5,
        ];
        single::rsi(&prices);
    }

    #[test]
    fn bulk_rsi() {
        let prices = vec![
            99.75, 99.55, 98.8, 98.97, 98.83, 98.15, 97.42, 96.94, 96.51, 96.71, 96.5, 97.22,
            98.03, 98.21, 98.05, 98.24,
        ];

        assert_eq!(
            vec![49.49693728728258, 51.7387808206744, 49.93948387240627],
            bulk::rsi(&prices)
        );
    }

    #[test]
    #[should_panic]
    fn bulk_rsi_panic() {
        let prices = vec![
            99.75, 99.55, 98.8, 98.97, 98.83, 98.15, 97.42, 96.94, 96.51, 96.71, 96.5,
        ];
        bulk::rsi(&prices);
    }
}
