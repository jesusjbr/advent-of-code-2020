use itertools::Itertools;

fn get_input() -> Vec<Result<i32, std::num::ParseIntError>> {
    let raw = include_str!("../inputs/day1_p1.txt");
    let expenses = raw.lines().map(|ex| ex.parse::<i32>()).collect();
    expenses
}

fn combinations(number_permutations: i32) -> Vec<Vec<i32>> {
    let permutations = get_input()
        .iter()
        .map(|ex| ex.clone().unwrap())
        .permutations(number_permutations as usize)
        .collect();
    permutations
}

fn entries_sum_2020(number_of_entries: i32) -> Vec<i32> {
    for permutation in combinations(number_of_entries) {
        if permutation.iter().sum::<i32>() == 2020 {
            return permutation;
        }
    }
    return vec![0];
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn part1() {
        let product: i32 = entries_sum_2020(2).iter().product();
        assert_eq!(product, 1005459)
    }

    #[test]
    fn part2() {
        let product: i32 = entries_sum_2020(3).iter().product();
        assert_eq!(product, 92643264)
    }
}
