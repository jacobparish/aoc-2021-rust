from aocd import lines, submit


def part_a() -> int:
    x = 0
    y = 0
    for l in lines:
        [dir, n] = l.split(" ")
        if dir == "forward":
            x += int(n)
        elif dir == "up":
            y -= int(n)
        elif dir == "down":
            y += int(n)
    return x * y


def part_b() -> int:
    x = 0
    y = 0
    aim = 0
    for l in lines:
        [dir, n] = l.split(" ")
        if dir == "forward":
            x += int(n)
            y += aim * int(n)
        elif dir == "up":
            aim -= int(n)
        elif dir == "down":
            aim += int(n)
    return x * y


if __name__ == "__main__":
    submit(part_a(), part="a", day=2, year=2021)
    submit(part_b(), part="b", day=2, year=2021)
