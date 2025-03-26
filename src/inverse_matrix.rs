use std::io::{self, BufRead};

use nalgebra::DMatrix;

use crate::common::{
    matrix_op::{factorize_lup, inverse_upper_triangular},
    zp::Zp,
};

// A-1 = P U-1 L
pub fn main() {
    for (matrix, p) in read_input() {
        let zp = Zp::new(p);
        if let Ok((l, u, p)) = factorize_lup(&matrix, &zp) {
            println!("YES");

            let mut a_inv = p * inverse_upper_triangular(&u, &zp) * l;
            a_inv.apply(|x| zp.fix(x));
            println!("{}", a_inv);
            let mut i = matrix * a_inv;
            i.apply(|x| zp.fix(x));
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
        let mut matrix = DMatrix::zeros(n, n);

        for i in 0..n {
            let row: Vec<u32> = lines
                .next()
                .unwrap()
                .unwrap()
                .split_whitespace()
                .map(|x| x.parse::<u32>().unwrap())
                .collect();
            for j in 0..n {
                matrix[(i, j)] = row[j];
            }
        }
        matrices.push((matrix, p));
    }
    matrices
}
