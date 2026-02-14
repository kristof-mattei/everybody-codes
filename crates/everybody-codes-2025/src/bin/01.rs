use everybody_codes_2025::shared::{PartSolution, Parts};

everybody_codes_2025::solution!(1031, 5831);

#[derive(Debug)]
enum Instruction {
    Left(u32),
    Right(u32),
}

fn parse_line(line: &str) -> Instruction {
    let chars = line.chars().collect::<Vec<_>>();
    let direction = chars[0];
    let ticks = chars[1..].iter().collect::<String>().parse().unwrap();

    let instruction = match direction {
        'L' => Instruction::Left(ticks),
        'R' => Instruction::Right(ticks),
        _ => panic!("Invalid input"),
    };

    instruction
}

fn parse_input(input: &str) -> Vec<Instruction> {
    let parsed: Vec<_> = input.lines().map(parse_line).collect();

    parsed
}

impl Parts for Solution {
    fn part_1(&self, input: &str) -> PartSolution {
        let parsed = parse_input(input);

        let mut result = 0;
        let mut position: u32 = 50;

        for instruction in parsed {
            match instruction {
                Instruction::Left(ticks) => {
                    let remainder = ticks % 100;

                    if remainder > position {
                        position = 100 - (remainder - position);
                    } else {
                        position -= remainder;
                    }
                },
                Instruction::Right(ticks) => {
                    position += ticks;

                    position %= 100;
                },
            }

            if position == 0 {
                result += 1;
            }
        }

        PartSolution::U32(result)
    }

    fn part_2(&self, input: &str) -> PartSolution {
        let parsed = parse_input(input);

        let mut result = 0;
        let mut position: u32 = 50;

        for instruction in parsed {
            match instruction {
                Instruction::Left(ticks) => {
                    let times = ticks / 100;
                    let remainder = ticks - (times * 100);

                    result += times;

                    if remainder > position {
                        if position != 0 {
                            result += 1;
                        } else {
                            // don't count this is a pass if we're already at 0
                        }

                        position = 100 - (remainder - position);
                    } else {
                        position -= remainder;
                    }

                    if position == 0 {
                        // we ended up on zero, so add to password
                        result += 1;
                    }
                },
                Instruction::Right(ticks) => {
                    position += ticks;

                    let times = position / 100;

                    position %= 100;

                    result += times;
                },
            }
        }

        PartSolution::U32(result)
    }
}

#[cfg(test)]
mod test {

    mod part_1 {
        use everybody_codes_2025::{test_example_part_1, test_part_1};
        use pretty_assertions::assert_eq;

        #[test]
        fn outcome() {
            test_part_1!(1031);
        }

        #[test]
        fn example() {
            test_example_part_1!(3);
        }
    }

    mod part_2 {
        use everybody_codes_2025::{test_example_part_2, test_part_2};
        use pretty_assertions::assert_eq;

        #[test]
        fn outcome() {
            test_part_2!(5831);
        }

        #[test]
        fn example() {
            test_example_part_2!(6);
        }
    }
}
