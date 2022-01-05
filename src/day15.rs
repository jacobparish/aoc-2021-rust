use std::cmp::Ordering;
use std::collections::BinaryHeap;

use crate::utils::{parse_grid, parse_grid_padded};

#[derive(Eq, PartialEq, Clone, Copy)]
struct DijkstraNode {
    i: usize,
    j: usize,
    dist: usize,
}

impl Ord for DijkstraNode {
    fn cmp(&self, other: &Self) -> Ordering {
        return other.dist.cmp(&self.dist);
    }
}

impl PartialOrd for DijkstraNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn part_a(input: &str) -> i64 {
    const width: usize = 100;
    const height: usize = 100;

    let mut grid = parse_grid::<width, height>(input);

    let mut queue = BinaryHeap::new();
    queue.push(DijkstraNode {
        i: 0,
        j: 0,
        dist: 0,
    });
    let mut marked = [[false; width]; height];
    while let Some(v) = queue.pop() {
        if marked[v.i][v.j] {
            continue;
        }
        marked[v.i][v.j] = true;
        if v.i + 1 == height && v.j + 1 == width {
            return v.dist as i64;
        }
        if v.i > 0 {
            queue.push(DijkstraNode {
                i: v.i - 1,
                j: v.j,
                dist: v.dist + (grid[v.i - 1][v.j] as usize),
            })
        }
        if v.i + 1 < height {
            queue.push(DijkstraNode {
                i: v.i + 1,
                j: v.j,
                dist: v.dist + (grid[v.i + 1][v.j] as usize),
            })
        }

        if v.j > 0 {
            queue.push(DijkstraNode {
                i: v.i,
                j: v.j - 1,
                dist: v.dist + (grid[v.i][v.j - 1] as usize),
            })
        }
        if v.j + 1 < width {
            queue.push(DijkstraNode {
                i: v.i,
                j: v.j + 1,
                dist: v.dist + (grid[v.i][v.j + 1] as usize),
            })
        }
    }

    panic!("no path found to end node")
}

pub fn part_b(input: &str) -> i64 {
    const width: usize = 500;
    const height: usize = 500;

    let mut grid = parse_grid_padded::<100, 100, 0, 400, 0, 400>(input, 0);

    for a in 0..5 {
        for b in 0..5 {
            if a == 0 && b == 0 {
                continue;
            }
            for i in 0..100 {
                for j in 0..100 {
                    grid[100 * a + i][100 * b + j] = (grid[i][j] + (a + b) as u8 - 1) % 9 + 1;
                }
            }
        }
    }

    let mut queue = BinaryHeap::new();
    queue.push(DijkstraNode {
        i: 0,
        j: 0,
        dist: 0,
    });
    let mut marked = [[false; width]; height];
    while let Some(v) = queue.pop() {
        if marked[v.i][v.j] {
            continue;
        }
        marked[v.i][v.j] = true;
        if v.i + 1 == height && v.j + 1 == width {
            return v.dist as i64;
        }
        if v.i > 0 {
            queue.push(DijkstraNode {
                i: v.i - 1,
                j: v.j,
                dist: v.dist + (grid[v.i - 1][v.j] as usize),
            })
        }
        if v.i + 1 < height {
            queue.push(DijkstraNode {
                i: v.i + 1,
                j: v.j,
                dist: v.dist + (grid[v.i + 1][v.j] as usize),
            })
        }

        if v.j > 0 {
            queue.push(DijkstraNode {
                i: v.i,
                j: v.j - 1,
                dist: v.dist + (grid[v.i][v.j - 1] as usize),
            })
        }
        if v.j + 1 < width {
            queue.push(DijkstraNode {
                i: v.i,
                j: v.j + 1,
                dist: v.dist + (grid[v.i][v.j + 1] as usize),
            })
        }
    }

    panic!("no path found to end node")
}
