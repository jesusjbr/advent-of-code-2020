# Hacks

# Day 2

Parse with split instead of nom 20% slower, but more readable.
```Rust 
use itertools::Itertools;
fn from_str(input: &'a str) -> Self {
    input
        .split([':', '-', ' '].as_ref())
        .filter(|s| !s.is_empty())
        .collect_tuple()
        .map(|(min, max, letter, password)| Database {
            policy: Policy {
                min: min.parse().unwrap(),
                max: max.parse().unwrap(),
                letter: letter.chars().next().unwrap(),
            },
            password: password,
        })
        .unwrap()
}
```
## Part 1
70% improved
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
20% improved
```Rust 
fn is_valid_part2_v2(&self) -> bool {
        let password = self.password.as_bytes();
        (password[self.policy.min - 1] == self.policy.letter as u8)
            ^ (password[self.policy.max - 1] == self.policy.letter as u8)
    }
```

