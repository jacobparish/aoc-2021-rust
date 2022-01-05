use std::convert::TryInto;

fn parse_input(input: &str, pad_amount: usize) -> (Vec<Vec<bool>>, usize, usize, [bool; 1 << 9]) {
    let mut lines = input.lines();

    let enhancement_alg: [bool; 1 << 9] = lines
        .next()
        .unwrap()
        .bytes()
        .map(|b| b == b'#')
        .collect::<Vec<bool>>()
        .try_into()
        .unwrap();

    let mut image: Vec<Vec<bool>> = lines
        .skip(1)
        .map(|line| {
            let mut row: Vec<bool> = line.bytes().map(|b| b == b'#').collect();
            for _ in 0..pad_amount {
                row.push(false);
                row.insert(0, false);
            }
            row
        })
        .collect();
    let width = image[0].len();
    for _ in 0..pad_amount {
        image.insert(0, vec![false; width]);
        image.push(vec![false; width]);
    }
    let height = image.len();
    (image, width, height, enhancement_alg)
}

fn enhance_image(
    mut image: Vec<Vec<bool>>,
    width: usize,
    height: usize,
    enhancement_alg: [bool; 1 << 9],
    steps: usize,
) -> Vec<Vec<bool>> {
    for _ in 0..steps {
        let mut next_image = vec![vec![false; width]; height];

        for i in 0..height {
            for j in 0..width {
                if i == 0 || i + 1 == height || j == 0 || j + 1 == height {
                    next_image[i][j] = !image[i][j];
                    continue;
                }
                let index = (1 << 8) * (image[i - 1][j - 1] as usize)
                    + (1 << 7) * (image[i - 1][j] as usize)
                    + (1 << 6) * (image[i - 1][j + 1] as usize)
                    + (1 << 5) * (image[i][j - 1] as usize)
                    + (1 << 4) * (image[i][j] as usize)
                    + (1 << 3) * (image[i][j + 1] as usize)
                    + (1 << 2) * (image[i + 1][j - 1] as usize)
                    + (1 << 1) * (image[i + 1][j] as usize)
                    + (1 << 0) * (image[i + 1][j + 1] as usize);
                next_image[i][j] = enhancement_alg[index];
            }
        }

        image = next_image;
    }

    image
}

fn count_lit_pixels(image: Vec<Vec<bool>>, width: usize, height: usize) -> usize {
    let mut num_lit = 0;
    for i in 0..height {
        for j in 0..width {
            if image[i][j] {
                num_lit += 1;
            }
        }
    }
    num_lit
}

pub fn part_a(input: &str) -> i64 {
    let (mut image, width, height, enhancement_alg) = parse_input(input, 3);
    image = enhance_image(image, width, height, enhancement_alg, 2);
    count_lit_pixels(image, width, height) as i64
}

pub fn part_b(input: &str) -> i64 {
    let (mut image, width, height, enhancement_alg) = parse_input(input, 51);
    image = enhance_image(image, width, height, enhancement_alg, 50);
    count_lit_pixels(image, width, height) as i64
}
