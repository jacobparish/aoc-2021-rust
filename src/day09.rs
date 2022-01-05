use std::collections::HashMap;

use crate::utils::parse_grid;
use crate::utils::parse_grid_padded;

pub fn part_a(input: &str) -> i64 {
    let grid = parse_grid_padded::<100, 100, 1, 1, 1, 1>(input, 9);

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
    let grid = parse_grid::<100, 100>(input);

    let mut parents = [[[0usize; 2]; 100]; 100];
    for i in 0..100 {
        for j in 0..100 {
            parents[i][j] = [i, j];
        }
    }

    let find_root = |parents: [[[usize; 2]; 100]; 100], i: usize, j: usize| {
        let [mut ri, mut rj] = [i, j];
        while parents[ri][rj] != [ri, rj] {
            [ri, rj] = parents[ri][rj];
        }
        [ri, rj]
    };

    for i in 0..100 {
        for j in 0..100 {
            if i < 99 && grid[i][j] < 9 && grid[i + 1][j] < 9 {
                let [ri, rj] = find_root(parents, i, j);
                parents[ri][rj] = find_root(parents, i + 1, j);
            }
            if j < 99 && grid[i][j] < 9 && grid[i][j + 1] < 9 {
                let [ri, rj] = find_root(parents, i, j);
                parents[ri][rj] = find_root(parents, i, j + 1);
            }
        }
    }

    let mut basins: HashMap<[usize; 2], i64> = HashMap::new();
    for i in 0..100 {
        for j in 0..100 {
            if grid[i][j] < 9 {
                *basins.entry(find_root(parents, i, j)).or_insert(0) += 1;
            }
        }
    }

    let mut basin_sizes: Vec<i64> = basins.values().cloned().collect();
    basin_sizes.sort_unstable_by(|a, b| b.cmp(a));
    basin_sizes.iter().take(3).product()
}
