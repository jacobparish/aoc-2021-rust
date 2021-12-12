#![feature(iter_zip)]
#[macro_use] extern crate scan_fmt;

use std::fs;
use std::path::Path;

mod day01;
mod day02;

fn main() {
    let path = Path::new("../inputs/day02.txt");
    let input = fs::read_to_string(path).expect("failed to read input file");

    println!("part a: {:}", day02::part_a(&input));
    println!("part b: {:}", day02::part_b(&input));
}

