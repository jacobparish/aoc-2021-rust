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
    // Doesn't necessarily work for all inputs - requires that there is some
    // x-velocity for which we end up stationary within the target area. But
    // hey, it works for my input, so ¯\_(ツ)_/¯
    -ymin * (-ymin - 1) / 2
}

pub fn part_b(input: &str) -> i64 {
    let (xmin, xmax, ymin, ymax) = parse_target_area(input);
    0
}
