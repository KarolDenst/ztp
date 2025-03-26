use std::{
    cmp::min,
    io::{self, BufRead},
    time::Instant,
};

use rand::{rng, Rng};

type Matrix = Vec<Vec<i32>>;

pub fn main() {
    let data = read_input();
    let data2 = data.clone();
    let data3 = data.clone();
    let start = Instant::now();
    println!("Matrix multip: ");
    for (a, b, c) in data {
        let mut c2 = c.clone();
        let result = check_matrix_multip(&a, &b, &mut c2);
        println!("{}", if result { "YES" } else { "NO" });
    }
    println!("Execution time: {:?}", start.elapsed().as_secs_f32());

    let start = Instant::now();
    println!("Fast matrix multip: ");
    for (a, b, c) in data3 {
        let result = check_fast_matrix_multip(&a, &b, &c);
        println!("{}", if result { "YES" } else { "NO" });
    }
    println!("Execution time: {:?}", start.elapsed().as_secs_f32());

    let start = Instant::now();
    println!("Proba: ");
    for (a, b, c) in data2 {
        let result = check_matrix_multip_proba(&a, &b, &c);
        println!("{}", if result { "YES" } else { "NO" });
    }
    println!("Execution time: {:?}", start.elapsed().as_secs_f32());
}

fn check_matrix_multip_proba(a: &Matrix, b: &Matrix, c: &Matrix) -> bool {
    let v = generate_random_vec(a[0].len(), 2663);
    let m1 = mult_vec_matrix(&v, a);
    let m2 = mult_vec_matrix(&m1, b);
    let m3 = mult_vec_matrix(&v, c);
    for i in 0..m2.len() {
        if m2[i] != m3[i] {
            return false;
        }
    }

    true
}

fn mult_vec_matrix(v: &[i32], m: &Matrix) -> Vec<i32> {
    let mut result = Vec::with_capacity(m[0].len());
    (0..m[0].len()).for_each(|j| {
        let mut sum = 0;
        (0..v.len()).for_each(|i| {
            sum += v[i] * m[i][j];
        });
        result.push(sum);
    });
    result
}

fn generate_random_vec(size: usize, p: i32) -> Vec<i32> {
    let mut rng = rng();
    let mut result = Vec::with_capacity(size);
    (0..size).for_each(|_| {
        result.push(rng.random_range(0..p));
    });
    result
}

fn check_matrix_multip(a: &Matrix, b: &Matrix, c: &mut Matrix) -> bool {
    let n = a.len();
    let m = b[0].len();
    let x = a[0].len();

    for i in 0..n {
        for j in 0..m {
            for k in 0..x {
                c[i][j] -= a[i][k] * b[k][j];
            }
        }
    }

    for i in 0..n {
        for j in 0..m {
            if c[i][j] != 0 {
                return false;
            }
        }
    }

    true
}

fn check_fast_matrix_multip(a: &Matrix, b: &Matrix, c: &Matrix) -> bool {
    let a_flat = matrix_to_flat(a);
    let b_flat = matrix_to_flat(b);
    let mut c_flat = matrix_to_flat(c);

    fast_mat_mul(
        &a_flat,
        &b_flat,
        &mut c_flat,
        a.len(),
        b[0].len(),
        a[0].len(),
    );

    for i in 0..c_flat.len() {
        if c_flat[i] != 0 {
            return false;
        }
    }

    true
}

fn matrix_to_flat(matrix: &Matrix) -> Vec<i32> {
    matrix.iter().flatten().copied().collect()
}

const TILE_SIZE: usize = 16;
fn fast_mat_mul(a: &[i32], b: &[i32], c: &mut [i32], rows: usize, columns: usize, inners: usize) {
    for row_tile in (0..rows).step_by(256) {
        for column_tile in (0..columns).step_by(256) {
            for inner_tile in (0..inners).step_by(TILE_SIZE) {
                let row_tile_end = min(rows, row_tile + 256);
                let col_tile_end = min(columns, column_tile + 256);
                let inner_tile_end = min(inners, inner_tile + TILE_SIZE);

                for row in row_tile..row_tile_end {
                    for inner in inner_tile..inner_tile_end {
                        for col in column_tile..col_tile_end {
                            let l = a[row * inners + inner];
                            let r = b[inner * columns + col];
                            let res_index = row * columns + col;
                            c[res_index] -= l * r;
                        }
                    }
                }
            }
        }
    }
}

fn read_input() -> Vec<(Matrix, Matrix, Matrix)> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let num_examples: usize = lines.next().unwrap().unwrap().parse().unwrap();
    let mut data = Vec::with_capacity(num_examples);

    for _ in 0..num_examples {
        let size: usize = lines.next().unwrap().unwrap().parse().unwrap();
        let mut a = Vec::with_capacity(size);
        for _ in 0..size {
            let row = lines
                .next()
                .unwrap()
                .unwrap()
                .split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect();
            a.push(row);
        }

        let mut b = Vec::with_capacity(size);
        for _ in 0..size {
            let row = lines
                .next()
                .unwrap()
                .unwrap()
                .split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect();
            b.push(row);
        }

        let mut c = Vec::with_capacity(size);
        for _ in 0..size {
            let row = lines
                .next()
                .unwrap()
                .unwrap()
                .split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect();
            c.push(row);
        }
        data.push((a, b, c));
    }

    data
}
