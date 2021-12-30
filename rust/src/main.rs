#[macro_use]
extern crate scan_fmt;

use std::env;
use std::fs;
use std::path;
use std::time;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day13;
mod day14;
mod day16;
mod day17;
mod day18;
mod day20;
mod day21;
mod day22;
mod day25;
mod utils;

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
        9 => (day09::part_a, day09::part_b),
        10 => (day10::part_a, day10::part_b),
        11 => (day11::part_a, day11::part_b),
        13 => (day13::part_a, day13::part_b),
        14 => (day14::part_a, day14::part_b),
        16 => (day16::part_a, day16::part_b),
        17 => (day17::part_a, day17::part_b),
        18 => (day18::part_a, day18::part_b),
        20 => (day20::part_a, day20::part_b),
        21 => (day21::part_a, day21::part_b),
        22 => (day22::part_a, day22::part_b),
        25 => (day25::part_a, day25::part_b),
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

    let a_start = time::Instant::now();
    let a_result = part_a(&input);
    let a_elapsed = a_start.elapsed();
    println!("part a: {:}, {:?}", a_result, a_elapsed);

    let b_start = time::Instant::now();
    let b_result = part_b(&input);
    let b_elapsed = b_start.elapsed();
    println!("part b: {:}, {:?}", b_result, b_elapsed);
}
