use scan_fmt::scan_fmt;
use std::cmp::min;
use std::collections::HashSet;
use std::convert::TryInto;

pub fn part_a(input: &str) -> i64 {
    let mut lines = input.lines();
    let points: HashSet<(i16, i16)> = lines
        .by_ref()
        .take_while(|line| !line.is_empty())
        .map(|line| scan_fmt!(line, "{d},{d}", i16, i16).unwrap())
        .collect();

    let instr = lines.next().unwrap();

    let (dir, pos) = scan_fmt!(instr, "fold along {}={d}", String, i16).unwrap();

    let mut points_after_fold: HashSet<(i16, i16)> = HashSet::new();
    match dir.as_bytes()[0] {
        b'x' => {
            for (x, y) in points {
                points_after_fold.insert((min(x, 2 * pos - x), y));
            }
        }
        b'y' => {
            for (x, y) in points {
                points_after_fold.insert((x, min(y, 2 * pos - y)));
            }
        }
        _ => panic!("direction should be x or y"),
    }

    points_after_fold.len().try_into().unwrap()
}

pub fn part_b(input: &str) -> i64 {
    let mut lines = input.lines();
    let mut points: HashSet<(i16, i16)> = lines
        .by_ref()
        .take_while(|line| !line.is_empty())
        .map(|line| scan_fmt!(line, "{d},{d}", i16, i16).unwrap())
        .collect();

    for line in lines {
        let (dir, pos) = scan_fmt!(line, "fold along {}={d}", String, i16).unwrap();

        let mut points_after_fold: HashSet<(i16, i16)> = HashSet::new();
        match dir.as_bytes()[0] {
            b'x' => {
                for (x, y) in points {
                    points_after_fold.insert((min(x, 2 * pos - x), y));
                }
            }
            b'y' => {
                for (x, y) in points {
                    points_after_fold.insert((x, min(y, 2 * pos - y)));
                }
            }
            _ => panic!("direction should be x or y"),
        }
        points = points_after_fold;
    }

    let xmax = points.iter().max_by_key(|(x, _)| x).unwrap().0;
    let ymax = points.iter().max_by_key(|(_, y)| y).unwrap().1;

    for y in 0..=ymax {
        for x in 0..=xmax {
            print!("{}", if points.contains(&(x, y)) { "#" } else { "." });
        }
        println!("");
    }

    // this one has no integer answer
    return 0;
}
