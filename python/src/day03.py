from aocd import lines, submit


def part_a() -> int:
    counts = []
    for line in lines:
        if not counts:
            counts = [0 for _ in range(len(line))]
        bits = list(line)
        for i in range(len(counts)):
            if bits[i] == "1":
                counts[i] += 1

    e = ["1" if i > len(lines) / 2 else "0" for i in counts]

    g = ["1" if i <= len(lines) / 2 else "0" for i in counts]

    return eval(f'0b{"".join(e)}') * eval(f'0b{"".join(g)}')


def part_b() -> int:
    length = len(lines[0])

    curr = lines
    for i in range(length):
        if len(curr) <= 1:
            break
        num_ones = sum(list(line)[i] == "1" for line in curr)
        curr = [
            line
            for line in curr
            if list(line)[i] == ("1" if num_ones >= len(curr) / 2 else "0")
        ]

    first = eval(f"0b{curr[0]}")

    curr = lines
    for i in range(length):
        if len(curr) <= 1:
            break
        num_ones = sum(list(line)[i] == "1" for line in curr)
        curr = [
            line
            for line in curr
            if list(line)[i] == ("0" if num_ones >= len(curr) / 2 else "1")
        ]

    second = eval(f"0b{curr[0]}")

    return first * second


if __name__ == "__main__":
    submit(part_a(), part="a", day=3, year=2021)
    submit(part_b(), part="b", day=3, year=2021)
