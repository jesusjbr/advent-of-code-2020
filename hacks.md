# Hacks

# Day 2

This was my initial solution. Using nom is 20% faster, but less readable, more difficult and an unnecessary dependency. A plain split is better.
```Rust
use nom::bytes::complete::{tag, take};
use nom::character::complete::{char, digit1, not_line_ending};
use nom::sequence::tuple;
fn from_str(input: &'a str) -> Self {
        // The target pattern looks like this:
        // 1-3 a: abcde
        let raw_parsed = tuple::<_, _, nom::error::Error<&str>, _>((
            digit1,
            char('-'),
            digit1,
            char(' '),
            take(1usize),
            tag(": "),
            not_line_ending,
        ))(input)
        .unwrap()
        .1;
        Database {
            policy: Policy {
                min: raw_parsed.0.parse().unwrap(),
                max: raw_parsed.2.parse().unwrap(),
                letter: raw_parsed.4.chars().next().unwrap(),
            },
            password: raw_parsed.6,
        }
} 

```
## Part 1
70% speed improvement. Needs bytecount crate.
```Rust 
fn is_validv2(&self) -> bool {
        // We can do this because we know all the inputs are ASCII,
        // this is faster than the unicode-safe method
        //
        // Clippy tells us to use the `bytecount` crate because it's faster than doing 
        //`.iter().filter().count()`
        let frequency = bytecount::count(self.password.as_bytes(), self.policy.letter as u8);
        (self.policy.min..=self.policy.max).contains(&frequency)
    }
```

## Part 2
20% speed improvement
```Rust 
fn is_valid_part2_v2(&self) -> bool {
        let password = self.password.as_bytes();
        (password[self.policy.min - 1] == self.policy.letter as u8)
            ^ (password[self.policy.max - 1] == self.policy.letter as u8)
    }
```

# Day 6

## Part 2
85% speed improvement. Mask / bitset method
```Rust 
fn mask(input: &str) -> u32 {
    input
        .split("\n\n")
        .map(|group| {
            group
                .split_whitespace()
                .map(|line| line.bytes().fold(0u32, |x, b| x | 1 << (b - b'a')))
                .fold(!0, |acc, x| acc & x)
                .count_ones()
        })
        .sum()
}
```

# Day 7
## Part 2
An possible improvement would be adding memoization.

# Day 18
This day I took a rogue route instead of implementing a solution myself.
I grabbed the crate `eval` and modified the priority of the operators sum and mul.

```Rust
#[test]
fn part1_same_priority() {
    let input = include_str!("../inputs/input.txt");
    let res : u64 = input.lines().map(|l| match eval(l).unwrap() {
        Value::Null => 0,
        Value::Bool(_) => 0,
        Value::Number(n) => n.as_u64().unwrap(),
        Value::String(_) => 0,
        Value::Array(_) => 0,
        Value::Object(_) => 0,
    }).sum();
    assert_eq!(res, 3885386961962);
}

#[test]
fn part2_sum_higher_priority() {
    let input = include_str!("../inputs/input.txt");
    let res : u64 = input.lines().map(|l| match eval(l).unwrap() {
        Value::Null => 0,
        Value::Bool(_) => 0,
        Value::Number(n) => n.as_u64().unwrap(),
        Value::String(_) => 0,
        Value::Array(_) => 0,
        Value::Object(_) => 0,
    }).sum();
    assert_eq!(res, 112899558798666);
}
```