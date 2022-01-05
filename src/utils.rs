use std::convert::TryInto;

pub fn parse_grid<const WIDTH: usize, const HEIGHT: usize>(s: &str) -> [[u8; WIDTH]; HEIGHT] {
    s.lines()
        .map(|line| {
            line.bytes()
                .map(|c| (c - b'0') as u8)
                .collect::<Vec<u8>>()
                .try_into()
                .unwrap()
        })
        .collect::<Vec<[u8; WIDTH]>>()
        .try_into()
        .unwrap()
}

pub fn parse_grid_padded<
    const WIDTH: usize,
    const HEIGHT: usize,
    const PAD_LEFT: usize,
    const PAD_RIGHT: usize,
    const PAD_TOP: usize,
    const PAD_BOTTOM: usize,
>(
    s: &str,
    pad_with: u8,
) -> [[u8; WIDTH + PAD_LEFT + PAD_RIGHT]; HEIGHT + PAD_TOP + PAD_BOTTOM] {
    let mut rows_vec: Vec<[u8; WIDTH + PAD_LEFT + PAD_RIGHT]> = s
        .lines()
        .map(|line| {
            let mut vec: Vec<u8> = line.bytes().map(|c| c - b'0').collect();
            for _ in 0..PAD_LEFT {
                vec.insert(0, pad_with);
            }
            for _ in 0..PAD_RIGHT {
                vec.push(pad_with);
            }
            vec.try_into().unwrap()
        })
        .collect();
    for _ in 0..PAD_TOP {
        rows_vec.insert(0, [pad_with; WIDTH + PAD_LEFT + PAD_RIGHT]);
    }
    for _ in 0..PAD_BOTTOM {
        rows_vec.push([pad_with; WIDTH + PAD_LEFT + PAD_RIGHT]);
    }
    rows_vec.try_into().unwrap()
}
