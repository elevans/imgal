use numpy::PyReadwriteArray1;
use pyo3::prelude::*;

use crate::error::map_array_error;
use imgal::statistics;

/// Compute the effective sample size (ESS) of a weighted sample set.
///
/// This function computes the effective sample size (ESS) of a weighted sample
/// set. Only the weights of the associated sample set are needed. The ESS is
/// defined as:
///
/// ESS = (Σ wᵢ)² / Σ (wᵢ²)
///
/// :param weights: A slice of non-negative weights where each element represents
///     the weight of an associated sample.
/// :return: The effective number of independent samples.
#[pyfunction]
#[pyo3(name = "effective_sample_size")]
pub fn statistics_effective_sample_size(weights: Vec<f64>) -> f64 {
    statistics::effective_sample_size(&weights)
}

/// Compute the sum of a sequence of numbers.
///
/// :param data: The sequence of numbers.
/// :return: The sum.
#[pyfunction]
#[pyo3(name = "sum")]
pub fn statistics_sum(data: Vec<f64>) -> f64 {
    statistics::sum(&data)
}

/// Compute the weighted Kendall's Tau-b rank correlation coefficient.
///
/// This function calculates a weighted Kendall's Tau-b rank correlation
/// coefficient between two datasets. This implementation uses a weighted merge
/// sort to count discordant pairs (inversions), and applies tie corrections for
/// both variables to compute the final Tau-b coefficient. Here the weighted
/// observations contribute unequally to the final correlation coefficient.
///
/// The weighted Kendall's Tau-b is calculated using:
///
/// τ_b = (C - D) / √((n₀ - n₁)(n₀ - n₂))
///
/// Where:
/// - `C` = number of weighted concordant pairs
/// - `D` = number of weighted discordant pairs
/// - `n₀` = total weighted pairs = `(Σwᵢ)² - Σwᵢ²`
/// - `n₁` = weighted tie correction for first variable
/// - `n₂` = weighted tie correction for second variable
///
/// :param data_a: The first dataset for correlation analysis. Must be the same
///     length as `data_b`.
/// :param data_b: The second dataset for correlation analysis. Must be the same
///     length as `data_a`.
/// :param weights: The associated weights for each observation pait. Must be the
///     same length as both input datasets.
/// :return: The weighted Kendall's Tau-b correlation coefficient, ranging
///     between -1.0 (negative correlation), 0.0 (no correlation) and 1.0
///     (positive correlation).
#[pyfunction]
#[pyo3(name = "weighted_kendall_tau_b")]
pub fn statistics_weighted_kendall_tau_b(
    data_a: Vec<f64>,
    data_b: Vec<f64>,
    weights: Vec<f64>,
) -> PyResult<f64> {
    statistics::weighted_kendall_tau_b(&data_a, &data_b, &weights)
        .map(|output| output)
        .map_err(map_array_error)
}

/// Sort 1-dimensional arrays of values and their associated weights.
///
/// This function performs a bottom up merge sort on the input 1-dimensional
/// data array along with it's associated weights. Both the "data" and "weights"
/// arrays are mutated during the sorting. The output of this function is a
/// weighted inversion count.
///
/// :param data: A 1-dimensional array/slice of numbers of the same length as
///    "weights".
/// :param weights: A 1-dimensional array/slice of weights of the same length as
///    "data".
/// :return: The number of swaps needed to sort the input array.
#[pyfunction]
#[pyo3(name = "weighted_merge_sort_mut")]
pub fn statistics_weighted_merge_sort_mut<'py>(
    data: Bound<'py, PyAny>,
    mut weights: PyReadwriteArray1<f64>,
) -> PyResult<f64> {
    // pattern match and extract the allowed array type
    if let Ok(mut d) = data.extract::<PyReadwriteArray1<f64>>() {
        return statistics::weighted_merge_sort_mut(
            d.as_slice_mut().unwrap(),
            weights.as_slice_mut().unwrap(),
        )
        .map(|output| output)
        .map_err(map_array_error);
    } else if let Ok(mut d) = data.extract::<PyReadwriteArray1<i32>>() {
        return statistics::weighted_merge_sort_mut(
            d.as_slice_mut().unwrap(),
            weights.as_slice_mut().unwrap(),
        )
        .map(|output| output)
        .map_err(map_array_error);
    } else {
        return Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "Unsupported array dtype.",
        ));
    }
}
