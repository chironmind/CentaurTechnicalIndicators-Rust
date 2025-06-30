/// What central value to use for calculations.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum CentralPoint {
    Mean,
    Median,
    Mode,
}

/// Type of moving average.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum MovingAverageType {
    Simple,
    Smoothed,
    Exponential,
    Personalised { alpha_num: f64, alpha_den: f64 },
}

/// Determines which constant model to use for a center point.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ConstantModelType {
    SimpleMovingAverage,
    SmoothedMovingAverage,
    ExponentialMovingAverage,
    PersonalisedMovingAverage { alpha_num: f64, alpha_den: f64 },
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
