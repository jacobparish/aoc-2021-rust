from aocd import numbers, submit


def part_a() -> int:
    return sum(d1 < d2 for d1, d2 in zip(numbers, numbers[1:]))


def part_b() -> int:
    return sum(d1 < d2 for d1, d2 in zip(numbers, numbers[3:]))


if __name__ == "__main__":
    submit(part_a(), part="a", day=1, year=2021)
    submit(part_b(), part="b", day=1, year=2021)
