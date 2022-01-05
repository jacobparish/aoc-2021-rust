pub fn part_a(input: &str) -> i64 {
    let crabs: Vec<i64> = input
        .trim()
        .split(",")
        .map(|s| s.parse().unwrap())
        .collect();
    let &min = crabs.iter().min().unwrap();
    let &max = crabs.iter().max().unwrap();
    (min..=max)
        .map(|pos| crabs.iter().map(|crab| (crab - pos).abs()).sum())
        .min()
        .unwrap()
}

fn sum_to_n(n: i64) -> i64 {
    n * (n + 1) / 2
}

pub fn part_b(input: &str) -> i64 {
    let crabs: Vec<i64> = input
        .trim()
        .split(",")
        .map(|s| s.parse().unwrap())
        .collect();
    let min = crabs.iter().cloned().min().unwrap_or(0);
    let max = crabs.iter().cloned().max().unwrap_or(0);
    (min..=max)
        .map(|pos| crabs.iter().map(|crab| sum_to_n((crab - pos).abs())).sum())
        .min()
        .unwrap()
}
