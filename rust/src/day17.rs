use std::str;

fn parse_target_area(input: &str) -> (i64, i64, i64, i64) {
    scan_fmt!(
        input,
        "target area: x={d}..{d}, y={d}..{d}",
        i64,
        i64,
        i64,
        i64
    )
    .unwrap()
}

pub fn part_a(input: &str) -> i64 {
    let (xmin, xmax, ymin, ymax) = parse_target_area(input);
    -ymin * (-ymin - 1) / 2
}

pub fn part_b(input: &str) -> i64 {
    let (xmin, xmax, ymin, ymax) = parse_target_area(input);
    0
}
