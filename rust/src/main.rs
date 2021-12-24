#![feature(iter_zip)]
#![feature(destructuring_assignment)]

#[macro_use]
extern crate scan_fmt;

use std::env;
use std::fs;
use std::path;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day21;

// todo: figure out how to let return type of functions be arbitrary integer types
// so we don't have to use i64 for everything
fn lookup_day(day: i8) -> (fn(&str) -> i64, fn(&str) -> i64) {
    match day {
        1 => (day01::part_a, day01::part_b),
        2 => (day02::part_a, day02::part_b),
        3 => (day03::part_a, day03::part_b),
        4 => (day04::part_a, day04::part_b),
        5 => (day05::part_a, day05::part_b),
        6 => (day06::part_a, day06::part_b),
        7 => (day07::part_a, day07::part_b),
        8 => (day08::part_a, day08::part_b),
        21 => (day21::part_a, day21::part_b),
        _ => panic!("day not found"),
    }
}

fn main() {
    let day: i8 = env::args()
        .nth(1)
        .expect("day is required")
        .parse()
        .expect("day must be an integer");
    let pathstr = format!("../inputs/day{:02}.txt", day);
    let path = path::Path::new(&pathstr);
    let input = fs::read_to_string(path).expect("failed to read input file");
    let (part_a, part_b) = lookup_day(day);
    println!("part a: {:}", part_a(&input));
    println!("part b: {:}", part_b(&input));
}
