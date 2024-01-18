use bencher_json::{project::boundary::BoundaryLimit, Boundary};
use slog::{debug, Logger};
use statrs::distribution::{ContinuousCDF, Normal, StudentsT};

use crate::{BoundaryError, PercentageBoundary, StatisticalBoundary};

#[derive(Debug, Default)]
pub struct MetricsLimits {
    pub baseline: Option<f64>,
    pub lower: Option<MetricsLimit>,
    pub upper: Option<MetricsLimit>,
}

#[derive(Debug, PartialEq)]
pub struct MetricsLimit {
    pub value: f64,
}

#[derive(Clone, Copy)]
pub enum StatisticalTestKind {
    Z,
    T { freedom: f64 },
}

impl MetricsLimits {
    pub fn new_static(lower_boundary: Option<Boundary>, upper_boundary: Option<Boundary>) -> Self {
        Self {
            baseline: None,
            lower: lower_boundary.map(Into::into),
            upper: upper_boundary.map(Into::into),
        }
    }

    pub fn new_percentage(
        log: &Logger,
        mean: f64,
        lower_boundary: Option<PercentageBoundary>,
        upper_boundary: Option<PercentageBoundary>,
    ) -> Self {
        if lower_boundary.is_none() && upper_boundary.is_none() {
            return Self::default();
        }

        debug!(log, "Percentage: mean={mean}");
        let lower = lower_boundary.map(|boundary| MetricsLimit::percentage_lower(mean, boundary));
        let upper = upper_boundary.map(|boundary| MetricsLimit::percentage_upper(mean, boundary));

        Self {
            baseline: Some(mean),
            lower,
            upper,
        }
    }

    pub fn new_statistical(
        log: &Logger,
        mean: f64,
        std_dev: f64,
        test_kind: StatisticalTestKind,
        lower_boundary: Option<StatisticalBoundary>,
        upper_boundary: Option<StatisticalBoundary>,
    ) -> Result<Self, BoundaryError> {
        if lower_boundary.is_none() && upper_boundary.is_none() {
            return Ok(Self::default());
        }

        Ok(match test_kind {
            // Create a normal distribution and calculate the boundary limits for the threshold based on the boundary percentiles.
            StatisticalTestKind::Z => {
                debug!(log, "Normal distribution: mean={mean}, std_dev={std_dev}");
                let normal = Normal::new(mean, std_dev).map_err(|error| BoundaryError::Normal {
                    mean,
                    std_dev,
                    error,
                })?;
                let lower = lower_boundary.map(|limit| {
                    let abs_limit = normal.inverse_cdf(limit.into());
                    MetricsLimit::statistical_lower(mean, abs_limit)
                });
                let upper = upper_boundary.map(|limit| {
                    let abs_limit = normal.inverse_cdf(limit.into());
                    MetricsLimit::statistical_upper(abs_limit)
                });
                Self {
                    baseline: Some(mean),
                    lower,
                    upper,
                }
            },
            // Create a Student's t distribution and calculate the boundary limits for the threshold based on the boundary percentiles.
            StatisticalTestKind::T { freedom } => {
                debug!(
                    log,
                    "Students T distribution: mean={mean}, scale={std_dev}, freedom={freedom}"
                );
                let students_t = StudentsT::new(mean, std_dev, freedom).map_err(|error| {
                    BoundaryError::StudentsT {
                        mean,
                        std_dev,
                        freedom,
                        error,
                    }
                })?;
                let lower = lower_boundary.map(|limit| {
                    let abs_limit = students_t.inverse_cdf(limit.into());
                    MetricsLimit::statistical_lower(mean, abs_limit)
                });
                let upper = upper_boundary.map(|limit| {
                    let abs_limit = students_t.inverse_cdf(limit.into());
                    MetricsLimit::statistical_upper(abs_limit)
                });
                Self {
                    baseline: Some(mean),
                    lower,
                    upper,
                }
            },
        })
    }

    // An outlier occurs when the  datum exceeds a boundary limit.
    pub fn outlier(&self, datum: f64) -> Option<BoundaryLimit> {
        match (self.lower.as_ref(), self.upper.as_ref()) {
            (Some(lower), Some(upper)) => {
                if datum < lower.value {
                    Some(BoundaryLimit::Lower)
                } else if datum > upper.value {
                    Some(BoundaryLimit::Upper)
                } else {
                    None
                }
            },
            (Some(lower), None) => (datum < lower.value).then_some(BoundaryLimit::Lower),
            (None, Some(upper)) => (datum > upper.value).then_some(BoundaryLimit::Upper),
            (None, None) => None,
        }
    }
}

impl MetricsLimit {
    fn percentage_lower(mean: f64, boundary: PercentageBoundary) -> Self {
        Self {
            value: mean - (mean * f64::from(boundary)),
        }
    }

    fn percentage_upper(mean: f64, boundary: PercentageBoundary) -> Self {
        Self {
            value: mean + (mean * f64::from(boundary)),
        }
    }

    // Flip the absolute limit to the other side of the mean, creating the actual boundary limit.
    fn statistical_lower(mean: f64, abs_limit: f64) -> Self {
        Self {
            value: mean * 2.0 - abs_limit,
        }
    }

    fn statistical_upper(abs_limit: f64) -> Self {
        Self { value: abs_limit }
    }
}

impl From<MetricsLimit> for f64 {
    fn from(limit: MetricsLimit) -> Self {
        limit.value
    }
}

impl From<Boundary> for MetricsLimit {
    fn from(boundary: Boundary) -> Self {
        Self {
            value: boundary.into(),
        }
    }
}

#[cfg(test)]
#[allow(clippy::expect_used, clippy::unreadable_literal, clippy::unwrap_used)]
mod test {
    use bencher_json::{project::boundary::BoundaryLimit, Boundary};
    use bencher_logger::bootstrap_logger;
    use once_cell::sync::Lazy;
    use ordered_float::OrderedFloat;
    use pretty_assertions::assert_eq;

    use crate::{PercentageBoundary, StatisticalBoundary};

    use super::{MetricsLimit, MetricsLimits, StatisticalTestKind};

    const MEAN: f64 = 0.0;
    const STD_DEV: f64 = 1.0;
    const FREEDOM: f64 = 5.0;

    static NEGATIVE_STATIC_LIMIT: Lazy<Boundary> =
        Lazy::new(|| (-5.0).try_into().expect("Failed to parse boundary."));
    static STATIC_LIMIT: Lazy<Boundary> =
        Lazy::new(|| 5.0.try_into().expect("Failed to parse boundary."));

    const STATIC_NEGATIVE_OUTLIER: f64 = -10.0;
    const STATIC_NEGATIVE: f64 = -3.0;
    const STATIC_ONE: f64 = 1.0;
    const STATIC_POSITIVE: f64 = 3.0;
    const STATIC_POSITIVE_OUTLIER: f64 = 10.0;

    static PERCENTAGE: Lazy<PercentageBoundary> = Lazy::new(|| {
        5.0.try_into()
            .expect("Failed to parse percentage boundary.")
    });
    const PERCENTAGE_NEGATIVE: f64 = -4.0;
    const PERCENTAGE_POSITIVE: f64 = 6.0;

    static PERCENTILE: Lazy<StatisticalBoundary> = Lazy::new(|| {
        0.85.try_into()
            .expect("Failed to parse statistical boundary.")
    });
    const Z_LIMIT: f64 = 1.0364333894937896;
    const T_LIMIT: f64 = 1.1557673428942912;

    const DATUM_NEGATIVE_OUTLIER: f64 = -1.5;
    const DATUM_NEGATIVE: f64 = -1.0;
    const DATUM_ZERO: f64 = 0.0;
    const DATUM_POSITIVE: f64 = 1.0;
    const DATUM_POSITIVE_OUTLIER: f64 = 1.5;

    #[test]
    fn test_limits_static_none() {
        let limits = MetricsLimits::new_static(None, None);
        assert_eq!(limits.baseline, None);
        assert_eq!(limits.lower, None);
        assert_eq!(limits.upper, None);

        let side = limits.outlier(STATIC_NEGATIVE_OUTLIER);
        assert_eq!(side, None);

        let side = limits.outlier(STATIC_NEGATIVE);
        assert_eq!(side, None);

        let side = limits.outlier(STATIC_ONE);
        assert_eq!(side, None);

        let side = limits.outlier(STATIC_POSITIVE);
        assert_eq!(side, None);

        let side = limits.outlier(STATIC_POSITIVE_OUTLIER);
        assert_eq!(side, None);
    }

    #[test]
    fn test_limits_static_lower() {
        let limits = MetricsLimits::new_static(Some(*NEGATIVE_STATIC_LIMIT), None);
        assert_eq!(limits.baseline, None);
        assert_eq!(
            limits.lower,
            Some(MetricsLimit {
                value: (*NEGATIVE_STATIC_LIMIT).into()
            })
        );
        assert_eq!(limits.upper, None);

        let side = limits.outlier(STATIC_NEGATIVE_OUTLIER);
        assert_eq!(side, Some(BoundaryLimit::Lower));

        let side = limits.outlier(STATIC_NEGATIVE);
        assert_eq!(side, None);

        let side = limits.outlier(STATIC_ONE);
        assert_eq!(side, None);

        let side = limits.outlier(STATIC_POSITIVE);
        assert_eq!(side, None);

        let side = limits.outlier(STATIC_POSITIVE_OUTLIER);
        assert_eq!(side, None);
    }

    #[test]
    fn test_limits_static_upper() {
        let _log = bootstrap_logger();
        let limits = MetricsLimits::new_static(None, Some(*STATIC_LIMIT));
        assert_eq!(limits.baseline, None);
        assert_eq!(limits.lower, None);
        assert_eq!(
            limits.upper,
            Some(MetricsLimit {
                value: (*STATIC_LIMIT).into()
            })
        );

        let side = limits.outlier(STATIC_NEGATIVE_OUTLIER);
        assert_eq!(side, None);

        let side = limits.outlier(STATIC_NEGATIVE);
        assert_eq!(side, None);

        let side = limits.outlier(STATIC_ONE);
        assert_eq!(side, None);

        let side = limits.outlier(STATIC_POSITIVE);
        assert_eq!(side, None);

        let side = limits.outlier(STATIC_POSITIVE_OUTLIER);
        assert_eq!(side, Some(BoundaryLimit::Upper));
    }

    #[test]
    fn test_limits_static_both() {
        let limits = MetricsLimits::new_static(Some(*NEGATIVE_STATIC_LIMIT), Some(*STATIC_LIMIT));
        assert_eq!(limits.baseline, None);
        assert_eq!(
            limits.lower,
            Some(MetricsLimit {
                value: (*NEGATIVE_STATIC_LIMIT).into()
            })
        );
        assert_eq!(
            limits.upper,
            Some(MetricsLimit {
                value: (*STATIC_LIMIT).into()
            })
        );

        let side = limits.outlier(STATIC_NEGATIVE_OUTLIER);
        assert_eq!(side, Some(BoundaryLimit::Lower));

        let side = limits.outlier(STATIC_NEGATIVE);
        assert_eq!(side, None);

        let side = limits.outlier(STATIC_ONE);
        assert_eq!(side, None);

        let side = limits.outlier(STATIC_POSITIVE);
        assert_eq!(side, None);

        let side = limits.outlier(STATIC_POSITIVE_OUTLIER);
        assert_eq!(side, Some(BoundaryLimit::Upper));
    }

    #[test]
    fn test_limits_percentage_none() {
        let log = bootstrap_logger();
        let limits = MetricsLimits::new_percentage(&log, STATIC_ONE, None, None);
        assert_eq!(limits.baseline, None);
        assert_eq!(limits.lower, None);
        assert_eq!(limits.upper, None);

        let side = limits.outlier(STATIC_NEGATIVE_OUTLIER);
        assert_eq!(side, None);

        let side = limits.outlier(STATIC_NEGATIVE);
        assert_eq!(side, None);

        let side = limits.outlier(STATIC_ONE);
        assert_eq!(side, None);

        let side = limits.outlier(STATIC_POSITIVE);
        assert_eq!(side, None);

        let side = limits.outlier(STATIC_POSITIVE_OUTLIER);
        assert_eq!(side, None);
    }

    #[test]
    fn test_limits_percentage_lower() {
        let log = bootstrap_logger();
        let limits = MetricsLimits::new_percentage(&log, STATIC_ONE, Some(*PERCENTAGE), None);
        assert_eq!(
            OrderedFloat::from(limits.baseline.unwrap()),
            OrderedFloat::from(STATIC_ONE)
        );
        assert_eq!(
            limits.lower,
            Some(MetricsLimit {
                value: PERCENTAGE_NEGATIVE
            })
        );
        assert_eq!(limits.upper, None);

        let side = limits.outlier(STATIC_NEGATIVE_OUTLIER);
        assert_eq!(side, Some(BoundaryLimit::Lower));

        let side = limits.outlier(STATIC_NEGATIVE);
        assert_eq!(side, None);

        let side = limits.outlier(STATIC_ONE);
        assert_eq!(side, None);

        let side = limits.outlier(STATIC_POSITIVE);
        assert_eq!(side, None);

        let side = limits.outlier(STATIC_POSITIVE_OUTLIER);
        assert_eq!(side, None);
    }

    #[test]
    fn test_limits_percentage_upper() {
        let log = bootstrap_logger();
        let limits = MetricsLimits::new_percentage(&log, STATIC_ONE, None, Some(*PERCENTAGE));
        assert_eq!(
            OrderedFloat::from(limits.baseline.unwrap()),
            OrderedFloat::from(STATIC_ONE)
        );
        assert_eq!(limits.lower, None);
        assert_eq!(
            limits.upper,
            Some(MetricsLimit {
                value: PERCENTAGE_POSITIVE
            })
        );

        let side = limits.outlier(STATIC_NEGATIVE_OUTLIER);
        assert_eq!(side, None);

        let side = limits.outlier(STATIC_NEGATIVE);
        assert_eq!(side, None);

        let side = limits.outlier(STATIC_ONE);
        assert_eq!(side, None);

        let side = limits.outlier(STATIC_POSITIVE);
        assert_eq!(side, None);

        let side = limits.outlier(STATIC_POSITIVE_OUTLIER);
        assert_eq!(side, Some(BoundaryLimit::Upper));
    }

    #[test]
    fn test_limits_percentage_both() {
        let log = bootstrap_logger();
        let limits =
            MetricsLimits::new_percentage(&log, STATIC_ONE, Some(*PERCENTAGE), Some(*PERCENTAGE));
        assert_eq!(
            OrderedFloat::from(limits.baseline.unwrap()),
            OrderedFloat::from(STATIC_ONE)
        );
        assert_eq!(
            limits.lower,
            Some(MetricsLimit {
                value: PERCENTAGE_NEGATIVE
            })
        );
        assert_eq!(
            limits.upper,
            Some(MetricsLimit {
                value: PERCENTAGE_POSITIVE
            })
        );

        let side = limits.outlier(STATIC_NEGATIVE_OUTLIER);
        assert_eq!(side, Some(BoundaryLimit::Lower));

        let side = limits.outlier(STATIC_NEGATIVE);
        assert_eq!(side, None);

        let side = limits.outlier(STATIC_ONE);
        assert_eq!(side, None);

        let side = limits.outlier(STATIC_POSITIVE);
        assert_eq!(side, None);

        let side = limits.outlier(STATIC_POSITIVE_OUTLIER);
        assert_eq!(side, Some(BoundaryLimit::Upper));
    }

    #[test]
    fn test_limits_z_none() {
        let log = bootstrap_logger();
        let limits =
            MetricsLimits::new_statistical(&log, MEAN, STD_DEV, StatisticalTestKind::Z, None, None)
                .unwrap();
        assert_eq!(limits.baseline, None);
        assert_eq!(limits.lower, None);
        assert_eq!(limits.upper, None);

        let side = limits.outlier(DATUM_NEGATIVE_OUTLIER);
        assert_eq!(side, None);

        let side = limits.outlier(DATUM_NEGATIVE);
        assert_eq!(side, None);

        let side = limits.outlier(DATUM_ZERO);
        assert_eq!(side, None);

        let side = limits.outlier(DATUM_POSITIVE);
        assert_eq!(side, None);

        let side = limits.outlier(DATUM_POSITIVE_OUTLIER);
        assert_eq!(side, None);
    }

    #[test]
    fn test_limits_z_lower() {
        let log = bootstrap_logger();
        let limits = MetricsLimits::new_statistical(
            &log,
            MEAN,
            STD_DEV,
            StatisticalTestKind::Z,
            Some(*PERCENTILE),
            None,
        )
        .unwrap();
        assert_eq!(
            OrderedFloat::from(limits.baseline.unwrap()),
            OrderedFloat::from(MEAN)
        );
        assert_eq!(limits.lower, Some(MetricsLimit { value: -Z_LIMIT }));
        assert_eq!(limits.upper, None);

        let side = limits.outlier(DATUM_NEGATIVE_OUTLIER);
        assert_eq!(side, Some(BoundaryLimit::Lower));

        let side = limits.outlier(DATUM_NEGATIVE);
        assert_eq!(side, None);

        let side = limits.outlier(DATUM_ZERO);
        assert_eq!(side, None);

        let side = limits.outlier(DATUM_POSITIVE);
        assert_eq!(side, None);

        let side = limits.outlier(DATUM_POSITIVE_OUTLIER);
        assert_eq!(side, None);
    }

    #[test]
    fn test_limits_z_upper() {
        let log = bootstrap_logger();
        let limits = MetricsLimits::new_statistical(
            &log,
            MEAN,
            STD_DEV,
            StatisticalTestKind::Z,
            None,
            Some(*PERCENTILE),
        )
        .unwrap();
        assert_eq!(
            OrderedFloat::from(limits.baseline.unwrap()),
            OrderedFloat::from(MEAN)
        );
        assert_eq!(limits.lower, None);
        assert_eq!(limits.upper, Some(MetricsLimit { value: Z_LIMIT }));

        let side = limits.outlier(DATUM_NEGATIVE_OUTLIER);
        assert_eq!(side, None);

        let side = limits.outlier(DATUM_NEGATIVE);
        assert_eq!(side, None);

        let side = limits.outlier(DATUM_ZERO);
        assert_eq!(side, None);

        let side = limits.outlier(DATUM_POSITIVE);
        assert_eq!(side, None);

        let side = limits.outlier(DATUM_POSITIVE_OUTLIER);
        assert_eq!(side, Some(BoundaryLimit::Upper));
    }

    #[test]
    fn test_limits_z_both() {
        let log = bootstrap_logger();
        let limits = MetricsLimits::new_statistical(
            &log,
            MEAN,
            STD_DEV,
            StatisticalTestKind::Z,
            Some(*PERCENTILE),
            Some(*PERCENTILE),
        )
        .unwrap();
        assert_eq!(
            OrderedFloat::from(limits.baseline.unwrap()),
            OrderedFloat::from(MEAN)
        );
        assert_eq!(limits.lower, Some(MetricsLimit { value: -Z_LIMIT }));
        assert_eq!(limits.upper, Some(MetricsLimit { value: Z_LIMIT }));

        let side = limits.outlier(DATUM_NEGATIVE_OUTLIER);
        assert_eq!(side, Some(BoundaryLimit::Lower));

        let side = limits.outlier(DATUM_NEGATIVE);
        assert_eq!(side, None);

        let side = limits.outlier(DATUM_ZERO);
        assert_eq!(side, None);

        let side = limits.outlier(DATUM_POSITIVE);
        assert_eq!(side, None);

        let side = limits.outlier(DATUM_POSITIVE_OUTLIER);
        assert_eq!(side, Some(BoundaryLimit::Upper));
    }

    #[test]
    fn test_limits_z_docs() {
        const MEAN_100: f64 = 100.0;
        let log = bootstrap_logger();
        let boundary = 0.977.try_into().expect("Failed to create boundary.");
        let limits = MetricsLimits::new_statistical(
            &log,
            MEAN_100,
            10.0,
            StatisticalTestKind::Z,
            Some(boundary),
            Some(boundary),
        )
        .unwrap();
        assert_eq!(
            OrderedFloat::from(limits.baseline.unwrap()),
            OrderedFloat::from(MEAN_100)
        );
        assert_eq!(
            limits.lower,
            Some(MetricsLimit {
                value: 80.04606689832175
            })
        );
        assert_eq!(
            limits.upper,
            Some(MetricsLimit {
                value: 119.95393310167825
            })
        );

        let side = limits.outlier(75.0);
        assert_eq!(side, Some(BoundaryLimit::Lower));

        let side = limits.outlier(90.0);
        assert_eq!(side, None);

        let side = limits.outlier(100.0);
        assert_eq!(side, None);

        let side = limits.outlier(110.0);
        assert_eq!(side, None);

        let side = limits.outlier(125.0);
        assert_eq!(side, Some(BoundaryLimit::Upper));
    }

    #[test]
    fn test_limits_t_none() {
        let log = bootstrap_logger();
        let limits = MetricsLimits::new_statistical(
            &log,
            MEAN,
            STD_DEV,
            StatisticalTestKind::T { freedom: FREEDOM },
            None,
            None,
        )
        .unwrap();
        assert_eq!(limits.baseline, None);
        assert_eq!(limits.lower, None);
        assert_eq!(limits.upper, None);

        let side = limits.outlier(DATUM_NEGATIVE_OUTLIER);
        assert_eq!(side, None);

        let side = limits.outlier(DATUM_NEGATIVE);
        assert_eq!(side, None);

        let side = limits.outlier(DATUM_ZERO);
        assert_eq!(side, None);

        let side = limits.outlier(DATUM_POSITIVE);
        assert_eq!(side, None);

        let side = limits.outlier(DATUM_POSITIVE_OUTLIER);
        assert_eq!(side, None);
    }

    #[test]
    fn test_limits_t_lower() {
        let log = bootstrap_logger();
        let limits = MetricsLimits::new_statistical(
            &log,
            MEAN,
            STD_DEV,
            StatisticalTestKind::T { freedom: FREEDOM },
            Some(*PERCENTILE),
            None,
        )
        .unwrap();
        assert_eq!(
            OrderedFloat::from(limits.baseline.unwrap()),
            OrderedFloat::from(MEAN)
        );
        assert_eq!(limits.lower, Some(MetricsLimit { value: -T_LIMIT }));
        assert_eq!(limits.upper, None);

        let side = limits.outlier(DATUM_NEGATIVE_OUTLIER);
        assert_eq!(side, Some(BoundaryLimit::Lower));

        let side = limits.outlier(DATUM_NEGATIVE);
        assert_eq!(side, None);

        let side = limits.outlier(DATUM_ZERO);
        assert_eq!(side, None);

        let side = limits.outlier(DATUM_POSITIVE);
        assert_eq!(side, None);

        let side = limits.outlier(DATUM_POSITIVE_OUTLIER);
        assert_eq!(side, None);
    }

    #[test]
    fn test_limits_t_upper() {
        let log = bootstrap_logger();
        let limits = MetricsLimits::new_statistical(
            &log,
            MEAN,
            STD_DEV,
            StatisticalTestKind::T { freedom: FREEDOM },
            None,
            Some(*PERCENTILE),
        )
        .unwrap();
        assert_eq!(
            OrderedFloat::from(limits.baseline.unwrap()),
            OrderedFloat::from(MEAN)
        );
        assert_eq!(limits.lower, None);
        assert_eq!(limits.upper, Some(MetricsLimit { value: T_LIMIT }));

        let side = limits.outlier(DATUM_NEGATIVE_OUTLIER);
        assert_eq!(side, None);

        let side = limits.outlier(DATUM_NEGATIVE);
        assert_eq!(side, None);

        let side = limits.outlier(DATUM_ZERO);
        assert_eq!(side, None);

        let side = limits.outlier(DATUM_POSITIVE);
        assert_eq!(side, None);

        let side = limits.outlier(DATUM_POSITIVE_OUTLIER);
        assert_eq!(side, Some(BoundaryLimit::Upper));
    }

    #[test]
    fn test_limits_t_both() {
        let log = bootstrap_logger();
        let limits = MetricsLimits::new_statistical(
            &log,
            MEAN,
            STD_DEV,
            StatisticalTestKind::T { freedom: FREEDOM },
            Some(*PERCENTILE),
            Some(*PERCENTILE),
        )
        .unwrap();
        assert_eq!(
            OrderedFloat::from(limits.baseline.unwrap()),
            OrderedFloat::from(MEAN)
        );
        assert_eq!(limits.lower, Some(MetricsLimit { value: -T_LIMIT }));
        assert_eq!(limits.upper, Some(MetricsLimit { value: T_LIMIT }));

        let side = limits.outlier(DATUM_NEGATIVE_OUTLIER);
        assert_eq!(side, Some(BoundaryLimit::Lower));

        let side = limits.outlier(DATUM_NEGATIVE);
        assert_eq!(side, None);

        let side = limits.outlier(DATUM_ZERO);
        assert_eq!(side, None);

        let side = limits.outlier(DATUM_POSITIVE);
        assert_eq!(side, None);

        let side = limits.outlier(DATUM_POSITIVE_OUTLIER);
        assert_eq!(side, Some(BoundaryLimit::Upper));
    }
}
