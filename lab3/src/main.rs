extern crate nalgebra as na;

use na::{DMatrix, DVector};

fn main() {
    let matrix_row = [1.0, 0.0, 3.0, -4.0, 1.0, 333.0, 6.0, 99.0, 2.0];
    let vector_row = [11.0, 2.0, 3.0];
    let n = 3;
    let A = DMatrix::from_row_slice(n, n, &matrix_row);
    let f = DVector::from_row_slice(&vector_row);

    println!("A = {} f = {}", A, f);

    let mut L = DMatrix::from_element(n, n, 0.0);
    let mut U = DMatrix::from_element(n, n, 0.0);

    for i in 0..n {
        for j in i..n {
            let mut temp = DVector::from_element(n, 0.0);
            for k in 0..n {
                temp[k] = L[(i, k)] * U[(k, j)];
            }
            U[(i, j)] = A[(i, j)] - temp.sum();
        }
        for j in i..n {
            let mut temp = DVector::from_element(n, 0.0);
            for k in 0..n {
                temp[k] = L[(j, k)] * U[(k, i)];
            }
            L[(j, i)] = (A[(j, i)] - temp.sum()) / U[(i, i)];
        }
    }

    println!("L = {} U = {}", L, U);

    let mut ys = DVector::from_element(n, 0.0);
    for i in 0..n {
        let mut temp = DVector::from_element(n, 0.0);
        for k in 0..i {
            temp[k] = L[(i, k)] * ys[k];
        }
        ys[i] = f[i] - temp.sum();
    }

    let mut xs = DVector::from_element(n, 0.0);
    for i in (0..n).rev() {
        let mut temp = DVector::from_element(n, 0.0);
        for k in (i + 1)..n {
            temp[k] = U[(i, k)] * xs[k];
        }
        xs[i] = (ys[i] - temp.sum()) / U[(i, i)];
    }

    println!("x = {}", xs);
}
