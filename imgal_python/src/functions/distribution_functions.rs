use numpy::{IntoPyArray, PyArray1};
use pyo3::prelude::*;

use imgal_core::distribution;

/// 1-dimensional gaussian curve with a controllable center.
#[pyfunction]
#[pyo3(name = "gaussian")]
pub fn distribution_gaussian(
    py: Python,
    sigma: f64,
    bins: usize,
    range: f64,
    center: f64,
) -> PyResult<Bound<PyArray1<f64>>> {
    let output = distribution::gaussian(sigma, bins, range, center);
    Ok(output.into_pyarray(py))
}
