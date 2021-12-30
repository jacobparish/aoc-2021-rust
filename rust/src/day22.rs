use scan_fmt::parse::ScanError;
use std::cmp::{max, min};
use std::convert::TryInto;

#[derive(Debug)]
struct Cube {
    x1: i64,
    x2: i64,
    y1: i64,
    y2: i64,
    z1: i64,
    z2: i64,
}

impl Cube {
    fn volume(&self) -> i64 {
        // +1 since a cube includes both its min and its max
        (self.x2 - self.x1 + 1) * (self.y2 - self.y1 + 1) * (self.z2 - self.z1 + 1)
    }

    fn intersection(&self, other: &Cube) -> Option<Cube> {
        if self.x1 <= other.x2
            && other.x1 <= self.x2
            && self.y1 <= other.y2
            && other.y1 <= self.y2
            && self.z1 <= other.z2
            && other.z1 <= self.z2
        {
            Some(Cube {
                x1: max(self.x1, other.x1),
                x2: min(self.x2, other.x2),
                y1: max(self.y1, other.y1),
                y2: min(self.y2, other.y2),
                z1: max(self.z1, other.z1),
                z2: min(self.z2, other.z2),
            })
        } else {
            None
        }
    }
}

impl TryInto<Cube> for String {
    type Error = ScanError;

    fn try_into(self) -> Result<Cube, ScanError> {
        scan_fmt!(
            self.as_str(),
            "x={d}..{d},y={d}..{d},z={d}..{d}",
            i64,
            i64,
            i64,
            i64,
            i64,
            i64
        )
        .map(|(x1, x2, y1, y2, z1, z2)| Cube {
            x1,
            x2,
            y1,
            y2,
            z1,
            z2,
        })
    }
}

pub fn part_a(input: &str) -> i64 {
    let init_region = Cube {
        x1: -50,
        x2: 50,
        y1: -50,
        y2: 50,
        z1: -50,
        z2: 50,
    };

    input
        .lines()
        .fold(Vec::<(Cube, bool)>::new(), |mut acc, line| {
            let (on_or_off, cube_str) = scan_fmt!(line, "{} {}", String, String).unwrap();
            let cube: Cube = cube_str.try_into().unwrap();
            if let Some(init_cube) = cube.intersection(&init_region) {
                for i in 0..acc.len() {
                    let (other_cube, parity) = &acc[i];
                    if let Some(inter) = init_cube.intersection(&other_cube) {
                        acc.push((inter, !parity));
                    }
                }
                if on_or_off == "on" {
                    acc.push((cube, true));
                }
            }
            acc
        })
        .into_iter()
        .map(
            |(cube, count)| {
                if count {
                    cube.volume()
                } else {
                    -cube.volume()
                }
            },
        )
        .sum()
}

pub fn part_b(input: &str) -> i64 {
    input
        .lines()
        .fold(Vec::<(Cube, bool)>::new(), |mut acc, line| {
            let (on_or_off, cube_str) = scan_fmt!(line, "{} {}", String, String).unwrap();
            let cube: Cube = cube_str.try_into().unwrap();
            for i in 0..acc.len() {
                let (other_cube, parity) = &acc[i];
                if let Some(inter) = cube.intersection(&other_cube) {
                    acc.push((inter, !parity));
                }
            }
            if on_or_off == "on" {
                acc.push((cube, true));
            }
            acc
        })
        .into_iter()
        .map(|(cube, parity)| {
            if parity {
                cube.volume()
            } else {
                -cube.volume()
            }
        })
        .sum()
}
