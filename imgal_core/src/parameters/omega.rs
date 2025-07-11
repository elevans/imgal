use std::f64::consts::PI;

use crate::traits::numeric::ToFloat64;

/// Compute the angular frequency (omega) value.
///
/// # Description
///
/// Compute the angular frequency, omega (ω), using the following equation:
///
/// ω = 2π/T
///
/// Where "T" is the period.
///
/// # Arguments
///
/// * `period`: The time period.
///
/// # Returns
///
/// * `f64`: The omega (ω) value.
pub fn omega<T>(period: T) -> f64
where
    T: ToFloat64,
{
    2.0 * PI / period.into()
}
