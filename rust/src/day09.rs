use std::str;

use crate::utils::parse_grid_and_pad;

pub fn part_a(input: &str) -> i64 {
    let grid = parse_grid_and_pad::<102, 102>(input, 9);

    let mut total_risk: i64 = 0;
    for i in 1..=100 {
        for j in 1..=100 {
            if grid[i][j] < grid[i + 1][j]
                && grid[i][j] < grid[i - 1][j]
                && grid[i][j] < grid[i][j + 1]
                && grid[i][j] < grid[i][j - 1]
            {
                total_risk += (grid[i][j] + 1) as i64;
            }
        }
    }

    total_risk
}

pub fn part_b(input: &str) -> i64 {
    0
}
