pub fn part_a(input: &str) -> i64 {
    let lines: Vec<&str> = input.lines().collect();
    let len = lines[0].len();
    let gamma: i64 = (0..len)
        .map(|i| {
            let count = lines.iter().fold(0, |acc, line| match line.as_bytes()[i] {
                b'0' => acc - 1,
                b'1' => acc + 1,
                _ => panic!("unexpected character in line"),
            });
            if count > 0 {
                1 << (len - i - 1)
            } else {
                0
            }
        })
        .sum();
    let epsilon = (1 << len) - gamma - 1;
    gamma * epsilon
}

fn find_rating(mut lines: Vec<&str>, use_most_common: bool) -> i64 {
    for i in 0..lines[0].len() {
        let count = lines.iter().fold(0, |acc, line| match line.as_bytes()[i] {
            b'0' => acc - 1,
            b'1' => acc + 1,
            _ => panic!("unexpected character in line"),
        });
        let target_bit = if (count >= 0) == use_most_common {
            b'1'
        } else {
            b'0'
        };
        lines = lines
            .iter()
            .filter(|line| line.as_bytes()[i] == target_bit)
            .cloned()
            .collect();
        if lines.len() == 1 {
            return i64::from_str_radix(lines[0], 2).unwrap();
        }
    }
    panic!("failed to calculate rating");
}

pub fn part_b(input: &str) -> i64 {
    let lines: Vec<&str> = input.lines().collect();
    let o2_rating = find_rating(lines.clone(), true);
    let co2_rating = find_rating(lines, false);
    o2_rating * co2_rating
}
