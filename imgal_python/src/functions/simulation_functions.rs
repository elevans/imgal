use numpy::{
    IntoPyArray, PyArray1, PyArray3, PyArrayMethods, PyReadonlyArray1, PyReadonlyArray3,
    PyReadwriteArray1, PyReadwriteArray3,
};
use pyo3::prelude::*;

use crate::error::map_array_error;
use imgal::simulation;

/// Simulate a 1-dimensional Gaussian IRF convolved monoexponential or
/// multiexponential decay curve.
///
/// This function generates a 1-dimensonal Gaussian instrument response function
/// (IRF) convolved monoexponential or multiexponential decay curve. The ideal
/// decay curve is defined as the sum of one or more exponential components,
/// each characterized by a lifetime (tau) and fractional intensity:
///
/// I(t) = Σᵢ αᵢ × exp(-t/τᵢ)
///
/// :param samples: The number of discrete points that make up the decay curve.
/// :param period: The period (_i.e._ time interval).
/// :param taus: An array of lifetimes. For a monoexponential decay curve use a
///     single tau value and a fractional intensity of 1.0. For a
///     multiexponential decay curve use two or more tau values, matched with
///     their respective fractional intensity. The "taus" and "fractions" arrays
///     must have the same length. Tau values set to 0.0 will be skipped.
/// :param fractions: An array of fractional intensities for each tau in the "taus"
///     array. The "fractions" array must be the same length as the "taus" array
///     and sum to 1.0. Fraction values set to 0.0 will be skipped.
/// :param total_counts: The total intensity count (_e.g._ photon count) of the
///     decay curve.
/// :param irf_center: The temporal position of the IRF peak within the time range.
/// :param irf_width: The full width at half maximum (FWHM) of the IRF.
/// :return: The 1-dimensonal Gaussian IRF convolved monoexponential
///     or multiexponential decay curve.
#[pyfunction]
#[pyo3(name = "gaussian_exponential_1d")]
pub fn decay_gaussian_exponential_1d(
    py: Python,
    samples: usize,
    period: f64,
    taus: Vec<f64>,
    fractions: Vec<f64>,
    total_counts: f64,
    irf_center: f64,
    irf_width: f64,
) -> PyResult<Bound<PyArray1<f64>>> {
    simulation::decay::gaussian_exponential_1d(
        samples,
        period,
        &taus,
        &fractions,
        total_counts,
        irf_center,
        irf_width,
    )
    .map(|output| output.into_pyarray(py))
    .map_err(map_array_error)
}

/// Simulate a 3-dimensional Gaussian IRF convolved monoexponential or
/// multiexponential decay curve.
///
/// This function generates a 3-dimensonal Gaussian instrument response function
/// (IRF) convolved monoexponential or multiexponential decay curve. The ideal
/// decay curve is defined as the sum of one or more exponential components,
/// each characterized by a lifetime (tau) and fractional intensity:
///
/// I(t) = Σᵢ αᵢ × exp(-t/τᵢ)
///
/// :param samples: The number of discrete points that make up the decay curve.
/// :param period: The period (_i.e._ time interval).
/// :param taus: An array of lifetimes. For a monoexponential decay curve use a
///     single tau value and a fractional intensity of 1.0. For a
///     multiexponential decay curve use two or more tau values, matched with
///     their respective fractional intensity. The "taus" and "fractions" arrays
///     must have the same length. Tau values set to 0.0 will be skipped.
/// :param fractions: An array of fractional intensities for each tau in the "taus"
///     array. The "fractions" array must be the same length as the "taus" array
///     and sum to 1.0. Fraction values set to 0.0 will be skipped.
/// :param total_counts: The total intensity count (_e.g._ photon count) of the
///     decay curve.
/// :param irf_center: The temporal position of the IRF peak within the time range.
/// :param irf_width: The full width at half maximum (FWHM) of the IRF.
/// :param shape: The row and col shape to broadcast the decay curve into.
/// :return: The 3-dimensional Gaussian IRF convolved monoexponential
///     or multiexponential decay curve.
#[pyfunction]
#[pyo3(name = "gaussian_exponential_3d")]
pub fn decay_gaussian_exponential_3d(
    py: Python,
    samples: usize,
    period: f64,
    taus: Vec<f64>,
    fractions: Vec<f64>,
    total_counts: f64,
    irf_center: f64,
    irf_width: f64,
    shape: (usize, usize),
) -> PyResult<Bound<PyArray3<f64>>> {
    simulation::decay::gaussian_exponential_3d(
        samples,
        period,
        &taus,
        &fractions,
        total_counts,
        irf_center,
        irf_width,
        shape,
    )
    .map(|output| output.into_pyarray(py))
    .map_err(map_array_error)
}

/// Simulate an ideal 1-dimensional monoexponential or multiexponential decay
/// curve.
///
/// This function generates a 1-dimensonal ideal exponential decay curve by
/// computing the sum of one or more exponential components, each characterized
/// by a lifetime (tau) and fractional intensity as defined by:
///
/// I(t) = Σᵢ αᵢ × exp(-t/τᵢ)
///
/// where αᵢ are the pre-exponential factors derived from the fractional
/// intensities and lifetimes.
///
/// :param samples: The number of discrete points that make up the decay curve.
/// :param period: The period (_i.e._ time interval).
/// :param taus: An array of lifetimes. For a monoexponential decay curve use a
///     single tau value and a fractional intensity of 1.0. For a
///     multiexponential decay curve use two or more tau values, matched with
///     their respective fractional intensity. The "taus" and "fractions" arrays
///     must have the same length. Tau values set to 0.0 will be skipped.
/// :param fractions: An array of fractional intensities for each tau in the "taus"
///     array. The "fractions" array must be the same length as the "taus" array
///     and sum to 1.0. Fraction values set to 0.0 will be skipped.
/// :param total_counts: The total intensity count (_e.g._ photon count) of the
///     decay curve.
/// :return: The 1-dimensonal monoexponential or multiexponential
///     decay curve.
#[pyfunction]
#[pyo3(name = "ideal_exponential_1d")]
pub fn decay_ideal_exponential_1d(
    py: Python,
    samples: usize,
    period: f64,
    taus: Vec<f64>,
    fractions: Vec<f64>,
    total_counts: f64,
) -> PyResult<Bound<PyArray1<f64>>> {
    simulation::decay::ideal_exponential_1d(samples, period, &taus, &fractions, total_counts)
        .map(|output| output.into_pyarray(py))
        .map_err(map_array_error)
}

/// Simulate an ideal 3-dimensional monoexponential or multiexponential decay
/// curve.
///
/// This function generates a 3-dimensonal ideal exponential decay curve by
/// computing the sum of one or more exponential components, each characterized
/// by a lifetime (tau) and fractional intensity as defined by:
///
/// I(t) = Σᵢ αᵢ × exp(-t/τᵢ)
///
/// where αᵢ are the pre-exponential factors derived from the fractional
/// intensities and lifetimes.
///
/// <https://doi.org/10.1111/j.1749-6632.1969.tb56231.x>
///
/// :param samples: The number of discrete points that make up the decay curve.
/// :param period: The period (_i.e._ time interval).
/// :param taus: An array of lifetimes. For a monoexponential decay curve use a
///     single tau value and a fractional intensity of 1.0. For a
///     multiexponential decay curve use two or more tau values, matched with
///     their respective fractional intensity. The "taus" and "fractions" arrays
///     must have the same length. Tau values set to 0.0 will be skipped.
/// :param fractions: An array of fractional intensities for each tau in the "taus"
///     array. The "fractions" array must be the same length as the "taus" array
///     and sum to 1.0. Fraction values set to 0.0 will be skipped.
/// :param total_counts: The total intensity count (_e.g._ photon count) of the
///     decay curve.
/// :param shape: The row and col shape to broadcast the decay curve into.
/// :return: The 3-dimensonal monoexponential or multiexponential
///     decay curve.
#[pyfunction]
#[pyo3(name = "ideal_exponential_3d")]
pub fn decay_ideal_exponential_3d(
    py: Python,
    samples: usize,
    period: f64,
    taus: Vec<f64>,
    fractions: Vec<f64>,
    total_counts: f64,
    shape: (usize, usize),
) -> PyResult<Bound<PyArray3<f64>>> {
    simulation::decay::ideal_exponential_3d(samples, period, &taus, &fractions, total_counts, shape)
        .map(|output| output.into_pyarray(py))
        .map_err(map_array_error)
}

/// Simulate a 1-dimensional IRF convolved monoexponential or multiexponential
/// decay curve.
///
/// This function generates a 1-dimensonal instrument response function (IRF)
/// convolved monoexponential or multiexponential decay curve. The ideal
/// decay curve is defined as the sum of one or more exponential components,
/// each characterized by a lifetime (tau) and fractional intensity:
///
/// I(t) = Σᵢ αᵢ × exp(-t/τᵢ)
///
/// :param irf: The IRF as a 1-dimensonal array.
/// :param samples: The number of discrete points that make up the decay curve.
/// :param period: The period (_i.e._ time interval).
/// :param taus: An array of lifetimes. For a monoexponential decay curve use a
///     single tau value and a fractional intensity of 1.0. For a
///     multiexponential decay curve use two or more tau values, matched with
///     their respective fractional intensity. The "taus" and "fractions" arrays
///     must have the same length. Tau values set to 0.0 will be skipped.
/// :param fractions: An array of fractional intensities for each tau in the "taus"
///     array. The "fractions" array must be the same length as the "taus" array
///     and sum to 1.0. Fraction values set to 0.0 will be skipped.
/// :param total_counts: The total intensity count (_e.g._ photon count) of the
///     decay curve.
/// :return: The 1-dimensional IRF convolved monoexponential or
///     multiexponential decay curve.
#[pyfunction]
#[pyo3(name = "irf_exponential_1d")]
pub fn decay_irf_exponential_1d(
    py: Python,
    irf: Vec<f64>,
    samples: usize,
    period: f64,
    taus: Vec<f64>,
    fractions: Vec<f64>,
    total_counts: f64,
) -> PyResult<Bound<PyArray1<f64>>> {
    simulation::decay::irf_exponential_1d(&irf, samples, period, &taus, &fractions, total_counts)
        .map(|output| output.into_pyarray(py))
        .map_err(map_array_error)
}

/// Simulate a 3-dimensional IRF convolved monoexponential or multiexponential
/// decay curve.
///
/// This function generates a 3-dimensonal instrument response function (IRF)
/// convolved monoexponential or multiexponential decay curve. The ideal
/// decay curve is defined as the sum of one or more exponential components,
/// each characterized by a lifetime (tau) and fractional intensity:
///
/// I(t) = Σᵢ αᵢ × exp(-t/τᵢ)
///
/// :param irf: The IRF as a 1-dimensonal array.
/// :param samples: The number of discrete points that make up the decay curve.
/// :param period: The period (_i.e._ time interval).
/// :param taus: An array of lifetimes. For a monoexponential decay curve use a
///     single tau value and a fractional intensity of 1.0. For a
///     multiexponential decay curve use two or more tau values, matched with
///     their respective fractional intensity. The "taus" and "fractions" arrays
///     must have the same length. Tau values set to 0.0 will be skipped.
/// :param fractions: An array of fractional intensities for each tau in the "taus"
///     array. The "fractions" array must be the same length as the "taus" array
///     and sum to 1.0. Fraction values set to 0.0 will be skipped.
/// :param total_counts: The total intensity count (_e.g._ photon count) of the
///     decay curve.
/// :param shape: The row and col shape to broadcast the decay curve into.
/// :return: The 3-dimensional IRF convolved monoexponential or
///     multiexponential decay curve.
#[pyfunction]
#[pyo3(name = "irf_exponential_3d")]
pub fn decay_irf_exponential_3d(
    py: Python,
    irf: Vec<f64>,
    samples: usize,
    period: f64,
    taus: Vec<f64>,
    fractions: Vec<f64>,
    total_counts: f64,
    shape: (usize, usize),
) -> PyResult<Bound<PyArray3<f64>>> {
    simulation::decay::irf_exponential_3d(
        &irf,
        samples,
        period,
        &taus,
        &fractions,
        total_counts,
        shape,
    )
    .map(|output| output.into_pyarray(py))
    .map_err(map_array_error)
}

/// Simulate a 1-dimensional Gaussian instruement response function (IRF).
///
/// This function creates a Gaussian IRF by converting "full width at half maximum"
/// (FWHM) parameters into a normalized Gaussian distribution. The FWHM is
/// converted to standard deviation using the relationship:
///
/// σ = FWHM / (2 × √(2 × ln(2)))
///
/// where ln(2) ≈ 0.693147 is the natural logarithm of 2.
///
/// :param bins: The number of discrete points to sample the Gaussian distribution.
/// :param time_range: The total time range over which to simulate the IRF.
/// :param irf_center: The temporal position of the IRF peak within the time range.
/// :param irf_width: The full width at half maximum (FWHM) of the IRF.
/// :return : The simulated 1-dimensional IRF curve.
#[pyfunction]
#[pyo3(name = "gaussian_irf_1d")]
pub fn instrument_gaussian_irf_1d(
    py: Python,
    bins: usize,
    time_range: f64,
    irf_center: f64,
    irf_width: f64,
) -> PyResult<Bound<PyArray1<f64>>> {
    let output = simulation::instrument::gaussian_irf_1d(bins, time_range, irf_center, irf_width);
    Ok(output.into_pyarray(py))
}

/// Simulate Poisson noise on a 1-dimensional array.
///
/// The function applies Poisson noise (i.e. shot noise) on a 1-dimensional
/// array of data. An element-wise lambda value (scaled by the "scale" parameter)
/// is used to simulate the Poisson noise with variable signal strength.
///
/// The function creates a new array and does not mutate the input array.f
///
/// :param data: The input 1-dimensional array.
/// :param scale: The scale factor.
/// :param seed: Pseudorandom number generator seed. Set the "seed" value to apply
///     homogenous noise to the input array. If "None", then heterogenous noise
///     is applied to the input array.
/// :return: A 1-dimensonal array of the input data with Poisson noise applied.
#[pyfunction]
#[pyo3(name = "poisson_1d")]
#[pyo3(signature = (data, scale, seed=None))]
pub fn noise_poisson_1d<'py>(
    py: Python<'py>,
    data: Bound<'py, PyAny>,
    scale: f64,
    seed: Option<u64>,
) -> PyResult<Bound<'py, PyArray1<f64>>> {
    // pattern match and extract allowed array types
    if let Ok(arr) = data.extract::<PyReadonlyArray1<u8>>() {
        let output = simulation::noise::poisson_1d(arr.as_slice().unwrap(), scale, seed);
        return Ok(output.into_pyarray(py));
    } else if let Ok(arr) = data.extract::<PyReadonlyArray1<u16>>() {
        let output = simulation::noise::poisson_1d(arr.as_slice().unwrap(), scale, seed);
        return Ok(output.into_pyarray(py));
    } else if let Ok(arr) = data.extract::<PyReadonlyArray1<f32>>() {
        let output = simulation::noise::poisson_1d(arr.as_slice().unwrap(), scale, seed);
        return Ok(output.into_pyarray(py));
    } else if let Ok(arr) = data.extract::<PyReadonlyArray1<f64>>() {
        let output = simulation::noise::poisson_1d(arr.as_slice().unwrap(), scale, seed);
        return Ok(output.into_pyarray(py));
    } else {
        return Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "Unsupported array dtype, supported array dtypes are u8, u16, f32, and f64.",
        ));
    }
}

/// Simulate Poisson noise on a 1-dimensional array.
///
/// The function applies Poisson noise (i.e. shot noise) on a 1-dimensional
/// array of data. An element-wise lambda value (scaled by the "scale" parameter)
/// is used to simulate the Poisson noise with variable signal strength.
///
/// This function mutates the input array and does not create a new array.
///
/// :param data: The input 1-dimensonal array to mutate.
/// :param scale: The scale factor.
/// :param seed: Pseudorandom number generator seed. Set the "seed" value to apply
///     homogenous noise to the input array. If "None", then heterogenous noise
///     is applied to the input array.
#[pyfunction]
#[pyo3(name = "poisson_1d_mut")]
#[pyo3(signature= (data, scale, seed=None))]
pub fn noise_poisson_1d_mut(mut data: PyReadwriteArray1<f64>, scale: f64, seed: Option<u64>) {
    // get mutable slice, all 1D arrays are contiguous
    let d = data.as_slice_mut().unwrap();
    simulation::noise::poisson_1d_mut(d, scale, seed);
}

/// Simulate Poisson noise on a 3-dimensional array.
///
/// This function applies Poisson noise (i.e. shot noise) on a 3-dimensional
/// array of data. An element-wise lambda value (scaled by the "scale" parameter)
/// is used to simulate Poisson noise with variable signal strength.
///
/// This function creates a new array and does not mutate the input array.
///
///
/// :param data: The input 3-dimensional array.
/// :param scale: The scale factor.
/// :param seed: Pseudorandom number generator seed. Set the "seed" value to apply
///     homogenous noise to the input array. If "None", then heterogenous noise
///     is applied to the input array.
/// :param axis: The signal data axis, default = 2.
/// :return: A 3-dimensional array of the input data with Poisson noise
///     applied.
#[pyfunction]
#[pyo3(name = "poisson_3d")]
#[pyo3(signature = (data, scale, seed=None, axis=None))]
pub fn noise_poisson_3d<'py>(
    py: Python<'py>,
    data: Bound<'py, PyAny>,
    scale: f64,
    seed: Option<u64>,
    axis: Option<usize>,
) -> PyResult<Bound<'py, PyArray3<f64>>> {
    // pattern match and extract allowed array types
    if let Ok(arr) = data.extract::<PyReadonlyArray3<u8>>() {
        simulation::noise::poisson_3d(arr.as_array(), scale, seed, axis)
            .map(|output| output.into_pyarray(py))
            .map_err(map_array_error)
    } else if let Ok(arr) = data.extract::<PyReadonlyArray3<u16>>() {
        simulation::noise::poisson_3d(arr.as_array(), scale, seed, axis)
            .map(|output| output.into_pyarray(py))
            .map_err(map_array_error)
    } else if let Ok(arr) = data.extract::<PyReadonlyArray3<f32>>() {
        simulation::noise::poisson_3d(arr.as_array(), scale, seed, axis)
            .map(|output| output.into_pyarray(py))
            .map_err(map_array_error)
    } else if let Ok(arr) = data.extract::<PyReadonlyArray3<f64>>() {
        simulation::noise::poisson_3d(arr.as_array(), scale, seed, axis)
            .map(|output| output.into_pyarray(py))
            .map_err(map_array_error)
    } else {
        return Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "Unsupported array dtype, supported array dtypes are u8, u16, f32, and f64.",
        ));
    }
}

/// Simulate Poisson noise on a 3-dimensional array.
///
/// This function applies Poisson noise (i.e. shot noise) on a 3-dimensional
/// array of data. An element-wise lambda value (scaled by the "scale" parameter)
/// is used to simulate Poisson noise with variable signal strength.
///
/// This function mutates the input array and does not create a new array.
///
/// :param data: The input 3-dimensional array to mutate.
/// :param scale: The scale factor.
/// :param seed: Pseudorandom number generator seed. Set the "seed" value to apply
///     homogenous noise to the input array. If "None", then heterogenous noise
///     is applied to the input array.
/// :param axis: The signal data axis, default = 2.
#[pyfunction]
#[pyo3(name = "poisson_3d_mut")]
#[pyo3(signature = (data, scale, seed=None, axis=None))]
pub fn noise_poisson_3d_mut(
    mut data: PyReadwriteArray3<f64>,
    scale: f64,
    seed: Option<u64>,
    axis: Option<usize>,
) {
    let arr = data.as_array_mut();
    simulation::noise::poisson_3d_mut(arr, scale, seed, axis);
}
