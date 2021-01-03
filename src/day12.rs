fn navigation_system(input: &str) -> u32 {
    let mut compass = ["N", "W", "S", "E"];
    let (x, y) = input
        .lines()
        .map(|line| {
            let (instr, value) = line.split_at(1);
            let value: i32 = value.parse().unwrap();
            (instr, value)
        })
        .fold((0, 0), |(x, y), (mut instr, value)| {
            if instr == "F" {
                instr = compass.last().unwrap();
            }
            match instr {
                "N" => (x, y + value),
                "S" => (x, y - value),
                "E" => (x + value, y),
                "W" => (x - value, y),
                "L" => {
                    compass.rotate_left(value as usize % 360 / 90);
                    (x, y)
                }
                "R" => {
                    compass.rotate_right(value as usize % 360 / 90);
                    (x, y)
                }
                _ => unreachable!(),
            }
        });
    //Manhattan distance
    (x.abs() + y.abs()) as u32
}

fn waypoint_system(input: &str) -> u32 {
    let compass = [(-1, 1), (-1, -1), (1, -1)];
    let (x, y, _waypoint_x, _waypoint_y) = input
        .lines()
        .map(|line| {
            let (instr, value) = line.split_at(1);
            let value: i32 = value.parse().unwrap();
            (instr, value)
        })
        .fold((0, 0, 10, 1), |(x, y, wp_x, wp_y), (instr, value)| {
            //wp means waypoint
            match instr {
                "N" => (x, y, wp_x, wp_y + value),
                "S" => (x, y, wp_x, wp_y - value),
                "E" => (x, y, wp_x + value, wp_y),
                "W" => (x, y, wp_x - value, wp_y),
                "L" => {
                    let direction = compass[(value as usize % 360 / 90) - 1];
                    if (value % 360 / 90) % 2 != 0 {
                        //Swap wp_x with wp_y
                        (x, y, wp_y * direction.0, wp_x * direction.1)
                    } else {
                        (x, y, wp_x * direction.0, wp_y * direction.1)
                    }
                }
                "R" => {
                    let direction = compass[compass.len() - (value as usize % 360 / 90)];
                    if (value % 360 / 90) % 2 != 0 {
                        //Swap wp_x with wp_y
                        (x, y, wp_y * direction.0, wp_x * direction.1)
                    } else {
                        (x, y, wp_x * direction.0, wp_y * direction.1)
                    }
                }
                "F" => (x + value * wp_x, y + value * wp_y, wp_x, wp_y),
                _ => unreachable!(),
            }
        });
    //Manhattan distance
    (x.abs() + y.abs()) as u32
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn example1_part1() {
        let input = include_str!("../inputs/day12_example1.txt");
        assert_eq!(navigation_system(input), 25);
    }

    #[test]
    fn part1() {
        let input = include_str!("../inputs/day12.txt");
        assert_eq!(navigation_system(input), 2847);
    }

    #[test]
    fn example1_part2() {
        let input = include_str!("../inputs/day12_example1.txt");
        assert_eq!(waypoint_system(input), 286);
    }

    #[test]
    fn part2() {
        let input = include_str!("../inputs/day12.txt");
        assert_eq!(waypoint_system(input), 29839);
    }
}
