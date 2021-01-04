use std::collections::HashMap;

struct MemoryGame {
    numbers: HashMap<usize, (usize, usize)>,
    current_position: usize,
    target: usize,
}

impl MemoryGame {
    fn new(input: Vec<usize>) -> Self {
        let numbers = input
            .iter()
            .enumerate()
            .map(|(i, n)| (*n, (0_usize, i + 1)))
            .collect();
        MemoryGame {
            numbers: numbers,
            current_position: input.len() + 1,
            target: input[input.len() - 1],
        }
    }
}

impl Iterator for MemoryGame {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let age;
        let position = self.numbers.get(&self.target);

        if position.is_none() {
            age = 0;
            let older = if let Some(old) = self.numbers.get(&age) {
                old.1
            } else {
                self.current_position
            };
            self.numbers
                .insert(age, (self.current_position, self.current_position));
        } else if position.unwrap().0 == 0 {
            age = 0;
            let older = if let Some(old) = self.numbers.get(&age) {
                old.1
            } else {
                self.current_position
            };
            self.numbers.insert(age, (older, self.current_position));
        } else {
            let p = position.unwrap();
            age = p.1 - p.0;
            let older = self.numbers.get(&age);
            let older = if let Some(old) = older { old.1 } else { 0 };
            self.numbers.insert(age, (older, self.current_position));
        }
        self.target = age;
        self.current_position += 1;
        Some(age)
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn example1_part1() {
        let input = vec![0, 3, 6];
        let length = input.len();
        let game = MemoryGame::new(input);
        let nth = 2020 - length - 1;
        assert_eq!(game.into_iter().nth(nth).unwrap(), 436);
    }

    #[test]
    fn part1() {
        let input = vec![14, 3, 1, 0, 9, 5];
        let length = input.len();
        let game = MemoryGame::new(input);
        let nth = 2020 - length - 1;
        assert_eq!(game.into_iter().nth(nth).unwrap(), 614);
    }

    #[test]
    fn example1_part2() {
        let input = vec![0, 3, 6];
        let length = input.len();
        let game = MemoryGame::new(input);
        let nth = 30000000 - length - 1;
        assert_eq!(game.into_iter().nth(nth).unwrap(), 175594);
    }

    #[test]
    fn example2_part2() {
        let input = vec![1, 3, 2];
        let length = input.len();
        let game = MemoryGame::new(input);
        let nth = 30000000 - length - 1;
        assert_eq!(game.into_iter().nth(nth).unwrap(), 2578);
    }

    #[test]
    fn example3_part2() {
        let input = vec![2, 1, 3];
        let length = input.len();
        let game = MemoryGame::new(input);
        let nth = 30000000 - length - 1;
        assert_eq!(game.into_iter().nth(nth).unwrap(), 3544142);
    }

    #[test]
    fn example4_part2() {
        let input = vec![1, 2, 3];
        let length = input.len();
        let game = MemoryGame::new(input);
        let nth = 30000000 - length - 1;
        assert_eq!(game.into_iter().nth(nth).unwrap(), 261214);
    }

    #[test]
    fn example5_part2() {
        let input = vec![2, 3, 1];
        let length = input.len();
        let game = MemoryGame::new(input);
        let nth = 30000000 - length - 1;
        assert_eq!(game.into_iter().nth(nth).unwrap(), 6895259);
    }

    #[test]
    fn example6_part2() {
        let input = vec![3, 2, 1];
        let length = input.len();
        let game = MemoryGame::new(input);
        let nth = 30000000 - length - 1;
        assert_eq!(game.into_iter().nth(nth).unwrap(), 18);
    }

    #[test]
    fn example7_part2() {
        let input = vec![3, 1, 2];
        let length = input.len();
        let game = MemoryGame::new(input);
        let nth = 30000000 - length - 1;
        assert_eq!(game.into_iter().nth(nth).unwrap(), 362);
    }

    #[test]
    fn part2() {
        let input = vec![14, 3, 1, 0, 9, 5];
        let length = input.len();
        let game = MemoryGame::new(input);
        let nth = 30000000 - length - 1;
        assert_eq!(game.into_iter().nth(nth).unwrap(), 1065);
    }
}
