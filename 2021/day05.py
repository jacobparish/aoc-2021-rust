import numpy as np
from aocd import lines, submit


def part_a() -> int:
    grid = np.zeros((1000, 1000), dtype=int)
    for line in lines:
        [first, last] = line.split(" -> ")
        [x1, y1] = first.split(",")
        [x2, y2] = last.split(",")
        x1 = int(x1)
        x2 = int(x2)
        y1 = int(y1)
        y2 = int(y2)
        if x1 == x2:
            for i in range(min(y1, y2), max(y1, y2) + 1):
                grid[x1][i] += 1
        elif y1 == y2:
            for i in range(min(x1, x2), max(x1, x2) + 1):
                grid[i][y1] += 1
    return sum(grid[i][j] > 1 for i in range(1000) for j in range(1000))


def part_b() -> int:
    grid = np.zeros((1000, 1000), dtype=int)
    for line in lines:
        [first, last] = line.split(" -> ")
        [x1, y1] = first.split(",")
        [x2, y2] = last.split(",")
        x1 = int(x1)
        x2 = int(x2)
        y1 = int(y1)
        y2 = int(y2)
        if x1 == x2:
            for i in range(min(y1, y2), max(y1, y2) + 1):
                grid[x1][i] += 1
        elif y1 == y2:
            for i in range(min(x1, x2), max(x1, x2) + 1):
                grid[i][y1] += 1
        else:
            stepx = 1 if x1 < x2 else -1
            stepy = 1 if y1 < y2 else -1
            for i, j in zip(
                range(x1, x2 + stepx, stepx),
                range(y1, y2 + stepy, stepy),
            ):
                grid[i][j] += 1

    return sum(grid[i][j] > 1 for i in range(1000) for j in range(1000))


if __name__ == "__main__":
    submit(part_a(), part="a", day=5, year=2021)
    submit(part_b(), part="b", day=5, year=2021)
