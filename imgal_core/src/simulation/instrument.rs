use std::f64::consts::LN_2;

use ndarray::Array1;

use crate::distribution::gaussian;

/// Simulate 1-dimensional gaussian instruement response function (IRF)
pub fn gaussian_irf_1d(
    bins: usize,
    time_range: f64,
    irf_width: f64,
    irf_center: f64,
) -> Array1<f64> {
    let sigma = irf_width / (2.0 * (2.0 * LN_2).sqrt());
    gaussian(sigma, bins, time_range, irf_center)
}
