use std::convert::TryInto;
use std::str;

fn parse_grid<const WIDTH: usize, const HEIGHT: usize>(s: &str) -> [[u8; WIDTH]; HEIGHT] {
    let mut rows_vec: Vec<[u8; WIDTH]> = s
        .lines()
        .map(|line| {
            let mut vec: Vec<u8> = line.as_bytes().iter().map(|c| c - b'0').collect();
            vec.insert(0, 0);
            vec.push(0);
            vec.try_into().unwrap()
        })
        .collect();

    rows_vec.insert(0, [0u8; WIDTH]);
    rows_vec.push([0u8; WIDTH]);
    rows_vec.try_into().unwrap()
}

fn simulate_step<const WIDTH: usize, const HEIGHT: usize>(grid: &mut [[u8; WIDTH]; HEIGHT]) -> i64 {
    for i in 1..HEIGHT - 1 {
        for j in 1..WIDTH - 1 {
            grid[i][j] += 1;
        }
    }

    let mut flashes = [[false; WIDTH]; HEIGHT];
    let mut num_flashes = 0;
    loop {
        let mut did_flash = false;
        for i in 1..HEIGHT - 1 {
            for j in 1..WIDTH - 1 {
                if grid[i][j] > 9 && !flashes[i][j] {
                    did_flash = true;
                    flashes[i][j] = true;
                    num_flashes += 1;
                    for ii in i - 1..=i + 1 {
                        for jj in j - 1..=j + 1 {
                            if i != ii || j != jj {
                                grid[ii][jj] += 1;
                            }
                        }
                    }
                }
            }
        }
        if !did_flash {
            break;
        }
    }

    for i in 1..HEIGHT - 1 {
        for j in 1..WIDTH - 1 {
            if flashes[i][j] {
                grid[i][j] = 0;
            }
        }
    }

    num_flashes
}

pub fn part_a(input: &str) -> i64 {
    let mut grid = parse_grid::<12, 12>(input);

    let mut num_flashes = 0;

    for step in 0..100 {
        num_flashes += simulate_step(&mut grid);
    }

    num_flashes
}

pub fn part_b(input: &str) -> i64 {
    let mut grid = parse_grid::<12, 12>(input);

    let mut step = 1;

    loop {
        if simulate_step(&mut grid) == 100 {
            return step;
        }
        step += 1;
    }
}
