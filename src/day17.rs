use itertools::Itertools;
use std::collections::HashSet;
use std::iter;
use std::ops::RangeInclusive;
use std::{num::ParseIntError, str::FromStr};

#[derive(PartialEq, Eq, Hash, Debug)]
struct Cell {
    x: i32,
    y: i32,
    z: i32,
    w: i32,
}

impl Cell {
    fn neighbours<'a>(&'a self) -> impl Iterator<Item = Cell> + 'a {
        (self.x - 1..=self.x + 1)
            .cartesian_product(self.y - 1..=self.y + 1)
            .cartesian_product(self.z - 1..=self.z + 1)
            .cartesian_product(self.w - 1..=self.w + 1)
            .map(|(((x, y), z), w)| Cell { x, y, z, w })
            .filter(move |cell| cell != self) //A cell is not neighbour of itself.
    }
}

struct World {
    active_cells: HashSet<Cell>,
    borders_x: RangeInclusive<i32>,
    borders_y: RangeInclusive<i32>,
    borders_z: RangeInclusive<i32>,
    borders_w: RangeInclusive<i32>,
}

impl Iterator for World {
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        let new_active_cells: HashSet<Cell> = (self.borders_x.clone())
            .cartesian_product(self.borders_y.clone())
            .cartesian_product(self.borders_z.clone())
            .cartesian_product(self.borders_w.clone())
            .map(|(((x, y), z), w)| Cell { x, y, z, w })
            .filter(|cell| {
                let total = cell
                    .neighbours()
                    .filter(|c| self.active_cells.contains(&c))
                    .count();
                (self.active_cells.contains(cell) && (2..=3).contains(&total))
                    || (!self.active_cells.contains(cell) && total == 3)
            })
            .collect();
        //Grow the borders
        self.borders_x = self.borders_x.start() - 1..=self.borders_x.end() + 1;
        self.borders_y = self.borders_y.start() - 1..=self.borders_y.end() + 1;
        self.borders_z = self.borders_z.start() - 1..=self.borders_z.end() + 1;
        self.borders_w = self.borders_w.start() - 1..=self.borders_w.end() + 1;
        self.active_cells = new_active_cells;
        Some(self.active_cells.len())
    }
}

impl FromStr for World {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let borders_y = -1..=s.lines().count() as i32;
        let borders_x = -1..=s.lines().last().unwrap().len() as i32;
        let borders_z = -1..=1;
        let borders_w = -1..=1;

        let active_cells: HashSet<Cell> = s
            .lines()
            .rev()
            .enumerate()
            .flat_map(|(i, row)| iter::repeat(i).zip(row.chars().enumerate()))
            .filter(|(_i, (_j, c))| *c == '#')
            .map(|(i, (j, _c))| Cell {
                x: j as i32,
                y: i as i32,
                z: 0,
                w: 0,
            })
            .collect();

        Ok(World {
            active_cells,
            borders_x,
            borders_y,
            borders_z,
            borders_w,
        })
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn example1_part2() {
        let input = include_str!("../inputs/day17_example1.txt");
        let world = World::from_str(input).unwrap();
        assert_eq!(world.into_iter().nth(6 - 1).unwrap(), 848);
    }

    #[test]
    fn part2() {
        let input = include_str!("../inputs/day17.txt");
        let world = World::from_str(input).unwrap();
        assert_eq!(world.into_iter().nth(6 - 1).unwrap(), 1696);
    }
}
