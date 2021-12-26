use std::convert::TryInto;
use std::str;

const WIDTH: usize = 139;
const HEIGHT: usize = 137;

pub fn part_a(input: &str) -> i64 {
    let mut grid: [[u8; WIDTH]; HEIGHT] = input
        .lines()
        .map(|line| line.as_bytes().try_into().unwrap())
        .collect::<Vec<[u8; 139]>>()
        .try_into()
        .unwrap();

    for step in 1.. {
        let mut next_grid = [[b'.'; WIDTH]; HEIGHT];

        let mut did_move = false;

        for i in 0..HEIGHT {
            for j in 0..WIDTH {
                if grid[i][j] == b'>' {
                    if grid[i][(j + 1) % WIDTH] == b'.' {
                        next_grid[i][(j + 1) % WIDTH] = b'>';
                        did_move = true;
                    } else {
                        next_grid[i][j] = b'>';
                    }
                }
            }
        }

        for i in 0..HEIGHT {
            for j in 0..WIDTH {
                if grid[i][j] == b'v' {
                    if grid[(i + 1) % HEIGHT][j] != b'v' && next_grid[(i + 1) % HEIGHT][j] == b'.' {
                        next_grid[(i + 1) % HEIGHT][j] = b'v';
                        did_move = true;
                    } else {
                        next_grid[i][j] = b'v';
                    }
                }
            }
        }

        if !did_move {
            return step;
        }
        grid = next_grid;
    }

    panic!("should never happpen")
}

pub fn part_b(input: &str) -> i64 {
    0
}
