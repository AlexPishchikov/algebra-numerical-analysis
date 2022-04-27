extern crate nalgebra as na;
use na::{DMatrix, DVector};

use std::f32::consts::PI;

fn main() {
    let matrix_row = [10.0, 1.0, 1.0,
                      2.0, 10.0, 1.0,
                      2.0, 2.0, 15.0];

    let vector_row = [12.0, 13.0, 19.0];

    let n = 3;
    let A : DMatrix<f32> = DMatrix::from_row_slice(n, n, &matrix_row);
    let f = DVector::from_row_slice(&vector_row);

    println!("A = {} f = {}", A, f);


    let mut xs : DVector<f32> = DVector::from_element(n, 0.0).into();

    let a : f32 = 8.610447;
    let b : f32 = 15.9018;

    let k = 8;

    for i in 0..k {
        let tau = 2.0 / ((a + b) + (b - a) * ((2.0 * i as f32 - 1.0) * PI / (2.0 * k as f32)).cos());
        xs = &xs + tau * (&f - &A * &xs);
    }

    println!("x = {}", xs);

    // println!("{}", (A * xs - f).norm());
}
