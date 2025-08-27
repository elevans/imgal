use ndarray::{Array3, ArrayView3, ArrayViewMut3, Axis, Zip};
use rayon::prelude::*;

use crate::phasor::plot;
use crate::traits::numeric::ToFloat64;

/// Calibrate a real and imaginary (G, S) coordinates.
///
/// # Description
///
/// Calibrate the real and imaginary (_e.g._ G and S) coordinates by rotating
/// and scaling by phase (φ) and modulation (M) respectively using:
///
/// ```text
/// g = M * cos(φ)
/// s = M * sin(φ)
/// G' = G * g - S * s
/// S' = G * s + S * g
/// ```
///
/// Where G' and S' are the calibrated real and imaginary values after rotation
/// and scaling.
///
/// # Arguments
///
/// * `g`: The real component (G) to calibrate.
/// * `s`: The imaginary (S) to calibrate.
/// * `modulation`: The modulation to scale the input (G, S) coordinates.
/// * `phase`: The phase, φ angle, to rotate the input (G, S) coordinates.
///
/// # Returns
///
/// * `(f64, f64)`: The calibrated coordinates, (G, S).
pub fn coordinates(g: f64, s: f64, modulation: f64, phase: f64) -> (f64, f64) {
    let g_trans = modulation * phase.cos();
    let s_trans = modulation * phase.sin();
    let g_cal = g * g_trans - s * s_trans;
    let s_cal = g * s_trans + s * g_trans;
    (g_cal, s_cal)
}

/// Calibrate the real and imaginary (G, S) coordinates of a 3-dimensional phasor
/// image.
///
/// # Description
///
/// This function calibrates an input 3-dimensional phasor image by rotating and
/// scaling G and S coordinates by phase (φ) and modulation (M) respectively using:
///
/// ```text
/// g = M * cos(φ)
/// s = M * sin(φ)
/// G' = G * g - S * s
/// S' = G * s + S * g
/// ```
///
/// Where G' and S' are the calibrated real and imaginary values after rotation
/// and scaling.
///
/// This function creates a new array and does not mutate the input array.
///
/// # Arguments
///
/// * `data`: The 3-dimensional phasor image, where G and S are channels 0 and 1
///    respectively.
/// * `modulation`: The modulation to scale the input (G, S) coordinates.
/// * `phase`: The phase, φ angle, to rotate the input (G, S) coordinates.
/// * `axis`: The channel axis, default = 2.
///
/// # Returns
///
/// * `Array3<f64>`: A 3-dimensional array with the calibrated phasor values,
///    where calibrated G and S are channels 0 and 1 respectively.
pub fn image<T>(
    data: ArrayView3<T>,
    modulation: f64,
    phase: f64,
    axis: Option<usize>,
) -> Array3<f64>
where
    T: ToFloat64,
{
    // set optional parameters if needed
    let a = axis.unwrap_or(2);

    // allocate new array of the same shape for calibrated data
    let shape = data.dim();
    let mut c_data = Array3::<f64>::zeros(shape);

    // read input data and save calibration to the new array
    let g_trans = modulation * phase.cos();
    let s_trans = modulation * phase.sin();
    let src_lanes = data.lanes(Axis(a));
    let dst_lanes = c_data.lanes_mut(Axis(a));
    Zip::from(src_lanes)
        .and(dst_lanes)
        .par_for_each(|s_ln, mut d_ln| {
            d_ln[0] = s_ln[0].into() * g_trans - s_ln[1].into() * s_trans;
            d_ln[1] = s_ln[0].into() * s_trans + s_ln[1].into() * g_trans;
        });

    c_data
}

/// Calibrate the real and imaginary (G, S) coordinates of a 3-dimensional phasor
/// image.
///
/// # Description
///
/// This function calibrates an input 3-dimensional phasor image by rotating and
/// scaling G and S coordinates by phase (φ) and modulation (M) respectively using:
///
/// ```text
/// g = M * cos(φ)
/// s = M * sin(φ)
/// G' = G * g - S * s
/// S' = G * s + S * g
/// ```
///
/// Where G' and S' are the calibrated real and imaginary values after rotation
/// and scaling.
///
/// This function mutates the input array and does not create a new array.
///
/// # Arguments
///
/// * `data`: The 3-dimensional phasor image, where G and S are channels 0 and 1
///    respectively.
/// * `modulation`: The modulation to scale the input (G, S) coordinates.
/// * `phase`: The phase, φ angle, to rotate the input (G, S) coordinates.
/// * `axis`: The channel axis, default = 2.
pub fn image_mut(mut data: ArrayViewMut3<f64>, modulation: f64, phase: f64, axis: Option<usize>) {
    // set optional axis parameter if needed
    let a = axis.unwrap_or(2);

    // initialize calibration parameters
    let g_trans = modulation * phase.cos();
    let s_trans = modulation * phase.sin();

    let lanes = data.lanes_mut(Axis(a));
    lanes.into_iter().par_bridge().for_each(|mut ln| {
        let g_cal = ln[0] * g_trans - ln[1] * s_trans;
        let s_cal = ln[0] * s_trans + ln[1] * g_trans;
        ln[0] = g_cal;
        ln[1] = s_cal;
    });
}

/// Find the modulation and phase calibration values.
///
/// # Description
///
/// This function calculates the modulation and phase calibration values from
/// theoretical monoexponential coordinates (computed from `tau` and
/// `omega`) and measured coordinates. The output, (M, φ), are the
/// modulation and phase values to calibrate with.
///
/// # Arguments
///
/// * `g`: The measured real (G) value.
/// * `s`: The measured imaginary (S) value.
/// * `tau`: The lifetime, τ.
/// * `omega`: The angular frequency, ω.
///
/// # Returns
///
/// * `(f64, f64)`: The modulation and phase calibration values, (M, φ).
pub fn modulation_and_phase(g: f64, s: f64, tau: f64, omega: f64) -> (f64, f64) {
    // get calibration modulation and phase
    let cal_point = plot::monoexponential_coordinates(tau, omega);
    let cal_mod = plot::modulation(cal_point.0, cal_point.1);
    let cal_phs = plot::phase(cal_point.0, cal_point.1);

    // get data modulation and phase
    let data_mod = plot::modulation(g, s);
    let data_phs = plot::phase(g, s);

    // find delta values
    let d_mod = cal_mod / data_mod;
    let d_phs = cal_phs - data_phs;

    (d_mod, d_phs)
}
