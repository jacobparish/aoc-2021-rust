use itertools::Itertools;
use std::collections::HashMap;

fn count_fish(input: &str, days: usize) -> usize {
    let mut fish: HashMap<u8, _> = input.trim().split(",").map(|s| s.parse().unwrap()).counts();
    for _ in 0..days {
        let timed_out_fish = *fish.entry(0).or_insert(0);
        for i in 0..8 {
            *fish.entry(i).or_insert(0) = *fish.entry(i + 1).or_insert(0);
        }
        *fish.entry(8).or_insert(0) = timed_out_fish;
        *fish.entry(6).or_insert(0) += timed_out_fish;
    }
    fish.into_values().sum()
}

pub fn part_a(input: &str) -> i64 {
    count_fish(input, 80) as i64
}

pub fn part_b(input: &str) -> i64 {
    count_fish(input, 256) as i64
}
