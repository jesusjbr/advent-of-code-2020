use itertools::Itertools;

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
