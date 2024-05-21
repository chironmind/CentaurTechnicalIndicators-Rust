//! # Strength Indicators
//!
//! Strength indicators show the strength of a trend

/// `single` module holds functions that return a singular values
pub mod single {
    /// The `accumulation_distribution` shows whether the stock is being accumulated or
    /// distributed.
    ///
    /// # Arguments
    ///
    /// * `high` - High for the period
    /// * `low` - Low for the period
    /// * `close` - Closing price for the period
    /// * `volume` - Volume of transaction for the period
    /// * `previous_accumulation_distribution` - Previous value of accumulation distribution. If no
    /// previous use 0.0
    ///
    /// # Examples
    ///
    /// ```
    /// let high = 103.0;
    /// let low = 99.0;
    /// let close = 102.0;
    /// let volume = 1000.0;
    /// let previous = 0.0;
    /// let accumulation_distribution =
    /// rust_ti::strength_indicators::single::accumulation_distribution(&high, &low, &close,
    /// &volume, &previous);
    /// assert_eq!(500.0, accumulation_distribution);
    ///
    /// let high = 102.0;
    /// let low = 99.0;
    /// let close = 100.0;
    /// let volume = 1500.0;
    /// let accumulation_distribution =
    /// rust_ti::strength_indicators::single::accumulation_distribution(&high, &low, &close,
    /// &volume, &accumulation_distribution);
    /// assert_eq!(0.0, accumulation_distribution);
    /// ```
    pub fn accumulation_distribution(
        high: &f64,
        low: &f64,
        close: &f64,
        volume: &f64,
        previous_accumulation_distribution: &f64,
    ) -> f64 {
        let money_flow_multiplier = ((close - low) - (high - close)) / (high - low);
        let money_flow_volume = money_flow_multiplier * volume;
        return previous_accumulation_distribution + money_flow_volume;
    }
}

/// `bulk` module holds functions that return multiple values
pub mod bulk {
    use crate::strength_indicators::single;
    /// The `accumulation_distribution` shows whether the stock is being accumulated or
    /// distributed.
    ///
    /// # Arguments
    /// * `high` - Slice of highs
    /// * `low` - Slice of lows
    /// * `close` - Slice of closing prices
    /// * `volumes` - Slice of volumes
    /// * `previous_accumulation_distribution` - Previous value of accumulation distribution. If no
    /// previous use 0.0
    ///
    /// # Examples
    ///
    /// ```
    /// let high = vec![103.0, 102.0, 105.0];
    /// let low = vec![99.0, 99.0, 100.0];
    /// let close = vec![102.0, 100.0, 103.0];
    /// let volume = vec![1000.0, 1500.0, 1200.0];
    /// let previous = 0.0;
    /// let accumulation_distribution =
    /// rust_ti::strength_indicators::bulk::accumulation_distribution(&high, &low, &close,
    /// &volume, &previous);
    /// assert_eq!(vec![500.0, 0.0, 240.0], accumulation_distribution);
    /// ```
    pub fn accumulation_distribution(
        high: &[f64],
        low: &[f64],
        close: &[f64],
        volume: &[f64],
        previous_accumulation_distribution: &f64,
    ) -> Vec<f64> {
        let length = close.len();
        if length != high.len() || length != close.len() || length != volume.len() {
            panic!("Length of close prices ({}) must match length of high ({}), low ({}), and volume ({})", length, high.len(), close.len(), volume.len());
        };
        let mut ads = vec![single::accumulation_distribution(
            &high[0],
            &low[0],
            &close[0],
            &volume[0],
            &previous_accumulation_distribution,
        )];
        for i in 1..length {
            ads.push(single::accumulation_distribution(
                &high[i],
                &low[i],
                &close[i],
                &volume[i],
                &ads[i - 1],
            ));
        }
        return ads;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_accumulation_distribution_no_previous() {
        assert_eq!(
            -38.28571428571309,
            single::accumulation_distribution(&100.53, &99.62, &100.01, &268.0, &0.0)
        )
    }

    #[test]
    fn single_accumulation_distribution_previous() {
        assert_eq!(
            0.6342857142869107,
            single::accumulation_distribution(&100.53, &99.62, &100.01, &268.0, &38.92)
        )
    }

    #[test]
    fn bulk_accumulation_distribution_no_previous() {
        let highs = vec![100.53, 100.68];
        let lows = vec![99.62, 99.97];
        let close = vec![100.01, 100.44];
        let volume = vec![268.0, 319.0];
        assert_eq!(
            vec![-38.28571428571309, 65.05231388329526],
            bulk::accumulation_distribution(&highs, &lows, &close, &volume, &0.0)
        );
    }

    #[test]
    fn bulk_accumulation_distribution_previous() {
        let highs = vec![100.53, 100.68];
        let lows = vec![99.62, 99.97];
        let close = vec![100.01, 100.44];
        let volume = vec![268.0, 319.0];
        assert_eq!(
            vec![0.6342857142869107, 103.97231388329524],
            bulk::accumulation_distribution(&highs, &lows, &close, &volume, &38.92)
        );
    }

    #[test]
    #[should_panic]
    fn bulk_accumulation_distribution_panic_high_length() {
        let highs = vec![100.53];
        let lows = vec![99.62, 99.97];
        let close = vec![100.01, 100.44];
        let volume = vec![268.0, 319.0];
        bulk::accumulation_distribution(&highs, &lows, &close, &volume, &0.0);
    }

    #[test]
    #[should_panic]
    fn bulk_accumulation_distribution_panic_low_length() {
        let highs = vec![100.53, 100.68];
        let lows = vec![99.62];
        let close = vec![100.01, 100.44];
        let volume = vec![268.0, 319.0];
        bulk::accumulation_distribution(&highs, &lows, &close, &volume, &0.0);
    }

    #[test]
    #[should_panic]
    fn bulk_accumulation_distribution_panic_close_length() {
        let highs = vec![100.53, 100.68];
        let lows = vec![99.62, 99.97];
        let close = vec![100.01];
        let volume = vec![268.0, 319.0];
        bulk::accumulation_distribution(&highs, &lows, &close, &volume, &0.0);
    }

    #[test]
    #[should_panic]
    fn bulk_accumulation_distribution_panic_volume_length() {
        let highs = vec![100.53, 100.68];
        let lows = vec![99.62, 99.97];
        let close = vec![100.01, 100.44];
        let volume = vec![268.0];
        bulk::accumulation_distribution(&highs, &lows, &close, &volume, &0.0);
    }
}
