fn parse_boards(lines: Vec<&str>) -> Vec<Vec<i64>> {
    lines
        .chunks(6)
        .map(|chunk| {
            chunk[..5]
                .iter()
                .map(|line| line.split_whitespace().map(|s| s.parse().unwrap()))
                .flatten()
                .collect()
        })
        .collect()
}

fn mark_board(b: &mut Vec<i64>, c: i64) {
    for i in 0..25 {
        if b[i] == c {
            // use -1 to represent marked position
            b[i] = -1
        }
    }
}

pub fn part_a(input: &str) -> i64 {
    let lines: Vec<&str> = input.lines().collect();
    let calls = lines[0].split(",").map(|s| s.parse().unwrap());
    let mut boards = parse_boards(lines[2..].to_vec());
    for c in calls {
        for mut b in boards.iter_mut() {
            mark_board(b, c);
            let win = (0..5)
                .any(|i| (0..5).all(|j| b[5 * i + j] == -1) || (0..5).all(|j| b[5 * j + i] == -1));
            if win {
                let total: i64 = b.iter().filter(|x| x > &&0).sum();
                return c * total;
            }
        }
    }

    panic!("none of the boards won");
}

pub fn part_b(input: &str) -> i64 {
    let lines: Vec<&str> = input.lines().collect();
    let calls = lines[0].split(",").map(|s| s.parse().unwrap());
    let mut boards = parse_boards(lines[2..].to_vec());
    let mut wins = vec![false; boards.len()];
    for c in calls {
        for (n, mut b) in boards.iter_mut().enumerate() {
            mark_board(b, c);
            wins[n] = (0..5)
                .any(|i| (0..5).all(|j| b[5 * i + j] == -1) || (0..5).all(|j| b[5 * j + i] == -1));
            if wins.iter().cloned().all(|w| w) {
                let total: i64 = b.iter().filter(|x| x > &&0).sum();
                return c * total;
            }
        }
    }

    panic!("some of the boards never won");
}
