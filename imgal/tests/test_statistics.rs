use imgal::statistics;

#[test]
fn statistics_sum() {
    // create some test vecs
    let int_data = vec![2, 5, 10, 23];
    let float_data = vec![1.0, 10.5, 3.25, 37.11];

    // assert arrays
    assert_eq!(statistics::sum(&int_data), 40);
    assert_eq!(statistics::sum(&float_data), 51.86);
}

#[test]
fn statistics_weighted_merge_sort_mut() {
    // create data and associated weights
    let mut d: [i32; 5] = [3, 10, 87, 22, 5];
    let mut w: [f64; 5] = [0.51, 12.83, 4.24, 9.25, 0.32];

    // sort the data and weights, get inversion count
    let s = statistics::weighted_merge_sort_mut(&mut d, &mut w).unwrap();

    // check arrays are sorted
    assert_eq!(d, [3, 5, 10, 22, 87]);
    assert_eq!(w, [0.51, 0.32, 12.83, 9.25, 4.24]);
    assert_eq!(s, 47.64239999999998);
}
