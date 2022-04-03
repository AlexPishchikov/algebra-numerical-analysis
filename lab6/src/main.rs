extern crate nalgebra as na;
use na::{DMatrix, DVector};

fn invert(A : &DMatrix<f32>, n : usize) -> DMatrix<f32> {
    if n == 1 {
        return DMatrix::from_element(1, 1, 1.0 / A[(0, 0)] as f32);
    }
    let A_submatrix : DMatrix<f32> = A.slice((0, 0), (A.nrows() - 1, A.nrows() - 1)).into();
    let A_submatrix_inv = invert(&A_submatrix, A_submatrix.nrows());

    let v = (A.row(A.nrows() - 1)).remove_columns(A_submatrix_inv.nrows(), 1);
    let u = (A.column(A.ncols() - 1)).remove_rows(A_submatrix_inv.ncols(), 1);

    let alpha : f32 = 1.0 / (A[(A.nrows() - 1, A.ncols() - 1)] - (&v * &A_submatrix_inv * &u)[(0, 0)]);

    let q = -(&v * &A_submatrix_inv) * alpha;
    let r = -(&A_submatrix_inv * &u) * alpha;

    let P = &A_submatrix_inv - &A_submatrix_inv * &u * &q;

    let mut A_inv : DMatrix<f32> = DMatrix::from_element(n, n, 0.0);

    for i in 0..(n - 1) {
        for k in 0..(n - 1) {
            A_inv[(i, k)] = P[(i, k)];
        }
    }

    for i in 0..q.ncols() {
        A_inv[(n - 1, i)] = q[i];
    }

    for i in 0..r.nrows() {
        A_inv[(i, n - 1)] = r[i];
    }

    A_inv[(n - 1, n - 1)] = alpha;

    return A_inv.into();
}

fn main() {
    let matrix_row = [1.0, 2.0, 4.0,
                      3.0, 3.0, 2.0,
                      4.0, 1.0, 3.0];

    let vector_row = [0.0, 0.0, 31.0];
    let n = 3;
    let A : DMatrix<f32> = DMatrix::from_row_slice(n, n, &matrix_row);
    let f = DVector::from_row_slice(&vector_row);

    println!("A = {} f = {}", A, f);

    let A_invert = invert(&A, n);
    println!("A_invert = {}", A_invert);
    let xs = A_invert * f;
    println!("x = {}", xs);
}
