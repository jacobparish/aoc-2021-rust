use scan_fmt::scan_fmt;

struct Instr {
    dir: String,
    x: i64,
}

impl From<&str> for Instr {
    fn from(s: &str) -> Instr {
        let (dir, x) = scan_fmt!(s, "{} {d}", String, i64).unwrap();
        Instr { dir, x }
    }
}

pub fn part_a(input: &str) -> i64 {
    struct SubState {
        h: i64, // horizontal position
        d: i64, // depth
    }
    let final_state = input.lines().map(Instr::from).fold(
        SubState { h: 0, d: 0 },
        |SubState { h, d }, Instr { dir, x }| match dir.as_str() {
            "forward" => SubState { h: h + x, d },
            "up" => SubState { h, d: d - x },
            "down" => SubState { h, d: d + x },
            _ => panic!("invalid direction"),
        },
    );
    final_state.h * final_state.d
}

pub fn part_b(input: &str) -> i64 {
    struct SubState {
        h: i64, // horizontal position
        d: i64, // depth
        aim: i64,
    }
    let final_state = input.lines().map(Instr::from).fold(
        SubState { h: 0, d: 0, aim: 0 },
        |SubState { h, d, aim }, Instr { dir, x }| match dir.as_str() {
            "forward" => SubState {
                h: h + x,
                d: d + aim * x,
                aim,
            },
            "up" => SubState { h, d, aim: aim - x },
            "down" => SubState { h, d, aim: aim + x },
            _ => panic!("invalid direction"),
        },
    );
    final_state.h * final_state.d
}
