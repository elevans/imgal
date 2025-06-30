use std::f64::consts::LN_2;

use ndarray::Array1;

use crate::distribution::gaussian;

/// Simulate a 1-dimensional Gaussian instruement response function (IRF).
///
/// # Description
///
/// This function creates a Gaussian IRF by converting "full width at half maximum"
/// (FWHM) parameters into a normalized Gaussian distribution. The FWHM is
/// converted to standard deviation using the relationship:
///
/// ```text
/// σ = FWHM / (2 × √(2 × ln(2)))
/// ```
/// where `ln(2) ≈ 0.693147` is the natural logarithm of 2.
///
/// # Arguments
///
/// * `bins`: The number of discrete points to sample the Gaussian distribution.
/// * `time_range`: The total time range over which to simulate the IRF.
/// * `irf_width`: The full width at half maximum (FWHM) of the IRF.
/// * `irf_center`: The temporal position of the IRF peak within the time range.
///
/// # Returns
///
/// * `Array1<f64>`: The simulated 1-dimensional IRF curve.
pub fn gaussian_irf_1d(
    bins: usize,
    time_range: f64,
    irf_width: f64,
    irf_center: f64,
) -> Array1<f64> {
    let sigma = irf_width / (2.0 * (2.0 * LN_2).sqrt());
    gaussian(sigma, bins, time_range, irf_center)
}
