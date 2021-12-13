use std::collections::HashMap;

fn count_fish(input: &str, days: i64) -> i64 {
    let mut fish: HashMap<i8, i64> = input
        .lines()
        .next()
        .unwrap()
        .split(",")
        .map(|s| s.parse().unwrap())
        .fold(HashMap::new(), |mut fish, x| {
            *fish.entry(x).or_insert(0) += 1;
            fish
        });
    for _ in 0..days {
        let timed_out_fish = *fish.entry(0).or_insert(0);
        for i in 0..8 {
            *fish.entry(i).or_insert(0) = *fish.entry(i + 1).or_insert(0);
        }
        *fish.entry(8).or_insert(0) = timed_out_fish;
        *fish.entry(6).or_insert(0) += timed_out_fish;
    }
    fish.values().sum()
}

pub fn part_a(input: &str) -> i64 {
    count_fish(input, 80)
}

pub fn part_b(input: &str) -> i64 {
    count_fish(input, 256)
}
