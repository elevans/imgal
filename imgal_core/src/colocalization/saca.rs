use ndarray::{Array2, Array3, ArrayView2, Axis, Zip};

use crate::error::ArrayError;

pub fn saca_2d<T>(
    data_a: ArrayView2<T>,
    data_b: ArrayView2<T>,
    mask_a: ArrayView2<bool>,
    mask_b: ArrayView2<bool>,
) -> Result<Array2<f64>, ArrayError> {
    // create image buffers
    // TODO make 2D output for now, final output should be 3D (heatmap + p-values)
    let shape = data_a.dim();
    let saca_arr = Array2::<f64>::zeros(shape);
    let mut new_tau_arr = Array2::<f64>::zeros(shape);
    let old_tau_arr = Array2::<f64>::zeros(shape);
    let mut new_sqrt_n_arr = Array2::<f64>::zeros(shape);
    let old_sqrt_n_arr = Array2::<f64>::ones(shape);
    let mut stop_arr = Array3::<f64>::zeros((shape.0, shape.1, 3));

    // set up saca parameters, see reference on "n" value selection for lambda
    let dn = ((shape.0 * shape.1) as f64).ln().sqrt() * 2.0;
    let lambda = dn * 1.0;
    let tu: usize = 15;
    let tl: usize = 8;
    let mut size_f: f64 = 1.0;
    let mut size_i: i32 = 1;
    let step_size: f64 = 1.15;
    let mut lower_bound_check = false;

    // try doing this rust style!
    (0..tu).for_each(|s| {
        size_i = size_f.floor() as i32;
        // TODO single iteration here
        size_f *= step_size;
        if s == tl {
            lower_bound_check = true;
            let lanes = stop_arr.lanes_mut(Axis(2));
            Zip::from(lanes)
                .and(new_tau_arr.view())
                .and(new_sqrt_n_arr.view())
                .par_for_each(|mut ln, nt, ns| {
                    ln[1] = *nt;
                    ln[2] = *ns;
                });
        }
    });

    Ok(saca_arr)
}

fn single_iteration() {}
