extern crate nalgebra as na;
use na::{DMatrix, DVector};

pub fn seidel(A : DMatrix<f32>, f : DVector<f32>, n : usize, precision : f32) -> DVector<f32> {
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
            xs[i] = (f[i] - sum1 - sum2) / A[(i, i)];
        }

        xs_prev = xs.clone();

        if (&A * &xs - &f).norm() < precision {
            break;
        }
    }

    return xs;
}
