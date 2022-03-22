extern crate nalgebra as na;

use na::{DMatrix, DVector};

fn main() {
    let matrix_row = [2.0, 1.0,  4.0,
                      1.0, 1.0,  3.0,
                      4.0, 3.0, 14.0];

    let vector_row = [16.0, 12.0, 52.0];
    let n = 3;
    let A = DMatrix::from_row_slice(n, n, &matrix_row);
    let f = DVector::from_row_slice(&vector_row);

    println!("A = {} f = {}", A, f);

    let mut S = DMatrix::from_element(n, n, 0.0);

    S[(0, 0)] = (A[(0, 0)] as f32).sqrt();

    for i in 1..n {
        S[(0, i)] = A[(0, i)] / S[(0, 0)];
    }

    for i in 0..n {
        let mut temp = DVector::from_element(n, 0.0);
        for k in 0..i {
            temp[k] = (S[(k, i)] as f32).powf(2.0);
        }
        S[(i, i)] = (((A[(i, i)] - temp.sum())) as f32).abs().sqrt();
        for j in (i + 1)..n {
            if i != j {
                let mut temp = DVector::from_element(n, 0.0);
                for k in 0..i {
                    temp[k] = S[(k, i)] * S[(k, j)];
                }
                S[(i, j)] = (A[(i, j)] - temp.sum()) / S[(i, i)];
            }
        }
    }

    println!("S = {}", S);

    let mut ys = DVector::from_element(n, 0.0);
    ys[0] = f[0] / S[(0, 0)];
    for i in 1..n {
        let mut temp = DVector::from_element(n, 0.0);
        for k in 0..i {
            temp[k] = S[(k, i)] * ys[k];
        }
        ys[i] = (f[i] - temp.sum()) / S[(i, i)];
    }

    let mut xs = DVector::from_element(n, 0.0);
    for i in (0..n).rev() {
        let mut temp = DVector::from_element(n, 0.0);
        for k in (i + 1)..n {
            temp[k] = S[(i, k)] * xs[k];
        }
        xs[i] = (ys[i] - temp.sum()) / S[(i, i)];
    }

    println!("x = {}", xs);
}
