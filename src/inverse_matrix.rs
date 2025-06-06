use std::io::{self, BufRead};

use nalgebra::DMatrix;

use crate::common::{
    matrix_op::{factorize_lup, inverse_lower_triangular, inverse_upper_triangular, print_matrix},
    zp::ZpNumber,
};

// A-1 = U-1L-1P
pub fn main() {
    for matrix in read_input() {
        if let Ok((l, u, p)) = factorize_lup(&matrix) {
            println!("YES");

            let a_inv = inverse_upper_triangular(&u) * inverse_lower_triangular(&l) * p;
            print_matrix(&a_inv);
        } else {
            println!("NO");
        }
    }
}

fn read_input() -> Vec<DMatrix<ZpNumber>> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let first_line = lines.next().unwrap().unwrap();
    let mut input = first_line.split_whitespace();
    let z: usize = input.next().unwrap().parse().unwrap();
    let p: u32 = input.next().unwrap().parse().unwrap();

    let mut matrices = vec![];
    for _ in 0..z {
        let n: usize = lines.next().unwrap().unwrap().parse().unwrap();
        let mut matrix = DMatrix::zeros(n, n);

        for i in 0..n {
            let row: Vec<_> = lines
                .next()
                .unwrap()
                .unwrap()
                .split_whitespace()
                .map(|x| x.parse::<u32>().unwrap())
                .map(|x| ZpNumber::new(x, p))
                .collect();
            for j in 0..n {
                matrix[(i, j)] = row[j];
            }
        }
        matrices.push(matrix);
    }
    matrices
}
