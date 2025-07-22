use ndarray::{ArrayBase, Data, Ix1, s};

use crate::traits::numeric::ToFloat64;

/// Integrate a curve with Simpson's 1/3 rule and the trapezoid rule.
///
/// # Description
///
/// Approximates the definite integral using Simpson's 1/3 rule and
/// the trapezoid rule (for odd number of subintervals) with pre-computed
/// x-values:
///
/// ```text
/// ∫(f(x)dx) ≈ (Δx/3) * [f(x₀) + 4f(x₁) + 2f(x₂) + 4f(x₃) + ... + 2f(xₙ₋₂) + 4f(xₙ₋₁) + f(xₙ)]
/// ```
///
/// Where "n" is the number of evenly spaced points in the data. If there is an
/// odd number of subintervals, the final subinterval is integrated using the
/// trapezoid rule:
///
/// ```text
/// ∫(f(x)dx) ≈ (Δx/2) * [f(x₀) + f(x₁)]
/// ```
///
/// # Arguments
///
/// * `x`: The 1-dimensional data to integrate.
/// * `delta_x`: The width between data points, default = 1.0.
///
/// # Returns
///
/// * `f64`: The computed integral.
pub fn composite_simpson<T, S>(x: &ArrayBase<S, Ix1>, delta_x: Option<f64>) -> f64
where
    T: ToFloat64,
    S: Data<Elem = T>,
{
    // set default delta x if necessary
    let d_x: f64 = delta_x.unwrap_or(1.0);
    // find the number of subintervals
    let n: usize = x.len() - 1;
    // check for even number of subintervals
    if n % 2 == 0 {
        simpson(x, delta_x).unwrap()
    } else {
        // compute the even subintervals with Simpson's rule
        let integral: f64 = simpson(&x.slice(s![..n]), delta_x).unwrap();
        // compute the last subinterval with a trapizoid
        let trap: f64 = (d_x / 2.0) * (x[n - 1] + x[n]).into();
        integral + trap
    }
}

/// Integrate a curve with Simpson's 1/3 rule.
///
/// # Description
///
/// Approximates the definite integral using Simpson's 1/3 rule and
/// with pre-computed x-values:
///
/// ```text
/// ∫(f(x)dx) ≈ (Δx/3) * [f(x₀) + 4f(x₁) + 2f(x₂) + 4f(x₃) + ... + 2f(xₙ₋₂) + 4f(xₙ₋₁) + f(xₙ)]
/// ```
///
/// Where "n" is the number of evenly spaced points in the data.
///
/// # Arguments
///
/// * `x`: The 1-dimensional data to integrate with an even number of subintervals.
/// * `delta_x`: The width between data points, default = 1.0.
///
/// # Returns
///
/// * `Ok(f64)`: The computed integral.
/// * `Err(&str)`: Error message if the number of subintervals is odd.
pub fn simpson<T, S>(x: &ArrayBase<S, Ix1>, delta_x: Option<f64>) -> Result<f64, &'static str>
where
    T: ToFloat64,
    S: Data<Elem = T>,
{
    // set default delta x if necessary
    let d_x: f64 = delta_x.unwrap_or(1.0);
    // find the number of subintervals
    let n: usize = x.len() - 1;
    // check for even number of subintervals
    if n % 2 == 0 {
        // compute integal with Simpson's rule
        let mut coef: f64;
        let mut integral: f64 = (x[0] + x[n]).into();
        for i in 1..n {
            coef = if i % 2 == 1 { 4.0 } else { 2.0 };
            integral += coef * x[i].into();
        }
        Ok((d_x / 3.0) * integral)
    } else {
        Err("Odd number of subintervals is not supported for Simpson's 1/3 rule.")
    }
}
