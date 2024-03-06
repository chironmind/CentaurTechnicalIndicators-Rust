//! # Basic Indicators
//!
//! `basic_indicators` is a collection of simple functions to perform simple calculations on prices, such as mean, median, log...

/// `single` module holds functions that return a singular value for `basic_indicators`
pub mod single {
    use std::cmp::Ordering;
    /// Calculates the mean (average) for a slice of prices and returns it as an `f64`
    ///
    /// # Arguments
    ///
    /// * `prices` - A `f64` slice of prices
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

    fn cmp_f64(a: &f64, b: &f64) -> Ordering {
        if a < b {
            return Ordering::Less;
        } else if a > b {
            return Ordering::Greater;
        }
        return Ordering::Equal;
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
    /// * `period` - A `usize` period over which to calculate the mean
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
        single::mean(&prices);
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
}
