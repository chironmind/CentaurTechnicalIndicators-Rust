//! # Basic Indicators
//!
//! `basic_indicators` is a module of simple functions to perform simple calculations on prices, such as mean, median, log...

/// `single` module holds functions that return a singular value for `basic_indicators`
pub mod single {
    use std::cmp::Ordering;
    use std::collections::HashMap;
    /// Calculates the mean (average) for a slice of prices and returns it as an `f64`
    ///
    /// # Arguments
    ///
    /// * `prices` - A `f64` slice of prices
    ///
    /// # Panics
    ///
    /// The fuction will panic if given an empty `slice`
    ///
    /// # Examples
    ///
    /// ```
    /// let prices = vec![100.0, 102.0, 103.0, 101.0];
    /// let mean = rust_ti::basic_indicators::single::mean(&prices);
    /// assert_eq!(101.5, mean);
    /// ```
    pub fn mean(prices: &[f64]) -> f64 {
        if prices.is_empty() {
            panic!("Prices ({:?}) is empty", prices);
        };
        let sum: f64 = prices.iter().sum();
        return sum / prices.len() as f64;
    }

    /// Calculates the median (middle value) for a slice of prices and returns it as an `f64`.
    ///
    /// `median` orders the numbers and takes the middle value. If the number of prices is even it will take the average of the two middle values.
    ///
    /// # Argument
    ///
    /// * `prices` - A `f64` slice of prices
    ///
    /// # Panics
    ///
    /// The fuction will panic if given an empty `slice`
    ///
    /// # Examples
    ///
    /// ```
    /// // Odd number of prices
    /// let prices = vec![100.0, 102.0, 103.0, 101.0, 100.0];
    /// let median = rust_ti::basic_indicators::single::median(&prices);
    /// assert_eq!(101.0, median);
    ///
    /// // Even number of prices
    /// let prices = vec![100.0, 102.0, 103.0, 101.0];
    /// let median = rust_ti::basic_indicators::single::median(&prices);
    /// assert_eq!(101.5, median);
    /// ```
    pub fn median(prices: &[f64]) -> f64 {
        if prices.is_empty() {
            panic!("Prices ({:?}) is empty", prices);
        };

        let mut ordered_prices = prices
            .iter()
            .filter_map(|f| if f.is_nan() { None } else { Some(*f) })
            .collect::<Vec<f64>>();
        ordered_prices.sort_by(cmp_f64);
        let middle: usize = prices.len() / 2;
        if prices.len() % 2 == 0 {
            return mean(&ordered_prices[middle - 1..middle + 1]);
        };

        return ordered_prices[middle];
    }

    /// Calculates the mode (most common price) for a slice of prices.
    ///
    /// `mode` will round the numbers to get most frequently occuring integer.
    ///
    /// If it finds multiple prices that occur an equal number of times it will the average of those
    /// numbers.
    ///
    /// # Arguments
    ///
    /// * `prices` - A `f64` slice of prices
    ///
    /// # Panics
    ///
    /// The fuction will panic if given an empty `slice`
    ///
    /// # Examples
    ///
    /// ```
    /// let prices = vec![100.0, 102.0, 101.0, 101.0, 100.0];
    /// let mode = rust_ti::basic_indicators::single::mode(&prices);
    /// assert_eq!(100.5, mode);
    ///
    /// let prices = vec![100.0, 102.0, 103.0, 101.0, 100.0];
    /// let mode = rust_ti::basic_indicators::single::mode(&prices);
    /// assert_eq!(100.0, mode);
    /// ```
    pub fn mode(prices: &[f64]) -> f64 {
        if prices.is_empty() {
            panic!("Prices ({:?}) is empty", prices);
        };

        let rounded_prices = prices.iter().map(|x| x.round() as i64).collect();
        return most_frequent(rounded_prices);
    }

    /// Calculates the difference between the natural logarithm at t and t-1
    ///
    /// # Arguments
    ///
    /// * `price_t` - `&f64` price at t
    /// * `price_t_1` - `&f64` price at t-1
    ///
    /// # Panics
    ///
    /// If one of the prices less or equal to 0.0
    ///
    /// # Examples
    ///
    /// ```
    /// let prices = vec![100.0, 102.0, 103.0, 101.0];
    /// let log_difference = rust_ti::basic_indicators::single::log_difference(&prices[3], &prices[2]);
    /// assert_eq!(-0.01960847138837618, log_difference);
    /// ```
    pub fn log_difference(price_t: &f64, price_t_1: &f64) -> f64 {
        if price_t <= &0.0 || price_t_1 <= &0.0 {
            panic!(
                "price_t ({}) and price_t_1 ({}) need to be greater than 0.0",
                price_t, price_t_1
            );
        }
        return price_t.ln() - price_t_1.ln();
    }

    /// Calculates the variance of slice of prices
    ///
    /// Assumes a normal distribution
    ///
    /// # Arguments
    ///
    /// * `prices` - `f64` slice of prices
    ///
    /// # Panics
    ///
    /// The function will panic if prices is empty
    ///
    /// # Examples
    ///
    /// ```
    /// let prices = vec![100.0, 102.0, 103.0, 101.0];
    /// let variance = rust_ti::basic_indicators::single::variance(&prices);
    /// assert_eq!(1.25, variance);
    /// ```
    // TODO: Allow for distributions other than normal distributions
    pub fn variance(prices: &[f64]) -> f64 {
        if prices.is_empty() {
            panic!("Prices ({:?}) is empty", prices);
        }
        let prices_mean = mean(prices);
        let mean_diff_sq: Vec<f64> = prices.iter().map(|x| (x - prices_mean).powi(2)).collect();
        return mean(&mean_diff_sq);
    }

    /// Calculates the standard deviation of slice of prices
    ///
    /// Assumes a normal distribution
    ///
    /// # Arguments
    ///
    /// * `prices` - `f64` slice of prices
    ///
    /// # Panics
    ///
    /// The function will panic if prices is empty
    ///
    /// # Examples
    ///
    /// ```
    /// let prices = vec![100.0, 102.0, 103.0, 101.0];
    /// let standard_deviation = rust_ti::basic_indicators::single::standard_deviation(&prices);
    /// assert_eq!(1.118033988749895, standard_deviation);
    /// ```
    // TODO: Allow for distributions other than normal distributions
    pub fn standard_deviation(prices: &[f64]) -> f64 {
        let variance = variance(prices);
        return variance.sqrt();
    }

    /// Calculates the absolute deviation from the mean, median, or mode.
    ///
    /// # Arguments
    ///
    /// * `prices` - A `f64` slice of prices
    /// * `central_point` - A variant of the `CentralPoint enum`
    ///
    /// # Panics
    ///
    /// The function will panic if prices is empty
    ///
    /// # Examples
    ///
    /// ```
    /// let prices = vec![100.0, 102.0, 103.0, 101.0, 100.0];
    /// let mean_absolute_deviation = rust_ti::basic_indicators::single::absolute_deviation(&prices, &rust_ti::CentralPoint::Mean);
    /// // The answer is `1.04` but due to how Rust implements `f64` `1.0400000000000005` gets
    /// // returned which should be negligible.
    /// assert_eq!(1.0400000000000005, mean_absolute_deviation);
    ///
    /// let median_absolute_deviation = rust_ti::basic_indicators::single::absolute_deviation(&prices, &rust_ti::CentralPoint::Median);
    /// assert_eq!(1.0, median_absolute_deviation);
    ///
    /// let mode_absolute_deviation = rust_ti::basic_indicators::single::absolute_deviation(&prices, &rust_ti::CentralPoint::Mode);
    /// assert_eq!(1.2, mode_absolute_deviation);
    /// ```
    pub fn absolute_deviation(prices: &[f64], central_point: &crate::CentralPoint) -> f64 {
        if prices.is_empty() {
            panic!("Prices is empty")
        };
        let mid_point = match central_point {
            crate::CentralPoint::Mean => mean(prices),
            crate::CentralPoint::Median => median(prices),
            crate::CentralPoint::Mode => mode(prices),
            _ => panic!("Unsupported central_point provided"), // TODO: add debug to Central point
                                                               // so the panic can provide it
        };
        let deviation: f64 = prices.iter().map(|x| (x - mid_point).abs()).sum();
        return deviation / prices.len() as f64;
    }

    /// Calculates the maximum for a slice of prices
    ///
    /// Max doesn't currently exist in Rust for `f64`
    ///
    /// # Arguments
    ///
    /// * `prices` - slice of prices
    ///
    /// # Examples
    ///
    /// ```
    /// let prices = vec![100.0, 102.0, 103.0, 101.0, 100.0];
    /// let max = rust_ti::basic_indicators::single::max(&prices);
    /// assert_eq!(103.0, max);
    /// ```
    pub fn max(prices: &[f64]) -> f64 {
        if prices.is_empty() {
            panic!("Prices is empty")
        };
        let mut ordered_prices = prices
            .iter()
            .filter_map(|f| if f.is_nan() { None } else { Some(*f) })
            .collect::<Vec<f64>>();
        ordered_prices.sort_by(cmp_f64);
        return ordered_prices[ordered_prices.len() - 1];
    }

    /// Calculates the minimum for a slice of prices
    ///
    /// Min doesn't currently exist in Rust for `f64`
    ///
    /// # Arguments
    ///
    /// * `prices` - slice of prices
    ///
    /// # Examples
    ///
    /// ```
    /// let prices = vec![100.0, 102.0, 103.0, 101.0, 100.0];
    /// let min = rust_ti::basic_indicators::single::min(&prices);
    /// assert_eq!(100.0, min);
    /// ```
    pub fn min(prices: &[f64]) -> f64 {
        if prices.is_empty() {
            panic!("Prices is empty")
        };
        let mut ordered_prices = prices
            .iter()
            .filter_map(|f| if f.is_nan() { None } else { Some(*f) })
            .collect::<Vec<f64>>();
        ordered_prices.sort_by(cmp_f64);
        return ordered_prices[0];
    }

    fn cmp_f64(a: &f64, b: &f64) -> Ordering {
        if a < b {
            return Ordering::Less;
        } else if a > b {
            return Ordering::Greater;
        }
        return Ordering::Equal;
    }

    // TODO: Surely this can be improved
    //     sorting could eventually be done by f64 sort_floats method once it is no longer
    //     experimental
    fn most_frequent(vector: Vec<i64>) -> f64 {
        let mut map: HashMap<i64, usize> = HashMap::new();
        for x in vector {
            *map.entry(x as i64).or_default() += 1;
        }
        let mut max_price = vec![0];
        let mut max_count = usize::MIN;
        for (key, value) in map {
            if value > max_count {
                max_count = value;
                max_price = vec![key];
            } else if value == max_count {
                max_price.push(key);
            } else {
                continue;
            }
        }
        if max_price.len() > 1 {
            return max_price.iter().sum::<i64>() as f64 / max_price.len() as f64;
        }
        return max_price[0] as f64;
    }
}

/// `bulk` module holds functions that return multiple values for `basic_indicators`
pub mod bulk {
    use crate::basic_indicators::single;
    /// Calculates the mean (averages) for a slice of prices for a provided period and returns
    /// them as a `vector` of `f64`
    ///
    /// # Arguments
    ///
    /// * `prices` - A `f64` slice of prices
    /// * `period` - A `usize` period over which to calculate the mean
    ///
    /// # Panics
    ///
    /// The fuction will panic if `period` is greater than length of `prices`
    ///
    /// # Examples
    ///
    /// ```
    /// let prices = vec![101.0, 102.0, 103.0, 101.0];
    /// let period: usize = 3;
    /// let mean = rust_ti::basic_indicators::bulk::mean(&prices, &period);
    /// assert_eq!(vec![102.0, 102.0], mean);
    /// ```
    pub fn mean(prices: &[f64], period: &usize) -> Vec<f64> {
        let length = prices.len();

        if period > &length {
            panic!(
                "Period ({}) cannot be longer than the length of provided prices ({})",
                period, length
            );
        };

        let mut means = Vec::new();
        for i in 0..length {
            let end_index = period + i;
            if end_index > length {
                break;
            }
            means.push(single::mean(&prices[i..end_index]));
        }
        return means;
    }

    /// Calculates the median (middle value) for a slice of prices and returns it as an f64.
    ///
    /// `median` orders the numbers and takes the middle value. If the number of prices is even it will take the average of the two middle values.
    ///
    /// # Arguments
    ///
    /// * `prices` - A `f64` slice of prices
    /// * `period` - A `usize` period over which to calculate the median
    ///
    /// # Panics
    ///
    /// The fuction will panic if `period` is greater than length of `prices`
    ///
    /// # Examples
    ///
    /// ```
    /// let prices = vec![101.0, 102.0, 103.0, 101.0];
    /// let period: usize = 3;
    /// let median = rust_ti::basic_indicators::bulk::median(&prices, &period);
    /// assert_eq!(vec![102.0, 102.0], median);
    /// ```
    pub fn median(prices: &[f64], period: &usize) -> Vec<f64> {
        let length = prices.len();

        if period > &length {
            panic!(
                "Period ({}) cannot be longer than the length of provided prices ({})",
                period, length
            );
        };

        let mut medians = Vec::new();
        for i in 0..length {
            let end_index = period + i;
            if end_index > length {
                break;
            }
            medians.push(single::median(&prices[i..end_index]));
        }
        return medians;
    }

    /// Calculates the mode (most common price) for a slice of prices.
    ///
    /// `mode` will round the numbers to get most frequently occuring integer.
    ///
    /// If it finds multiple prices that occur an equal number of times it will the average of those
    /// numbers.
    ///
    /// # Arguments
    ///
    /// * `prices` - A `f64` slice of prices
    /// * `period` - A `usize` period over which to calculate the mode
    ///
    /// # Panics
    ///
    /// The fuction will panic if `period` is greater than length of `prices`
    ///
    /// # Examples
    ///
    /// ```
    /// let prices = vec![101.0, 102.0, 101.0, 102.0];
    /// let period: usize = 3;
    /// let mode = rust_ti::basic_indicators::bulk::mode(&prices, &period);
    /// assert_eq!(vec![101.0, 102.0], mode);
    /// ```
    pub fn mode(prices: &[f64], period: &usize) -> Vec<f64> {
        let length = prices.len();

        if period > &length {
            panic!(
                "Period ({}) cannot be longer than the length of provided prices ({})",
                period, length
            );
        };

        let mut modes = Vec::new();
        for i in 0..length {
            let end_index = period + i;
            if end_index > length {
                break;
            }
            modes.push(single::mode(&prices[i..end_index]));
        }
        return modes;
    }

    /// Calculates the natural logrithm for slice of prices
    ///
    /// `log` is essentially just a wrapper for the `f64` `ln` method that iterates over a passed
    /// in slice and returns the natural logarithm for those prices
    ///
    /// # Arguments
    ///
    /// * `prices` - A `f64` slice of prices
    ///
    /// # Panics
    ///
    /// The function will panic if passed in an empty slice
    ///
    /// # Examples
    ///
    /// ```
    /// let prices = vec![101.0, 102.0, 103.0, 101.0];
    /// let log = rust_ti::basic_indicators::bulk::log(&prices);
    /// assert_eq!(vec![4.61512051684126, 4.624972813284271, 4.634728988229636, 4.61512051684126], log);
    /// ```
    pub fn log(prices: &[f64]) -> Vec<f64> {
        if prices.len() < 1 {
            panic!("Prices ({:?}) is empty", prices);
        }

        let mut logs = Vec::new();
        for price in prices {
            logs.push(price.ln());
        }
        return logs;
    }

    /// Calculates the difference between the natural logarithm at t and t-1
    ///
    /// # Arguments
    ///
    /// * `prices` - A `f64` slice of prices
    ///
    /// # Panics
    ///
    /// The function will panic if passed an empty slice
    ///
    /// # Examples
    ///
    /// ```
    /// let prices = vec![100.0, 102.0, 103.0, 101.0];
    /// let log_difference = rust_ti::basic_indicators::bulk::log_difference(&prices);
    /// assert_eq!(vec![0.019802627296178876, 0.009756174945365181, -0.01960847138837618], log_difference);
    /// ```
    pub fn log_difference(prices: &[f64]) -> Vec<f64> {
        let length = prices.len();
        if length < 1 {
            panic!("Prices ({:?}) is empty", prices);
        }

        let mut log_diffs = Vec::new();
        for i in 0..length {
            let end_index = i + 1;
            if end_index >= length {
                break;
            }
            log_diffs.push(single::log_difference(&prices[end_index], &prices[i]));
        }
        return log_diffs;
    }

    /// Calculates the variance of slice of prices
    ///
    /// Assumes a normal distribution
    ///
    /// # Arguments
    ///
    /// * `prices` - `f64` slice of prices
    /// * `period` - `usize` period over which to calculate the variance
    ///
    /// # Panics
    ///
    /// The function will panic if the period is greater than the length of prices
    ///
    /// # Examples
    ///
    /// ```
    /// let prices = vec![100.0, 102.0, 103.0, 101.0];
    /// let period: usize = 3;
    /// let variance = rust_ti::basic_indicators::bulk::variance(&prices, &period);
    /// assert_eq!(vec![1.5555555555555556, 0.6666666666666666], variance);
    /// ```
    // TODO: Allow for distributions other than normal distributions
    pub fn variance(prices: &[f64], period: &usize) -> Vec<f64> {
        let length = prices.len();
        if period > &length {
            panic!(
                "Period ({}) cannot be longer than the length of provided prices ({})",
                period, length
            );
        };
        let mut variances = Vec::new();
        for i in 0..length {
            let end_index = period + i;
            if end_index > length {
                break;
            }
            variances.push(single::variance(&prices[i..end_index]));
        }
        return variances;
    }

    /// Calculates the standard deviation of a slice of prices
    ///
    /// Assumes a normal distribution
    ///
    /// # Arguments
    ///
    /// * `prices` - `f64` slice of prices
    /// * `period` - `usize` period over which to calculate the standard deviation
    ///
    /// # Panics
    ///
    /// The function will panic if prices is empty
    ///
    /// # Examples
    ///
    /// ```
    /// let prices = vec![100.0, 102.0, 103.0, 101.0];
    /// let period: usize = 3;
    /// let standard_deviation = rust_ti::basic_indicators::bulk::standard_deviation(&prices, &period);
    /// assert_eq!(vec![1.247219128924647, 0.816496580927726], standard_deviation);
    /// ```
    // TODO: Allow for distributions other than normal distributions
    pub fn standard_deviation(prices: &[f64], period: &usize) -> Vec<f64> {
        let length = prices.len();
        if period > &length {
            panic!(
                "Period ({}) cannot be longer than the length of provided prices ({})",
                period, length
            );
        };
        let mut stddevs = Vec::new();
        for i in 0..length {
            let end_index = period + i;
            if end_index > length {
                break;
            }
            stddevs.push(single::standard_deviation(&prices[i..end_index]));
        }
        return stddevs;
    }

    /// Calculates the absolute deviation from the mean, median, or mode over a provided period.
    ///
    /// # Arguments
    ///
    /// * `prices` - A `f64` slice of prices
    /// * `period` - `usize` period over which to calculate the standard deviation
    /// * `central_point` - A variant of the `CentralPoint enum`
    ///
    /// # Panics
    ///
    /// The function will panic if the period is longer than the length of prices
    ///
    /// # Examples
    ///
    /// ```
    /// let prices = vec![100.0, 102.0, 103.0, 101.0, 100.0];
    /// let period: usize = 3;
    ///
    /// let mean_absolute_deviation = rust_ti::basic_indicators::bulk::absolute_deviation(&prices, &period, &rust_ti::CentralPoint::Mean);
    /// assert_eq!(vec![1.1111111111111096, 0.6666666666666666, 1.1111111111111096], mean_absolute_deviation);
    ///
    /// let median_absolute_deviation = rust_ti::basic_indicators::bulk::absolute_deviation(&prices, &period, &rust_ti::CentralPoint::Median);
    /// assert_eq!(vec![1.0, 0.6666666666666666, 1.0], median_absolute_deviation);
    ///
    /// let mode_absolute_deviation = rust_ti::basic_indicators::bulk::absolute_deviation(&prices, &period, &rust_ti::CentralPoint::Mode);
    /// assert_eq!(vec![1.1111111111111096, 0.6666666666666666, 1.1111111111111096], mode_absolute_deviation);
    /// ```
    pub fn absolute_deviation(
        prices: &[f64],
        period: &usize,
        central_point: &crate::CentralPoint,
    ) -> Vec<f64> {
        let length = prices.len();
        if period > &length {
            panic!(
                "Period ({}) cannot be longer than the length of provided prices ({})",
                period, length
            );
        };
        let mut absolute_deviations = Vec::new();
        for i in 0..length {
            let end_index = period + i;
            if end_index > length {
                break;
            }
            absolute_deviations.push(single::absolute_deviation(
                &prices[i..end_index],
                central_point,
            ));
        }
        return absolute_deviations;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_mean() {
        let prices = vec![100.2, 100.46, 100.53, 100.38, 100.19];
        assert_eq!(100.352, single::mean(&prices));
    }

    #[test]
    fn single_mean_identical_prices() {
        let prices = vec![100.0, 100.0, 100.0];
        assert_eq!(100.0, single::mean(&prices));
    }

    #[test]
    #[should_panic]
    fn single_mean_empty_prices() {
        let prices = Vec::new();
        single::mean(&prices);
    }

    #[test]
    fn bulk_mean() {
        let prices = vec![100.2, 100.46, 100.53, 100.38, 100.19];
        let period: usize = 3;
        assert_eq!(
            vec![100.39666666666666, 100.45666666666666, 100.36666666666667],
            bulk::mean(&prices, &period)
        );
    }

    #[test]
    #[should_panic]
    fn bulk_mean_panic() {
        let prices = vec![100.2, 100.46, 100.53, 100.38, 100.19];
        let period: usize = 30;
        bulk::mean(&prices, &period);
    }

    #[test]
    fn single_median_odd() {
        let prices = vec![100.2, 100.46, 100.53, 100.38, 100.19];
        assert_eq!(100.38, single::median(&prices));
    }

    #[test]
    fn single_median_even() {
        let prices = vec![100.2, 100.46, 100.53, 100.38];
        // Should be
        // assert_eq!(100.42, single::median(&prices));
        // but due to how floating points are calculated we have to assert on
        assert_eq!(100.41999999999999, single::median(&prices));
    }

    #[test]
    #[should_panic]
    fn single_median_panic() {
        let prices = Vec::new();
        single::median(&prices);
    }

    #[test]
    fn bulk_median() {
        let prices = vec![100.2, 100.46, 100.53, 100.38, 100.19];
        let period: usize = 3;
        assert_eq!(vec![100.46, 100.46, 100.38], bulk::median(&prices, &period));
    }

    #[test]
    #[should_panic]
    fn bulk_median_panic() {
        let prices = vec![100.2, 100.46, 100.53, 100.38, 100.19];
        let period: usize = 30;
        bulk::median(&prices, &period);
    }

    #[test]
    fn single_mode_round_up() {
        let prices = vec![100.2, 100.46, 100.53, 101.08, 101.19];
        assert_eq!(101.0, single::mode(&prices));
    }

    #[test]
    fn single_mode_round_down() {
        let prices = vec![100.2, 100.46, 100.35, 101.08, 101.19];
        assert_eq!(100.0, single::mode(&prices));
    }

    #[test]
    fn single_mode_average() {
        let prices = vec![100.46, 100.35, 101.08, 101.19];
        assert_eq!(100.5, single::mode(&prices));
    }

    #[test]
    #[should_panic]
    fn single_mode_panic() {
        let prices = Vec::new();
        single::mode(&prices);
    }

    #[test]
    fn bulk_mode() {
        let prices = vec![100.2, 100.46, 100.53, 101.08, 101.19];
        let period: usize = 3;
        assert_eq!(vec![100.0, 101.0, 101.0], bulk::mode(&prices, &period));
    }

    #[test]
    #[should_panic]
    fn bulk_mode_panic() {
        let prices = vec![100.2, 100.46, 100.53, 101.08, 101.19];
        let period: usize = 30;
        bulk::mode(&prices, &period);
    }

    #[test]
    fn bulk_log() {
        let prices = vec![100.2, 100.46, 100.53, 100.38, 100.19];
        assert_eq!(
            vec![
                4.607168188650764,
                4.609759638321899,
                4.610456190417329,
                4.608962984226787,
                4.607068383271171
            ],
            bulk::log(&prices)
        );
    }

    #[test]
    #[should_panic]
    fn bulk_log_panic() {
        let prices = Vec::new();
        bulk::log(&prices);
    }

    #[test]
    fn single_log_difference() {
        assert_eq!(
            -0.0018946009556159993,
            single::log_difference(&100.19, &100.38)
        );
    }

    #[test]
    #[should_panic]
    fn single_log_difference_panic() {
        single::log_difference(&0.0, &100.38);
    }

    #[test]
    #[should_panic]
    fn single_log_difference_panic_2() {
        single::log_difference(&100.19, &-100.38);
    }

    #[test]
    fn bulk_log_difference() {
        let prices = vec![100.2, 100.46, 100.53, 100.38, 100.19];
        assert_eq!(
            vec![
                0.0025914496711347823,
                0.0006965520954302917,
                -0.0014932061905419403,
                -0.0018946009556159993
            ],
            bulk::log_difference(&prices)
        );
    }

    #[test]
    #[should_panic]
    fn bulk_log_difference_difference() {
        bulk::log_difference(&Vec::new());
    }

    #[test]
    fn single_variance() {
        let prices = vec![100.2, 100.46, 100.53, 100.38, 100.19];
        assert_eq!(0.018695999999999734, single::variance(&prices));
    }

    #[test]
    #[should_panic]
    fn single_variance_panic() {
        let prices = Vec::new();
        single::variance(&prices);
    }

    #[test]
    fn bulk_variance() {
        let prices = vec![100.2, 100.46, 100.53, 100.38, 100.19];
        let period = 3;
        assert_eq!(
            vec![
                0.02015555555555502,
                0.0037555555555558295,
                0.019355555555555907
            ],
            bulk::variance(&prices, &period)
        );
    }

    #[test]
    #[should_panic]
    fn bulk_variance_panic() {
        let prices = vec![100.2, 100.46, 100.53, 100.38, 100.19];
        let period = 30;
        bulk::variance(&prices, &period);
    }

    #[test]
    fn single_standard_deviation() {
        let prices = vec![100.2, 100.46, 100.53, 100.38, 100.19];
        assert_eq!(0.1367333170810967, single::standard_deviation(&prices));
    }

    #[test]
    #[should_panic]
    fn single_standard_deviation_panic() {
        let prices = Vec::new();
        single::standard_deviation(&prices);
    }

    #[test]
    fn bulk_standard_deviation() {
        let prices = vec![100.2, 100.46, 100.53, 100.38, 100.19];
        let period = 3;
        assert_eq!(
            vec![
                0.14197026292697715,
                0.06128258770283635,
                0.13912424503139598
            ],
            bulk::standard_deviation(&prices, &period)
        );
    }

    #[test]
    #[should_panic]
    fn bulk_standard_deviation_panic() {
        let prices = vec![100.2, 100.46, 100.53, 100.38, 100.19];
        let period = 30;
        bulk::standard_deviation(&prices, &period);
    }

    #[test]
    fn single_absolute_deviation() {
        let prices = vec![100.2, 100.46, 100.53, 100.38, 100.19];
        assert_eq!(
            0.12559999999999719,
            single::absolute_deviation(&prices, &crate::CentralPoint::Mean)
        );
        assert_eq!(
            0.11999999999999886,
            single::absolute_deviation(&prices, &crate::CentralPoint::Median)
        );
        assert_eq!(
            0.3519999999999982,
            single::absolute_deviation(&prices, &crate::CentralPoint::Mode)
        );
    }

    #[test]
    #[should_panic]
    fn singe_absolute_deviation_panic() {
        let prices = Vec::new();
        single::absolute_deviation(&prices, &crate::CentralPoint::Mean);
    }

    #[test]
    fn bulk_absolute_deviation() {
        let prices = vec![100.2, 100.46, 100.53, 100.38, 100.19];
        let period: usize = 3;

        assert_eq!(
            vec![
                0.1311111111111103,
                0.051111111111111995,
                0.11777777777777487
            ],
            bulk::absolute_deviation(&prices, &period, &crate::CentralPoint::Mean)
        );
        assert_eq!(
            vec![0.10999999999999943, 0.0500000000000019, 0.11333333333333447],
            bulk::absolute_deviation(&prices, &period, &crate::CentralPoint::Median)
        );
        assert_eq!(
            vec![0.3966666666666659, 0.45666666666666345, 0.36666666666666475],
            bulk::absolute_deviation(&prices, &period, &crate::CentralPoint::Mode)
        );
    }

    #[test]
    #[should_panic]
    fn bulk_absolute_deviation_panic() {
        let prices = vec![100.2, 100.46, 100.53, 100.38, 100.19];
        let period: usize = 30;
        bulk::absolute_deviation(&prices, &period, &crate::CentralPoint::Mean);
    }

    #[test]
    fn test_single_max() {
        let prices = vec![100.2, 100.46, 100.53, 100.38, 100.19];
        assert_eq!(100.53, single::max(&prices));
    }

    #[test]
    #[should_panic]
    fn test_single_max_panic() {
        let prices = Vec::new();
        single::max(&prices);
    }

    #[test]
    fn test_single_min() {
        let prices = vec![100.2, 100.46, 100.53, 100.38, 100.19];
        assert_eq!(100.19, single::min(&prices));
    }

    #[test]
    #[should_panic]
    fn test_single_min_panic() {
        let prices = Vec::new();
        single::min(&prices);
    }
}
