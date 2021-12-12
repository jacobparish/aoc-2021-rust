#![feature(iter_zip)]

use std::fs;
use std::path::Path;

mod day01;

fn main() {
    let path = Path::new("../inputs/day01.txt");
    let input = fs::read_to_string(path).expect("failed to read input file");

    println!("part a: {:}", day01::part_a(&input));
    println!("part b: {:}", day01::part_b(&input));
}

