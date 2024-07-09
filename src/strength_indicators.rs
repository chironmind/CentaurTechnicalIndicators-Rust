//! # Strength Indicators
//!
//! Strength indicators show the strength of a trend
//!
//! ## Bulk
//!
//! * [`accumulation_distribution`](bulk::accumulation_distribution) - Calculates the Accumulation
//! Distribution
//! * [`positive_volume_index`](bulk::positive_volume_index)
//! * [`negative_volume_index`](bulk::negative_volume_index)
//! * [`relative_vigor_index`](bulk::relative_vigor_index)
//!
//! ## Single
//!
//! * [`accumulation_distribution`](single::accumulation_distribution) - Calculates the Accumulation
//! Distribution
//! * [`volume_index`](single::volume_index) - Generic version of PVI and NVI
//! * [`relative_vigor_index`](single::relative_vigor_index)

/// `single` module holds functions that return a singular values
pub mod single {
    use crate::basic_indicators::single::{median, mode};
    use crate::moving_average::single::moving_average;
    use crate::{ConstantModelType, MovingAverageType};
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
    /// ```rust
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

    /// The `volume_index` measures volume trend strength.
    ///
    /// The `volume_index` is a variation of the Postive Volumne Index (PVI) and the Negative
    /// Volumne Index (NVI). As the underlying calculationis are the same for both PVI and NVI, the
    /// single function is made generic so the caller can call the same function whether the difference between
    /// volume today is positive or negative.
    ///
    /// If there is no `previous_volume_index` use 0.0.
    ///
    /// # Arguments
    ///
    /// * `current_close` - Value of current close price
    /// * `previous_close` - Value of previous close price
    /// * `previous_volume_index` - Previous PVI/NVI value, if none use 0.0
    ///
    /// # Examples
    ///
    /// ```rust
    /// let current_close = 105.0;
    /// let previous_close = 100.0;
    ///
    /// let volume_index = rust_ti::strength_indicators::single::volume_index(
    ///     &current_close,
    ///     &previous_close,
    ///     &0.0
    /// );
    ///
    /// assert_eq!(0.052500000000000005, volume_index);
    ///
    /// let next_close = 103.0;
    /// let volume_index = rust_ti::strength_indicators::single::volume_index(
    ///     &next_close,
    ///     &current_close,
    ///     &volume_index
    /// );
    ///
    /// assert_eq!(0.051500000000000004, volume_index);
    /// ```
    pub fn volume_index(
        current_close: &f64,
        previous_close: &f64,
        previous_volume_index: &f64,
    ) -> f64 {
        let division = (current_close - previous_close) / previous_close;
        if previous_volume_index == &0.0 {
            return division + (division * division);
        };
        return previous_volume_index + (division * previous_volume_index);
    }

    /// The `relative_vigor_index` measures the strength of an asset by looking at previous prices.
    ///
    /// The standard model to use is a simple moving average.
    ///
    /// The function assumes that the correct length of prices is passed in. The function needs 4
    /// previous prices to do the base calculation, extra prices passed will be used to calculate
    /// the average.
    ///
    /// # Arguments
    ///
    /// * `open` - Slice of opening prices
    /// * `high` - Slice of highs
    /// * `low` - Slice of lows
    /// * `close` - Slice of closing prices
    /// * `constant_model_type` - Variant of [`ConstantModelType`]
    ///
    /// # Panics
    ///
    /// `relative_vigor_index` will panic if:
    ///     * If lengths of `open`, `high`, `low`, and `close` aren't equal
    ///     * If prices are empty
    ///     * If length of prices is less than 4
    ///
    /// # Examples
    ///
    /// ```rust
    /// let open = vec![95.0, 105.0, 110.0, 115.0, 120.0, 115.0, 110.0, 100.0];
    /// let high = vec![110.0, 115.0, 120.0, 130.0, 135.0, 130.0, 120.0, 105.0];
    /// let low = vec![90.0, 110.0, 105.0, 110.0, 120.0, 105.0, 95.0, 85.0];
    /// let close = vec![100.0, 115.0, 115.0, 120.0, 125.0, 110.0, 100.0, 90.0];
    ///
    /// let relative_vigor_index = rust_ti::strength_indicators::single::relative_vigor_index(
    ///     &open,
    ///     &high,
    ///     &low,
    ///     &close,
    ///     &rust_ti::ConstantModelType::SimpleMovingAverage
    /// );
    ///
    /// assert_eq!(0.10185185185185186, relative_vigor_index);
    /// ```
    pub fn relative_vigor_index(
        open: &[f64],
        high: &[f64],
        low: &[f64],
        close: &[f64],
        constant_model_type: &ConstantModelType,
    ) -> f64 {
        let length = open.len();
        if length != high.len() || length != low.len() || length != close.len() {
            panic!(
                "Length of open ({}), high ({}), low ({}), and close ({}) must be equal",
                length,
                high.len(),
                low.len(),
                close.len()
            )
        };
        if open.is_empty() {
            panic!("Prices cannot be empty")
        };
        if length < 4 {
            panic!("Prices must be at least 4 in length")
        };

        let mut close_open_diff = Vec::new();
        let mut high_low_diff = Vec::new();

        for i in 0..length {
            close_open_diff.push(close[i] - open[i]);
            high_low_diff.push(high[i] - low[i]);
        }

        let mut numerator = Vec::new();
        let mut denominator = Vec::new();

        for i in 3..close_open_diff.len() {
            numerator.push(
                (close_open_diff[i]
                    + (2.0 * close_open_diff[i - 1])
                    + (2.0 * close_open_diff[i - 2])
                    + close_open_diff[i - 3])
                    / 6.0,
            );
            denominator.push(
                (high_low_diff[i]
                    + (2.0 * high_low_diff[i - 1])
                    + (2.0 * high_low_diff[i - 2])
                    + high_low_diff[i - 3])
                    / 6.0,
            );
        }

        let (smoothed_nominator, smoothed_denominator) = match constant_model_type {
            ConstantModelType::SimpleMovingAverage => (
                moving_average(&numerator, &MovingAverageType::Simple),
                moving_average(&denominator, &MovingAverageType::Simple),
            ),
            ConstantModelType::SmoothedMovingAverage => (
                moving_average(&numerator, &MovingAverageType::Smoothed),
                moving_average(&denominator, &MovingAverageType::Smoothed),
            ),
            ConstantModelType::ExponentialMovingAverage => (
                moving_average(&numerator, &MovingAverageType::Exponential),
                moving_average(&denominator, &MovingAverageType::Exponential),
            ),
            ConstantModelType::PersonalisedMovingAverage(alpha_nominator, alpha_denominator) => (
                moving_average(
                    &numerator,
                    &MovingAverageType::Personalised(alpha_nominator, alpha_denominator),
                ),
                moving_average(
                    &denominator,
                    &MovingAverageType::Personalised(alpha_nominator, alpha_denominator),
                ),
            ),
            ConstantModelType::SimpleMovingMedian => (median(&numerator), median(&denominator)),
            ConstantModelType::SimpleMovingMode => (mode(&numerator), mode(&denominator)),
            _ => panic!("Unsupported ConstantModelType"),
        };

        return smoothed_nominator / smoothed_denominator;
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
    /// # Panics
    ///
    /// `accumulation_distribution` will panic if lengths of `high`, `low`, `close`, and `volumes`
    /// aren't equal
    ///
    /// # Examples
    ///
    /// ```rust
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

    /// The `positive_volume_index` measure the volume trend strength when the volume today is
    /// greater than the volume yesterday.
    ///
    /// The function takes in a `previous_positive_volume_index` if not available use 0.0
    ///
    /// # Arguments
    ///
    /// * `close` - Slice of closing prices
    /// * `volume` - Slice of volumes
    /// * `previous_positive_volume_index` - Previous PVI value
    ///
    /// # Panics
    ///
    /// `positive_volume_index` will panic if:
    ///     * length of `close` and `volume` aren't equal
    ///     * `close` or `volume` are empty
    ///
    /// # Examples
    ///
    /// ```rust
    /// let close = vec![100.0, 115.0, 118.0, 120.0, 125.0];
    /// let volume = vec![1000.0, 1200.0, 1300.0, 1100.0, 1100.0];
    ///
    /// let positive_volume_index = rust_ti::strength_indicators::bulk::positive_volume_index(
    ///     &close,
    ///     &volume,
    ///     &0.0
    /// );
    ///
    /// assert_eq!(vec![0.1725, 0.177, 0.177, 0.177], positive_volume_index);
    ///
    /// // Note that the last value from close and volume are the first values here
    /// let next_close = vec![125.0, 122.0, 115.0, 120.0];
    /// let next_volume = vec![1100.0, 1000.0, 1500.0, 1600.0];
    ///
    /// let positive_volume_index = rust_ti::strength_indicators::bulk::positive_volume_index(
    ///     &next_close,
    ///     &next_volume,
    ///     &positive_volume_index.last().unwrap()
    /// );
    ///
    /// assert_eq!(vec![0.177, 0.16684426229508195, 0.1740983606557377], positive_volume_index);
    /// ```
    pub fn positive_volume_index(
        close: &[f64],
        volume: &[f64],
        previous_positive_volume_index: &f64,
    ) -> Vec<f64> {
        let length = close.len();
        if length != volume.len() {
            panic!(
                "Length of close ({}) and volume ({}) need to be equal",
                length,
                volume.len()
            )
        };
        if close.is_empty() {
            panic!("Prices cannot be empty")
        };

        let mut pvis = Vec::new();
        if volume[1] > volume[0] {
            pvis.push(single::volume_index(
                &close[1],
                &close[0],
                previous_positive_volume_index,
            ));
        } else {
            pvis.push(*previous_positive_volume_index);
        };

        for i in 2..length {
            let ppvi = pvis[i - 2];
            if volume[i] > volume[i - 1] {
                pvis.push(single::volume_index(&close[i], &close[i - 1], &ppvi));
            } else {
                pvis.push(ppvi);
            };
        }

        return pvis;
    }

    /// The `negative_volume_index` measure the volume trend strength when the volume today is
    /// less than the volume yesterday.
    ///
    /// The function takes in a `previous_negative_volume_index` if not available use 0.0
    ///
    /// # Arguments
    ///
    /// * `close` - Slice of closing prices
    /// * `volume` - Slice of volumes
    /// * `previous_negative_volume_index` - Previous NVI value
    ///
    /// # Panics
    ///
    /// `negative_volume_index` will panic if:
    ///     * length of `close` and `volume` aren't equal
    ///     * `close` or `volume` are empty
    ///
    /// # Examples
    ///
    /// ```rust
    /// let close = vec![100.0, 115.0, 118.0, 120.0, 125.0];
    /// let volume = vec![1000.0, 1200.0, 1300.0, 1100.0, 1100.0];
    ///
    /// let negative_volume_index = rust_ti::strength_indicators::bulk::negative_volume_index(
    ///     &close,
    ///     &volume,
    ///     &0.0
    /// );
    ///
    /// assert_eq!(vec![0.0, 0.0, 0.017236426314277506, 0.017236426314277506], negative_volume_index);
    ///
    ///
    /// // Note that the last value from close and volume are the first values here
    /// let next_close = vec![125.0, 122.0, 115.0, 120.0];
    /// let next_volume = vec![1100.0, 1000.0, 1500.0, 1600.0];
    ///
    /// let negative_volume_index = rust_ti::strength_indicators::bulk::negative_volume_index(
    ///     &next_close,
    ///     &next_volume,
    ///     &negative_volume_index.last().unwrap()
    /// );
    ///
    /// assert_eq!(vec![0.016822752082734847, 0.016822752082734847, 0.016822752082734847], negative_volume_index);
    /// ```
    pub fn negative_volume_index(
        close: &[f64],
        volume: &[f64],
        previous_negative_volume_index: &f64,
    ) -> Vec<f64> {
        let length = close.len();
        if length != volume.len() {
            panic!(
                "Length of close ({}) and volume ({}) need to be equal",
                length,
                volume.len()
            )
        };
        if close.is_empty() {
            panic!("Prices cannot be empty")
        };

        let mut nvis = Vec::new();
        if volume[1] < volume[0] {
            nvis.push(single::volume_index(
                &close[1],
                &close[0],
                previous_negative_volume_index,
            ));
        } else {
            nvis.push(*previous_negative_volume_index);
        };

        for i in 2..length {
            let pnvi = nvis[i - 2];
            if volume[i] < volume[i - 1] {
                nvis.push(single::volume_index(&close[i], &close[i - 1], &pnvi));
            } else {
                nvis.push(pnvi);
            };
        }

        return nvis;
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

    #[test]
    fn single_volume_index_no_previous() {
        assert_eq!(
            0.004318056345550251,
            single::volume_index(&100.44, &100.01, &0.0)
        );
    }

    #[test]
    fn single_volume_index_previous() {
        assert_eq!(
            0.004324075141730827,
            single::volume_index(&100.58, &100.44, &0.004318056345550251)
        );
    }

    #[test]
    fn bulk_positive_volume_index_no_previous() {
        let close = vec![100.14, 98.98, 99.07, 100.1];
        let volume = vec![1000.0, 1200.0, 1300.0, 1100.0];
        assert_eq!(
            vec![
                -0.011449598682475618,
                -0.011460009511748427,
                -0.011460009511748427
            ],
            bulk::positive_volume_index(&close, &volume, &0.0)
        );
    }

    #[test]
    fn bulk_positive_volume_index_previous() {
        let close = vec![100.1, 99.96, 99.56, 100.72, 101.16];
        let volume = vec![1100.0, 1100.0, 1000.0, 1500.0, 1600.0];
        assert_eq!(
            vec![
                -0.011460009511748427,
                -0.011460009511748427,
                -0.011593533125987359,
                -0.011644180014146955
            ],
            bulk::positive_volume_index(&close, &volume, &-0.011460009511748427)
        );
    }

    #[test]
    fn bulk_positive_volume_index_all_positive() {
        let close = vec![100.14, 98.98, 99.07, 100.1];
        let volume = vec![1000.0, 1200.0, 1300.0, 1400.0];
        assert_eq!(
            vec![
                -0.011449598682475618,
                -0.011460009511748427,
                -0.011579155668981706
            ],
            bulk::positive_volume_index(&close, &volume, &0.0)
        );
    }

    #[test]
    fn bulk_positive_volume_index_all_negative() {
        let close = vec![100.14, 98.98, 99.07, 100.1];
        let volume = vec![1000.0, 900.0, 800.0, 700.0];
        assert_eq!(
            vec![0.0, 0.0, 0.0],
            bulk::positive_volume_index(&close, &volume, &0.0)
        );
    }

    #[test]
    #[should_panic]
    fn bulk_positive_volume_index_panic_length() {
        let close = vec![100.14, 98.98, 100.1];
        let volume = vec![1000.0, 900.0, 800.0, 700.0];
        bulk::positive_volume_index(&close, &volume, &0.0);
    }

    #[test]
    #[should_panic]
    fn bulk_positive_volume_index_panic_empty() {
        let close = Vec::new();
        let volume = Vec::new();
        bulk::positive_volume_index(&close, &volume, &0.0);
    }

    #[test]
    fn bulk_negative_volume_index_no_previous() {
        let close = vec![100.14, 98.98, 99.07, 100.1];
        let volume = vec![1000.0, 1200.0, 1300.0, 1100.0];
        assert_eq!(
            vec![0.0, 0.0, 0.010504780356171802],
            bulk::negative_volume_index(&close, &volume, &0.0)
        );
    }

    #[test]
    fn bulk_negative_volume_index_previous() {
        let close = vec![100.1, 99.96, 99.56, 100.72, 101.16];
        let volume = vec![1100.0, 1100.0, 1000.0, 1500.0, 1600.0];
        assert_eq!(
            vec![
                0.010504780356171802,
                0.010462744420372797,
                0.010462744420372797,
                0.010462744420372797
            ],
            bulk::negative_volume_index(&close, &volume, &0.010504780356171802)
        );
    }

    #[test]
    fn bulk_negative_volume_index_all_positive() {
        let close = vec![100.14, 98.98, 99.07, 100.1];
        let volume = vec![1000.0, 1200.0, 1300.0, 1400.0];
        assert_eq!(
            vec![0.0, 0.0, 0.0],
            bulk::negative_volume_index(&close, &volume, &0.0)
        );
    }

    #[test]
    fn bulk_negative_volume_index_all_negative() {
        let close = vec![100.14, 98.98, 99.07, 100.1];
        let volume = vec![1000.0, 900.0, 800.0, 700.0];
        assert_eq!(
            vec![
                -0.011449598682475618,
                -0.011460009511748427,
                -0.011579155668981706
            ],
            bulk::negative_volume_index(&close, &volume, &0.0)
        );
    }

    #[test]
    #[should_panic]
    fn bulk_negative_volume_index_panic_length() {
        let close = vec![100.14, 98.98, 100.1];
        let volume = vec![1000.0, 900.0, 800.0, 700.0];
        bulk::negative_volume_index(&close, &volume, &0.0);
    }

    #[test]
    #[should_panic]
    fn bulk_negative_volume_index_panic_empty() {
        let close = Vec::new();
        let volume = Vec::new();
        bulk::negative_volume_index(&close, &volume, &0.0);
    }

    #[test]
    fn single_relative_vigor_index_ma() {
        let open = vec![100.73, 99.62, 99.82, 100.38, 100.97, 101.81];
        let high = vec![102.32, 100.69, 100.83, 101.73, 102.01, 102.75];
        let low = vec![100.14, 98.98, 99.07, 100.1, 99.96, 100.55];
        let close = vec![100.55, 99.01, 100.43, 101.0, 101.76, 102.03];
        assert_eq!(
            0.2063784115302081,
            single::relative_vigor_index(
                &open,
                &high,
                &low,
                &close,
                &crate::ConstantModelType::SimpleMovingAverage
            )
        );
    }

    #[test]
    fn single_relative_vigor_index_sma() {
        let open = vec![100.73, 99.62, 99.82, 100.38, 100.97, 101.81];
        let high = vec![102.32, 100.69, 100.83, 101.73, 102.01, 102.75];
        let low = vec![100.14, 98.98, 99.07, 100.1, 99.96, 100.55];
        let close = vec![100.55, 99.01, 100.43, 101.0, 101.76, 102.03];
        assert_eq!(
            0.2424082260234505,
            single::relative_vigor_index(
                &open,
                &high,
                &low,
                &close,
                &crate::ConstantModelType::SmoothedMovingAverage
            )
        );
    }

    #[test]
    fn single_relative_vigor_index_ema() {
        let open = vec![100.73, 99.62, 99.82, 100.38, 100.97, 101.81];
        let high = vec![102.32, 100.69, 100.83, 101.73, 102.01, 102.75];
        let low = vec![100.14, 98.98, 99.07, 100.1, 99.96, 100.55];
        let close = vec![100.55, 99.01, 100.43, 101.0, 101.76, 102.03];
        assert_eq!(
            0.2635196472571675,
            single::relative_vigor_index(
                &open,
                &high,
                &low,
                &close,
                &crate::ConstantModelType::ExponentialMovingAverage
            )
        );
    }

    #[test]
    fn single_relative_vigor_index_pma() {
        let open = vec![100.73, 99.62, 99.82, 100.38, 100.97, 101.81];
        let high = vec![102.32, 100.69, 100.83, 101.73, 102.01, 102.75];
        let low = vec![100.14, 98.98, 99.07, 100.1, 99.96, 100.55];
        let close = vec![100.55, 99.01, 100.43, 101.0, 101.76, 102.03];
        assert_eq!(
            0.29194621866781373,
            single::relative_vigor_index(
                &open,
                &high,
                &low,
                &close,
                &crate::ConstantModelType::PersonalisedMovingAverage(&5.0, &4.0)
            )
        );
    }

    #[test]
    fn single_relative_vigor_index_median() {
        let open = vec![100.73, 99.62, 99.82, 100.38, 100.97, 101.81];
        let high = vec![102.32, 100.69, 100.83, 101.73, 102.01, 102.75];
        let low = vec![100.14, 98.98, 99.07, 100.1, 99.96, 100.55];
        let close = vec![100.55, 99.01, 100.43, 101.0, 101.76, 102.03];
        assert_eq!(
            0.24558139534884124,
            single::relative_vigor_index(
                &open,
                &high,
                &low,
                &close,
                &crate::ConstantModelType::SimpleMovingMedian
            )
        );
    }

    #[test]
    fn single_relative_vigor_index_mode() {
        let open = vec![100.73, 99.62, 99.82, 100.38, 100.97, 101.81];
        let high = vec![102.32, 100.69, 100.83, 101.73, 102.01, 102.75];
        let low = vec![100.14, 98.98, 99.07, 100.1, 99.96, 100.55];
        let close = vec![100.55, 99.01, 100.43, 101.0, 101.76, 102.03];
        assert_eq!(
            0.0,
            single::relative_vigor_index(
                &open,
                &high,
                &low,
                &close,
                &crate::ConstantModelType::SimpleMovingMode
            )
        );
    }

    #[test]
    fn single_relative_vigor_index_minimum() {
        let open = vec![100.73, 99.62, 99.82, 100.38];
        let high = vec![102.32, 100.69, 100.83, 101.73];
        let low = vec![100.14, 98.98, 99.07, 100.1];
        let close = vec![100.55, 99.01, 100.43, 101.0];
        assert_eq!(
            0.04093023255814197,
            single::relative_vigor_index(
                &open,
                &high,
                &low,
                &close,
                &crate::ConstantModelType::SimpleMovingAverage
            )
        );
    }

    #[test]
    #[should_panic]
    fn single_relative_vigor_index_panic_length_open() {
        let open = vec![100.73, 99.62, 99.82];
        let high = vec![102.32, 100.69, 100.83, 101.73];
        let low = vec![100.14, 98.98, 99.07, 100.1];
        let close = vec![100.55, 99.01, 100.43, 101.0];
        single::relative_vigor_index(
            &open,
            &high,
            &low,
            &close,
            &crate::ConstantModelType::SimpleMovingAverage,
        );
    }

    #[test]
    #[should_panic]
    fn single_relative_vigor_index_panic_length_high() {
        let open = vec![100.73, 99.62, 99.82, 100.38];
        let high = vec![102.32, 100.69, 100.83];
        let low = vec![100.14, 98.98, 99.07, 100.1];
        let close = vec![100.55, 99.01, 100.43, 101.0];
        single::relative_vigor_index(
            &open,
            &high,
            &low,
            &close,
            &crate::ConstantModelType::SimpleMovingAverage,
        );
    }

    #[test]
    #[should_panic]
    fn single_relative_vigor_index_panic_length_low() {
        let open = vec![100.73, 99.62, 99.82, 100.38];
        let high = vec![102.32, 100.69, 100.83, 101.73];
        let low = vec![100.14, 98.98, 100.1];
        let close = vec![100.55, 99.01, 100.43, 101.0];
        single::relative_vigor_index(
            &open,
            &high,
            &low,
            &close,
            &crate::ConstantModelType::SimpleMovingAverage,
        );
    }

    #[test]
    #[should_panic]
    fn single_relative_vigor_index_panic_length_close() {
        let open = vec![100.73, 99.62, 99.82, 100.38];
        let high = vec![102.32, 100.69, 100.83, 101.73];
        let low = vec![100.14, 98.98, 99.07, 100.1];
        let close = vec![100.55, 99.01, 100.40];
        single::relative_vigor_index(
            &open,
            &high,
            &low,
            &close,
            &crate::ConstantModelType::SimpleMovingAverage,
        );
    }

    #[test]
    #[should_panic]
    fn single_relative_vigor_index_panic_length_overall() {
        let open = vec![100.73, 99.62, 99.82];
        let high = vec![102.32, 100.69, 100.83];
        let low = vec![100.14, 98.98, 99.07];
        let close = vec![100.55, 99.01, 100.43];
        single::relative_vigor_index(
            &open,
            &high,
            &low,
            &close,
            &crate::ConstantModelType::SimpleMovingAverage,
        );
    }

    #[test]
    #[should_panic]
    fn single_relative_vigor_index_panic_empty() {
        let open = Vec::new();
        let high = Vec::new();
        let low = Vec::new();
        let close = Vec::new();
        single::relative_vigor_index(
            &open,
            &high,
            &low,
            &close,
            &crate::ConstantModelType::SimpleMovingAverage,
        );
    }
}
