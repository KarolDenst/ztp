use std::io::{self, BufRead};

use crate::common::{matrix_op::factorize_lup, zp::Zp};
use nalgebra::DMatrix;
use rand::{rngs::StdRng, Rng, SeedableRng};

const P: u32 = 7919;

pub fn main() {
    let zp = Zp::new(P);
    let mut matrix = read_input();
    let mut rng = StdRng::seed_from_u64(P as u64);
    for i in 0..matrix.nrows() {
        for j in 0..matrix.ncols() {
            if i > j || matrix[(i, j)] == 0 {
                continue;
            }
            matrix[(i, j)] = rng.random_range(1..P);
            matrix[(j, i)] = zp.neg(matrix[(i, j)]);
        }
    }

    if factorize_lup(&matrix, &zp).is_ok() {
        println!("YES");
    } else {
        println!("NO");
    }
}

fn read_input() -> DMatrix<u32> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let size: usize = lines
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .collect::<Vec<_>>()[0]
        .parse()
        .unwrap();

    let mut data = DMatrix::zeros(size, size);
    let edges: Vec<_> = lines
        .map(|line| {
            line.unwrap()
                .split_whitespace()
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect();

    for edge in edges {
        data[(edge[0], edge[1])] = 1;
        data[(edge[1], edge[0])] = 1;
    }

    data
}
