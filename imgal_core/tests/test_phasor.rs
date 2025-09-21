use ndarray::{Array2, Axis, s};

use imgal_core::parameter::omega;
use imgal_core::phasor::{calibration, plot, time_domain};
use imgal_core::simulation::{decay, noise};

// simulated bioexponential decay parameters
const SAMPLES: usize = 256;
const PERIOD: f64 = 12.5;
const TAUS: [f64; 2] = [1.0, 3.0];
const FRACTIONS: [f64; 2] = [0.7, 0.3];
const TOTAL_COUNTS: f64 = 5000.0;
const IRF_CENTER: f64 = 3.0;
const IRF_WIDTH: f64 = 0.5;
const SHAPE: (usize, usize) = (10, 10);
const MODULATION: f64 = 0.7;
const PHASE: f64 = -0.981;

// helper functions
fn ensure_within_tolerance(a: f64, b: f64, tolerance: f64) -> bool {
    (a - b).abs() < tolerance
}

fn get_circle_mask(shape: (usize, usize), center: (isize, isize), radius: isize) -> Array2<bool> {
    // set circle parameters
    let (row, col) = shape;
    let (cx, cy) = center;
    let r2 = radius * radius;
    let y_min = (cy - radius).max(0);
    let y_max = (cy + radius).min(row as isize - 1);
    let x_min = (cx - radius).max(0);
    let x_max = (cx + radius).min(col as isize - 1);

    // create empty bool array and a filled draw circle
    let mut mask = Array2::<bool>::default(shape);
    for y in y_min..=y_max {
        for x in x_min..=x_max {
            let dx = cx - x;
            let dy = cy - y;
            // use the squared distance formula for a quick circle mask
            if dx * dx + dy * dy <= r2 {
                mask[[y as usize, x as usize]] = true;
            }
        }
    }

    mask
}

// test the phasor::calibration
#[test]
fn calibration_coordinates() {
    // phasor coordinates to calibrate
    let g = -0.37067312732350316;
    let s = 0.6841432489903166;

    // set a modulation and phase value to calibrate with
    let coords_cal = calibration::coordinates(g, s, MODULATION, PHASE);

    assert_eq!(coords_cal, (0.2536762376620283, 0.48199495552386873));
}

#[test]
fn calibration_image() {
    // get simulated data
    let i = decay::gaussian_exponential_3d(
        SAMPLES,
        PERIOD,
        &TAUS,
        &FRACTIONS,
        TOTAL_COUNTS,
        IRF_CENTER,
        IRF_WIDTH,
        SHAPE,
    )
    .unwrap();

    // calculate the phasor image, (G, S)
    let gs_arr = time_domain::image(i.view(), PERIOD, None, None, None).unwrap();

    // calibrate the phasor image
    let cal_gs_arr = calibration::image(gs_arr.view(), MODULATION, PHASE, None);

    // pick a point in the calibrated data
    let g_mean = cal_gs_arr.index_axis(Axis(2), 0).mean().unwrap();
    let s_mean = cal_gs_arr.index_axis(Axis(2), 1).mean().unwrap();

    assert!(ensure_within_tolerance(g_mean, 0.2536762376620283, 1e-12));
    assert!(ensure_within_tolerance(s_mean, 0.48199495552386873, 1e-12));
}

#[test]
fn calibration_image_mut() {
    // get simulated data
    let sim_data = decay::gaussian_exponential_3d(
        SAMPLES,
        PERIOD,
        &TAUS,
        &FRACTIONS,
        TOTAL_COUNTS,
        IRF_CENTER,
        IRF_WIDTH,
        SHAPE,
    )
    .unwrap();

    // calculate the phasor image, (G, S)
    let mut gs_arr = time_domain::image(sim_data.view(), PERIOD, None, None, None).unwrap();

    // calibrate the phasor image
    calibration::image_mut(gs_arr.view_mut(), MODULATION, PHASE, None);

    // pick a point in the calibrated data
    let g_mean = gs_arr.index_axis(Axis(2), 0).mean().unwrap();
    let s_mean = gs_arr.index_axis(Axis(2), 1).mean().unwrap();

    assert!(ensure_within_tolerance(g_mean, 0.2536762376620283, 1e-12));
    assert!(ensure_within_tolerance(s_mean, 0.48199495552386873, 1e-12));
}

#[test]
fn calibration_modulation_and_phase() {
    // use 1.1 ns tau and 12.5 ns period
    let w = omega(PERIOD);
    let mod_phs = calibration::modulation_and_phase(-0.055, 0.59, 1.1, w);

    assert_eq!(mod_phs, (1.4768757234403935, -1.1586655116823268));
}

// test the phasor::plot module
#[test]
fn plot_modulation() {
    let m = plot::modulation(0.71, 0.43);

    assert_eq!(m, 0.8300602387778853);
}

#[test]
fn plot_phase() {
    let p = plot::phase(0.71, 0.43);

    assert_eq!(p, 0.5445517081560367);
}

#[test]
fn plot_monoexponential_coordinates() {
    // use 1.1 ns tau and 12.5 ns period
    let w = omega(PERIOD);
    let coords = plot::monoexponential_coordinates(1.1, w);

    assert_eq!(coords, (0.7658604730109534, 0.4234598078807387));
}

#[test]
fn plot_map_image() {
    // get simulated data
    let mut i = decay::gaussian_exponential_3d(
        SAMPLES,
        PERIOD,
        &TAUS,
        &FRACTIONS,
        TOTAL_COUNTS,
        IRF_CENTER,
        IRF_WIDTH,
        (50, 50),
    )
    .unwrap();
    noise::poisson_3d_mut(i.view_mut(), 0.3, None, None);

    // compute phasor array and select coordinates to map back
    let gs_arr = time_domain::image(i.view(), PERIOD, None, None, None).unwrap();
    let g_coords = gs_arr.slice(s![25..30, 25..30, 0]).flatten().to_vec();
    let s_coords = gs_arr.slice(s![25..30, 25..30, 1]).flatten().to_vec();

    // map the coords back to the image
    let mask = plot::map_image(gs_arr.view(), &g_coords, &s_coords, None).unwrap();

    // check a spot in mask and outside of it
    assert_eq!(mask[[28, 28]], true);
    assert_eq!(mask[[5, 5]], false);
}
// test the phasor::time_domain module
#[test]
fn time_domain_image() {
    // get simulated data
    let i = decay::gaussian_exponential_3d(
        SAMPLES,
        PERIOD,
        &TAUS,
        &FRACTIONS,
        TOTAL_COUNTS,
        IRF_CENTER,
        IRF_WIDTH,
        (100, 100),
    )
    .unwrap();

    // get simulated data and circle mask
    let mask = get_circle_mask((100, 100), (50, 50), 8);

    // compute phasors with and without a mask
    let gs_no_mask = time_domain::image(i.view(), PERIOD, None, None, None).unwrap();
    let gs_with_mask = time_domain::image(i.view(), PERIOD, Some(mask.view()), None, None).unwrap();

    // get views of each channel
    let g_no_mask_view = gs_no_mask.index_axis(Axis(2), 0);
    let s_no_mask_view = gs_no_mask.index_axis(Axis(2), 1);
    let g_with_mask_view = gs_with_mask.index_axis(Axis(2), 0);
    let s_with_mask_view = gs_with_mask.index_axis(Axis(2), 1);

    // expected uncalibrated values
    let exp_g = -0.37067312732350316;
    let exp_s = 0.6841432489903166;

    // assert G and S values, no mask
    assert!(ensure_within_tolerance(
        g_no_mask_view.mean().unwrap(),
        exp_g,
        1e-12
    ));
    assert!(ensure_within_tolerance(
        s_no_mask_view.mean().unwrap(),
        exp_s,
        1e-12
    ));

    // assert G, S and 0.0 values, with mask
    assert!(ensure_within_tolerance(
        g_with_mask_view[[45, 52]],
        exp_g,
        1e-12
    ));
    assert!(ensure_within_tolerance(
        s_with_mask_view[[45, 52]],
        exp_s,
        1e-12
    ));
    assert!(ensure_within_tolerance(
        g_with_mask_view[[5, 8]],
        0.0,
        1e-12
    ));
    assert!(ensure_within_tolerance(
        s_with_mask_view[[5, 8]],
        0.0,
        1e-12
    ));
}

#[test]
fn time_domain_imaginary() {
    let i = decay::ideal_exponential_1d(SAMPLES, PERIOD, &TAUS, &FRACTIONS, TOTAL_COUNTS).unwrap();
    let s = time_domain::imaginary(&i, PERIOD, None);

    assert_eq!(s, 0.4102178630685902);
}

#[test]
fn time_domain_real() {
    let i = decay::ideal_exponential_1d(SAMPLES, PERIOD, &TAUS, &FRACTIONS, TOTAL_COUNTS).unwrap();
    let g = time_domain::real(&i, PERIOD, None);

    assert_eq!(g, 0.6601376050345189);
}
