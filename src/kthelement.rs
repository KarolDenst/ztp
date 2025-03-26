// Quick sort:
// 1999999
// Execution time: 0.1816151
// Probabilistic:
// 1999999
// Execution time: 0.0489349

use rand::rngs::StdRng;
use rand::Rng;
use rand::SeedableRng;
use std::{
    io::{self, BufRead},
    time::Instant,
};

pub fn main() {
    let data = read_input();
    for (_, k, arr) in data {
        let mut arr2 = arr.clone();
        let start = Instant::now();
        println!("Quick sort: ");
        arr2.sort_unstable();
        println!("{}", arr2[k - 1]);
        let duration = start.elapsed();
        println!("Execution time: {:?}", duration.as_secs_f32());

        println!("Probabilistic: ");
        let mut arr3 = arr.clone();
        let start = Instant::now();
        println!("{}", proba_kth_smallest(&mut arr3, k - 1));
        let duration = start.elapsed();
        println!("Execution time: {:?}", duration.as_secs_f32());
    }
}

const NUM_ELEMENTS: usize = 10000;
const OFFSET: usize = 500;
fn proba_kth_smallest(arr: &mut [usize], k: usize) -> usize {
    let mut rng = StdRng::seed_from_u64(42);
    let mut sample = [0; NUM_ELEMENTS];
    (0..NUM_ELEMENTS).for_each(|i| {
        let j = rng.random_range(0..arr.len());
        sample[i] = arr[j];
    });
    sample.sort_unstable();

    let pivot = NUM_ELEMENTS * k / arr.len();
    let l1 = sample[pivot - OFFSET];
    let l2 = sample[pivot + OFFSET];
    let i = partition(arr, 0, arr.len() - 1, l1);
    let j = partition(arr, i, arr.len() - 1, l2);
    arr[i..j].sort_unstable();
    arr[k]
}

fn partition(arr: &mut [usize], left: usize, right: usize, pivor: usize) -> usize {
    let mut i = left;
    for j in left..right {
        if arr[j] < pivor {
            arr.swap(i, j);
            i += 1;
        }
    }
    arr.swap(i, right);
    i
}

fn read_input() -> Vec<(usize, usize, Vec<usize>)> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let num_examples: usize = lines.next().unwrap().unwrap().parse().unwrap();
    let mut data = Vec::with_capacity(num_examples);

    for _ in 0..num_examples {
        let first_line = lines.next().unwrap().unwrap();
        let mut iter = first_line
            .split_whitespace()
            .map(|x| x.parse::<usize>().unwrap());
        let num_elements = iter.next().unwrap();
        let k = iter.next().unwrap();

        let second_line = lines.next().unwrap().unwrap();
        let numbers = second_line
            .split_whitespace()
            .map(|x| x.parse::<usize>().unwrap())
            .collect();

        data.push((num_elements, k, numbers));
    }

    data
}
