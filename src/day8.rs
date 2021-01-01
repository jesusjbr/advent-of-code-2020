use itertools::Itertools;

fn boot_code(input: &str) -> Vec<(&str, i32, bool)> {
    //Returns a Vec of instructions(tuples of pairs operation:value)
    //The tuple include a bool to check if a given instruction has been visited.
    input
        .lines()
        .map(|instruction| {
            let mut it = instruction.split(' ');
            (
                it.next().unwrap(),
                it.next().unwrap().parse::<i32>().unwrap(),
                false,
            )
        })
        .collect()
}

fn acc_before_repeat(instructions: Vec<(&str, i32, bool)>) -> (i32, bool) {
    //Returns a tuple with the acc and the status of the program.
    //When the program dont finish, the value of the accumulator is the one
    //before repeating an instruction.
    let mut instructions = instructions;
    let mut acc: i32 = 0;
    //Instruction pointer
    let mut ip: i32 = 0;
    let mut repeated = false;
    let mut terminated = false;

    while !repeated && !terminated {
        //Unpack the instruction
        let (operation, argument, done) = instructions[ip as usize];
        if !done {
            //Update this instruction as done
            instructions[ip as usize].2 = true;
            //Execute the operation
            match operation {
                "acc" => {
                    acc += argument;
                    ip += 1
                }
                "jmp" => ip += argument,
                "nop" => ip += 1,
                _ => unreachable!(),
            }
        } else {
            repeated = true;
        }
        if ip as usize == instructions.len() {
            //We have reached the end
            terminated = true;
        }
    }

    (acc, terminated)
}

fn search_valid_change(instructions: Vec<(&str, i32, bool)>) -> i32 {
    //When you dont want to think brute-force is your friend
    (0..=instructions.len() - 1)
        .rev()
        .filter(|index_changed| !instructions[*index_changed].0.contains("acc"))
        .map(|index_changed| {
            let mut instr = instructions.clone();
            //Changing the just operation of that index
            if instructions[index_changed].0.contains("nop") {
                instr[index_changed].0 = "jmp";
            } else {
                instr[index_changed].0 = "nop";
            }
            acc_before_repeat(instr)
        })
        .find(|(_acc, terminated)| *terminated)
        .unwrap()
        .0
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn example_part1() {
        let input = include_str!("../inputs/day8_example1.txt");
        assert_eq!(acc_before_repeat(boot_code(input)).0, 5);
    }

    #[test]
    fn part1() {
        let input = include_str!("../inputs/day8.txt");
        assert_eq!(acc_before_repeat(boot_code(input)).0, 2080);
    }

    #[test]
    fn example_part2() {
        let input = include_str!("../inputs/day8_example1.txt");
        assert_eq!(search_valid_change(boot_code(input)), 8);
    }

    #[test]
    fn part2() {
        let input = include_str!("../inputs/day8.txt");
        assert_eq!(search_valid_change(boot_code(input)), 2477);
    }
}
