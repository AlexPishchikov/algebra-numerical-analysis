extern crate nalgebra as na;
use na::{DMatrix, DVector};

pub fn jacobi(A : DMatrix<f32>, f : DVector<f32>, n : usize, precision : f32) -> DVector<f32> {
    let mut xs = DVector::from_element(n, 0.0);
    loop {
        for i in 0..n {
            let mut sum = 0.0;
            for k in 0..n {
                if i != k {
                    sum += A[(i, k)] * xs[k];
                }
            }
            xs[i] = (f[i] - sum) / A[(i, i)];
        }

        if (&A * &xs - &f).norm() < precision {
            break;
        }
    }

    return xs;
}
