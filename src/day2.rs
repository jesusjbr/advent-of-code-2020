use nom::bytes::complete::{tag, take};
use nom::character::complete::{char, digit1, not_line_ending};
use nom::sequence::tuple;

#[derive(Debug, Clone)]
struct Policy {
    min: usize,
    max: usize,
    letter: char,
}

#[derive(Debug, Clone)]
struct Database<'a> {
    policy: Policy,
    password: &'a str,
}

impl<'a> Database<'a> {
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

    fn is_valid(&self) -> bool {
        let frequency = self.password.matches(self.policy.letter).count();
        (self.policy.min..=self.policy.max).contains(&frequency)
    }

    fn is_valid_part2(&self) -> bool {
        (self.password.chars().nth(self.policy.min - 1).unwrap() == self.policy.letter)
            ^ (self.password.chars().nth(self.policy.max - 1).unwrap() == self.policy.letter)
    }
}

fn get_input(raw_input: &str) -> impl Iterator<Item = Database> {
    let databases = raw_input.lines().map(Database::from_str);
    databases
}

pub fn count_valids_first(input: &str) -> usize {
    let databases = get_input(input);
    databases.filter(|d| d.is_valid()).count()
}

pub fn count_valids_second(input: &str) -> usize {
    let databases = get_input(input);
    databases.filter(|d| d.is_valid_part2()).count()
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    pub fn part1() {
        let raw = include_str!("../inputs/day2_p2.txt");
        let are_valid = count_valids_first(&raw);
        assert_eq!(are_valid, 465);
    }

    #[test]
    pub fn part2() {
        let raw = include_str!("../inputs/day2_p2.txt");
        let are_valid = count_valids_second(&raw);
        assert_eq!(are_valid, 294);
    }
}
