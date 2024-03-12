//! # Basic Indicators
//!
//! `basic_indicators` is a collection of simple functions to perform simple calculations on prices, such as mean, median, log...

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
        let length = prices.len() as f64;
        if length == 0.0 {
            panic!("Prices ({:?}) is empty", prices);
        };
        let sum: f64 = prices.iter().sum();
        return sum / length;
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
        let length = prices.len();

        if length == 0 {
            panic!("Prices ({:?}) is empty", prices);
        };

        let mut ordered_prices = prices
            .iter()
            .filter_map(|f| if f.is_nan() { None } else { Some(*f) })
            .collect::<Vec<f64>>();
        ordered_prices.sort_by(cmp_f64);
        let middle: usize = length / 2;
        if length % 2 == 0 {
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
        let length = prices.len();

        if length == 0 {
            panic!("Prices ({:?}) is empty", prices);
        };

        let rounded_prices = prices.iter().map(|x| x.round() as u64).collect();
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
            panic!("price_t ({}) and price_t_1 ({}) need to be greater than 0.0", price_t, price_t_1);
        }
        return price_t.ln() - price_t_1.ln();
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
    fn most_frequent(vector: Vec<u64>) -> f64 {
        let mut map: HashMap<u64, usize> = HashMap::new();
        for x in vector {
            *map.entry(x as u64).or_default() += 1;
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
            return max_price.iter().sum::<u64>() as f64 / max_price.len() as f64;
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
        for (index, _value) in prices.iter().enumerate() {
            let end_index = period + index;
            if end_index > length {
                break;
            }
            means.push(single::mean(&prices[index..end_index]));
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
        for (index, _value) in prices.iter().enumerate() {
            let end_index = period + index;
            if end_index > length {
                break;
            }
            medians.push(single::median(&prices[index..end_index]));
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
        for (index, _value) in prices.iter().enumerate() {
            let end_index = period + index;
            if end_index > length {
                break;
            }
            modes.push(single::mode(&prices[index..end_index]));
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

    // TODO: Finish log diff
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
        assert_eq!(vec![4.607168188650764, 4.609759638321899, 4.610456190417329, 4.608962984226787, 4.607068383271171], bulk::log(&prices));
    }

    #[test]
    #[should_panic]
    fn bulk_log_panic() {
        let prices = Vec::new();
        bulk::log(&prices);
    }

    #[test]
    fn single_log_difference() {
        assert_eq!(-0.0018946009556159993, single::log_difference(&100.19, &100.38));
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
}
