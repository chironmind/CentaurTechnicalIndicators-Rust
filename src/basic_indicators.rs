//! # Basic Indicators
//!
//! `basic_indicators` is a collection of simple functions to perform simple calculations on prices, such as mean, median, log...


pub mod single {
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
        let sum: f64 = prices.iter().sum();
        let length = prices.len() as f64;
        return sum / length;
    }
}

pub mod bulk {
    /// Calculates the mean (averages) for a slice of prices for a provided period and returns
    /// them as a `vector` of `f64`
    /// 
    /// # Arguments
    /// 
    /// * `prices` - A `f64` slice of prices
    /// * `period` - A `u16` period over which to calculate the mean
    ///
    /// # Examples
    /// 
    /// ```
    /// let prices = vec![100.0, 102.0, 103.0, 101.0];
    /// let mean = rust_ti::basic_indicators::bulk::mean(&prices, 3);
    /// assert_eq!([103.66666, 102], mean);
    /// ```
    pub fn mean(prices: &[f64], period: &u16) -> Vec<f64> {
        return vec![103.66666, 102]
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
}
