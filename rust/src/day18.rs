use itertools::Itertools;
use std::ops::Add;

#[derive(Debug, Clone, Copy)]
struct SfNode {
    val: usize,
    depth: usize,
}

// After spending a long time struggling to implement a binary tree with prev/next
// pointers, I realized that we can much more easily represent a snailfish number as an
// array of value/depth pairs.
#[derive(Debug, Clone)]
struct SfNum {
    data: Vec<SfNode>,
}

impl Add for SfNum {
    type Output = Self;

    fn add(self, rhs: SfNum) -> <Self as Add<SfNum>>::Output {
        let result = SfNum {
            data: [self.data, rhs.data]
                .concat()
                .into_iter()
                .map(|node| SfNode {
                    depth: node.depth + 1,
                    ..node
                })
                .collect(),
        };
        result.reduce()
    }
}

fn tree_pos_weight(depth: usize, mut tree_pos: usize) -> usize {
    let mut weight = 1;
    for i in 0..depth {
        weight *= if (tree_pos & (1 << i)) == 0 { 3 } else { 2 };
    }
    weight
}

impl SfNum {
    fn magnitude(&self) -> usize {
        self.data
            .iter()
            .fold(None, |state, node| {
                if let Some((acc, depth, mut tree_pos)) = state {
                    tree_pos += 1;
                    if node.depth > depth {
                        tree_pos <<= node.depth - depth;
                    } else {
                        tree_pos >>= depth - node.depth;
                    }
                    Some((
                        acc + node.val * tree_pos_weight(node.depth, tree_pos),
                        node.depth,
                        tree_pos,
                    ))
                } else {
                    Some((node.val * tree_pos_weight(node.depth, 0), node.depth, 0))
                }
            })
            .unwrap()
            .0
    }

    fn reduce(mut self) -> SfNum {
        loop {
            if let Some(i) = self
                .data
                .iter()
                .tuple_windows()
                .position(|(node1, node2)| node1.depth >= 5 && node1.depth == node2.depth)
            {
                let popped_val = self.data.remove(i + 1).val;
                if i > 0 {
                    self.data[i - 1].val += self.data[i].val;
                }
                if i + 1 < self.data.len() {
                    self.data[i + 1].val += popped_val;
                }
                self.data[i].val = 0;
                self.data[i].depth -= 1;
            } else if let Some(i) = self.data.iter().position(|node| node.val >= 10) {
                self.data[i].depth += 1;
                self.data.insert(
                    i + 1,
                    SfNode {
                        val: (self.data[i].val + 1) / 2,
                        ..self.data[i]
                    },
                );
                self.data[i].val /= 2;
            } else {
                break;
            }
        }
        self
    }

    fn _parse(s: &[u8], depth: usize) -> (SfNum, &[u8]) {
        match s[0] {
            b'[' => {
                let (fst, rest) = SfNum::_parse(&s[1..], depth + 1);
                assert_eq!(rest[0], b',');
                let (snd, rest) = SfNum::_parse(&rest[1..], depth + 1);
                assert_eq!(rest[0], b']');
                (
                    SfNum {
                        data: [fst.data, snd.data].concat(),
                    },
                    &rest[1..],
                )
            }
            d => (
                SfNum {
                    data: vec![SfNode {
                        val: (d - b'0') as usize,
                        depth,
                    }],
                },
                &s[1..],
            ),
        }
    }

    fn parse(s: &str) -> SfNum {
        SfNum::_parse(s.as_bytes(), 0).0
    }
}

pub fn part_a(input: &str) -> i64 {
    input
        .lines()
        .map(SfNum::parse)
        .fold(None, |acc, a| match acc {
            None => Some(a),
            Some(prev) => Some(prev + a),
        })
        .unwrap()
        .magnitude() as i64
}

pub fn part_b(input: &str) -> i64 {
    input
        .lines()
        .map(SfNum::parse)
        .tuple_combinations()
        .map(|(a, b)| (a + b).magnitude())
        .max()
        .unwrap() as i64
}
