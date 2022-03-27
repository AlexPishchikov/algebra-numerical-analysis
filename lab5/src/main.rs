extern crate nalgebra as na;

use na::{DMatrix, DVector};

fn main() {
    let matrix_row = [1.0, 2.0, 4.0,
                      3.0, 3.0, 2.0,
                      4.0, 1.0, 3.0];

    let vector_row = [0.0, 0.0, 31.0];
    let n = 3;
    let A = DMatrix::from_row_slice(n, n, &matrix_row);
    let f = DVector::from_row_slice(&vector_row);

    println!("A = {} f = {}", A, f);

    let mut Q = DMatrix::from_element(n, n, 0.0);

    for i in 0..n {
        let mut q : DVector<f32> = A.column(i).into();
        for k in 0..i {
            let coef = (A.column(i)).dot(&Q.column(k)) as f32 / (Q.column(k)).dot(&Q.column(k)) as f32;
            q = q - coef * Q.column(k).clone();
        }
        Q.set_column(i, &q);
    }

    for i in 0..n {
        let mut q : na::DVector<f32> = Q.column(i).into();
        q.normalize_mut();
        Q.set_column(i, &q);
    }

    println!("Q = {}", Q);

    let R = Q.transpose() * A;
    let b = Q.transpose() * f;

    println!("R = {}", R);
    println!("b = {}", b);

    let mut xs = DVector::from_element(n, 0.0);
    for i in (0..n).rev() {
        let mut temp = DVector::from_element(n, 0.0);
        for k in (i + 1)..n {
            temp[k] = R[(i, k)] * xs[k];
        }
        xs[i] = (b[i] - temp.sum()) / R[(i, i)];
    }

    println!("x = {}", xs);
}
