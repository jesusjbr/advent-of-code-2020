use std::{collections::HashMap, num::ParseIntError, str::FromStr};

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
enum Cell {
    EmptySeat,
    OccupiedSeat,
    Floor,
}

impl Cell {
    fn transition_adjacents(&self, adjacent_occupied_seats: usize) -> Self {
        match self {
            Cell::Floor => Cell::Floor,
            Cell::EmptySeat => {
                if adjacent_occupied_seats == 0 {
                    Cell::OccupiedSeat
                } else {
                    Cell::EmptySeat
                }
            }
            Cell::OccupiedSeat => {
                if adjacent_occupied_seats >= 4 {
                    Cell::EmptySeat
                } else {
                    Cell::OccupiedSeat
                }
            }
        }
    }

    fn transition_visibles(&self, visibles_occupied_seats: usize) -> Self {
        match self {
            Cell::Floor => Cell::Floor,
            Cell::EmptySeat => {
                if visibles_occupied_seats == 0 {
                    Cell::OccupiedSeat
                } else {
                    Cell::EmptySeat
                }
            }
            Cell::OccupiedSeat => {
                if visibles_occupied_seats >= 5 {
                    Cell::EmptySeat
                } else {
                    Cell::OccupiedSeat
                }
            }
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn new(x: i32, y: i32) -> Self {
        Position { x, y }
    }

    fn adjacent_cells(&self) -> Vec<Position> {
        vec![
            Position::new(self.x - 1, self.y - 1),
            Position::new(self.x, self.y - 1),
            Position::new(self.x + 1, self.y - 1),
            Position::new(self.x - 1, self.y),
            Position::new(self.x + 1, self.y),
            Position::new(self.x - 1, self.y + 1),
            Position::new(self.x, self.y + 1),
            Position::new(self.x + 1, self.y + 1),
        ]
    }
}

struct Grid {
    cells: HashMap<Position, Cell>,
}

impl FromStr for Grid {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cells: HashMap<Position, Cell> = s
            .lines()
            .enumerate()
            .flat_map(|(i, line)| {
                line.chars().enumerate().map(move |(j, c)| {
                    (
                        Position::new(j as i32, i as i32),
                        match c {
                            'L' => Cell::EmptySeat,
                            '.' => Cell::Floor,
                            '#' => Cell::OccupiedSeat,
                            _ => unreachable!(),
                        },
                    )
                })
            })
            .collect();

        Ok(Grid { cells })
    }
}

impl Grid {
    fn adjacent_occupied_seats(&self, source_seat: &Position) -> usize {
        /*Given the position of a seat, returns the number of adjacent
        occupied seats */
        source_seat
            .adjacent_cells()
            .iter()
            .filter_map(|p| self.cells.get(p))
            .filter(|c| match c {
                Cell::OccupiedSeat => true,
                _ => false,
            })
            .count()
    }

    fn visible_occupied_seats(&self, source_seat: &Position) -> usize {
        /*Given the position of a seat, returns the number of visible
        occupied seats */
        let directions = vec![
            (-1, -1),
            (0, -1),
            (1, -1),
            (-1, 0),
            (1, 0),
            (-1, 1),
            (0, 1),
            (1, 1),
        ];
        //Ugly and unreadable, abuse of iterators
        directions
            .iter()
            .map(|direction| {
                (1..)
                    .map(move |distance| {
                        self.cells.get(&Position {
                            x: source_seat.x + distance * direction.0,
                            y: source_seat.y + distance * direction.1,
                        })
                    })
                    .take_while(|cell| cell.is_some())
                    .find(|cell| {
                        cell.is_some()
                            && match cell.unwrap() {
                                Cell::Floor => false,
                                _ => true,
                            }
                    })
            })
            .filter(|cell| {
                if let Some(Some(c)) = cell {
                    match c {
                        Cell::OccupiedSeat => true,
                        _ => false,
                    }
                } else {
                    false
                }
            })
            .count()
    }

    fn count_occupied_seats(&self) -> usize {
        /*Return the total number of occupied seats in the grid */
        self.cells
            .iter()
            .filter(|(_position, cell)| match cell {
                Cell::OccupiedSeat => true,
                _ => false,
            })
            .count()
    }

    fn apply_rules(&mut self) {
        /*
            While isn't stabilized keep applying the rules:
            If a seat is empty (L) and there are no occupied seats adjacent to it, the seat becomes occupied.
            If a seat is occupied (#) and four or more seats adjacent to it are also occupied, the seat becomes empty.
            Otherwise, the seat's state does not change.

        */
        let mut stabilized = false;
        let mut next_grid;
        while !stabilized {
            next_grid = self.cells.clone();
            self.cells = next_grid
                .iter()
                .map(|(position, cell)| {
                    let total = self.adjacent_occupied_seats(position);
                    (position.clone(), cell.transition_adjacents(total))
                })
                .collect();
            stabilized = next_grid == self.cells;
        }
    }

    fn apply_vision_rules(&mut self) {
        /*
            While isn't stabilized keep applying the rules:
            To determine the number of occupied seats People don't just care about adjacent seats -
            they care about the first seat they can see in each of those eight directions!
            If a seat is empty (L) and there are no occupied seats, the seat becomes occupied.
            If a seat is occupied (#) and four or more seats are also occupied, the seat becomes empty.
            Otherwise, the seat's state does not change.

        */
        let mut stabilized = false;
        let mut next_grid;
        while !stabilized {
            next_grid = self.cells.clone();
            self.cells = next_grid
                .iter()
                .map(|(position, cell)| {
                    let total = self.visible_occupied_seats(position);
                    (position.clone(), cell.transition_visibles(total))
                })
                .collect();
            stabilized = next_grid == self.cells;
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn example1_part1() {
        let input = include_str!("../inputs/day11_example1.txt");
        let mut grid = Grid::from_str(input).unwrap();
        grid.apply_rules();
        assert_eq!(grid.count_occupied_seats(), 37);
    }

    #[test]
    pub fn part1() {
        let input = include_str!("../inputs/day11.txt");
        let mut grid = Grid::from_str(input).unwrap();
        grid.apply_rules();
        assert_eq!(grid.count_occupied_seats(), 2368);
    }

    #[test]
    fn example1_part2() {
        let input = include_str!("../inputs/day11_example1.txt");
        let mut grid = Grid::from_str(input).unwrap();
        grid.apply_vision_rules();
        assert_eq!(grid.count_occupied_seats(), 26);
    }

    #[test]
    fn part2() {
        let input = include_str!("../inputs/day11.txt");
        let mut grid = Grid::from_str(input).unwrap();
        grid.apply_vision_rules();
        assert_eq!(grid.count_occupied_seats(), 2124);
    }
}
