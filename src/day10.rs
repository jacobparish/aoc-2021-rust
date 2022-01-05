fn match_char(c: u8) -> u8 {
    match c {
        b')' => b'(',
        b']' => b'[',
        b'}' => b'{',
        b'>' => b'<',
        _ => 0,
    }
}

fn score_char(c: u8) -> i64 {
    match c {
        b')' => 3,
        b']' => 57,
        b'}' => 1197,
        b'>' => 25137,
        _ => 0,
    }
}

pub fn part_a(input: &str) -> i64 {
    input
        .lines()
        .map(|line| {
            let mut stack: Vec<u8> = vec![];
            for &c in line.as_bytes() {
                let m = match_char(c);
                if m == 0 {
                    stack.push(c)
                } else if stack.pop() != Some(m) {
                    return score_char(c);
                }
            }
            0
        })
        .sum()
}

fn score_remaining(mut chars: Vec<u8>) -> i64 {
    chars.reverse();
    let mut score = 0;
    for c in chars {
        score *= 5;
        score += match c {
            b'(' => 1,
            b'[' => 2,
            b'{' => 3,
            b'<' => 4,
            _ => 0,
        }
    }
    score
}

pub fn part_b(input: &str) -> i64 {
    let mut scores: Vec<i64> = input
        .lines()
        .filter_map(|line| {
            let mut stack: Vec<u8> = vec![];
            for &c in line.as_bytes() {
                let m = match_char(c);
                if m == 0 {
                    stack.push(c)
                } else if stack.pop() != Some(m) {
                    return None;
                }
            }
            Some(score_remaining(stack))
        })
        .collect();

    scores.sort_unstable();
    *scores.get(scores.len() / 2).unwrap()
}
