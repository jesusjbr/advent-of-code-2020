use itertools::Itertools;

fn count_unique_answers(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|group| group.replace("\n", "").chars().unique().count())
        .sum()
}

fn count_answers_common_everyone(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|(group)| {
            group
                .lines()
                .next()
                .unwrap()
                .chars()
                .filter(|c| group.lines().all(|line| line.contains(*c)))
                .count()
        })
        .sum()
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn example_part1() {
        let input = include_str!("../inputs/day6_example1.txt");
        assert_eq!(count_unique_answers(input), 11);
    }

    #[test]
    fn part1() {
        let input = include_str!("../inputs/day6.txt");
        assert_eq!(count_unique_answers(input), 6885);
    }

    #[test]
    pub fn example_part2() {
        let input = include_str!("../inputs/day6_example1.txt");
        assert_eq!(count_answers_common_everyone(input), 6);
    }

    #[test]
    pub fn part2() {
        let input = include_str!("../inputs/day6.txt");
        assert_eq!(count_answers_common_everyone(input), 3550);
    }
}
