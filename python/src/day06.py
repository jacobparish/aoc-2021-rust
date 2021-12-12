from collections import defaultdict
from aocd import data, submit


def part_a() -> int:
    fish = [int(n) for n in data.split(",")]
    for _ in range(80):
        num_fish = len(fish)
        for i in range(num_fish):
            if fish[i] == 0:
                fish[i] = 6
                fish.append(8)
            else:
                fish[i] -= 1
    return len(fish)


def part_b() -> int:
    fish = [int(n) for n in data.split(",")]
    curr_day = defaultdict(int)
    for f in fish:
        curr_day[f] += 1

    for _ in range(256):
        next_day = defaultdict(int)
        for i in range(8):
            next_day[i] = curr_day[i + 1]
        next_day[8] += curr_day[0]
        next_day[6] += curr_day[0]
        curr_day = next_day

    return sum(curr_day.values())


if __name__ == "__main__":
    submit(part_a(), part="a", day=6, year=2021)
    submit(part_b(), part="b", day=6, year=2021)
