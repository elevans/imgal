use crate::traits::numeric::ToFloat64;

/// Compute the Abbe diffraction limit.
///
/// # Description
///
/// This function computes Ernst Abbe's diffraction limit
/// for a microscope using:
///
/// ```text
/// d = wavelength / 2 * NA
/// ```
///
/// Where NA is the numerical aperture of the objective.
///
/// # Arguments
///
/// * `wavelength`: The wavelength of light in nanometers.
/// * `na`: The numerical aperture.
///
/// # Returns
///
/// * `f64`: Abbe's diffraction limit.
pub fn abbe_diffraction_limit<T>(wavelength: T, na: f64) -> f64
where
    T: ToFloat64,
{
    wavelength.to_f64() / (2.0 * na)
}
