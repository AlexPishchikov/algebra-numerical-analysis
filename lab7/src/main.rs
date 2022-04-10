extern crate nalgebra as na;
use na::{DMatrix, DVector};

fn main() {
    let matrix_row = [10.0, 1.0, 1.0,
                      2.0, 10.0, 1.0,
                      2.0, 2.0, 10.0];

    let vector_row = [12.0, 13.0, 14.0];

    let n = 3;
    let A : DMatrix<f32> = DMatrix::from_row_slice(n, n, &matrix_row);
    let f = DVector::from_row_slice(&vector_row);

    let precision : f32 = 0.000001;

    println!("A = {} f = {}", A, f);

    let mut xs = DVector::from_element(n, 0.0);
    loop {
        let diff = 0.1 * (&f - &A * &xs);
        xs = &xs + &diff;
        if diff.norm() < precision {
            break;
        }
    }

    println!("x = {}", xs);
}
