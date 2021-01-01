use itertools::Itertools;

fn parse_input(input: &str) -> Vec<u64> {
    input
        .lines()
        .map(|line| line.parse::<u64>().unwrap())
        .collect()
}

fn check_bad_data(encrypted_data: Vec<u64>, preamble_size: usize) -> u64 {
    encrypted_data
        .windows(preamble_size + 1)
        .find(|window| !sum_of_two(window))
        .unwrap()[preamble_size]
}

fn sum_of_two(numbers: &[u64]) -> bool {
    let target = numbers[numbers.len() - 1];
    numbers[..numbers.len() - 1]
        .iter()
        .permutations(2)
        .find(|p| p[0] + p[1] == target)
        .is_some()
}

fn encryption_weakness(encrypted_data: Vec<u64>, preamble_size: usize) -> u64 {
    let invalid_number = check_bad_data(encrypted_data.clone(), preamble_size);
    let position = encrypted_data
        .iter()
        .position(|n| *n == invalid_number)
        .unwrap();
    for start in 0..position {
        let searched_chunk = (2..position)
            .filter_map(|i| {
                encrypted_data[start..position]
                    .chunks_exact(i)
                    .find(|c| c.iter().sum::<u64>() == invalid_number)
            })
            .next();
        if let Some(data) = searched_chunk {
            return *data.iter().max().unwrap() + *data.iter().min().unwrap();
        }
    }
    //Unreachable
    0
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn example_part1() {
        let input = include_str!("../inputs/day9_example1.txt");
        let encrypted_data = parse_input(input);
        assert_eq!(check_bad_data(encrypted_data, 5), 127);
    }

    #[test]
    fn part1() {
        let input = include_str!("../inputs/day9.txt");
        let encrypted_data = parse_input(input);
        assert_eq!(check_bad_data(encrypted_data, 25), 104054607);
    }

    #[test]
    fn example_part2() {
        let input = include_str!("../inputs/day9_example1.txt");
        let encrypted_data = parse_input(input);
        assert_eq!(encryption_weakness(encrypted_data, 5), 62);
    }

    #[test]
    pub fn part2() {
        let input = include_str!("../inputs/day9.txt");
        let encrypted_data = parse_input(input);
        assert_eq!(encryption_weakness(encrypted_data, 25), 13935797);
    }
}
