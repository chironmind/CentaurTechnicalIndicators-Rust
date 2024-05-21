//! # Volatility Indicators
//!
//! Volatility indicators show how volatile an asset are.

/// `single` module holds functions that return a singular values
pub mod single {
    use crate::basic_indicators::single::max;
    /// The `ulcer_index` calculates how quickly the price at t is able to get back to its former high
    ///
    /// It can be used to instead of the standard deviation so is an option in the `DeviationModel`
    /// enum.
    ///
    /// # Arguments
    ///
    /// * `prices` - An `f64` slice of prices
    ///
    /// # Examples
    ///
    /// ```
    /// let prices = vec![100.0, 102.0, 103.0, 101.0, 99.0];
    /// let ulcer_index = rust_ti::volatility_indicators::single::ulcer_index(&prices);
    /// assert_eq!(1.9417475728155338, ulcer_index);
    /// ```
    pub fn ulcer_index(prices: &[f64]) -> f64 {
        if prices.is_empty() {
            panic!("Prices cannot be empty")
        };

        let length = prices.len();
        let mut sqaured_percentage_drawdown: Vec<f64> = Vec::new();
        for i in 1..length {
            let period_max = max(&prices[..i + 1]);
            let percentage_drawdown = ((&prices[i] - &period_max) / &period_max) * 100.0;
            sqaured_percentage_drawdown.push(percentage_drawdown.powi(2));
        }
        let squared_average: f64 = sqaured_percentage_drawdown.iter().sum::<f64>() / length as f64;
        return squared_average.sqrt();
    }
}

/// `bulk` module holds functions that return multiple values
pub mod bulk {
    use crate::volatility_indicators::single;
    /// The `ulcer_index` how quickly the price at t is able to get back to its former high
    ///
    /// It can be used to instead of the standard deviation so is an option in the `DeviationModel`
    /// enum.
    ///
    /// # Arguments
    ///
    /// * `prices` - An `f64` slice of prices
    /// * `period` - Period over which to calculate the Ulcer index
    ///
    /// # Examples
    ///
    /// ```
    /// let prices = vec![100.0, 102.0, 103.0, 101.0, 99.0, 99.0, 102.0];
    /// let period: usize = 5;
    /// let ulcer_index = rust_ti::volatility_indicators::bulk::ulcer_index(&prices, &period);
    /// assert_eq!(vec![1.9417475728155338, 2.6051277407764535, 2.641062234705911], ulcer_index);
    /// ```
    pub fn ulcer_index(prices: &[f64], period: &usize) -> Vec<f64> {
        let length = prices.len();
        if period > &length {
            panic!(
                "Period ({}) cannot be longer than length of prices ({})",
                period, length
            )
        };

        let mut ulcer_indexes = Vec::new();
        let loop_max = length - period + 1;
        for i in 0..loop_max {
            ulcer_indexes.push(single::ulcer_index(&prices[i..i + period]));
        }
        return ulcer_indexes;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_ulcer_index() {
        let prices = vec![100.46, 100.53, 100.38, 100.19, 100.21];
        assert_eq!(0.21816086938686668, single::ulcer_index(&prices));
    }

    #[test]
    #[should_panic]
    fn single_ucler_index_panic() {
        let prices = Vec::new();
        single::ulcer_index(&prices);
    }

    #[test]
    fn bulk_ulcer_index() {
        let prices = vec![100.46, 100.53, 100.38, 100.19, 100.21, 100.32, 100.28];
        assert_eq!(
            vec![0.21816086938686668, 0.2373213243162752, 0.12490478596260104],
            bulk::ulcer_index(&prices, &5_usize)
        );
    }

    #[test]
    #[should_panic]
    fn bulk_ulcer_index_panic() {
        let prices = vec![100.46, 100.53, 100.38, 100.19, 100.21, 100.32, 100.28];
        bulk::ulcer_index(&prices, &50_usize);
    }
}
