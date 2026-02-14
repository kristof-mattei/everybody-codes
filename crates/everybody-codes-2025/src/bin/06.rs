use std::vec::Vec;

use everybody_codes_2025::shared::{PartSolution, Parts};

everybody_codes_2025::solution!(6_891_729_672_676_u64, 9_770_311_947_567_u64);

enum Instruction {
    Times,
    Plus,
}

impl std::fmt::Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match *self {
            Instruction::Times => '*',
            Instruction::Plus => '+',
        };

        write!(f, "{}", c)
    }
}

impl Instruction {
    fn apply(&self, op1: u64, op2: u64) -> u64 {
        match *self {
            Instruction::Times => op1 * op2,
            Instruction::Plus => op1 + op2,
        }
    }
}

fn parse_input(input: &str) -> (Vec<Vec<u64>>, Vec<Instruction>) {
    let lines = input.lines().collect::<Vec<&str>>();

    let mut numbers = Vec::with_capacity(lines.len() - 1);

    for line in lines.iter().take(lines.len() - 1) {
        numbers.push(
            line.split_ascii_whitespace()
                .map(|n| n.parse().expect("Number"))
                .collect::<Vec<u64>>(),
        );
    }

    let instructions = lines.last().expect("Guaranteed by instructions");

    let instructions = instructions
        .split_ascii_whitespace()
        .map(|s| match s {
            "+" => Instruction::Plus,
            "*" => Instruction::Times,
            _ => panic!("Invalid input"),
        })
        .collect::<Vec<_>>();

    (numbers, instructions)
}

fn parse_input_right_to_left_columns(input: &str) -> (Vec<Vec<u64>>, Vec<Instruction>) {
    let lines = input.lines().collect::<Vec<&str>>();

    let instructions = lines.last().expect("Guaranteed by instructions");

    let instructions = instructions
        .split_ascii_whitespace()
        .map(|s| match s {
            "+" => Instruction::Plus,
            "*" => Instruction::Times,
            _ => panic!("Invalid input"),
        })
        .collect::<Vec<_>>();

    let mut numbers = Vec::with_capacity(lines.len() - 1);

    let lines = lines
        .iter()
        .take(lines.len() - 1)
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut current_numbers = Vec::<u64>::new();

    let columns = lines.iter().map(Vec::len).max().unwrap_or_default();

    for i in 0..columns {
        // get all the digits
        let digits = lines
            .iter()
            .map(|line| (line.get(i)).map_or(' ', |c| *c))
            .collect::<String>();
        let digits = digits.trim();

        if digits.is_empty() {
            numbers.push(current_numbers);
            current_numbers = Vec::new();
        } else {
            let parsed = digits.parse().expect("Input guarantees");
            current_numbers.push(parsed);
        }
    }

    numbers.push(current_numbers);

    (numbers, instructions)
}

fn process_input(numbers: &[Vec<u64>], instructions: &[Instruction]) -> u64 {
    let mut total = 0;

    for (index, instruction) in instructions.iter().enumerate() {
        let intermediate_sum = numbers
            .iter()
            .map(|numbers| numbers[index])
            .reduce(|acc, curr| instruction.apply(acc, curr))
            .expect("Guaranteed by instructions");

        println!(
            "{} {:?} = {}",
            instruction,
            numbers
                .iter()
                .map(|numbers| numbers[index])
                .collect::<Vec<_>>(),
            intermediate_sum
        );

        total += intermediate_sum;
    }

    total
}

fn process_input_right_to_left_columns(numbers: &[Vec<u64>], instructions: &[Instruction]) -> u64 {
    let mut total = 0;

    for (index, instruction) in instructions.iter().enumerate() {
        let intermediate_sum = numbers[index]
            .iter()
            .copied()
            .reduce(|acc, curr| instruction.apply(acc, curr))
            .expect("Guaranteed by instructions");

        println!(
            "{} {:?} = {}",
            instruction, &numbers[index], intermediate_sum
        );

        total += intermediate_sum;
    }

    total
}

impl Parts for Solution {
    fn part_1(&self, input: &str) -> PartSolution {
        let (numbers, instructions) = parse_input(input);

        let total = process_input(&numbers, &instructions);

        PartSolution::U64(total)
    }

    fn part_2(&self, input: &str) -> PartSolution {
        let (numbers, instructions) = parse_input_right_to_left_columns(input);

        let total = process_input_right_to_left_columns(&numbers, &instructions);

        PartSolution::U64(total)
    }
}

#[cfg(test)]
mod test {
    mod part_1 {
        use everybody_codes_2025::{test_example_part_1, test_part_1};
        use pretty_assertions::assert_eq;

        #[test]
        fn outcome() {
            test_part_1!(6_891_729_672_676_u64);
        }

        #[test]
        fn example() {
            test_example_part_1!(4_277_556_u64);
        }
    }

    mod part_2 {
        use everybody_codes_2025::{test_example_part_2, test_part_2};
        use pretty_assertions::assert_eq;

        #[test]
        fn outcome() {
            test_part_2!(9_770_311_947_567_u64);
        }

        #[test]
        fn example() {
            test_example_part_2!(3_263_827_u64);
        }
    }
}
