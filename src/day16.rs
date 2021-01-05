use std::collections::HashSet;
use std::ops::RangeInclusive;

fn parse_input(input: &str) -> (&str, &str, &str) {
    let (rules, rest) = (
        input.split("your ticket:").nth(0).unwrap(),
        input.split("your ticket:").nth(1).unwrap(),
    );

    let (your_ticket, nearby_tickets) = (
        rest.split("nearby tickets:").nth(0).unwrap(),
        rest.split("nearby tickets:").nth(1).unwrap(),
    );
    (rules.trim(), your_ticket.trim(), nearby_tickets.trim())
}

fn ticket_scannning_error_rate(input: &str) -> usize {
    let (rules, _, nearby_tickets) = parse_input(input);
    let rules: Vec<RangeInclusive<usize>> = rules
        .lines()
        .flat_map(|s| {
            s.split(':').last().unwrap().split(" or ").map(|r| {
                r.trim()
                    .split('-')
                    .next()
                    .unwrap()
                    .parse::<usize>()
                    .unwrap()
                    ..=r.trim()
                        .split('-')
                        .nth(1)
                        .unwrap()
                        .parse::<usize>()
                        .unwrap()
            })
        })
        .collect();
    let sum: usize = nearby_tickets
        .lines()
        .flat_map(|s| s.split(',').map(|n| n.parse::<usize>().unwrap()))
        .filter(|n| !rules.iter().any(|range| range.contains(n)))
        .sum();
    sum
}

fn who_is_who(input: &str) -> usize {
    let (rules, your_ticket, nearby_tickets) = parse_input(input);
    let your_ticket: Vec<usize> = your_ticket
        .split(',')
        .map(|s| s.parse::<usize>().unwrap())
        .collect();
    let rules: Vec<Vec<RangeInclusive<usize>>> = rules
        .lines()
        .map(|s| {
            s.split(':')
                .last()
                .unwrap()
                .split(" or ")
                .map(|r| {
                    r.trim()
                        .split('-')
                        .next()
                        .unwrap()
                        .parse::<usize>()
                        .unwrap()
                        ..=r.trim()
                            .split('-')
                            .nth(1)
                            .unwrap()
                            .parse::<usize>()
                            .unwrap()
                })
                .collect::<Vec<RangeInclusive<usize>>>()
        })
        .collect();

    //Fields enumerated by column
    let fields = nearby_tickets
        .lines()
        .flat_map(|s| {
            s.split(',')
                .map(|n| n.parse::<usize>().unwrap())
                .enumerate()
        })
        .filter(|(_i, n)| {
            rules
                .iter()
                .any(|rule| rule[0].contains(n) || rule[1].contains(n))
        });
    let rules_size = rules.len();

    //Matches the position of the rule with the positions of the posible columns in the tickets
    let positions: Vec<HashSet<usize>> = rules
        .iter()
        .map(|rule| {
            let mut v = HashSet::new();
            for column in 0..rules_size {
                if fields
                    .clone()
                    .filter(|(i, _field)| *i == column as usize)
                    .all(|(_i, f)| rule[0].contains(&f) || rule[1].contains(&f))
                {
                    v.insert(column);
                }
            }
            v
        })
        .collect();
    sieve(positions)
        .iter()
        .take(6)
        .map(|p| your_ticket[*p])
        .product()
}

fn sieve(mut options: Vec<HashSet<usize>>) -> Vec<usize> {
    /*Input example
    [6, 7, 10, 11, 12, 15, 16, 17, 19]
    [4, 6, 7, 10, 11, 12, 14, 15, 16, 17, 18, 19]
    [4, 6, 7, 10, 11, 12, 14, 15, 16, 17, 19]
    [6, 7, 10, 11, 12, 14, 15, 16, 17, 19]
    [6, 7, 11, 12, 15, 16, 17, 19]
    [6, 7, 11, 12, 15, 16, 19]
    [1, 2, 4, 6, 7, 8, 10, 11, 12, 14, 15, 16, 17, 18, 19]
    [12, 16, 19]
    [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19]
    [12, 19]
    [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19]
    [12]
    [1, 2, 4, 6, 7, 10, 11, 12, 14, 15, 16, 17, 18, 19]
    [7, 11, 12, 16, 19]
    [1, 2, 4, 6, 7, 8, 9, 10, 11, 12, 14, 15, 16, 17, 18, 19]
    [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 14, 15, 16, 17, 18, 19]
    [6, 7, 11, 12, 16, 19]
    [1, 4, 6, 7, 10, 11, 12, 14, 15, 16, 17, 18, 19]
    [1, 2, 3, 4, 6, 7, 8, 9, 10, 11, 12, 14, 15, 16, 17, 18, 19]
    [7, 12, 16, 19]
    */
    let size = options.len();
    let mut results: Vec<usize> = Vec::new();
    results.resize(size, size);
    while results.contains(&size) {
        while let Some(i) = options.iter().position(|s| s.len() == 1) {
            let found = *options[i].iter().next().unwrap();
            results[i] = found;
            //Remove the found item from the rest
            for s in &mut options {
                s.remove(&found);
            }
        }
    }
    results
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn example1_part1() {
        let input = include_str!("../inputs/day16_example1.txt");
        assert_eq!(ticket_scannning_error_rate(input), 71);
    }

    #[test]
    fn part1() {
        let input = include_str!("../inputs/day16.txt");
        assert_eq!(ticket_scannning_error_rate(input), 21956);
    }

    #[test]
    fn test2() {
        let input = include_str!("../inputs/day16_example2.txt");
        assert_eq!(who_is_who(input), 0);
    }

    #[test]
    fn part2() {
        let input = include_str!("../inputs/day16.txt");
        assert_eq!(who_is_who(input), 3709435214239);
    }
}
