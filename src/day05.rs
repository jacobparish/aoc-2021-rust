use scan_fmt::scan_fmt;
use std::collections::HashMap;
use std::iter::{zip, Iterator};

fn signed_range_inclusive(a: i64, b: i64) -> Box<dyn Iterator<Item = i64>> {
    if a <= b {
        Box::new(a..=b)
    } else {
        Box::new((b..=a).rev())
    }
}

pub fn part_a(input: &str) -> i64 {
    input
        .lines()
        .map(|s| scan_fmt!(s, "{d},{d} -> {d},{d}", i64, i64, i64, i64).unwrap())
        .fold(HashMap::new(), |mut grid, (x1, y1, x2, y2)| {
            if x1 == x2 {
                for y in signed_range_inclusive(y1, y2) {
                    *grid.entry((x1, y)).or_insert(0) += 1;
                }
            } else if y1 == y2 {
                for x in signed_range_inclusive(x1, x2) {
                    *grid.entry((x, y1)).or_insert(0) += 1;
                }
            }
            grid
        })
        .into_values()
        .filter(|&x| x > 1)
        .count() as i64
}

pub fn part_b(input: &str) -> i64 {
    input
        .lines()
        .map(|s| scan_fmt!(s, "{d},{d} -> {d},{d}", i64, i64, i64, i64).unwrap())
        .fold(HashMap::new(), |mut grid, (x1, y1, x2, y2)| {
            if x1 == x2 {
                for y in signed_range_inclusive(y1, y2) {
                    *grid.entry((x1, y)).or_insert(0) += 1;
                }
            } else if y1 == y2 {
                for x in signed_range_inclusive(x1, x2) {
                    *grid.entry((x, y1)).or_insert(0) += 1;
                }
            } else {
                for (x, y) in zip(
                    signed_range_inclusive(x1, x2),
                    signed_range_inclusive(y1, y2),
                ) {
                    *grid.entry((x, y)).or_insert(0) += 1;
                }
            }
            grid
        })
        .into_values()
        .filter(|&x| x > 1)
        .count() as i64
}
