use itertools::FoldWhile;
use itertools::Itertools;

fn hex_to_bit_vec(s: &str) -> Vec<bool> {
    s.bytes()
        .filter_map(|c| match c {
            b'0'..=b'9' => Some(c - b'0'),
            b'A'..=b'F' => Some(c - b'A' + 0xA),
            _ => None,
        })
        .map(|b| vec![(b & 8) == 8, (b & 4) == 4, (b & 2) == 2, (b & 1) == 1])
        .flatten()
        .collect()
}

fn bit_vec_to_num(bits: &[bool]) -> usize {
    bits.iter().fold(0, |acc, &b| (acc << 1) + b as usize)
}

enum PacketData {
    Lit(usize),
    Op {
        type_id: usize,
        subpackets: Vec<Packet>,
    },
}

struct Packet {
    version: usize,
    data: PacketData,
}

fn parse_packet(mut bits: &[bool]) -> (Packet, usize) {
    let version = bit_vec_to_num(&bits[0..3]);
    let type_id = bit_vec_to_num(&bits[3..6]);
    let (data, n) = parse_packet_data(type_id, &bits[6..]);
    (Packet { version, data }, 6 + n)
}

fn parse_packet_data(type_id: usize, mut bits: &[bool]) -> (PacketData, usize) {
    // Handle literals first
    if type_id == 4 {
        let lit_bits: Vec<bool> = bits
            .chunks_exact(5)
            .fold_while(Vec::new(), |mut acc, chunk| {
                acc.extend_from_slice(&chunk[1..]);
                if chunk[0] {
                    FoldWhile::Continue(acc)
                } else {
                    FoldWhile::Done(acc)
                }
            })
            .into_inner();
        return (
            PacketData::Lit(bit_vec_to_num(lit_bits.as_slice())),
            lit_bits.len() * 5 / 4,
        );
    }

    let length_type_id = bits[0];
    bits = &bits[1..];

    let mut subpackets = Vec::new();
    if length_type_id {
        let mut count_bits;
        (count_bits, bits) = bits.split_at(11);
        let subpackets_count = bit_vec_to_num(count_bits);
        let mut subpackets_length = 0;
        for _ in 0..subpackets_count {
            let (p, n) = parse_packet(bits);
            subpackets.push(p);
            bits = &bits[n..];
            subpackets_length += n;
        }
        (
            PacketData::Op {
                type_id,
                subpackets,
            },
            1 + 11 + subpackets_length,
        )
    } else {
        let mut length_bits;
        (length_bits, bits) = bits.split_at(15);
        let subpackets_length_expected = bit_vec_to_num(length_bits);
        let mut subpackets_length_actual = 0;
        while subpackets_length_actual < subpackets_length_expected {
            let (p, n) = parse_packet(bits);
            subpackets.push(p);
            bits = &bits[n..];
            subpackets_length_actual += n;
        }
        assert_eq!(subpackets_length_actual, subpackets_length_expected);
        (
            PacketData::Op {
                type_id,
                subpackets,
            },
            1 + 15 + subpackets_length_actual,
        )
    }
}

fn sum_packet_versions(packet: &Packet) -> usize {
    if let PacketData::Op {
        type_id: _,
        subpackets,
    } = &packet.data
    {
        packet.version + subpackets.iter().map(sum_packet_versions).sum::<usize>()
    } else {
        packet.version
    }
}

fn eval_packet(packet: &Packet) -> usize {
    match &packet.data {
        PacketData::Op {
            type_id,
            subpackets,
        } => match type_id {
            0 => subpackets.iter().map(eval_packet).sum(),
            1 => subpackets.iter().map(eval_packet).product(),
            2 => subpackets.iter().map(eval_packet).min().unwrap(),
            3 => subpackets.iter().map(eval_packet).max().unwrap(),
            5 => (eval_packet(&subpackets[0]) > eval_packet(&subpackets[1])) as usize,
            6 => (eval_packet(&subpackets[0]) < eval_packet(&subpackets[1])) as usize,
            7 => (eval_packet(&subpackets[0]) == eval_packet(&subpackets[1])) as usize,
            _ => panic!("unexpected type_id in operator packet"),
        },
        &PacketData::Lit(x) => x,
    }
}

pub fn part_a(input: &str) -> i64 {
    let bits = hex_to_bit_vec(input);
    let (root, _) = parse_packet(bits.as_slice());
    sum_packet_versions(&root) as i64
}

pub fn part_b(input: &str) -> i64 {
    let bits = hex_to_bit_vec(input);
    let (root, _) = parse_packet(bits.as_slice());
    eval_packet(&root) as i64
}
