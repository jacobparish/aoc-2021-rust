use std::convert::TryFrom;

pub fn part_a(input: &str) -> i64 {
    input
        .lines()
        .map(|line| {
            i64::try_from(
                line.split(" | ")
                    .nth(1)
                    .unwrap()
                    .split_whitespace()
                    .filter(|s| match s.len() {
                        2 => true,
                        3 => true,
                        4 => true,
                        7 => true,
                        _ => false,
                    })
                    .count(),
            )
            .unwrap()
        })
        .sum()
}

pub fn part_b(input: &str) -> i64 {
    0
}
