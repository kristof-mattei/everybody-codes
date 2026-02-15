use everybody_codes_2025::shared::{PartSolution, Parts};

everybody_codes_2025::solution!("Dalkryth", "Mornfeth", "Tharnlorath");

#[derive(Debug)]
enum Instruction {
    Left(usize),
    Right(usize),
}

fn parse_instructions(line: &str) -> Vec<Instruction> {
    let raw_instructions = line.split(',');

    let mut instructions = Vec::new();

    for raw_instruction in raw_instructions {
        let raw_instruction = raw_instruction.chars().collect::<Vec<_>>();
        let direction = raw_instruction[0];
        let ticks = raw_instruction[1..]
            .iter()
            .collect::<String>()
            .parse()
            .unwrap();

        let instruction = match direction {
            'L' => Instruction::Left(ticks),
            'R' => Instruction::Right(ticks),
            _ => panic!("Invalid input"),
        };

        instructions.push(instruction);
    }

    instructions
}

fn parse_input(input: &str) -> (Vec<String>, Vec<Instruction>) {
    let mut lines = input.lines();

    let names = lines
        .next()
        .unwrap()
        .split(',')
        .map(str::to_owned)
        .collect::<Vec<_>>();

    let _: &str = lines.next().unwrap();

    let parsed: Vec<_> = parse_instructions(lines.next().unwrap());

    (names, parsed)
}

impl Parts for Solution {
    fn part_1(&self, input: &str) -> PartSolution {
        let (names, parsed) = parse_input(input);

        let mut result: usize = 0;

        for instruction in parsed {
            match instruction {
                Instruction::Left(ticks) => {
                    result = result.saturating_sub(ticks);
                },
                Instruction::Right(ticks) => {
                    result += ticks;

                    if result >= names.len() {
                        result = names.len() - 1;
                    }
                },
            }
        }

        PartSolution::from(&*names[result])
    }

    fn part_2(&self, input: &str) -> PartSolution {
        let (names, parsed) = parse_input(input);

        let mut result: usize = 0;

        for instruction in parsed {
            match instruction {
                Instruction::Left(ticks) => {
                    let steps = ticks % names.len();

                    if steps > result {
                        let steps = steps - result;
                        result = names.len() - steps;
                    } else {
                        result -= steps;
                    }
                },
                Instruction::Right(ticks) => {
                    result += ticks;

                    result %= names.len();
                },
            }
        }

        PartSolution::from(&*names[result])
    }

    fn part_3(&self, input: &str) -> PartSolution {
        let (mut names, parsed) = parse_input(input);

        for instruction in parsed {
            match instruction {
                Instruction::Left(mut ticks) => {
                    ticks %= names.len();

                    if ticks != 0 {
                        let index = names.len() - ticks;

                        names.swap(0, index);
                    }
                },
                Instruction::Right(mut ticks) => {
                    ticks %= names.len();

                    if ticks != 0 {
                        names.swap(0, ticks);
                    }
                },
            }
        }

        PartSolution::from(&*names[0])
    }
}

#[cfg(test)]
mod test {
    use everybody_codes_2025::{test_example, test_solution};

    #[test]
    fn outcome_1() {
        test_solution!(1, "Dalkryth");
    }

    #[test]
    fn example_1() {
        test_example!(1, "Fyrryn");
    }

    #[test]
    fn outcome_2() {
        test_solution!(2, "Mornfeth");
    }

    #[test]
    fn example_2() {
        test_example!(2, "Elarzris");
    }

    #[test]
    fn outcome_3() {
        test_solution!(3, "Tharnlorath");
    }

    #[test]
    fn example_3() {
        test_example!(3, "Drakzyph");
    }
}
