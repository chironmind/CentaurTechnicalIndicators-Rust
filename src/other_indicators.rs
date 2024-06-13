//! # Other Indicators
//!
//! Indicators that don't really fit in anywhere else
//!
//! ## Bulk
//!
//! * [`return_on_investment`](bulk::return_on_investment) - Calculates the return on investment
//!
//! ## Single
//!
//! * [`return_on_investment`](single::return_on_investment) - Calculates the return on investment

/// `single` module holds functions that return a singular values
pub mod single {
    /// The `return_on_investment` function calculates the value of the investment at the end of
    /// the period, as well as the percentage change. Returns the final investment worth and the
    /// percent change of the investment.
    ///
    /// Also known as "how much money would I have made if I had invested when the price was x".
    ///
    /// # Arguments
    ///
    /// * `start_price` - Price of the asset at the start of the period
    /// * `end_price` - Price of the asset at the end of the period
    /// * `investment` - Amount of money invested in the asset
    ///
    /// # Examples
    ///
    /// ```rust
    /// let start_price = 100.0;
    /// let end_price = 110.0;
    /// let initial_investment = 1000.0;
    /// let return_on_investment =
    /// rust_ti::other_indicators::single::return_on_investment(&start_price, &end_price,
    /// &initial_investment);
    /// assert_eq!((1100.0, 10.0), return_on_investment);
    ///
    /// // new_price shifts end_price to be the new start price
    /// let new_price = 98.0;
    /// let return_on_investment =
    /// rust_ti::other_indicators::single::return_on_investment(&end_price, &new_price,
    /// &return_on_investment.0);
    /// assert_eq!((980.0, -10.909090909090908), return_on_investment);
    /// ```
    pub fn return_on_investment(
        start_price: &f64,
        end_price: &f64,
        investment: &f64,
    ) -> (f64, f64) {
        let initial_investment = investment / start_price;
        let final_investment_value = end_price * initial_investment;
        let percent_return = ((final_investment_value - investment) / investment) * 100.0;
        return (final_investment_value, percent_return);
    }
}

/// `bulk` module holds functions that return a vector of values
pub mod bulk {
    use crate::other_indicators::single;
    /// The `return_on_investment` function calculates the value of the investment at the end of
    /// the period, as well as the percentage change. Returns the final investment worth and the
    /// percent change of the investment.
    ///
    /// Also known as "how much money would I have made if I had invested when the price was x".
    ///
    /// # Arguments
    ///
    /// * `prices` - Price of the asset at the start of the period
    /// * `investment` - Amount of money invested in the asset
    ///
    /// # Panics
    ///
    /// `return_on_investment` will panic if `prices` is empty
    ///
    /// # Examples
    ///
    /// ```rust
    /// let prices = vec![100.0, 102.0, 103.0, 101.0, 99.0, 99.0, 102.0];
    /// let initial_investment = 1000.0;
    /// let return_on_investment =
    /// rust_ti::other_indicators::bulk::return_on_investment(&prices,
    /// &initial_investment);
    /// assert_eq!(vec![(1020.0, 2.0), (1030.0, 0.9803921568627451), (1010.0, -1.9417475728155338),
    /// (990.0, -1.9801980198019802), (990.0, 0.0), (1020.0, 3.030303030303030303)], return_on_investment);
    /// ```
    pub fn return_on_investment(prices: &[f64], investment: &f64) -> Vec<(f64, f64)> {
        if prices.is_empty() {
            panic!("Prices cannot be empty")
        };
        let mut rois = vec![single::return_on_investment(
            &prices[0], &prices[1], investment,
        )];
        let loop_max = prices.len();
        for i in 2..loop_max {
            rois.push(single::return_on_investment(
                &prices[i - 1],
                &prices[i],
                &rois[i - 2].0,
            ));
        }
        return rois;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_return_on_investment() {
        let start_price = 100.46;
        let end_price = 100.53;
        let investment = 1000.0;
        assert_eq!(
            (1000.6967947441768, 0.06967947441768274),
            single::return_on_investment(&start_price, &end_price, &investment)
        );
    }

    #[test]
    fn bulk_return_on_investment() {
        let prices = vec![100.46, 100.53, 100.38, 100.19, 100.21];
        let investment = 1000.0;
        assert_eq!(
            vec![
                (1000.6967947441768, 0.06967947441768274),
                (999.2036631495122, -0.14920919128619353),
                (997.3123631296038, -0.18928073321378402),
                (997.5114473422257, 0.01996207206307317)
            ],
            bulk::return_on_investment(&prices, &investment)
        );
    }

    #[test]
    #[should_panic]
    fn bulk_return_on_investment_panic() {
        let prices = Vec::new();
        let investment = 1000.0;
        bulk::return_on_investment(&prices, &investment);
    }
}
