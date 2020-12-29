use std::ops::Mul;
struct Slope {
    right: usize,
    down: usize,
}

impl Mul<usize> for &Slope {
    type Output = Slope;
    fn mul(self, rhs: usize) -> Slope {
        Slope {
            right: self.right * rhs,
            down: self.down * rhs,
        }
    }
}

struct Map {
    grid: Vec<char>,
    height: usize,
    width: usize,
}

impl Map {
    fn new(raw_map: &str) -> Self {
        Map {
            grid: raw_map
                .chars()
                .filter(|&cell| cell == '.' || cell == '#')
                .collect(),
            height: raw_map.lines().count(),
            width: raw_map.lines().next().unwrap().len(),
        }
    }

    fn get_value(&self, slope: &Slope) -> char {
        let position = (slope.down * self.width) + (slope.right % self.width);
        self.grid[position]
    }

    fn count_trees_traversed(&self, slope: Slope) -> usize {
        let steps = (self.height / slope.down) - 1;
        (0..steps)
            .map(|step| &slope * (step + 1))
            .map(|slope| self.get_value(&slope))
            .filter(|cell| *cell == '#')
            .count()
    }

    fn count_trees_together(&self, slopes: Vec<Slope>) -> usize {
        slopes
            .into_iter()
            .map(|slope| self.count_trees_traversed(slope))
            .product()
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    pub fn part1() {
        let raw = include_str!("../inputs/day3.txt");
        let map = Map::new(&raw);
        let slope = Slope { right: 3, down: 1 };
        assert_eq!(map.count_trees_traversed(slope), 209);
    }

    #[test]
    pub fn example_part2() {
        let raw = include_str!("../inputs/day3_example.txt");
        let map = Map::new(&raw);
        let slopes: Vec<Slope> = vec![
            Slope { right: 1, down: 1 },
            Slope { right: 3, down: 1 },
            Slope { right: 5, down: 1 },
            Slope { right: 7, down: 1 },
            Slope { right: 1, down: 2 },
        ];
        assert_eq!(map.count_trees_together(slopes), 336);
    }

    #[test]
    pub fn part2() {
        let raw = include_str!("../inputs/day3.txt");
        let map = Map::new(&raw);
        let slopes: Vec<Slope> = vec![
            Slope { right: 1, down: 1 },
            Slope { right: 3, down: 1 },
            Slope { right: 5, down: 1 },
            Slope { right: 7, down: 1 },
            Slope { right: 1, down: 2 },
        ];
        assert_eq!(map.count_trees_together(slopes), 1574890240);
    }
}
