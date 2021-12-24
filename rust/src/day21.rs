use itertools::Itertools;
use std::cmp::max;
use std::str;

pub fn part_a(input: &str) -> i64 {
    let (mut p1, mut p2) = input
        .lines()
        .map(|line| scan_fmt!(line, "Player {*d} starting position: {d}", i64).unwrap() - 1)
        .next_tuple()
        .unwrap();
    let mut turn = 0;
    let mut s1 = 0;
    let mut s2 = 0;
    for chunk in (1..=100).cycle().chunks(3).into_iter() {
        turn += 1;
        p1 += chunk.sum::<i64>() % 10;
        p1 %= 10;
        s1 += p1 + 1;
        if s1 >= 1000 {
            return 3 * turn * s2;
        }
        (p1, p2, s1, s2) = (p2, p1, s2, s1);
    }

    panic!("should never happen");
}

pub fn part_b(input: &str) -> i64 {
    let (p1s, p2s) = input
        .lines()
        .map(|line| scan_fmt!(line, "Player {*d} starting position: {d}", usize).unwrap() - 1)
        .next_tuple()
        .unwrap();

    let mut states = [[[[[0i64; 2]; 10]; 10]; 21]; 21];

    let smax = 21;

    let roll_combos = [0, 0, 0, 1, 3, 6, 7, 6, 3, 1];

    for ssum in (0..2 * smax - 1).rev() {
        for s1 in (0..smax).rev() {
            if s1 > ssum || s1 + smax <= ssum {
                continue;
            }
            let s2 = ssum - s1;
            for p1 in 0..10 {
                for p2 in 0..10 {
                    for r in 3..=9 {
                        // next position of current player
                        let p1n = (p1 + r) % 10;
                        // next score of current player
                        let s1n = s1 + p1n + 1;
                        if s1n >= smax {
                            // game is over
                            states[s1][s2][p1][p2][0] += roll_combos[r];
                        } else {
                            states[s1][s2][p1][p2][0] +=
                                roll_combos[r] * states[s2][s1n][p2][p1n][1];
                            states[s1][s2][p1][p2][1] +=
                                roll_combos[r] * states[s2][s1n][p2][p1n][0];
                        }
                    }
                }
            }
        }
    }

    max(states[0][0][p1s][p2s][0], states[0][0][p1s][p2s][1])
}
