use std::iter::zip;
use std::convert::TryInto;

struct Instr {
    dir: String,
    x: i32,
}

impl From<&str> for Instr {
    fn from(s: &str) -> Instr {
        let (dir, x) = scan_fmt!(s, "{} {d}", String, i32).unwrap();
        Instr { dir, x }
    }
}

pub fn part_a(input: &str) -> i32 {
    let instrs: Vec<Instr> = input.split_terminator("\n").map(Instr::from).collect();
    struct SubState {
        h: i32, // horizontal position
        d: i32, // depth
    }
    let final_state = instrs.iter().fold(SubState {h:0,d:0}, |SubState{h, d}, Instr{dir,x}| {
        match dir.as_str() {
            "forward" => SubState {h: h + x, d},
            "up" => SubState {h, d: d - x},
            "down" => SubState {h, d: d + x},
            _ => panic!("invalid direction")
        }
    });
    final_state.h * final_state.d
}

pub fn part_b(input: &str) -> i32 {
    let instrs: Vec<Instr> = input.split_terminator("\n").map(Instr::from).collect();
    struct SubState {
        h: i32, // horizontal position
        d: i32, // depth
        aim: i32,
    }
    let final_state = instrs.iter().fold(SubState {h:0,d:0,aim:0}, |SubState{h, d, aim}, Instr{dir,x}| {
        match dir.as_str() {
            "forward" => SubState {h: h + x, d: d + aim * x, aim},
            "up" => SubState {h, d, aim: aim - x},
            "down" => SubState {h, d, aim: aim + x},
            _ => panic!("invalid direction")
        }
    });
    final_state.h * final_state.d
}
