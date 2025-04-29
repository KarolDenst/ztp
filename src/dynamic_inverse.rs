use std::io::{self, BufRead};

use nalgebra::DMatrix;

use crate::common::{
    matrix_op::{print_matrix, remove_ij_swapped},
    zp::ZpNumber,
};

pub fn main() {
    let data = read_input();
    for d in data.iter() {
        let mut a = d.a.clone();
        let mut inv = d.inv.clone();
        if remove_ij_swapped(&mut a, &mut inv, d.row, d.col, d.a.nrows()) {
            println!("YES");
            let inv = inv.remove_column(a.ncols() - 1);
            let inv = inv.remove_row(a.ncols() - 1);
            print_matrix(&inv);
        } else {
            println!("NO");
        }
    }
}

struct Data {
    pub a: DMatrix<ZpNumber>,
    pub inv: DMatrix<ZpNumber>,
    pub row: usize,
    pub col: usize,
}
fn read_input() -> Vec<Data> {
    let stdin = io::stdin();
    let lines: Vec<String> = stdin.lock().lines().map(|l| l.unwrap()).collect();

    let first_line = &lines[0];
    let mut input = first_line.split_whitespace();
    let z: usize = input.next().unwrap().parse().unwrap();
    let p: u32 = input.next().unwrap().parse().unwrap();

    let mut data = vec![];
    let mut line_index = 1; // Start from the second line
    for _ in 0..z {
        let row: Vec<u32> = lines[line_index]
            .split_whitespace()
            .map(|x| x.parse::<u32>().unwrap())
            .collect();
        line_index += 1;

        let n = row[0] as usize;
        let r = row[1] as usize;
        let c = row[2] as usize;
        let a = read_matrix(n, &lines, &mut line_index, p);
        let inv = read_matrix(n, &lines, &mut line_index, p);

        data.push(Data {
            a,
            inv,
            row: r,
            col: c,
        });
    }
    data
}

fn read_matrix(n: usize, lines: &[String], line_index: &mut usize, p: u32) -> DMatrix<ZpNumber> {
    let mut matrix = DMatrix::zeros(n, n);
    for i in 0..n {
        let row: Vec<ZpNumber> = lines[*line_index]
            .split_whitespace()
            .map(|x| x.parse::<u32>().unwrap())
            .map(|x| ZpNumber::new(x, p))
            .collect();
        *line_index += 1;
        for j in 0..n {
            matrix[(i, j)] = row[j];
        }
    }
    matrix
}
