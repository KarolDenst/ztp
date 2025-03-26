#![allow(clippy::needless_range_loop)]

use std::env;

mod check_invertible;
mod common;
mod dynamic_inverse;
mod inverse_matrix;
mod kthelement;
mod matching;
mod matching_test;
mod matrix_multiplication_test;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <number>", args[0]);
        std::process::exit(1);
    }

    let input: usize = match args[1].parse() {
        Ok(n) => n,
        Err(_) => {
            eprintln!("Please provide a valid usize number");
            std::process::exit(1);
        }
    };

    match input {
        1 => kthelement::main(),
        2 => matrix_multiplication_test::main(),
        3 => matching_test::main(),
        4 => check_invertible::main(),
        5 => inverse_matrix::main(),
        6 => dynamic_inverse::main(),
        7 => matching::main(),
        _ => {
            eprintln!("Please provide a valid number");
            std::process::exit(1);
        }
    }
}
