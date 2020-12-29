use itertools::Itertools;
use std::collections::HashMap;

fn valid_passports(raw: &str) -> usize {
    let fields = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    raw.lines()
        .group_by(|line| *line != "")
        .into_iter()
        .filter(|(key, _group)| *key)
        .map(|(_key, group)| group.collect::<Vec<&str>>().concat())
        .filter(|group| fields.iter().all(|field| group.contains(field)))
        .count()
}

fn valid_passports_part2(raw: &str) -> usize {
    let fields = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    let passports: Vec<String> = raw
        .lines()
        .group_by(|line| *line != "")
        .into_iter()
        .filter(|(key, _group)| *key)
        .map(|(_key, group)| group.collect::<Vec<&str>>().join(" "))
        .filter(|group| fields.iter().all(|field| group.contains(field)))
        .collect();

    let mapped_passports: Vec<_> = passports
        .iter()
        .map(|p| {
            p.split_whitespace()
                .flat_map(|p| p.split(':'))
                .tuples()
                .collect::<HashMap<_, _>>()
        })
        .collect();

    mapped_passports
        .iter()
        .filter(|p| is_valid_passport(p))
        .count()
}

fn is_valid_passport(passport: &HashMap<&str, &str>) -> bool {
    passport.iter().all(|(&k, v)| match k {
        "byr" => (1920..=2002).contains(&v.parse().unwrap_or(0)),
        "iyr" => (2010..=2020).contains(&v.parse().unwrap_or(0)),
        "eyr" => (2020..=2030).contains(&v.parse().unwrap_or(0)),
        "hcl" => {
            v.starts_with('#') && v.len() == 7 && v.chars().skip(1).all(|c| c.is_ascii_hexdigit())
        }
        "ecl" => ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(v),
        "pid" => v.len() == 9 && v.chars().all(|c| c.is_ascii_digit()),
        "cid" => true,
        "hgt" => {
            let height = v[0..(v.len() - 2)].parse().unwrap_or(0);
            match &v[(v.len() - 2)..] {
                "cm" => (150..=193).contains(&height),
                "in" => (59..=76).contains(&height),
                _ => false,
            }
        }
        _ => unreachable!(),
    })
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    pub fn example_part1() {
        let raw = include_str!("../inputs/day4_example1.txt");
        assert_eq!(valid_passports(raw), 2);
    }

    #[test]
    pub fn part1() {
        let raw = include_str!("../inputs/day4.txt");
        assert_eq!(valid_passports(raw), 256);
    }

    #[test]
    pub fn example_part2() {
        let raw = include_str!("../inputs/day4_example2.txt");
        assert_eq!(valid_passports_part2(raw), 4);
    }

    #[test]
    pub fn part2() {
        let raw = include_str!("../inputs/day4.txt");
        assert_eq!(valid_passports_part2(raw), 198);
    }
}
