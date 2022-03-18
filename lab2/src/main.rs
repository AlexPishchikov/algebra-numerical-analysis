extern crate nalgebra as na;

use na::{DMatrix, DVector};

fn main() {
    let row = [1.0, 0.0, 3.0, 11.0, -4.0, 1.0, 2.0, 2.0, 6.0, 99.0, 2.0, 3.0];
    let n = 3;
    let mut A = DMatrix::from_row_slice(n, n + 1, &row);

    println!("{}", A);

    for j in 0..n {
        let mut max_el = (A[(j, j)] as f32).abs();
        let mut max_row_number = j;
        for i in (j + 1)..n {
            if max_el < (A[(j, i)] as f32).abs(){
                max_el = (A[(j, i)] as f32).abs();
                max_row_number = i;
            }
        }
        A.swap_rows(j, max_row_number);

        for i in (j + 1)..n {
            let coef = A[(i, j)] / A[(j, j)];
            for k in j..(n + 1) {
                A[(i, k)] -= coef * A[(j, k)];
            }
        }
    }

    println!("{}", A);
    
    let mut xs = DVector::from_element(n, 0.0);
    for i in (0..n).rev() {
        let mut temp = DVector::from_element(n, 0.0);
        for k in (i + 1)..n {
            temp[k] = A[(i, k)] * xs[k];
        }
        xs[i] = (A[(i, n)] - temp.sum()) / A[(i, i)];
    }

    println!("{}", xs);
}
