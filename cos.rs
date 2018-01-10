use std::f64::consts;

fn main() {
    let x:f64 = 2.0 * consts::PI;
    let abs_difference = (x.cos() - 1.0).abs();
    assert!(abs_difference < 1.0e-10);
}
