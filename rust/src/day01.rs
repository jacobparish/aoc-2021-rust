use std::convert::TryInto;
use std::iter::zip;

pub fn part_a(input: &str) -> i32 {
    let depths: Vec<i32> = input.lines().map(|s| s.parse().unwrap()).collect();
    zip(depths.iter(), depths.iter().skip(1))
        .filter(|(x, y)| x < y)
        .count()
        .try_into()
        .unwrap()
}

pub fn part_b(input: &str) -> i32 {
    let depths: Vec<i32> = input.lines().map(|s| s.parse().unwrap()).collect();
    zip(depths.iter(), depths.iter().skip(3))
        .filter(|(x, y)| x < y)
        .count()
        .try_into()
        .unwrap()
}
