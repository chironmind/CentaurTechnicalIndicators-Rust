use std::fmt;

/// What central value to use for calculations.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum CentralPoint {
    Mean,
    Median,
    Mode,
}

/// How to aggregate a set of absolute deviations.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum DeviationAggregate {
    Mean,
    Median,
    Mode,
}

/// Configuration that controls how absolute deviations are computed.
/// Example: center = Median, aggregate = Median => true MedianAD (median of |x - median|).
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct AbsDevConfig {
    pub center: CentralPoint,
    pub aggregate: DeviationAggregate,
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
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum DeviationModel {
    StandardDeviation,
    MeanAbsoluteDeviation,
    MedianAbsoluteDeviation,
    ModeAbsoluteDeviation,
    CustomAbsoluteDeviation { config: AbsDevConfig },
    UlcerIndex,
    LogStandardDeviation,
    StudentT { df: f64 },
    LaplaceStdEquivalent,
    CauchyIQRScale,
    EmpiricalQuantileRange { low: f64, high: f64, precision: f64 },
}

/// Trade position.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Position {
    Short,
    Long,
}

// Display impls use snake_case unit variants. Parameterized variants render as
// `name(field=value, ...)`. The output is stable and tested below.

impl fmt::Display for CentralPoint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            CentralPoint::Mean => "mean",
            CentralPoint::Median => "median",
            CentralPoint::Mode => "mode",
        };
        f.write_str(s)
    }
}

impl fmt::Display for DeviationAggregate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            DeviationAggregate::Mean => "mean",
            DeviationAggregate::Median => "median",
            DeviationAggregate::Mode => "mode",
        };
        f.write_str(s)
    }
}

impl fmt::Display for AbsDevConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "center={}, aggregate={}", self.center, self.aggregate)
    }
}

impl fmt::Display for MovingAverageType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MovingAverageType::Simple => f.write_str("simple"),
            MovingAverageType::Smoothed => f.write_str("smoothed"),
            MovingAverageType::Exponential => f.write_str("exponential"),
            MovingAverageType::Personalised {
                alpha_num,
                alpha_den,
            } => write!(
                f,
                "personalised(alpha_num={}, alpha_den={})",
                alpha_num, alpha_den
            ),
        }
    }
}

impl fmt::Display for ConstantModelType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConstantModelType::SimpleMovingAverage => f.write_str("simple_moving_average"),
            ConstantModelType::SmoothedMovingAverage => f.write_str("smoothed_moving_average"),
            ConstantModelType::ExponentialMovingAverage => {
                f.write_str("exponential_moving_average")
            }
            ConstantModelType::PersonalisedMovingAverage {
                alpha_num,
                alpha_den,
            } => write!(
                f,
                "personalised_moving_average(alpha_num={}, alpha_den={})",
                alpha_num, alpha_den
            ),
            ConstantModelType::SimpleMovingMedian => f.write_str("simple_moving_median"),
            ConstantModelType::SimpleMovingMode => f.write_str("simple_moving_mode"),
        }
    }
}

impl fmt::Display for DeviationModel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DeviationModel::StandardDeviation => f.write_str("standard_deviation"),
            DeviationModel::MeanAbsoluteDeviation => f.write_str("mean_absolute_deviation"),
            DeviationModel::MedianAbsoluteDeviation => f.write_str("median_absolute_deviation"),
            DeviationModel::ModeAbsoluteDeviation => f.write_str("mode_absolute_deviation"),
            DeviationModel::CustomAbsoluteDeviation { config } => {
                write!(f, "custom_absolute_deviation({})", config)
            }
            DeviationModel::UlcerIndex => f.write_str("ulcer_index"),
            DeviationModel::LogStandardDeviation => f.write_str("log_standard_deviation"),
            DeviationModel::StudentT { df } => write!(f, "student_t(df={})", df),
            DeviationModel::LaplaceStdEquivalent => f.write_str("laplace_std_equivalent"),
            DeviationModel::CauchyIQRScale => f.write_str("cauchy_iqr_scale"),
            DeviationModel::EmpiricalQuantileRange {
                low,
                high,
                precision,
            } => write!(
                f,
                "empirical_quantile_range(low={}, high={}, precision={})",
                low, high, precision
            ),
        }
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Position::Short => "short",
            Position::Long => "long",
        };
        f.write_str(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Lock-in tests: any change to these strings is a contract change.
    // Any future `FromStr` impl must roundtrip against these exact strings
    // for the unit variants.

    #[test]
    fn central_point_display_all_variants() {
        assert_eq!(CentralPoint::Mean.to_string(), "mean");
        assert_eq!(CentralPoint::Median.to_string(), "median");
        assert_eq!(CentralPoint::Mode.to_string(), "mode");
    }

    #[test]
    fn deviation_aggregate_display_all_variants() {
        assert_eq!(DeviationAggregate::Mean.to_string(), "mean");
        assert_eq!(DeviationAggregate::Median.to_string(), "median");
        assert_eq!(DeviationAggregate::Mode.to_string(), "mode");
    }

    #[test]
    fn abs_dev_config_display() {
        let cfg = AbsDevConfig {
            center: CentralPoint::Median,
            aggregate: DeviationAggregate::Mean,
        };
        assert_eq!(cfg.to_string(), "center=median, aggregate=mean");
    }

    #[test]
    fn moving_average_type_display_all_variants() {
        assert_eq!(MovingAverageType::Simple.to_string(), "simple");
        assert_eq!(MovingAverageType::Smoothed.to_string(), "smoothed");
        assert_eq!(MovingAverageType::Exponential.to_string(), "exponential");
        assert_eq!(
            MovingAverageType::Personalised {
                alpha_num: 1.0,
                alpha_den: 2.0
            }
            .to_string(),
            "personalised(alpha_num=1, alpha_den=2)"
        );
    }

    #[test]
    fn constant_model_type_display_all_variants() {
        assert_eq!(
            ConstantModelType::SimpleMovingAverage.to_string(),
            "simple_moving_average"
        );
        assert_eq!(
            ConstantModelType::SmoothedMovingAverage.to_string(),
            "smoothed_moving_average"
        );
        assert_eq!(
            ConstantModelType::ExponentialMovingAverage.to_string(),
            "exponential_moving_average"
        );
        assert_eq!(
            ConstantModelType::PersonalisedMovingAverage {
                alpha_num: 1.0,
                alpha_den: 2.0
            }
            .to_string(),
            "personalised_moving_average(alpha_num=1, alpha_den=2)"
        );
        assert_eq!(
            ConstantModelType::SimpleMovingMedian.to_string(),
            "simple_moving_median"
        );
        assert_eq!(
            ConstantModelType::SimpleMovingMode.to_string(),
            "simple_moving_mode"
        );
    }

    #[test]
    fn deviation_model_display_all_variants() {
        assert_eq!(
            DeviationModel::StandardDeviation.to_string(),
            "standard_deviation"
        );
        assert_eq!(
            DeviationModel::MeanAbsoluteDeviation.to_string(),
            "mean_absolute_deviation"
        );
        assert_eq!(
            DeviationModel::MedianAbsoluteDeviation.to_string(),
            "median_absolute_deviation"
        );
        assert_eq!(
            DeviationModel::ModeAbsoluteDeviation.to_string(),
            "mode_absolute_deviation"
        );
        assert_eq!(
            DeviationModel::CustomAbsoluteDeviation {
                config: AbsDevConfig {
                    center: CentralPoint::Median,
                    aggregate: DeviationAggregate::Median,
                }
            }
            .to_string(),
            "custom_absolute_deviation(center=median, aggregate=median)"
        );
        assert_eq!(DeviationModel::UlcerIndex.to_string(), "ulcer_index");
        assert_eq!(
            DeviationModel::LogStandardDeviation.to_string(),
            "log_standard_deviation"
        );
        assert_eq!(
            DeviationModel::StudentT { df: 3.0 }.to_string(),
            "student_t(df=3)"
        );
        assert_eq!(
            DeviationModel::LaplaceStdEquivalent.to_string(),
            "laplace_std_equivalent"
        );
        assert_eq!(
            DeviationModel::CauchyIQRScale.to_string(),
            "cauchy_iqr_scale"
        );
        assert_eq!(
            DeviationModel::EmpiricalQuantileRange {
                low: 0.1,
                high: 0.9,
                precision: 0.01
            }
            .to_string(),
            "empirical_quantile_range(low=0.1, high=0.9, precision=0.01)"
        );
    }

    #[test]
    fn position_display_all_variants() {
        assert_eq!(Position::Short.to_string(), "short");
        assert_eq!(Position::Long.to_string(), "long");
    }
}
