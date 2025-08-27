use ndarray::{Array1, Array3, ArrayView1};

use crate::filter::fft_convolve_1d;
use crate::simulation::instrument;

/// Simulate a 1-dimensional Gaussian IRF convolved monoexponential decay curve.
///
/// # Description
///
/// Compute a Gaussian instrument response function (IRF) convolved curve
/// (1-dimensional) by FFT convolving the IRF with an ideal monoexponential
/// decay cruve defined as:
///
/// ```text
/// I(t) = Io * e^(-t/τ)
/// ```
///
/// # Arguments
///
/// * `samples`: The number of discrete points that make up the decay curve
///    (_i.e._ time).
/// * `period`: The period (_i.e._ time interval).
/// * `tau`: The lifetime.
/// * `initial_value`: The initial decay value.
/// * `irf_width`: The full width at half maximum (FWHM) of the IRF.
/// * `irf_center`: The temporal position of the IRF peak within the time range.
///
/// # Returns
///
/// * `Vec<f64>`: The 1-dimensional Gaussian IRF convolved monoexponential decay
///    curve.
pub fn gaussian_monoexponential_1d(
    samples: usize,
    period: f64,
    tau: f64,
    initial_value: f64,
    irf_width: f64,
    irf_center: f64,
) -> Array1<f64> {
    let irf = instrument::gaussian_irf_1d(samples, period, irf_width, irf_center);
    let d = ideal_monoexponential_1d(samples, period, tau, initial_value);
    fft_convolve_1d(d.view(), irf.view())
}

/// Simulate a 3-dimensional Gaussian IRF convolved monoexponential decay curve.
///
/// # Description
///
/// Compute a Gaussian instrument response function (IRF) convolved curve
/// (3-dimensional) by FFT convolving the IRF with an ideal monoexponential
/// decay cruve defined as:
///
/// ```text
/// I(t) = Io * e^(-t/τ)
/// ```
///
/// # Arguments
///
/// * `samples`: The number of discrete points that make up the decay curve
///    (_i.e._ time).
/// * `period`: The period (_i.e_ time interval).
/// * `tau`: The lifetime.
/// * `initial_value`: The initial decay value.
/// * `irf_width`: The full width at half maximum (FWHM) of the IRF.
/// * `irf_center`: The temporal position of the IRF peak within the time range.
/// * `shape`: The row and col shape to broadcast the decay curve into.
///
/// # Returns
///
/// * `Array3<f64>`: The 3-dimensional Gaussian IRF convolved monoexponential
///    decay curve.
pub fn gaussian_monoexponential_3d(
    samples: usize,
    period: f64,
    tau: f64,
    initial_value: f64,
    irf_width: f64,
    irf_center: f64,
    shape: (usize, usize),
) -> Array3<f64> {
    // create 1-dimensional gaussian IRF convolved curve and broadcast
    let d = gaussian_monoexponential_1d(samples, period, tau, initial_value, irf_width, irf_center);
    let dims = (shape.0, shape.1, samples);
    d.broadcast(dims).unwrap().to_owned()
}

/// Simulate a 1-dimensional ideal monoexponential decay curve.
///
/// # Description
///
/// A monoexponential decay curve is defined as:
///
/// ```text
/// I(t) = Io * e^(-t/τ)
/// ```
///
/// Where "Io" is the initial decay value and "t" is time (_i.e._ the
/// number of samples).
///
/// # Arguments
///
/// * `samples`: The number of discrete points that make up the decay curve
///    (_i.e._ time).
/// * `period`: The period (_i.e_ time interval).
/// * `tau`: The lifetime.
/// * `initial_value`: The initial decay value.
///
/// # Returns
///
/// * `Array1<f64>`: The 1-dimensional monoexponential decay curve.
///
/// # Reference
///
/// <https://doi.org/10.1111/j.1749-6632.1969.tb56231.x>
pub fn ideal_monoexponential_1d(
    samples: usize,
    period: f64,
    tau: f64,
    initial_value: f64,
) -> Array1<f64> {
    // create time array and compute the decay curve
    let t: Array1<f64> = Array1::linspace(0.0, period, samples);
    t.map(|ti| initial_value * (-ti / tau).exp())
}

/// Simulate a 3-dimensional ideal monoexponential decay curve.
///
/// A monoexponential decay curve is defined as:
///
/// ```text
/// I(t) = Io * e^(-t/τ)
/// ```
///
/// Where "Io" is the initial decay value and "t" is the time (_i.e._ the
/// number of samples). The decay curve is then broadcasted to the specified input
/// shape with the number of samples along the last axis.
///
/// # Arguments
///
/// * `samples`: The number of discrete points that make up the decay curve (i.e. time).
/// * `period`: The period (_i.e._ time interval).
/// * `tau`: The lifetime.
/// * `initial_value`: The initial decay value.
/// * `shape`: The row and col shape to broadcast the decay curve into.
///
/// # Return
///
/// * `Array3<f64>`: The 3-dimensional monoexpoential decay curve.
///
/// # Reference
///
/// <https://doi.org/10.1111/j.1749-6632.1969.tb56231.x>
pub fn ideal_monoexponential_3d(
    samples: usize,
    period: f64,
    tau: f64,
    initial_value: f64,
    shape: (usize, usize),
) -> Array3<f64> {
    // create 1-dimensional decay curve and broadcast
    let d = ideal_monoexponential_1d(samples, period, tau, initial_value);
    let dims = (shape.0, shape.1, samples);
    d.broadcast(dims).unwrap().to_owned()
}

/// Simulate a 1-dimensional IRF convolved monoexponential decay curve.
///
/// # Description
///
/// Compute an instrument response function (IRF) convolved (1-dimensional)
/// curve by FFT convolving the given IRF with an ideal monoexponential decay
/// cruve. The monoexponential decay cruve is defined as:
///
/// ```text
/// I(t) = Io * e^(-t/τ)
/// ```
///
/// # Arguments
///
/// * `irf`: The IRF as a 1-dimensonal array.
/// * `samples`: The number of discrete points that make up the decay curve (_i.e._ time).
/// * `period`: The period (_i.e._ time interval).
/// * `tau`: The lifetime.
/// * `initial_value`: The initial decay value.
///
/// # Returns
///
/// * `Array1<f64>`: The 1-dimensional IRF convolved monoexponential decay curve.
pub fn irf_monoexponential_1d(
    irf: ArrayView1<f64>,
    samples: usize,
    period: f64,
    tau: f64,
    initial_value: f64,
) -> Array1<f64> {
    // create ideal decay curve and convolve with input irf
    let d = ideal_monoexponential_1d(samples, period, tau, initial_value);
    fft_convolve_1d(d.view(), irf)
}

/// Simulate a 3-dimensional IRF convolved monoexponential decay curve.
///
/// # Description
///
/// Compute an instrument response function (IRF) convolved (3-dimensional)
/// cruve by FFT convolving the given IRF with an ideal monoexponential decay
/// curve. The monoexponential decay curve is defined as:
///
/// ```text
/// I(t) = Io * e^(-t/τ)
/// ```
///
/// # Arguments
///
/// * `irf`: The IRF as a 1-dimensonal array.
/// * `samples`: The number of discrete points that make up the decay curve (_i.e._ time).
/// * `period`: The period (_i.e._ time interval).
/// * `tau`: The lifetime.
/// * `initial_value`: The initial decay value.
/// * `shape`: The row and col shape to broadcast the decay curve into.
///
/// # Returns
///
/// * `Array3<f64>`: The 3-dimensional IRF convolved monoexponential decay curve.
pub fn irf_monoexponential_3d(
    irf: ArrayView1<f64>,
    samples: usize,
    period: f64,
    tau: f64,
    initial_value: f64,
    shape: (usize, usize),
) -> Array3<f64> {
    // create 1-dimensional IRF convolved decay curve to broadcast
    let d = irf_monoexponential_1d(irf, samples, period, tau, initial_value);

    // broadcast IRF convolved decay curve
    let dims = (shape.0, shape.1, samples);
    d.broadcast(dims).unwrap().to_owned()
}
