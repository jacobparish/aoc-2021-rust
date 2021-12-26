use itertools::Itertools;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::iter::zip;

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
                        2 | 3 | 4 | 7 => true,
                        _ => false,
                    })
                    .count(),
            )
            .unwrap()
        })
        .sum()
}

fn parse_line(line: &str) -> (Vec<&str>, Vec<&str>) {
    line.split(" | ")
        .map(|s| s.split_whitespace().collect())
        .next_tuple()
        .unwrap()
}

pub fn part_b(input: &str) -> i64 {
    let letters: Vec<u8> = (b'a'..b'g' + 1).collect();
    let valid_combos = vec![
        "abcefg", "cf", "acdeg", "acdfg", "bcdf", "abdfg", "abdefg", "acf", "abcdefg", "abcdfg",
    ];

    // todo: clean this up
    input
        .lines()
        .map(|line| {
            let (sig_patterns, out_values) = parse_line(line);
            let decode = letters
                .iter()
                .permutations(7)
                .find(|perm| {
                    let map: HashMap<_, _> =
                        zip(letters.iter().cloned(), perm.iter().cloned().cloned()).collect();
                    sig_patterns.iter().all(|sig| {
                        let decoded = String::from_utf8(
                            sig.bytes()
                                .map(|c| map.get(&c).unwrap())
                                .cloned()
                                .sorted()
                                .collect(),
                        )
                        .unwrap();
                        valid_combos.contains(&decoded.as_str())
                    })
                })
                .unwrap();

            let map: HashMap<_, _> =
                zip(letters.iter().cloned(), decode.iter().cloned().cloned()).collect();

            out_values.iter().fold(0, |acc, val| {
                let decoded = String::from_utf8(
                    val.bytes()
                        .map(|c| map.get(&c).unwrap())
                        .cloned()
                        .sorted()
                        .collect(),
                )
                .unwrap();
                10 * acc + valid_combos.iter().position(|s| **s == decoded).unwrap() as i64
            })
        })
        .sum()
}
