use std::cmp::{max, min};
use std::collections::HashMap;
use std::convert::TryInto;
use std::iter::zip;
use std::iter::Iterator;

fn signed_range_inclusive(a: i32, b: i32) -> Box<dyn Iterator<Item = i32>> {
    if a <= b {
        Box::new(a..b + 1)
    } else {
        Box::new((b..a + 1).rev())
    }
}

pub fn part_a(input: &str) -> i32 {
    let final_grid = input
        .lines()
        .map(|s| scan_fmt!(s, "{d},{d} -> {d},{d}", i32, i32, i32, i32).unwrap())
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
        });
    final_grid
        .values()
        .filter(|x| x > &&1)
        .count()
        .try_into()
        .unwrap()
}

pub fn part_b(input: &str) -> i32 {
    let final_grid = input
        .lines()
        .map(|s| scan_fmt!(s, "{d},{d} -> {d},{d}", i32, i32, i32, i32).unwrap())
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
        });
    final_grid
        .values()
        .filter(|x| x > &&1)
        .count()
        .try_into()
        .unwrap()
}
