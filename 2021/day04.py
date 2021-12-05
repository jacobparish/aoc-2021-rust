from aocd import lines, submit


def part_a() -> int:
    calls = [int(c) for c in lines[0].split(",")]
    boards = []
    current_board = []
    for line in lines[2:]:
        if not line:
            boards.append(current_board)
            current_board = []
        else:
            current_board.append(line)
    boards = [
        [[[int(x), False] for x in line.split()] for line in board] for board in boards
    ]
    for c in calls:
        for board in boards:
            for i in range(5):
                for j in range(5):
                    if board[i][j][0] == c:
                        board[i][j][1] = True

            win = any(
                all(board[i][j][1] for j in range(5))
                or all(board[j][i][1] for j in range(5))
                for i in range(5)
            )
            if win:
                return c * sum(
                    board[i][j][0]
                    for i in range(5)
                    for j in range(5)
                    if not board[i][j][1]
                )


def part_b() -> int:
    calls = [int(c) for c in lines[0].split(",")]
    boards = []
    current_board = []
    for line in lines[2:]:
        if not line:
            boards.append(current_board)
            current_board = []
        else:
            current_board.append(line)
    boards = [
        [[[int(e), False] for e in line.split()] for line in board] for board in boards
    ]
    wins = len(boards) * [False]
    for c in calls:
        for n, board in enumerate(boards):
            for i in range(5):
                for j in range(5):
                    if board[i][j][0] == c:
                        board[i][j][1] = True

            win = any(
                all(board[i][j][1] for j in range(5))
                or all(board[j][i][1] for j in range(5))
                for i in range(5)
            )
            if win:
                wins[n] = True
                if all(wins):
                    return c * sum(
                        board[i][j][0]
                        for i in range(5)
                        for j in range(5)
                        if not board[i][j][1]
                    )


if __name__ == "__main__":
    submit(part_a(), part="a", day=4, year=2021)
    submit(part_b(), part="b", day=4, year=2021)
