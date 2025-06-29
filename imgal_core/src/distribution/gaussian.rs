use ndarray::Array1;

use crate::statistics::sum;

pub fn gaussian(sigma: f64, bins: usize, range: f64, center: f64) -> Array1<f64> {
    // create data range (i.e. time) and gaussian arrays
    let mut r = Array1::<f64>::zeros(bins);
    let mut g = Array1::<f64>::zeros(bins);

    // calculate range data
    let width = range / (bins as f64 - 1.0);
    r.iter_mut().enumerate().for_each(|(i, v)| {
        *v = i as f64 * width;
    });

    // calculate the gaussian distrubtion
    let sigma_sq_2 = 2.0 * sigma.powi(2);
    g.iter_mut().enumerate().for_each(|(i, v)| {
        *v = (-((r[i] - center).powi(2)) / sigma_sq_2).exp();
    });

    // normalize the gaussian distribution
    let g_sum = sum(g.as_slice().unwrap());
    g.iter_mut().for_each(|v| {
        *v /= g_sum;
    });
    g
}
