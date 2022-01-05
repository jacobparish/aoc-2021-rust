use itertools::Itertools;

pub fn part_a(input: &str) -> i64 {
    input
        .lines()
        .map(|line| {
            line.split(" | ")
                .nth(1)
                .unwrap()
                .split_whitespace()
                .filter(|s| match s.len() {
                    2 | 3 | 4 | 7 => true,
                    _ => false,
                })
                .count()
        })
        .sum::<usize>() as i64
}

fn parse_line(line: &str) -> (Vec<&str>, Vec<&str>) {
    line.split(" | ")
        .map(|s| s.split_whitespace().collect())
        .next_tuple()
        .unwrap()
}

pub fn part_b(input: &str) -> i64 {
    let letters: Vec<u8> = (b'a'..=b'g').collect();
    let valid_combos = vec![
        "abcefg", "cf", "acdeg", "acdfg", "bcdf", "abdfg", "abdefg", "acf", "abcdefg", "abcdfg",
    ];

    // todo: clean this up
    input
        .lines()
        .map(|line| {
            let (sig_patterns, out_values) = parse_line(line);
            let matching_perm = letters
                .iter()
                .permutations(7)
                .find(|perm| {
                    sig_patterns.iter().all(|sig| {
                        let decoded = String::from_utf8(
                            sig.bytes()
                                .map(|c| {
                                    **perm
                                        .get(letters.iter().position(|&d| d == c).unwrap())
                                        .unwrap()
                                })
                                .sorted()
                                .collect(),
                        )
                        .unwrap();
                        valid_combos.contains(&decoded.as_str())
                    })
                })
                .unwrap();

            out_values.iter().fold(0, |acc, val| {
                let decoded = String::from_utf8(
                    val.bytes()
                        .map(|c| {
                            **matching_perm
                                .get(letters.iter().position(|&d| d == c).unwrap())
                                .unwrap()
                        })
                        .sorted()
                        .collect(),
                )
                .unwrap();
                10 * acc + valid_combos.iter().position(|&s| *s == decoded).unwrap()
            })
        })
        .sum::<usize>() as i64
}
