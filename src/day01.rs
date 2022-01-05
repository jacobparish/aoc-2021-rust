use itertools::Itertools;

pub fn part_a(input: &str) -> i64 {
    input
        .lines()
        .map(|s| s.parse().unwrap())
        .tuple_windows()
        .filter(|(x, y): &(u16, u16)| x < y)
        .count() as i64
}

pub fn part_b(input: &str) -> i64 {
    input
        .lines()
        .map(|s| s.parse().unwrap())
        .tuple_windows()
        .filter(|(x, _, _, y): &(u16, _, _, u16)| x < y)
        .count() as i64
}
