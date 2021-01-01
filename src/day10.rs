use itertools::Itertools;

fn parse_input(input: &str) -> Vec<usize> {
    let mut v: Vec<usize> = input.lines().map(|line| line.parse().unwrap()).collect();
    v.sort_unstable();
    v
}

fn diffs(joltages: Vec<usize>) -> (usize, usize) {
    let (ones, threes) = joltages
        .windows(2)
        .map(|w| w[1] - w[0])
        .take_while(|joltage_gap| joltage_gap <= &3)
        .fold((0, 0), |(ones, threes), joltage_gap| match joltage_gap {
            1 => (ones + 1, threes),
            3 => (ones, threes + 1),
            _ => (ones, threes),
        });
    //We add one for the charging outlet and the built-in adapter
    (ones + 1, threes + 1)
}

struct Fibonacci {
    /* It took me two hours to see the pattern
    Numbers of combination -> Consecutive skipped adapters

    0
    1
    1
    2 -> 1
    4 -> 2
    7 -> 3
    13 -> 4
    24 -> 5

    2 = 1 + 1 + 0
    4 = 2 + 1 + 1
    7 = 4 + 2 + 1
    13 = 7 + 4 + 2
    24 = 13 + 7 + 4
    Its a fibonacci like sequence, but adding the last 3 elements.
    */
    last_values: Vec<usize>,
}

impl Fibonacci {
    fn new() -> Self {
        Fibonacci {
            last_values: vec![0, 1, 1],
        }
    }
}
impl Iterator for Fibonacci {
    type Item = usize;
    fn next(&mut self) -> Option<usize> {
        let next_item = self.last_values.iter().sum();
        self.last_values.rotate_left(1);
        self.last_values[2] = next_item;
        Some(next_item)
    }
}

fn arrangements(mut joltages: Vec<usize>) -> usize {
    joltages.push(0);
    joltages.rotate_right(1);
    joltages
        .windows(3)
        .group_by(|w| w[2] - w[0] <= 3)
        .into_iter()
        .filter(|(result, _group)| *result)
        .map(|(_result, group)| Fibonacci::new().nth(group.count() - 1).unwrap())
        .product()
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn example1_part1() {
        let input = include_str!("../inputs/day10_example1.txt");
        let joltages = parse_input(input);
        assert_eq!(diffs(joltages), (7, 5));
    }

    #[test]
    fn example2_part1() {
        let input = include_str!("../inputs/day10_example2.txt");
        let joltages = parse_input(input);
        assert_eq!(diffs(joltages), (22, 10));
    }

    #[test]
    fn part1() {
        let input = include_str!("../inputs/day10.txt");
        let joltages = parse_input(input);
        assert_eq!(diffs(joltages), (65, 29));
        //65 * 29 = 1885
    }

    #[test]
    fn example1_part2() {
        let input = include_str!("../inputs/day10_example1.txt");
        let joltages = parse_input(input);
        assert_eq!(arrangements(joltages), 8);
    }

    #[test]
    fn example2_part2() {
        let input = include_str!("../inputs/day10_example2.txt");
        let joltages = parse_input(input);
        assert_eq!(arrangements(joltages), 19208);
    }

    #[test]
    fn part2() {
        let input = include_str!("../inputs/day10.txt");
        let joltages = parse_input(input);
        assert_eq!(arrangements(joltages), 2024782584832);
    }
}
