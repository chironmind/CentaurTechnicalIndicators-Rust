/// What central value to use for calculations.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum CentralPoint {
    Mean,
    Median,
    Mode,
}

// TODO: These comments will need to be reversed
/// Type of moving average.
#[derive(Debug, Copy, Clone, PartialEq)]
//pub enum MovingAverageType {
pub enum MovingAverageType<'a> {
    Simple,
    Smoothed,
    Exponential,
    //Personalised { alpha_num: f64, alpha_den: f64 },
    Personalised(&'a f64, &'a f64),
}

/// Determines which constant model to use for a center point.
#[derive(Debug, Copy, Clone, PartialEq)]
//pub enum ConstantModelType {
pub enum ConstantModelType<'a> {
    SimpleMovingAverage,
    SmoothedMovingAverage,
    ExponentialMovingAverage,
    //PersonalisedMovingAverage { alpha_num: f64, alpha_den: f64 },
    PersonalisedMovingAverage(&'a f64, &'a f64),
    SimpleMovingMedian,
    SimpleMovingMode,
}

/// How to measure deviation from a center point.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum DeviationModel {
    StandardDeviation,
    MeanAbsoluteDeviation,
    MedianAbsoluteDeviation,
    ModeAbsoluteDeviation,
    UlcerIndex,
}

/// Trade position.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Position {
    Short,
    Long,
}
