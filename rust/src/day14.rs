use std::collections::HashMap;
use std::iter::zip;
use std::str;

fn parse_input(input: &str) -> (&str, Vec<(u8, u8, u8)>) {
    let mut lines = input.lines();
    let polymer = lines.next().unwrap();
    let rules: Vec<(u8, u8, u8)> = lines
        .skip(1)
        .map(|line| {
            let (pair_str, ins_str) = scan_fmt!(line, "{} -> {}", String, String).unwrap();
            let c1 = pair_str.as_bytes()[0];
            let c2 = pair_str.as_bytes()[1];
            let ins = ins_str.as_bytes()[0];
            (c1, c2, ins)
        })
        .collect();
    (polymer, rules)
}

fn do_pair_insertion(polymer: &str, rules: Vec<(u8, u8, u8)>, steps: usize) -> HashMap<u8, i64> {
    // Get pair counts for initial polymer
    let mut pair_counts: HashMap<(u8, u8), i64> = HashMap::new();
    for pair in zip(polymer.bytes(), polymer.bytes().skip(1)) {
        *pair_counts.entry(pair).or_insert(0) += 1;
    }

    // Do pair insertion for specified number of steps
    for step in 0..steps {
        let mut pair_counts_next = pair_counts.clone();
        for (c1, c2, ins) in rules.iter() {
            let num_matches = *pair_counts.entry((*c1, *c2)).or_default();
            *pair_counts_next.entry((*c1, *c2)).or_insert(0) -= num_matches;
            *pair_counts_next.entry((*c1, *ins)).or_insert(0) += num_matches;
            *pair_counts_next.entry((*ins, *c2)).or_insert(0) += num_matches;
        }
        pair_counts = pair_counts_next;
    }

    // Count the number of individual elements
    let mut elt_counts: HashMap<u8, i64> = HashMap::new();
    for (pair, count) in pair_counts {
        if count > 0 {
            *elt_counts.entry(pair.0).or_insert(0) += count;
            *elt_counts.entry(pair.1).or_insert(0) += count;
        }
    }

    // Every element is double counted except the first and last ones
    let cfirst = *polymer.as_bytes().first().unwrap();
    let clast = *polymer.as_bytes().last().unwrap();
    *elt_counts.entry(cfirst).or_insert(0) += 1;
    *elt_counts.entry(clast).or_insert(0) += 1;
    for mut count in elt_counts.values_mut() {
        *count /= 2;
    }

    elt_counts
}

pub fn part_a(input: &str) -> i64 {
    let (polymer, rules) = parse_input(input);
    let elt_counts = do_pair_insertion(polymer, rules, 10);
    elt_counts.values().max().unwrap() - elt_counts.values().min().unwrap()
}

pub fn part_b(input: &str) -> i64 {
    let (polymer, rules) = parse_input(input);
    let elt_counts = do_pair_insertion(polymer, rules, 40);
    elt_counts.values().max().unwrap() - elt_counts.values().min().unwrap()
}
