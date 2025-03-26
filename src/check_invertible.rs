use std::io::{self, BufRead};

use nalgebra::DMatrix;

use crate::common::{matrix_op::factorize_lup, zp::Zp};

pub fn main() {
    for (matrix, p) in read_input() {
        let zp = Zp::new(p);
        // println!("{}", matrix);
        // let matrix_f64 = matrix.map(|x| x as f64);
        // println!("{}", matrix_f64.determinant());
        if factorize_lup(&matrix, &zp).is_ok() {
            println!("YES");
        } else {
            println!("NO");
        }
    }
}

fn read_input() -> Vec<(DMatrix<u32>, u32)> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let first_line = lines.next().unwrap().unwrap();
    let mut input = first_line.split_whitespace();
    let z: usize = input.next().unwrap().parse().unwrap();
    let p: u32 = input.next().unwrap().parse().unwrap();

    let mut matrices = vec![];
    for _ in 0..z {
        let n: usize = lines.next().unwrap().unwrap().parse().unwrap();
        let mut matrix = vec![vec![0; n]; n];

        for i in 0..n {
            matrix[i] = lines
                .next()
                .unwrap()
                .unwrap()
                .split_whitespace()
                .map(|x| x.parse::<u32>().unwrap())
                .collect();
        }
        matrices.push((
            DMatrix::from_vec(n, n, matrix.into_iter().flatten().collect()),
            p,
        ));
    }
    matrices
}
