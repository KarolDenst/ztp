use std::{
    collections::HashSet,
    io::{self, BufRead},
};

use crate::common::{
    matrix_op::{factorize_lup, inverse_lower_triangular, inverse_upper_triangular, remove_ij},
    zp::ZpNumber,
};
use nalgebra::DMatrix;
use num_traits::Zero;
use rand::{rngs::StdRng, Rng, SeedableRng};

const P: u32 = 7919;

pub fn main() {
    let (mut matrix, edges) = read_input();
    let mut rng = StdRng::seed_from_u64(P as u64);
    for i in 0..matrix.nrows() {
        for j in 0..matrix.ncols() {
            if i > j || matrix[(i, j)].is_zero() {
                continue;
            }
            matrix[(i, j)] = ZpNumber::new(rng.random_range(1..P), P);
            matrix[(j, i)] = -matrix[(i, j)];
        }
    }

    let (l, u, p) = factorize_lup(&matrix).unwrap();
    let mut inv = inverse_upper_triangular(&u) * inverse_lower_triangular(&l) * p;

    let n = matrix.nrows();
    let mut removed = vec![];
    let mut indices = (0..n).collect::<Vec<_>>();
    for _ in (0..n).step_by(2) {
        for j in 1..matrix.nrows() {
            if !inv[(0, j)].is_zero() && edges.contains(&(indices[0], indices[j])) {
                removed.push((indices[0], indices[j]));
                indices.remove(j);
                indices.remove(0);

                let (new_mat, new_inv) = remove_ij(matrix, inv, j, 0).unwrap();
                let (new_mat, new_inv) = remove_ij(new_mat, new_inv, 0, j - 1).unwrap();
                matrix = new_mat;
                inv = new_inv;

                break;
            }
        }
    }
    println!("{:?}", removed);
    if check_correct(removed, edges, n) {
        println!("The matching is correct");
    } else {
        println!("The matching is incorrect");
    }
}

fn check_correct(removed: Vec<(usize, usize)>, edges: Vec<(usize, usize)>, n: usize) -> bool {
    let edges: HashSet<_> = edges.iter().collect();
    for edge in removed.iter() {
        if !edges.contains(&edge) {
            println!("Used non-existant edge: {:?}", edge);
            return false;
        }
    }
    let mut count = HashSet::new();
    for edge in removed {
        count.insert(edge.0);
        count.insert(edge.1);
    }
    if count.len() != n {
        println!("Not all vertices are covered");
        return false;
    }
    true
}

fn read_input() -> (DMatrix<ZpNumber>, Vec<(usize, usize)>) {
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
        .flat_map(|e| vec![(e[0], e[1]), (e[1], e[0])])
        .collect();

    for edge in edges.iter() {
        data[(edge.0, edge.1)] = ZpNumber::one();
        data[(edge.1, edge.0)] = ZpNumber::one();
    }

    (data, edges)
}
