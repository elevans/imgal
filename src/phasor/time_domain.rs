use parameter;

use std::f64::cos;

// TODO imaginary compute

/// Compute the real or G component of lifetime data.
///
/// The real, G, component time domain equation is calculated
/// using the following transformation:
///
/// G = ∫(data * cos(nωt) * dt) / ∫(data * dt)
///
/// Where 'n' and 'ω' are harmonic and omega values respectively.
///
/// # Arguments
///
/// # Returns
pub fn real(
    input: &[f64],
    bins: &i32,
    period: &f64,
    harmonic: &Option<f64>,
    omega: &Option<f64>,
    integration_time: &Option<Vec<f64>>,
) -> f64{
    // compute and set optional paramters if needed
    let h: f64 = harmonic.unwrap_or(1.0);
    let w: f64 = omega.unwrap_or_else(parameter::omega(period));
    let t: Vec<f64> = integration_time.unwrap_or_else(calculate_integration_time(bins, period));

    // compute the "real" integral
    let mut sum_data: f64 = 0.0;
    let mut g: f64 = 0.0;
    for (i, &value) in input.enumerate() {
        sum_data += value;
        g += value * cos(h * w * t[i]);
    }
    g /= sum_data
}

fn calculate_integration_time(
    bins: &i32,
    period: &f64,
) -> Vec<f64> {
    let mut time = vec![0.0; bins];
    let dt: f64 = period / bins;
    for i in 0..bins {
        time[i] = i * dt;
    }
    time
}
