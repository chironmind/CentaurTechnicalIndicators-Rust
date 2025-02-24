//! # Chart Trends
//!
//! The `chart_trends` module intention is to be used to show the trends on a chart.
//!
//! The points returned from the various functions should be plotted on an OHLC chart.
//!
//! All the functions that end in `_trend` run  a linear regression on specific points of the charts.
//! So `peak_trends` runs a linear regression to determine the best fit line for all peaks.

use crate::basic_indicators::single::{max, mean, min};

/// The `peaks` function returns all peaks over a period, and returns a vector of tuples where the
/// first item is the peak value, and the second it the peak index.
///
/// The caller provides a period with the slice of prices that is used to find the peak for
/// that period. The period should be at least the length of the slice of prices.
///
/// If the peak is the same over multiple periods it will only return the peak, index tuple once (see first
/// example).
///
/// If there are multiple peaks of the same value in the same period the function will only
/// return the latest peak (see last example)
///
/// # Arguments
///
/// * `prices` - Slice of prices
/// * `period` - Period over which to find the peak
///
/// # Panics
///
/// `peaks` will panic if `period` is greater than length of `prices`
///
/// # Examples
///
/// ```rust
/// let highs = vec![103.0, 102.0, 107.0, 104.0, 100.0];
/// let period: usize = 3;
/// let peaks = rust_ti::chart_trends::peaks(&highs, &period);
/// assert_eq!(vec![(107.0, 2)], peaks);
///
/// let highs = vec![103.0, 102.0, 107.0, 104.0, 100.0, 109.0];
/// let period: usize = 3;
/// let peaks = rust_ti::chart_trends::peaks(&highs, &period);
/// assert_eq!(vec![(107.0, 2), (109.0, 5)], peaks);
///
/// let highs = vec![103.0, 102.0, 107.0, 104.0, 100.0, 109.0];
/// let period: usize = 6;
/// let peaks = rust_ti::chart_trends::peaks(&highs, &period);
/// assert_eq!(vec![(109.0, 5)], peaks);
///
/// let highs = vec![103.0, 102.0, 107.0, 104.0, 100.0, 107.0];
/// let period: usize = 3;
/// let peaks = rust_ti::chart_trends::peaks(&highs, &period);
/// assert_eq!(vec![(107.0, 2), (107.0, 5)], peaks);
///
/// let highs = vec![103.0, 102.0, 107.0, 104.0, 100.0, 107.0];
/// let period: usize = 6;
/// let peaks = rust_ti::chart_trends::peaks(&highs, &period);
/// assert_eq!(vec![(107.0, 5)], peaks);
/// ```
pub fn peaks(prices: &[f64], period: &usize) -> Vec<(f64, usize)> {
    let length = prices.len();
    if period > &length {
        panic!(
            "Period ({}) cannot be longer than length of prices ({})",
            period, length
        )
    };

    let mut peaks = Vec::new();
    let loop_max = length - period + 1;
    for i in 0..loop_max {
        let peak = max(&prices[i..i + period]);
        let index = &prices[i..i + period]
            .iter()
            .rposition(|&x| x == peak)
            .unwrap();
        let peak_tuple = (peak, index + i);
        if !peaks.contains(&peak_tuple) {
            peaks.push(peak_tuple)
        };
    }
    return peaks;
}

/// The `valleys` function returns all valleys over a period, and returns a vector of tuples where the
/// first item is the valley value, and the second it the valley index.
///
/// The caller provides a period with the slice of prices that is used to find the valley for
/// that period. The period should be at least the length of the slice of prices.
///
/// If the valley is the same over multiple periods it will only return the peak, index tuple once (see first
/// example).
///
/// If there are multiple valleys of the same value in the same period the function will only
/// return the latest valley (see last example)
///
/// # Arguments
///
/// * `prices` - Slice of prices
/// * `period` - Period over which to find the valley
///
/// # Panics
///
/// `valleys` will panic if `period` is greater than length of `prices`
///
/// # Examples
///
/// ```rust
/// let lows = vec![98.0, 101.0, 95.0, 100.0, 97.0];
/// let period: usize = 3;
/// let valleys = rust_ti::chart_trends::valleys(&lows, &period);
/// let period: usize = 3;
/// let valleys = rust_ti::chart_trends::valleys(&lows, &period);
/// assert_eq!(vec![(95.0, 2)], valleys);
///
/// let lows = vec![98.0, 101.0, 95.0, 100.0, 97.0, 93.0];
/// let period: usize = 3;
/// let valleys = rust_ti::chart_trends::valleys(&lows, &period);
/// assert_eq!(vec![(95.0, 2), (93.0, 5)], valleys);
///
/// let lows = vec![98.0, 101.0, 95.0, 100.0, 97.0, 93.0];
/// let period: usize = 6;
/// let valleys = rust_ti::chart_trends::valleys(&lows, &period);
/// assert_eq!(vec![(93.0, 5)], valleys);
///
/// let lows = vec![98.0, 101.0, 95.0, 100.0, 97.0, 95.0];
/// let period: usize = 3;
/// let valleys = rust_ti::chart_trends::valleys(&lows, &period);
/// assert_eq!(vec![(95.0, 2), (95.0, 5)], valleys);
///
/// let lows = vec![98.0, 101.0, 95.0, 100.0, 97.0, 95.0];
/// let period: usize = 6;
/// let valleys = rust_ti::chart_trends::valleys(&lows, &period);
/// assert_eq!(vec![(95.0, 5)], valleys);
/// ```
pub fn valleys(prices: &[f64], period: &usize) -> Vec<(f64, usize)> {
    let length = prices.len();
    if period > &length {
        panic!(
            "Period ({}) cannot be longer than length of prices ({})",
            period, length
        )
    };

    let mut valleys = Vec::new();
    let loop_max = length - period + 1;
    for i in 0..loop_max {
        let valley = min(&prices[i..i + period]);
        let index = &prices[i..i + period]
            .iter()
            .rposition(|&x| x == valley)
            .unwrap();
        let valley_tuple = (valley, index + i);
        if !valleys.contains(&valley_tuple) {
            valleys.push(valley_tuple)
        };
    }
    return valleys;
}

/// Linear regression function
fn get_trend_line(p: Vec<(f64, usize)>) -> (f64, f64) {
    let length = p.len() as f64;
    let mut sum_x: f64 = 0.0;
    let mut sum_y: f64 = 0.0;
    let mut sum_xy: f64 = 0.0;
    let mut sum_x_sq: f64 = 0.0;
    p.iter().for_each(|x| {
        sum_x = sum_x + x.1 as f64;
        sum_y = sum_y + x.0;
        sum_xy = sum_xy + (x.0 * x.1 as f64);
        sum_x_sq = sum_x_sq + x.1.pow(2) as f64
    });
    let slope = ((length * sum_xy) - (sum_x * sum_y)) / ((length * sum_x_sq) - (sum_x.powi(2)));
    let intercept = (sum_y - (slope * sum_x)) / length;
    return (slope, intercept);
}

/// The `peak_trend` function gets the peaks for a slice of prices over a given period and calculates
/// the slope and intercept and returns them as a tuple. It is essentially a linear regression on peaks.
///
/// The caller provides a period with the slice of prices that is used to find the peak for
/// that period.
///
/// # Arguments
///
/// * `prices` - Slice of prices
/// * `period` - Period over which to calculate the peaks
///
/// # Examples
///
/// ```
/// let highs = vec![103.0, 102.0, 107.0, 104.0, 100.0, 109.0];
/// let period: usize = 3;
/// let peak_trend = rust_ti::chart_trends::peak_trend(&highs, &period);
/// assert_eq!((0.6666666666666666, 105.66666666666667), peak_trend);
/// ```
pub fn peak_trend(prices: &[f64], period: &usize) -> (f64, f64) {
    let peaks = peaks(prices, period);
    return get_trend_line(peaks);
}

/// The `valley_trend` function gets the valleys for a slice of prices over a given period
/// and calculates the slope and intercept and returns them as a tuple. It is essentially a
/// linear regression of valleys.
///
/// The caller provides a period with the slice of prices that is used to find the valley for
/// that period.
///
/// # Arguments
///
/// * `prices` - Slice of prices
/// * `period` - Period over which to calculate the valleys
///
/// # Examples
///
/// ```
/// let lows = vec![98.0, 101.0, 95.0, 100.0, 97.0, 93.0];
/// let period: usize = 3;
/// let valley_trend = rust_ti::chart_trends::valley_trend(&lows, &period);
/// assert_eq!((-0.6666666666666666, 96.33333333333333), valley_trend);
/// ```
pub fn valley_trend(prices: &[f64], period: &usize) -> (f64, f64) {
    let valleys = valleys(prices, period);
    return get_trend_line(valleys);
}

/// The `overall_trend` function calculates the slope and intercept from a slice of prices and
/// returns them as a tuple. This essentially a linear regression of all prices passed in.
///
/// The caller provides a period with the slice of prices that is used to find the valley for
/// that period.
///
/// # Arguments
///
/// * `prices` - Slice of prices
///
/// # Examples
///
/// ```
/// let prices = vec![100.0, 102.0, 103.0, 101.0, 100.0];
/// let overall_trend = rust_ti::chart_trends::overall_trend(&prices);
/// assert_eq!((-0.1, 101.4), overall_trend);
/// ```
pub fn overall_trend(prices: &[f64]) -> (f64, f64) {
    let mut indexed_prices = Vec::new();
    for i in 0..prices.len() {
        indexed_prices.push((prices[i], i));
    }
    return get_trend_line(indexed_prices);
}

/// The `break_down_trends` function breaks down the different trends in a slice of prices. It
/// returns a tuple with the index where the trend starts, an index of where the trend ends, the
/// slope, and intercept.
///
/// To determine a new trend, the function runs a linear regression to get the slope and intercept 
/// for a given set of prices, it then adds new prices until r squared, standard
/// error or adjusted chi squared, exceed passed in limits. At that point it assumes that it is a new period.
///
/// The soft limits are limits that all needs to be passed in order to create a new trend, the hard
/// limits only need to be passed by one variable to create a new trend.
///
/// # Arguments
///
/// * `prices` - Slice of prices
/// * `max_outliers` - The maximum number of times the price can exceed limits without creating a
/// new trend. To be thought of as a protection against flash crashes creating new trends.
/// * `soft_r_squared_minimum` - The soft minimum value for r squared
/// * `soft_r_squared_maximum` - The soft maximum value for r squared
/// * `hard_r_squared_minimum` - The hard minimum value for r squared
/// * `hard_r_squared_maximum` - The hard maximum value for r squared
/// * `soft_standard_error_multiplier` - The soft multiplier for the standard error
/// * `hard_standard_error_multiplier` - The hard multiplier for the standard error
/// * `soft_reduced_chi_squared_multiplier` - The soft multiplier for the reduced chi squared
/// * `hard_reduced_chi_squared_multiplier` - The hard multiplier for the reduced chi squared
///
/// # Panics
///
/// `break_down_trends` will panic if `prices` is empty
///
///
/// # Examples
///
/// ```
/// let prices = vec![100.0, 102.0, 103.0, 101.0, 99.0, 99.0, 102.0, 103.0, 106.0, 107.0, 105.0,
/// 104.0, 101.0, 97.0, 100.0];
/// let max_outliers = 1;
/// let soft_r_squared_minimum = 0.75;
/// let soft_r_squared_maximum = 1.0;
/// let hard_r_squared_minimum = 0.5;
/// let hard_r_squared_maximum = 1.5;
/// let soft_standard_error_multiplier = 2.0;
/// let hard_standard_error_multiplier = 3.0;
/// let soft_reduced_chi_squared_multiplier = 2.0;
/// let hard_reduced_chi_squared_multiplier = 3.0;
/// let trend_break_down = rust_ti::chart_trends::break_down_trends(
///     &prices,
///     &max_outliers,
///     &soft_r_squared_minimum,
///     &soft_r_squared_maximum,
///     &hard_r_squared_minimum,
///     &hard_r_squared_maximum,
///     &soft_standard_error_multiplier,
///     &hard_standard_error_multiplier,
///     &soft_reduced_chi_squared_multiplier,
///     &hard_reduced_chi_squared_multiplier
/// );
/// assert_eq!(vec![
///         (0, 2, 1.5, 100.16666666666667), (2, 4, -2.0, 107.0), 
///         (4, 9, 1.7714285714285714, 91.15238095238095), (9, 11, -1.5, 120.33333333333333), 
///         (11, 13, -3.5, 142.66666666666666)],
///         trend_break_down);
/// ```
pub fn break_down_trends(
    prices: &[f64],
    max_outliers: &usize,
    soft_r_squared_minimum: &f64,
    soft_r_squared_maximum: &f64,
    hard_r_squared_minimum: &f64,
    hard_r_squared_maximum: &f64,
    soft_standard_error_multiplier: &f64,
    hard_standard_error_multiplier: &f64,
    soft_reduced_chi_squared_multiplier: &f64,
    hard_reduced_chi_squared_multiplier: &f64,
) -> Vec<(usize, usize, f64, f64)> {
    if prices.is_empty() {
        panic!("Prices cannot be empty");
    }
    let mut outliers: Vec<usize> = Vec::new();
    let mut trends: Vec<(usize, usize, f64, f64)> = Vec::new();
    let mut current_slope = 0.0;
    let mut current_intercept = 0.0;
    let mut start_index: usize = 0;
    let mut end_index: usize = 1;
    let mut indexed_points: Vec<(f64, usize)> = Vec::new();
    let mut previous_standard_error = 10000.0;
    let mut previous_reduced_chi_squared = 10000.0;
    for (index, price) in prices.iter().enumerate() {
        indexed_points.push((*price, index));

        if index == 0 {
            continue;
        }
        if &index > &end_index {
            let current_trend = get_trend_line(indexed_points.clone());
            let (standard_error, r_squared, reduced_chi_squared) =
                goodness_of_fit(&indexed_points, &current_trend);
            // TODO: Is the regression using OLS?
            if standard_error > soft_standard_error_multiplier * previous_standard_error
                && (&r_squared < soft_r_squared_minimum || &r_squared > soft_r_squared_maximum)
                && reduced_chi_squared
                    > soft_reduced_chi_squared_multiplier * previous_reduced_chi_squared
                || &r_squared < hard_r_squared_minimum
                || &r_squared > hard_r_squared_maximum
                || standard_error > hard_standard_error_multiplier * previous_standard_error
                || reduced_chi_squared
                    > hard_reduced_chi_squared_multiplier * previous_reduced_chi_squared
            {
                if &outliers.len() < max_outliers {
                    outliers.push(index);
                    indexed_points.pop();
                    continue;
                };
                trends.push((start_index, end_index, current_slope, current_intercept));
                start_index = end_index;
                end_index = index;
                indexed_points = (start_index..index + 1).map(|x| (prices[x], x)).collect();
                let current_trend = get_trend_line(indexed_points.clone());
                current_slope = current_trend.0;
                current_intercept = current_trend.1;
                // if list bigger than 2
                if indexed_points.len() > 2 {
                    (previous_standard_error, _, previous_reduced_chi_squared) =
                        goodness_of_fit(&indexed_points, &current_trend);
                } else {
                    previous_standard_error = 10000.0;
                    previous_reduced_chi_squared = 10000.0;
                };
                outliers.clear();
            } else {
                previous_standard_error = standard_error;
                previous_reduced_chi_squared = reduced_chi_squared;
                current_slope = current_trend.0;
                current_intercept = current_trend.1;
                outliers.clear();
            }
        }
        end_index = index;
    }
    trends.push((start_index, end_index, current_slope, current_intercept));
    return trends;
}

fn goodness_of_fit(indexed_points: &Vec<(f64, usize)>, trend: &(f64, f64)) -> (f64, f64, f64) {
    let mut trend_line: Vec<f64> = Vec::new();
    let mut observed_prices: Vec<f64> = Vec::new();

    for i in indexed_points.iter() {
        trend_line.push(trend.1 + (trend.0 * i.1 as f64));
        observed_prices.push(i.0);
    }

    let trend_length = trend_line.len();
    if trend_length != observed_prices.len() {
        panic!(
            "trend line length ({}) and prices length ({}) must be equal!",
            trend_length,
            trend_line.len()
        );
    };
    let mut squares_residuals: Vec<f64> = Vec::new();
    let mut total_squares: Vec<f64> = Vec::new();
    let observed_mean = mean(&observed_prices);
    for i in 0..trend_length {
        let square_residual = (observed_prices[i] - trend_line[i]).powi(2);
        squares_residuals.push(square_residual);
        let square_total = (observed_prices[i] - observed_mean).powi(2);
        total_squares.push(square_total);
    }
    let sum_squares_residuals = squares_residuals.iter().sum::<f64>();
    let ssr_length = squares_residuals.len() as f64;
    let total_sum_squares = total_squares.iter().sum::<f64>();
    let standard_error = (sum_squares_residuals / ssr_length).sqrt();
    let r_squared = 1.0 - (sum_squares_residuals / total_sum_squares);
    let reduced_chi_squared = sum_squares_residuals / ssr_length;
    return (standard_error, r_squared, reduced_chi_squared);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn peaks_single_peak() {
        let highs = vec![101.26, 102.57, 102.32, 100.69];
        assert_eq!(vec![(102.57, 1)], peaks(&highs, &4_usize));
    }

    #[test]
    fn peaks_multiple_peaks() {
        let highs = vec![101.26, 102.57, 102.32, 100.69, 100.83, 101.73, 102.01];
        assert_eq!(
            vec![(102.57, 1), (102.32, 2), (102.01, 6)],
            peaks(&highs, &4_usize)
        );
    }

    #[test]
    fn peaks_multiple_peaks_same_period() {
        let highs = vec![101.26, 102.57, 102.57, 100.69, 100.83, 101.73, 102.01];
        assert_eq!(vec![(102.57, 2), (102.01, 6)], peaks(&highs, &4_usize));
    }

    #[test]
    #[should_panic]
    fn peaks_panic() {
        let highs = vec![101.26, 102.57, 102.57, 100.69, 100.83, 101.73, 102.01];
        peaks(&highs, &40_usize);
    }

    #[test]
    fn valleys_single_valley() {
        let lows = vec![100.08, 98.75, 100.14, 98.98, 99.07, 100.1, 99.96];
        assert_eq!(vec![(98.75, 1)], valleys(&lows, &7_usize));
    }

    #[test]
    fn valleys_multiple_valleys() {
        let lows = vec![100.08, 98.75, 100.14, 98.98, 99.07, 100.1, 99.96];
        assert_eq!(vec![(98.75, 1), (98.98, 3)], valleys(&lows, &4_usize));
    }

    #[test]
    fn valleys_multiple_valleys_same_period() {
        let lows = vec![98.75, 98.75, 100.14, 98.98, 99.07, 100.1, 99.96];
        assert_eq!(vec![(98.75, 1), (98.98, 3)], valleys(&lows, &4_usize));
    }

    #[test]
    #[should_panic]
    fn valleys_panic() {
        let lows = vec![98.75, 98.75, 100.14, 98.98, 99.07, 100.1, 99.96];
        valleys(&lows, &40_usize);
    }

    #[test]
    fn peaks_trend() {
        let highs = vec![101.26, 102.57, 102.32, 100.69, 100.83, 101.73, 102.01];
        assert_eq!(
            (-0.10214285714285627, 102.60642857142857),
            peak_trend(&highs, &4_usize)
        );
    }

    #[test]
    fn valleys_trend() {
        let lows = vec![100.08, 98.75, 100.14, 98.98, 99.07, 100.1, 99.96];
        assert_eq!(
            (0.11499999999998067, 98.63500000000005),
            valley_trend(&lows, &4_usize)
        );
    }

    #[test]
    fn overall_trends() {
        let prices = vec![100.2, 100.46, 100.53, 100.38, 100.19];
        assert_eq!(
            (-0.01000000000001819, 100.37200000000004),
            overall_trend(&prices)
        );
    }

    #[test]
    fn break_down_trends_std_dev() {
        let prices = vec![100.2, 100.46, 100.53, 100.38, 100.19];
        let max_outliers = 1;
        let soft_r_squared_minimum = 0.75;
        let soft_r_squared_maximum = 1.0;
        let hard_r_squared_minimum = 0.5;
        let hard_r_squared_maximum = 1.5;
        let soft_standard_error_multiplier = 2.0;
        let hard_standard_error_multiplier = 3.0;
        let soft_reduced_chi_squared_multiplier = 2.0;
        let hard_reduced_chi_squared_multiplier = 3.0;
        let trend_break_down = break_down_trends(
            &prices,
            &max_outliers,
            &soft_r_squared_minimum,
            &soft_r_squared_maximum,
            &hard_r_squared_minimum,
            &hard_r_squared_maximum,
            &soft_standard_error_multiplier,
            &hard_standard_error_multiplier,
            &soft_reduced_chi_squared_multiplier,
            &hard_reduced_chi_squared_multiplier,
        );
        assert_eq!(
            vec![
                (0, 2, 0.1650000000000015, 100.23166666666667),
                (2, 4, -0.16999999999999696, 100.87666666666667)
            ],
            trend_break_down
        );
    }
}
