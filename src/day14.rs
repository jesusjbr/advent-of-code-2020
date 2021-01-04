use itertools::Itertools;
use std::{collections::HashMap, num::ParseIntError, str::FromStr};

/*
    0X1  Input mask divide into 2 masks
    001  Or mask (for ones in input)   1
    011  And mask (for zeros in input) 3
    100  mem[8]                        4
    -----------
    10101110101  Result = (mem[8] | OrMask) & AndMask
    let result = (4_u32 | 1_u32) &  3_u32;
    assert_eq!(result, 1);
*/
struct Mask {
    or: u64,
    and: u64,
}

impl FromStr for Mask {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (and, or) =
            s.bytes()
                .rev()
                .enumerate()
                .fold((u64::MAX, 0), |(and, or), (i, b)| match b {
                    b'0' => (and & !(1 << i), or),
                    b'1' => (and, or | 1 << i),
                    _ => (and, or),
                });

        Ok(Mask { and, or })
    }
}

impl Mask {
    fn apply(&self, n: u64) -> u64 {
        (n | self.or) & self.and
    }
}

fn sum_memory(input: &str) -> u64 {
    let mut memory: HashMap<u64, u64> = HashMap::new();
    let mut value: u64;
    let mut address: u64;
    //Null mask
    let mut mask = Mask::from_str("XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX").unwrap();
    for line in input.lines() {
        if line.contains("mask") {
            mask = Mask::from_str(line.split(" = ").last().unwrap()).unwrap();
        } else {
            let mut line_parts = line.split(" = ");
            value = line_parts.clone().last().unwrap().parse().unwrap();
            address = line_parts
                .nth(0)
                .unwrap()
                .split(|c| c == '[' || c == ']')
                .nth(1)
                .unwrap()
                .parse()
                .unwrap();
            memory.insert(address, mask.apply(value));
        }
    }
    memory.values().sum()
}

/*
    0X1  Input mask divide into 2 masks
    001  Or mask (for ones in input)   1
    010  Xor masks (for X in input) With powerset we get each combination in this case 000 and 010
    100  mem[8]                        4
    -----------
    10101110101  Result = (mem[8] | OrMask) ^ XorMasks
    let result1 = (4_u32 | 1_u32) ^  2_u32;
    let result2 = (4_u32 | 1_u32) ^  0_u32;
    assert_eq!(result1, 7);
    assert_eq!(result2, 5);
*/
struct DecoderMask {
    or: u64,
    xor: Vec<u64>,
}

impl FromStr for DecoderMask {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut xor = vec![];
        let (mut xor, or) = s
            .bytes()
            .rev()
            .enumerate()
            .fold((xor, 0), |(mut and, or), (i, b)| match b {
                b'X' => {
                    let n = 1 << i;
                    and.push(n);
                    (and, or)
                }
                b'1' => (and, or | 1 << i),
                _ => (and, or),
            });
        xor = xor
            .iter()
            .powerset()
            .map(|vec| vec.into_iter().sum())
            .collect();
        Ok(DecoderMask { xor: xor, or })
    }
}

impl DecoderMask {
    fn apply<'a>(&'a self, n: u64) -> impl Iterator<Item = u64> + 'a {
        let num = (n | self.or);
        self.xor.iter().map(move |floating| num ^ floating)
    }
}

fn sum_memory_decoder(input: &str) -> u64 {
    let mut memory: HashMap<u64, u64> = HashMap::new();
    let mut value: u64;
    let mut address: u64;
    //Null mask
    let mut mask = DecoderMask::from_str("00000000000000000000000000000000000").unwrap();
    for line in input.lines() {
        if line.contains("mask") {
            mask = DecoderMask::from_str(line.split(" = ").last().unwrap()).unwrap();
        } else {
            let mut line_parts = line.split(" = ");
            value = line_parts.clone().last().unwrap().parse().unwrap();
            address = line_parts
                .nth(0)
                .unwrap()
                .split(|c| c == '[' || c == ']')
                .nth(1)
                .unwrap()
                .parse()
                .unwrap();
            for decoded_address in mask.apply(address) {
                memory.insert(decoded_address, value);
            }
        }
    }
    memory.values().sum()
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn example1_part1() {
        let input = include_str!("../inputs/day14_example1.txt");
        assert_eq!(sum_memory(input), 165);
    }

    #[test]
    fn part1() {
        let input = include_str!("../inputs/day14.txt");
        assert_eq!(sum_memory(input), 5902420735773);
    }

    #[test]
    fn example1_part2() {
        let input = include_str!("../inputs/day14_example2.txt");
        assert_eq!(sum_memory_decoder(input), 208);
    }

    #[test]
    fn part2() {
        let input = include_str!("../inputs/day14.txt");
        assert_eq!(sum_memory_decoder(input), 3801988250775);
    }
}
