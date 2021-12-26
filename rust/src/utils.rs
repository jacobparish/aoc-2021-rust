use std::convert::TryInto;

pub fn parse_grid<const WIDTH: usize, const HEIGHT: usize>(s: &str) -> [[u8; WIDTH]; HEIGHT] {
    s.lines()
        .map(|line| {
            line.as_bytes()
                .iter()
                .map(|c| (c - b'0') as u8)
                .collect::<Vec<u8>>()
                .try_into()
                .unwrap()
        })
        .collect::<Vec<[u8; WIDTH]>>()
        .try_into()
        .unwrap()
}

pub fn parse_grid_and_pad<const WIDTH: usize, const HEIGHT: usize>(
    s: &str,
    pad_with: u8,
) -> [[u8; WIDTH]; HEIGHT] {
    let mut rows_vec: Vec<[u8; WIDTH]> = s
        .lines()
        .map(|line| {
            let mut vec: Vec<u8> = line.as_bytes().iter().map(|c| c - b'0').collect();
            vec.insert(0, pad_with);
            vec.push(pad_with);
            vec.try_into().unwrap()
        })
        .collect();

    rows_vec.insert(0, [pad_with; WIDTH]);
    rows_vec.push([pad_with; WIDTH]);
    rows_vec.try_into().unwrap()
}
