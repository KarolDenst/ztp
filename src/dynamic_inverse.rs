// use std::io::{self, BufRead};
//
// use nalgebra::DMatrix;
//
// use crate::common::zp::Zp;

pub fn main() {}
//     let data = read_input();
//     let p = data.1;
//     for d in data.0.iter() {
//         solve(d.a.clone(), d.inv.clone(), d.row, d.col, p);
//     }
// }
//
// fn solve(a: &mut DMatrix<u32>, inv: &mut DMatrix<u32>, row: usize, col: usize, p: u32, n: usize) {
//     let zp = Zp::new(p);
//     let mut b = a.clone();
//     b.swap_rows(row, n - 1);
//     b.swap_columns(col, n - 1);
//     for i in 0..n {
//     for j in 0..n {
//             inv[(i,j)] = (p + inv[i,j] - inv[i, n-1] * inv[n-1])
//     }
//     }
// }
//
// struct Data {
//     pub a: DMatrix<u32>,
//     pub inv: DMatrix<u32>,
//     pub row: usize,
//     pub col: usize,
// }
// fn read_input() -> (Vec<Data>, u32) {
//     let stdin = io::stdin();
//     let lines: Vec<String> = stdin.lock().lines().map(|l| l.unwrap()).collect();
//
//     let first_line = &lines[0];
//     let mut input = first_line.split_whitespace();
//     let z: usize = input.next().unwrap().parse().unwrap();
//     let p: u32 = input.next().unwrap().parse().unwrap();
//
//     let mut data = vec![];
//     let mut line_index = 1; // Start from the second line
//     for _ in 0..z {
//         let row: Vec<u32> = lines[line_index]
//             .split_whitespace()
//             .map(|x| x.parse::<u32>().unwrap())
//             .collect();
//         line_index += 1;
//
//         let n = row[0] as usize;
//         let r = row[1] as usize;
//         let c = row[2] as usize;
//         let a = read_matrix(n, &lines, &mut line_index);
//         let inv = read_matrix(n, &lines, &mut line_index);
//
//         data.push(Data {
//             a,
//             inv,
//             row: r,
//             col: c,
//         });
//     }
//     (data, p)
// }
//
// fn read_matrix(n: usize, lines: &[String], line_index: &mut usize) -> DMatrix<u32> {
//     let mut matrix = DMatrix::zeros(n, n);
//     for i in 0..n {
//         let row: Vec<u32> = lines[*line_index]
//             .split_whitespace()
//             .map(|x| x.parse::<u32>().unwrap())
//             .collect();
//         *line_index += 1;
//         for j in 0..n {
//             matrix[(i, j)] = row[j];
//         }
//     }
//     matrix
// }
