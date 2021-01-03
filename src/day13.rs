fn catch_bus(input: &str) -> (usize, usize) {
    //Returns a tuple with the bus ID and the number of minutes to wait
    let mut lines = input.lines();
    let earliest: usize = lines.next().unwrap().parse().unwrap();
    let bus_ids: Vec<usize> = lines
        .next()
        .unwrap()
        .split(',')
        .filter_map(|n| n.parse::<usize>().ok())
        .collect();
    bus_ids
        .iter()
        .map(|bus_id| (*bus_id, bus_id - (earliest % bus_id)))
        .min_by(|x, y| x.1.cmp(&y.1))
        .unwrap()
}

fn delay_buses(input: &str) -> Vec<(usize, usize)> {
    //Pairs of (delays, bus_ids)
    input
        .lines()
        .last()
        .unwrap()
        .split(',')
        .enumerate()
        .filter(|(_i, n)| *n != "x")
        .map(|(i, n)| (i, n.parse::<usize>().unwrap()))
        .collect()
}

fn subsequent_timestamp(input: &str) -> usize {
    //Pairs of (delays, bus_ids)
    let delays_buses: Vec<(usize, usize)> = delay_buses(input);
    //It has to be multiple of the first bus_id
    let first = delays_buses[0].1;
    (first..)
        .step_by(first)
        .find(|timestamp| {
            delays_buses
                .iter()
                .skip(1)
                .all(|(i, n)| (timestamp + i) % n == 0)
        })
        .unwrap()
}

fn improved_subsequent_timestamp(input: &str) -> usize {
    /*The above function works well for smalls series, since the number
    of combinations grows up exponentially. So brute-force it's unfeasible
    for the last test.
    Looking for alternatives to my inefficient heuristic solution, I found
    the chinese remainder theorem and the Search by sieving method
    https://en.wikipedia.org/wiki/Chinese_remainder_theorem#Search_by_sieving
    This is quite similar to what I have done, and I just adapted the above solution
    to follow these rules and make the steps bigger*/

    //Pairs of (delays, bus_ids)
    let delays_buses: Vec<(usize, usize)> = delay_buses(input);
    let (starting_point, _step) =
        delays_buses
            .iter()
            .fold((0, 1), |(starting_point, step), (delay, bus_id)| {
                let starting_point = (starting_point..)
                    .step_by(step)
                    .find(|timestamp| (timestamp + delay) % bus_id == 0)
                    .unwrap();
                let step = step * bus_id;
                (starting_point, step)
            });
    starting_point
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn example1_part1() {
        let input = include_str!("../inputs/day13_example1.txt");
        let tuple = catch_bus(input);
        assert_eq!(tuple.0 * tuple.1, 295);
    }

    #[test]
    fn part1() {
        let input = include_str!("../inputs/day13.txt");
        let tuple = catch_bus(input);
        assert_eq!(tuple.0 * tuple.1, 3606);
    }

    #[test]
    fn example1_part2() {
        let input = include_str!("../inputs/day13_example1.txt");
        assert_eq!(subsequent_timestamp(input), 1068781);
    }

    #[test]
    fn example2_part2() {
        let input = "17,x,13,19";
        assert_eq!(subsequent_timestamp(input), 3417);
    }

    #[test]
    fn example3_part2() {
        let input = "67,7,59,61";
        assert_eq!(subsequent_timestamp(input), 754018);
    }

    #[test]
    fn example4_part2() {
        let input = "67,x,7,59,61";
        assert_eq!(subsequent_timestamp(input), 779210);
    }

    #[test]
    fn example5_part2() {
        let input = "67,7,x,59,61";
        assert_eq!(subsequent_timestamp(input), 1261476);
    }

    #[test]
    fn example6_part2() {
        let input = "1789,37,47,1889";
        assert_eq!(subsequent_timestamp(input), 1202161486);
    }

    #[test]
    fn part2() {
        let input = include_str!("../inputs/day13.txt");
        assert_eq!(improved_subsequent_timestamp(input), 379786358533423);
    }
}
