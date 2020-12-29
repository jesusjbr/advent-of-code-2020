use std::ops::Range;

trait Halving {
    fn upper_half(&mut self);
    fn lower_half(&mut self);
}

impl Halving for Range<usize> {
    fn upper_half(&mut self) {
        self.start = ((self.end + self.start) as f64 / 2_f64).ceil() as usize;
    }
    fn lower_half(&mut self) {
        self.end = ((self.end + self.start) as f64 / 2_f64).floor() as usize;
    }
}

struct Seat {
    row: Range<usize>,
    column: Range<usize>,
    id: usize,
}

impl Seat {
    fn new(location: &str) -> Self {
        let mut seat = Seat {
            row: (0..127),
            column: (0..7),
            id: 0,
        };
        for c in location.chars().take(7) {
            match c {
                'F' => seat.row.lower_half(),
                'B' => seat.row.upper_half(),
                _ => unreachable!(),
            }
        }
        for c in location.chars().skip(7) {
            match c {
                'L' => seat.column.lower_half(),
                'R' => seat.column.upper_half(),
                _ => unreachable!(),
            }
        }
        seat.id = seat.row.start * 8 + seat.column.start;
        seat
    }
}

fn highest_id_in_boarding_pass(input: &str) -> usize {
    input.lines().map(|l| Seat::new(l).id).max().unwrap()
}

fn find_your_seat(input: &str) -> usize {
    let ids: Vec<usize> = input.lines().map(|l| Seat::new(l).id).collect();
    (1..1023)
        .find(|id| !ids.contains(id) && ids.contains(&(id - 1)) && ids.contains(&(id + 1)))
        .unwrap()
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn example_part1() {
        let location1 = "BFFFBBFRRR";
        let location2 = "FFFBBBFRRR";
        let location3 = "BBFFBBFRLL";

        let seat1 = Seat::new(location1);
        let seat2 = Seat::new(location2);
        let seat3 = Seat::new(location3);

        assert_eq!(seat1.row.start, 70);
        assert_eq!(seat1.column.start, 7);
        assert_eq!(seat1.id, 567);

        assert_eq!(seat2.row.start, 14);
        assert_eq!(seat2.column.start, 7);
        assert_eq!(seat2.id, 119);

        assert_eq!(seat3.row.start, 102);
        assert_eq!(seat3.column.start, 4);
        assert_eq!(seat3.id, 820);
    }

    #[test]
    fn part1() {
        let input = include_str!("../inputs/day5.txt");
        assert_eq!(highest_id_in_boarding_pass(input), 987);
    }

    #[test]
    fn part2() {
        let input = include_str!("../inputs/day5.txt");
        assert_eq!(find_your_seat(input), 603);
    }
}
