use ndarray::Array2;

use crate::error::DimensionError;

/// Simulate a cell
///
/// Filled circles at points defined in `centers` and size defined by `radii` are
/// drawn according to the equation:
///
/// (x - a)² + (y -  b)² = r²
///
/// # Description
///
/// # Arguments
///
/// # Returns
/// (row, col)
pub fn cell_2d(
    centers: Vec<(usize, usize)>,
    radii: Vec<(usize, usize)>,
    cell_intensity: f64,
    background_intensity: f64,
    noise_scale: f64,
    shape: (usize, usize),
) -> Result<Array2<f64>, DimensionError> {
    // validate that the centers are within the image bounds
    let (row, col) = shape;
    for i in 0..centers.len() {
        let (y, x) = centers[i];
        validate_dim_size(y, row, 0)?;
        validate_dim_size(x, col, 1)?;
    }

    // create new array f64 zeros with shape
    let mut cell_arr = Array2::<f64>::zeros(shape);

    // draw filled circles
    // cell_arr.par_iter_mut().for_each(|p| {
        // check if point satifys circle equation
    // });
    Ok(cell_arr)
}

/// Helper function to validate the dimension size and
/// issue errors.
///
/// # Arguments
///
/// * `v`: Value to check.
/// * `r`: Reference value to check against.
/// * `a`: Axis index of the value to check.
fn validate_dim_size(v:usize, r: usize, a: usize) -> Result<(), DimensionError> {
    if v >= r {
        return Err(DimensionError::InvalidDimensionSize {
            dim_a: v,
            dim_b: r,
            axis_idx: a,
        });
    }

    Ok(())
}
