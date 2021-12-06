from parse import parse
from aocd import lines, submit


def part_a() -> int:
    x = 0
    y = 0
    for line in lines:
        [dir, n] = parse("{} {:d}", line)
        if dir == "forward":
            x += n
        elif dir == "up":
            y -= n
        elif dir == "down":
            y += n
    return x * y


def part_b() -> int:
    x = 0
    y = 0
    aim = 0
    for line in lines:
        [dir, n] = parse("{} {:d}", line)
        if dir == "forward":
            x += n
            y += aim * n
        elif dir == "up":
            aim -= n
        elif dir == "down":
            aim += n
    return x * y


if __name__ == "__main__":
    submit(part_a(), part="a", day=2, year=2021)
    submit(part_b(), part="b", day=2, year=2021)
