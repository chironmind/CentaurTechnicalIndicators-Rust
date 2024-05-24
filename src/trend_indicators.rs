//! Trend Indicators
//!
//! Trend indicators show the trend direction of an asset

/// `single` module holds functions that return a singular values
pub mod single {
    use crate::basic_indicators::single::{max, min};
    /// The `aroon_up` indicator tracks the uptrends in the `aroon_indicator` and is used to
    /// calculate the `aroon_oscillator`.
    ///
    /// The Aroon up is included in the return of the `aroon_indicator`. If the caller wants the
    /// Aroon up, Aroon down, and Aroon oscillator, then it is easier to call the `aroon_indicator`
    /// rather that the three seperately.
    ///
    /// Standard period to use is 25 but the caller determines how many prices they want to provide
    /// and the period will be determined from the length of `highs`.
    ///
    /// # Arguments
    ///
    /// * `highs` - Slice of highs
    ///
    /// # Examples
    ///
    /// ```
    /// let highs = vec![103.0, 102.0, 107.0, 104.0, 100.0];
    /// let aroon_up = rust_ti::trend_indicators::single::aroon_up(&highs);
    /// assert_eq!(50.0, aroon_up);
    /// ```
    pub fn aroon_up(highs: &[f64]) -> f64 {
        if highs.is_empty() {
            panic!("Highs cannot be empty")
        };

        let period = highs.len() - 1; // current period should be excluded from length
        let period_max = max(highs);
        let periods_since_max = period - highs.iter().rposition(|&x| x == period_max).unwrap();
        return 100.0 * ((period as f64 - periods_since_max as f64) / period as f64);
    }

    /// The `aroon_down` indicator tracks the downtrends in the `aroon_indicator` and is used to
    /// calculate the `aroon_oscillator`.
    ///
    /// The Aroon down is included in the return of the `aroon_indicator`. If the caller wants the
    /// Aroon up, Aroon down, and Aroon oscillator, then it is easier to call the `aroon_indicator`
    /// rather that the three seperately.
    ///
    /// Standard period to use is 25 but the caller determines how many prices they want to provide
    /// and the period will be determined from the length of `lows`.
    ///
    /// # Arguments
    ///
    /// * `low` - Slice of lows
    ///
    /// # Examples
    ///
    /// ```
    /// let lows = vec![98.0, 95.0, 101.0, 100.0, 97.0];
    /// let aroon_down = rust_ti::trend_indicators::single::aroon_down(&lows);
    /// assert_eq!(25.0, aroon_down);
    /// ```
    pub fn aroon_down(lows: &[f64]) -> f64 {
        if lows.is_empty() {
            panic!("Lows cannot be empty")
        };

        let period = lows.len() - 1; // current period should be excluded from length
        let period_min = min(lows);
        let periods_since_min = period - lows.iter().rposition(|&x| x == period_min).unwrap();
        return 100.0 * ((period as f64 - periods_since_min as f64) / period as f64);
    }

    /// The `aroon_oscillators` takes the difference between the Aroon up and the Aroon down to
    /// give a general sense of trend.
    ///
    /// The Aroon oscillator is returned in the `aroon_indicator` so it is easier to call that one
    /// function rather than 3.
    ///
    /// # Arguments
    ///
    /// * `aroon_up` - Aroon up for the period
    /// * `aroon_down` - Aroon down for the period
    ///
    /// # Examples
    ///
    /// ```
    /// let aroon_up = 50.0;
    /// let aroon_down = 25.0;
    /// let aroon_oscillator = rust_ti::trend_indicators::single::aroon_oscillator(&aroon_up,
    /// &aroon_down);
    /// assert_eq!(25.0, aroon_oscillator);
    /// ```
    pub fn aroon_oscillator(aroon_up: &f64, aroon_down: &f64) -> f64 {
        return aroon_up - aroon_down;
    }

    /// The `aroon_indicator` returns the Aroon up, Aroon down, and Aroon oscillator in that order
    ///
    /// # Arguments
    ///
    /// * `high` - Slice of highs
    /// * `low` - Slice of lows
    ///
    /// # Examples
    ///
    /// ```
    /// let highs = vec![103.0, 102.0, 107.0, 104.0, 100.0];
    /// let lows = vec![98.0, 95.0, 101.0, 100.0, 97.0];
    /// let aroon_indicator = rust_ti::trend_indicators::single::aroon_indicator(&highs, &lows);
    /// assert_eq!((50.0, 25.0, 25.0), aroon_indicator);
    /// ```
    pub fn aroon_indicator(highs: &[f64], lows: &[f64]) -> (f64, f64, f64) {
        if highs.len() != lows.len() {
            panic!(
                "Length of highs ({}) must match length of lows ({})",
                highs.len(),
                lows.len()
            )
        };

        let aroon_up = aroon_up(&highs);
        let aroon_down = aroon_down(&lows);
        let aroon_oscillaor = aroon_oscillator(&aroon_up, &aroon_down);
        return (aroon_up, aroon_down, aroon_oscillaor);
    }

    // TODO: Probably need a long and short function otherwise need to pass a string in the if
    // which is meh
    /// The `long_parabolic_time_price_system` sets Stop and Reverse (SaR) points based on a significant point
    /// (period low) or previous SaR, time, and price. The function is to be used when considering
    /// a long position.
    ///
    /// Welles uses the significant point as the SaR when no SaR points have been calculated
    /// then uses the previous SaR. He also uses an acceleration factor that increases by 0.02 every
    /// day a new high is hit and never goes over 0.2, but in this function the value is determined
    /// by the function caller.
    ///
    /// Welles has a rule that the SaR at t+1 cannot be above the low for t and t-1.
    /// The single function doesn't take this into account and makes the assumption that the caller
    /// will pass in the correct value for `previous_sar`.
    ///
    /// # Arguments
    ///
    /// * `previous_sar` - Previous Stop and Reverse, if none use the significant point, which is
    /// the period low if going long, or the period high if going short.
    /// * `extreme_point` - Highest high or lowest low of the period being observed
    /// * `acceleration_factor` - Factor used to multiply the difference between extreme point and
    /// previous SaR.
    /// * `low` - Lowest low for t or t-1
    ///
    /// # Examples
    ///
    /// ```
    /// let previous_sar = 50.09306;
    /// let extreme_point = 52.35;
    /// let acceleration_factor = 0.02;
    /// let low = 50.6;
    /// let parabolic_time_price_system =
    /// rust_ti::trend_indicators::single::long_parabolic_time_price_system(&previous_sar,
    /// &extreme_point, &acceleration_factor, &low);
    /// assert_eq!(50.1381988, parabolic_time_price_system);
    ///
    /// let previous_sar = 51.96;
    /// let extreme_point = 54.2;
    /// let acceleration_factor = 0.12;
    /// let low = 52.1;
    /// let parabolic_time_price_system =
    /// rust_ti::trend_indicators::single::long_parabolic_time_price_system(&previous_sar,
    /// &extreme_point, &acceleration_factor, &low);
    /// assert_eq!(52.1, parabolic_time_price_system);
    /// ```
    pub fn long_parabolic_time_price_system(
        previous_sar: &f64,
        extreme_point: &f64,
        acceleration_factor: &f64,
        low: &f64,
    ) -> f64 {
        let sar = previous_sar + acceleration_factor * (extreme_point - previous_sar);
        if &sar > low {
            return *low;
        };
        return sar;
    }
}

/// `bulk` module holds functions that return multiple vaues
pub mod bulk {
    use crate::trend_indicators::single;
    /// The `aroon_up` indicator tracks the uptrends in the `aroon_indicator` and is used to
    /// calculate the `aroon_oscillator`.
    ///
    /// The Aroon up is included in the return of the `aroon_indicator`. If the caller wants the
    /// Aroon up, Aroon down, and Aroon oscillator, then it is easier to call the `aroon_indicator`
    /// rather that the three seperately.
    ///
    /// # Arguments
    ///
    /// * `highs` - Slice of highs
    /// * `period` - Period over which to calculate the Aroon up
    ///
    /// # Examples
    ///
    /// ```
    /// let highs = vec![103.0, 102.0, 107.0, 104.0, 100.0, 102.0, 99.0];
    /// let period: usize = 5;
    /// let aroon_up = rust_ti::trend_indicators::bulk::aroon_up(&highs, &period);
    /// assert_eq!(vec![50.0, 25.0, 0.0], aroon_up);
    /// ```
    pub fn aroon_up(highs: &[f64], period: &usize) -> Vec<f64> {
        let length = highs.len();
        if &length < period {
            panic!(
                "Period ({}) cannot be longer than length of highs ({})",
                period, length
            )
        };

        let mut aroon_ups = Vec::new();
        let loop_max = length - period + 1;
        for i in 0..loop_max {
            aroon_ups.push(single::aroon_up(&highs[i..i + period]));
        }
        return aroon_ups;
    }

    /// The `aroon_down` indicator tracks the downtrends in the `aroon_indicator` and is used to
    /// calculate the `aroon_oscillator`.
    ///
    /// The Aroon down is included in the return of the `aroon_indicator`. If the caller wants the
    /// Aroon up, Aroon down, and Aroon oscillator, then it is easier to call the `aroon_indicator`
    /// rather that the three seperately.
    ///
    /// # Arguments
    ///
    /// * `low` - Slice of lows
    /// * `period` - Period over which to calculate the Aroon down
    ///
    /// # Examples
    ///
    /// ```
    /// let lows = vec![98.0, 95.0, 101.0, 100.0, 97.0, 98.0, 97.0];
    /// let period: usize = 5;
    /// let aroon_down = rust_ti::trend_indicators::bulk::aroon_down(&lows, &period);
    /// assert_eq!(vec![25.0, 0.0, 100.0], aroon_down);
    /// ```
    pub fn aroon_down(lows: &[f64], period: &usize) -> Vec<f64> {
        let length = lows.len();
        if &length < period {
            panic!(
                "Period ({}) cannot be longer than length of lows ({})",
                period, length
            )
        };

        let mut aroon_downs = Vec::new();
        let loop_max = length - period + 1;
        for i in 0..loop_max {
            aroon_downs.push(single::aroon_down(&lows[i..i + period]));
        }
        return aroon_downs;
    }

    /// The `aroon_oscillators` takes the difference between the Aroon up and the Aroon down to
    /// give a general sense of trend.
    ///
    /// # Arguments
    ///
    /// * `aroon_up` - Slice of Aroon ups
    /// * `aroon_down` - Slice Aroon downs
    ///
    /// # Examples
    ///
    /// ```
    /// let aroon_up = vec![50.0, 25.0, 0.0];
    /// let aroon_down = vec![25.0, 0.0, 100.0];
    /// let aroon_oscillator = rust_ti::trend_indicators::bulk::aroon_oscillator(&aroon_up,
    /// &aroon_down);
    /// assert_eq!(vec![25.0, 25.0, -100.0], aroon_oscillator);
    /// ```
    pub fn aroon_oscillator(aroon_up: &[f64], aroon_down: &[f64]) -> Vec<f64> {
        let length = aroon_up.len();
        if length != aroon_down.len() {
            panic!(
                "Length of Aroon up ({}) and Aroon down ({}) must match",
                length,
                aroon_down.len()
            )
        };

        let mut aroon_oscillators = Vec::new();
        for i in 0..length {
            aroon_oscillators.push(single::aroon_oscillator(&aroon_up[i], &aroon_down[i]));
        }
        return aroon_oscillators;
    }

    /// The `aroon_indicator` returns the Aroon up, Aroon down, and Aroon oscillator in that order
    ///
    /// # Arguments
    ///
    /// * `high` - Slice of highs
    /// * `low` - Slice of lows
    /// * `period` - Period over which to calculate the Aroon indicator
    ///
    /// # Examples
    ///
    /// ```
    /// let highs = vec![103.0, 102.0, 107.0, 104.0, 100.0, 102.0, 99.0];
    /// let lows = vec![98.0, 95.0, 101.0, 100.0, 97.0, 98.0, 97.0];
    /// let period: usize = 5;
    /// let aroon_indicator = rust_ti::trend_indicators::bulk::aroon_indicator(&highs, &lows,
    /// &period);
    /// assert_eq!(vec![(50.0, 25.0, 25.0), (25.0, 0.0, 25.0), (0.0, 100.0, -100.0)], aroon_indicator);
    /// ```
    pub fn aroon_indicator(highs: &[f64], lows: &[f64], period: &usize) -> Vec<(f64, f64, f64)> {
        let length = highs.len();
        if length != lows.len() {
            panic!(
                "Length of highs ({}) must match length of lows ({})",
                highs.len(),
                lows.len()
            )
        };
        if &length < period {
            panic!(
                "Period ({}) cannot be longer than lengths of highs and lows ({})",
                period, length
            )
        };

        let mut aroon_indicators = Vec::new();
        let loop_max = length - period + 1;
        for i in 0..loop_max {
            aroon_indicators.push(single::aroon_indicator(
                &highs[i..i + period],
                &lows[i..i + period],
            ));
        }
        return aroon_indicators;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_aroon_up() {
        let highs = vec![101.26, 102.57, 102.32, 100.69];
        assert_eq!(33.33333333333333, single::aroon_up(&highs));
    }

    #[test]
    #[should_panic]
    fn singe_aroon_up_panic() {
        let highs = Vec::new();
        single::aroon_up(&highs);
    }

    #[test]
    fn bulk_aroon_up() {
        let highs = vec![101.26, 102.57, 102.32, 100.69, 100.83, 101.73, 102.01];
        assert_eq!(
            vec![33.33333333333333, 0.0, 0.0, 100.0],
            bulk::aroon_up(&highs, &4)
        );
    }

    #[test]
    #[should_panic]
    fn bulk_aroon_up_panic() {
        let highs = vec![101.26, 102.57, 102.32, 100.69, 100.83, 101.73, 102.01];
        bulk::aroon_up(&highs, &40);
    }

    #[test]
    fn single_aroon_down() {
        let lows = vec![100.08, 98.75, 100.14, 98.98];
        assert_eq!(33.33333333333333, single::aroon_down(&lows));
    }

    #[test]
    #[should_panic]
    fn single_aroon_down_panic() {
        let lows = Vec::new();
        single::aroon_down(&lows);
    }

    #[test]
    fn bulk_aroon_down() {
        let lows = vec![100.08, 98.75, 100.14, 98.98, 99.07, 100.1, 99.96];
        assert_eq!(
            vec![33.33333333333333, 0.0, 33.33333333333333, 0.0],
            bulk::aroon_down(&lows, &4)
        );
    }

    #[test]
    #[should_panic]
    fn bulk_aroon_down_panic() {
        let lows = vec![100.08, 98.75, 100.14, 98.98, 99.07, 100.1, 99.96];
        bulk::aroon_down(&lows, &40);
    }

    #[test]
    fn single_aroon_oscillator() {
        assert_eq!(
            0.0,
            single::aroon_oscillator(&33.33333333333333, &33.33333333333333)
        );
    }

    #[test]
    fn bulk_aroon_oscillator() {
        let aroon_up = vec![33.33333333333333, 0.0, 0.0, 100.0];
        let aroon_down = vec![33.33333333333333, 0.0, 33.33333333333333, 0.0];
        assert_eq!(
            vec![0.0, 0.0, -33.33333333333333, 100.0],
            bulk::aroon_oscillator(&aroon_up, &aroon_down)
        );
    }

    #[test]
    #[should_panic]
    fn bulk_aroon_oscillator_up_panic() {
        let aroon_up = vec![33.33333333333333, 0.0, 0.0];
        let aroon_down = vec![33.33333333333333, 0.0, 33.33333333333333, 0.0];
        bulk::aroon_oscillator(&aroon_up, &aroon_down);
    }

    #[test]
    #[should_panic]
    fn bulk_aroon_oscillator_down_panic() {
        let aroon_up = vec![33.33333333333333, 0.0, 0.0, 100.0];
        let aroon_down = vec![33.33333333333333, 0.0, 33.33333333333333];
        bulk::aroon_oscillator(&aroon_up, &aroon_down);
    }

    #[test]
    fn single_aroon_indicator() {
        let lows = vec![100.08, 98.75, 100.14, 98.98];
        let highs = vec![101.26, 102.57, 102.32, 100.69];
        assert_eq!(
            (33.33333333333333, 33.33333333333333, 0.0),
            single::aroon_indicator(&highs, &lows)
        );
    }

    #[test]
    #[should_panic]
    fn single_aroon_indicator_high_panic() {
        let lows = vec![100.08, 98.75, 100.14, 98.98];
        let highs = vec![101.26, 102.57, 102.32];
        single::aroon_indicator(&highs, &lows);
    }

    #[test]
    #[should_panic]
    fn single_aroon_indicator_low_panic() {
        let lows = vec![100.08, 98.75, 100.14];
        let highs = vec![101.26, 102.57, 102.32, 100.69];
        single::aroon_indicator(&highs, &lows);
    }

    #[test]
    fn bulk_aroon_indicator() {
        let highs = vec![101.26, 102.57, 102.32, 100.69, 100.83, 101.73, 102.01];
        let lows = vec![100.08, 98.75, 100.14, 98.98, 99.07, 100.1, 99.96];
        assert_eq!(
            vec![
                (33.33333333333333, 33.33333333333333, 0.0),
                (0.0, 0.0, 0.0),
                (0.0, 33.33333333333333, -33.33333333333333),
                (100.0, 0.0, 100.0)
            ],
            bulk::aroon_indicator(&highs, &lows, &4)
        );
    }

    #[test]
    #[should_panic]
    fn bulk_aroon_indicator_high_panic() {
        let highs = vec![102.57, 102.32, 100.69, 100.83, 101.73, 102.01];
        let lows = vec![100.08, 98.75, 100.14, 98.98, 99.07, 100.1, 99.96];
        bulk::aroon_indicator(&highs, &lows, &4);
    }

    #[test]
    #[should_panic]
    fn bulk_aroon_indicator_low_panic() {
        let highs = vec![101.26, 102.57, 102.32, 100.69, 100.83, 101.73, 102.01];
        let lows = vec![98.75, 100.14, 98.98, 99.07, 100.1, 99.96];
        bulk::aroon_indicator(&highs, &lows, &4);
    }

    #[test]
    #[should_panic]
    fn bulk_aroon_indicator_period_panic() {
        let highs = vec![101.26, 102.57, 102.32, 100.69, 100.83, 101.73, 102.01];
        let lows = vec![100.08, 98.75, 100.14, 98.98, 99.07, 100.1, 99.96];
        bulk::aroon_indicator(&highs, &lows, &40);
    }
}
