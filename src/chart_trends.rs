//! # Chart Trends
//!
//! The `chart_trends` module intention is to be used to show the trends on a chart.
//!
//! The points returned from the various functions should be plotted on an OHLC chart.

use crate::basic_indicators::single::{absolute_deviation, max, min, standard_deviation};
use crate::volatility_indicators::single::ulcer_index;
use crate::DeviationModel;

/// The `get_peaks` function returns all peaks over a period, and returns a vector of tuples where the
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
/// # Examples
///
/// ```
/// let highs = vec![103.0, 102.0, 107.0, 104.0, 100.0];
/// let period: usize = 3;
/// let peaks = rust_ti::chart_trends::get_peaks(&highs, &period);
/// assert_eq!(vec![(107.0, 2)], peaks);
///
/// let highs = vec![103.0, 102.0, 107.0, 104.0, 100.0, 109.0];
/// let period: usize = 3;
/// let peaks = rust_ti::chart_trends::get_peaks(&highs, &period);
/// assert_eq!(vec![(107.0, 2), (109.0, 5)], peaks);
///
/// let highs = vec![103.0, 102.0, 107.0, 104.0, 100.0, 109.0];
/// let period: usize = 6;
/// let peaks = rust_ti::chart_trends::get_peaks(&highs, &period);
/// assert_eq!(vec![(109.0, 5)], peaks);
///
/// let highs = vec![103.0, 102.0, 107.0, 104.0, 100.0, 107.0];
/// let period: usize = 3;
/// let peaks = rust_ti::chart_trends::get_peaks(&highs, &period);
/// assert_eq!(vec![(107.0, 2), (107.0, 5)], peaks);
///
/// let highs = vec![103.0, 102.0, 107.0, 104.0, 100.0, 107.0];
/// let period: usize = 6;
/// let peaks = rust_ti::chart_trends::get_peaks(&highs, &period);
/// assert_eq!(vec![(107.0, 5)], peaks);
/// ```
pub fn get_peaks(prices: &[f64], period: &usize) -> Vec<(f64, usize)> {
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

/// The `get_valleys` function returns all valleys over a period, and returns a vector of tuples where the
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
/// # Examples
///
/// ```
/// let lows = vec![98.0, 101.0, 95.0, 100.0, 97.0];
/// let period: usize = 3;
/// let valleys = rust_ti::chart_trends::get_valleys(&lows, &period);
/// let period: usize = 3;
/// let valleys = rust_ti::chart_trends::get_valleys(&lows, &period);
/// assert_eq!(vec![(95.0, 2)], valleys);
///
/// let lows = vec![98.0, 101.0, 95.0, 100.0, 97.0, 93.0];
/// let period: usize = 3;
/// let valleys = rust_ti::chart_trends::get_valleys(&lows, &period);
/// assert_eq!(vec![(95.0, 2), (93.0, 5)], valleys);
///
/// let lows = vec![98.0, 101.0, 95.0, 100.0, 97.0, 93.0];
/// let period: usize = 6;
/// let valleys = rust_ti::chart_trends::get_valleys(&lows, &period);
/// assert_eq!(vec![(93.0, 5)], valleys);
///
/// let lows = vec![98.0, 101.0, 95.0, 100.0, 97.0, 95.0];
/// let period: usize = 3;
/// let valleys = rust_ti::chart_trends::get_valleys(&lows, &period);
/// assert_eq!(vec![(95.0, 2), (95.0, 5)], valleys);
///
/// let lows = vec![98.0, 101.0, 95.0, 100.0, 97.0, 95.0];
/// let period: usize = 6;
/// let valleys = rust_ti::chart_trends::get_valleys(&lows, &period);
/// assert_eq!(vec![(95.0, 5)], valleys);
/// ```
pub fn get_valleys(prices: &[f64], period: &usize) -> Vec<(f64, usize)> {
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

/// The `get_peak_trend` function gets the peaks for a slice of prices and calculates the the slope and intercept
/// and returns them as a tuple.
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
/// let peak_trend = rust_ti::chart_trends::get_peak_trend(&highs, &period);
/// assert_eq!((0.6666666666666666, 105.66666666666667), peak_trend);
/// ```
pub fn get_peak_trend(prices: &[f64], period: &usize) -> (f64, f64) {
    let peaks = get_peaks(prices, period);
    return get_trend_line(peaks);
}

/// The `get_valley_trend` function gets the valleys for a slice of prices and calculates the slope and intercept
/// and returns them as a tuple.
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
/// ```
pub fn get_valley_trend(prices: &[f64], period: &usize) -> (f64, f64) {
    let valleys = get_valleys(prices, period);
    return get_trend_line(valleys);
}

/// The `get_overall_trend` function calculates the slope and intercept from a slice of prices and
/// returns them as a tuple.
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
/// let overall_trend = rust_ti::chart_trends::get_overall_trend(&prices);
/// assert_eq!((-0.1, 101.4), overall_trend);
/// ```
pub fn get_overall_trend(prices: &[f64]) -> (f64, f64) {
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
/// To determine a new trend, the function calculates the trend for a slice of prices, at t+1 it
/// calculates the next point of the trend line using the slope and intercept, if the price at t+1
/// exceeds the trend line point at t+1 a new trend is created.
///
/// The amount by which the price at t+1 can deviate from the trend line point at t+1 is determine
/// by the caller when they pass in the standard deviation multiplier and the sensitivity denominator.
///
/// The standard deviation multiplier multiplies the standard deviation of the trend line to create
/// an upper and lower limit value, if the price at t+1 is above or below those value then a new
/// trend is created.
///
/// The sensitivty denominator slightly adjusts the standard deviation multiplier by adding
/// `(1 / sensitivity denomintor)^(trend line length)`. The idea is as the when there are few points
/// in the trend line, the sensitivity multiplier will be higher, stopping the function from
/// creating new trends for outliers at the beginning of the trend. The bigger the trend line gets
/// the smaller the value gets.
///
/// # Arguments
///
/// * `prices` - Slice of prices
/// * `standard_deviation_multiplier` - Multiplier for the standard deviation to create a band that
/// determines a new trend. Use 2.0 as a default.
/// * `sensitivity_multiplier` - A time based adjustment for the standard deviation, to stop the
/// function from constantly creating new trends. Use 2.0 as a default.
/// * `deviation_model` - A variant of the DeviationModel enum
///
/// # Examples
///
/// ```
/// let prices = vec![100.0, 102.0, 103.0, 101.0, 99.0, 99.0, 102.0, 103.0, 106.0, 107.0, 105.0,
/// 104.0, 101.0, 97.0, 100.0];
/// let standard_deviation_multiplier = 2.0;
/// let sensitivity_multiplier = 2.0;
/// let trend_break_down = rust_ti::chart_trends::break_down_trends(&prices,
/// &standard_deviation_multiplier, &sensitivity_multiplier,
/// &rust_ti::DeviationModel::StandardDeviation);
/// assert_eq!(vec![
///         (0, 2, 1.5, 100.16666666666667),
///         (2, 5, -1.4, 105.4),
///         (5, 11, 0.8928571428571429, 96.57142857142857),
///         (11, 14, -1.6, 120.5)],
///         trend_break_down);
///
/// let ulcer_index_trend_break_down = rust_ti::chart_trends::break_down_trends(&prices,
/// &standard_deviation_multiplier, &sensitivity_multiplier,
/// &rust_ti::DeviationModel::UlcerIndex);
/// assert_eq!(vec![
///         (0, 1, 2.0, 100.0),
///         (1, 2, 1.0, 101.0),
///         (2, 6, -0.4, 102.4),
///         (6, 7, 1.0, 96.0),
///         (7, 8, 3.0, 82.0),
///         (8, 9, 1.0, 98.0),
///         (9, 14, -1.7714285714285714, 122.7047619047619)],
///         ulcer_index_trend_break_down);
/// ```
pub fn break_down_trends(
    prices: &[f64],
    standard_deviation_multiplier: &f64,
    sensitivity_multiplier: &f64,
    deviation_model: &crate::DeviationModel,
) -> Vec<(usize, usize, f64, f64)> {
    if prices.is_empty() {
        panic!("Prices cannot be empty");
    }
    let mut trends: Vec<(usize, usize, f64, f64)> = Vec::new();
    let mut current_slope = 0.0;
    let mut current_intercept = 0.0;
    let mut start_index: usize = 0;
    let mut end_index: usize = 1;
    for (index, price) in prices.iter().enumerate() {
        if index == 0 {
            continue;
        };
        if &index > &end_index {
            let trend_line: Vec<f64> = (start_index..index + 1)
                .map(|x| &current_intercept + (&current_slope * x as f64))
                .collect();
            let trend_length = trend_line.len() as i32;
            let adjusted_multiplier =
                standard_deviation_multiplier + ((1.0 / sensitivity_multiplier).powi(trend_length));
            let trend_deviation = match deviation_model {
                DeviationModel::StandardDeviation => standard_deviation(&trend_line),
                DeviationModel::MeanAbsoluteDeviation => {
                    absolute_deviation(&trend_line, &crate::CentralPoint::Mean)
                }
                DeviationModel::MedianAbsoluteDeviation => {
                    absolute_deviation(&trend_line, &crate::CentralPoint::Median)
                }
                DeviationModel::ModeAbsoluteDeviation => {
                    absolute_deviation(&trend_line, &crate::CentralPoint::Mode)
                }
                DeviationModel::UlcerIndex => ulcer_index(&trend_line),
                _ => panic!("Unsupported DeviationModel"),
            };
            let deviation_multiplied = trend_deviation * adjusted_multiplier;
            let upper_band = trend_line.last().unwrap() + deviation_multiplied;
            let lower_band = trend_line.last().unwrap() - deviation_multiplied;
            if price > &upper_band || price < &lower_band {
                trends.push((start_index, end_index, current_slope, current_intercept));
                start_index = index - 1;
                end_index = index;
            }
        }
        let indexed_points = (start_index..index + 1).map(|x| (prices[x], x)).collect();
        let current_trend = get_trend_line(indexed_points);
        current_slope = current_trend.0;
        current_intercept = current_trend.1;
        end_index = index;
    }
    trends.push((start_index, end_index, current_slope, current_intercept));
    return trends;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn peaks_single_peak() {
        let highs = vec![101.26, 102.57, 102.32, 100.69];
        assert_eq!(vec![(102.57, 1)], get_peaks(&highs, &4_usize));
    }

    #[test]
    fn peaks_multiple_peaks() {
        let highs = vec![101.26, 102.57, 102.32, 100.69, 100.83, 101.73, 102.01];
        assert_eq!(
            vec![(102.57, 1), (102.32, 2), (102.01, 6)],
            get_peaks(&highs, &4_usize)
        );
    }

    #[test]
    fn peaks_multiple_peaks_same_period() {
        let highs = vec![101.26, 102.57, 102.57, 100.69, 100.83, 101.73, 102.01];
        assert_eq!(vec![(102.57, 2), (102.01, 6)], get_peaks(&highs, &4_usize));
    }

    #[test]
    #[should_panic]
    fn peaks_panic() {
        let highs = vec![101.26, 102.57, 102.57, 100.69, 100.83, 101.73, 102.01];
        get_peaks(&highs, &40_usize);
    }

    #[test]
    fn valleys_single_valley() {
        let lows = vec![100.08, 98.75, 100.14, 98.98, 99.07, 100.1, 99.96];
        assert_eq!(vec![(98.75, 1)], get_valleys(&lows, &7_usize));
    }

    #[test]
    fn valleys_multiple_valleys() {
        let lows = vec![100.08, 98.75, 100.14, 98.98, 99.07, 100.1, 99.96];
        assert_eq!(vec![(98.75, 1), (98.98, 3)], get_valleys(&lows, &4_usize));
    }

    #[test]
    fn valleys_multiple_valleys_same_period() {
        let lows = vec![98.75, 98.75, 100.14, 98.98, 99.07, 100.1, 99.96];
        assert_eq!(vec![(98.75, 1), (98.98, 3)], get_valleys(&lows, &4_usize));
    }

    #[test]
    #[should_panic]
    fn valleys_panic() {
        let lows = vec![98.75, 98.75, 100.14, 98.98, 99.07, 100.1, 99.96];
        get_valleys(&lows, &40_usize);
    }

    #[test]
    fn peak_trend() {
        let highs = vec![101.26, 102.57, 102.32, 100.69, 100.83, 101.73, 102.01];
        assert_eq!(
            (-0.10214285714285627, 102.60642857142857),
            get_peak_trend(&highs, &4_usize)
        );
    }

    #[test]
    fn valley_trend() {
        let lows = vec![100.08, 98.75, 100.14, 98.98, 99.07, 100.1, 99.96];
        assert_eq!(
            (0.11499999999998067, 98.63500000000005),
            get_valley_trend(&lows, &4_usize)
        );
    }

    #[test]
    fn overall_trend() {
        let prices = vec![100.2, 100.46, 100.53, 100.38, 100.19];
        assert_eq!(
            (-0.01000000000001819, 100.37200000000004),
            get_overall_trend(&prices)
        );
    }

    #[test]
    fn break_down_trends_std_dev() {
        let prices = vec![100.2, 100.46, 100.53, 100.38, 100.19];
        assert_eq!(
            vec![
                (0, 3, 0.06099999999998999, 100.30100000000002),
                (3, 4, -0.19000000000005457, 100.95000000000019)
            ],
            break_down_trends(
                &prices,
                &2.0,
                &1.0,
                &crate::DeviationModel::StandardDeviation
            )
        );
    }

    #[test]
    fn break_down_trends_mean_abs_dev() {
        let prices = vec![100.2, 100.46, 100.53, 100.38, 100.19];
        assert_eq!(
            vec![
                (0, 3, 0.06099999999998999, 100.30100000000002),
                (3, 4, -0.19000000000005457, 100.95000000000019)
            ],
            break_down_trends(
                &prices,
                &2.0,
                &1.0,
                &crate::DeviationModel::MeanAbsoluteDeviation
            )
        );
    }

    #[test]
    fn break_down_trends_median_abs_dev() {
        let prices = vec![100.2, 100.46, 100.53, 100.38, 100.19];
        assert_eq!(
            vec![
                (0, 3, 0.06099999999998999, 100.30100000000002),
                (3, 4, -0.19000000000005457, 100.95000000000019)
            ],
            break_down_trends(
                &prices,
                &2.0,
                &1.0,
                &crate::DeviationModel::MedianAbsoluteDeviation
            )
        );
    }

    #[test]
    fn break_down_trends_mode_abs_dev() {
        let prices = vec![100.2, 100.46, 100.53, 100.38, 100.19];
        assert_eq!(
            vec![(0, 4, -0.01000000000001819, 100.37200000000004)],
            break_down_trends(
                &prices,
                &2.0,
                &1.0,
                &crate::DeviationModel::ModeAbsoluteDeviation
            )
        );
    }

    #[test]
    fn break_down_trends_ulcer_index() {
        let prices = vec![100.2, 100.46, 100.53, 100.38, 100.19];
        assert_eq!(
            vec![
                (0, 1, 0.2599999999999909, 100.2),
                (1, 2, 0.06999999999993634, 100.3900000000001),
                (2, 4, -0.16999999999999696, 100.87666666666667)
            ],
            break_down_trends(&prices, &2.0, &1.0, &crate::DeviationModel::UlcerIndex)
        );
    }

    #[test]
    #[should_panic]
    fn break_down_trends_panic() {
        let prices = Vec::new();
        break_down_trends(&prices, &2.0, &1.0, &crate::DeviationModel::UlcerIndex);
    }
}
