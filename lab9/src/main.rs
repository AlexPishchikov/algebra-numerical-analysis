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

    let omega : f32 = 1.1;

    let mut xs = DVector::from_element(n, 0.0);
    let mut xs_prev = xs.clone();

    loop {
        for i in 0..n {
            let mut sum1 = 0.0;
            let mut sum2 = 0.0;
            for k in 0..i {
                sum1 += A[(i, k)] * xs[k];
            }
            for k in (i + 1)..n {
                sum2 += A[(i, k)] * xs_prev[k];
            }
            xs[i] = omega * (f[i] - sum1 - sum2) / A[(i, i)] + (1.0 - omega) * xs_prev[i];
        }

        xs_prev = xs.clone();
        if (&A * &xs - &f).norm() < precision {
            break;
        }
    }

    println!("x = {}", xs);
}
