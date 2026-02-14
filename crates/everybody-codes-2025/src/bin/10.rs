use std::collections::VecDeque;
use std::vec::Vec;

use everybody_codes_2025::shared::{PartSolution, Parts};
use hashbrown::HashSet;
use z3::Optimize;
use z3::ast::Int;

everybody_codes_2025::solution!(479_u64, 19574_u64);

type Indicators = Vec<bool>;
type Schematics = Vec<Vec<usize>>;
type Joltage = Vec<u64>;

struct Machine {
    indicators: Indicators,
    schematics: Schematics,
    joltage: Joltage,
}

fn parse_input(input: &str) -> Vec<Machine> {
    input
        .trim()
        .lines()
        .map(|line| {
            let split = line.split(' ').collect::<Vec<_>>();

            let indicators = {
                split[0]
                    .trim_start_matches('[')
                    .trim_end_matches(']')
                    .chars()
                    .map(|c| match c {
                        '#' => true,
                        '.' => false,
                        _ => panic!("Bad input"),
                    })
                    .collect::<Vec<_>>()
            };

            let schematics = {
                let mut schematics = vec![];

                for schematic in &split[1..split.len() - 1] {
                    schematics.push(
                        schematic
                            .trim_start_matches('(')
                            .trim_end_matches(')')
                            .split(',')
                            .map(|s| s.parse::<usize>().unwrap())
                            .collect::<Vec<_>>(),
                    );
                }

                schematics
            };

            let joltage = {
                split[split.len() - 1]
                    .trim_start_matches('{')
                    .trim_end_matches('}')
                    .split(',')
                    .map(|s| s.parse::<u64>().unwrap())
                    .collect::<Vec<_>>()
            };

            Machine {
                indicators,
                schematics,
                joltage,
            }
        })
        .collect::<Vec<Machine>>()
}

fn solve_single_machine_indicators(
    &Machine {
        ref indicators,
        ref schematics,
        joltage: _,
    }: &Machine,
) -> usize {
    let bits: usize = indicators.len();

    let wanted_state = {
        let mut wanted_state = vec![false; bits];

        for (bit, &on) in indicators.iter().enumerate() {
            wanted_state[bit] = on;
        }

        wanted_state
    };

    let wirings = {
        let mut wirings = Vec::with_capacity(schematics.len());

        for wiring in schematics {
            let mut w = vec![false; bits];

            for &bit in wiring {
                w[bit] = true;
            }

            wirings.push(w);
        }

        wirings
    };

    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();

    queue.push_back((vec![false; bits], 0));
    visited.insert(vec![false; bits]);

    while let Some((state, presses)) = queue.pop_front() {
        for w in &wirings {
            // let next_state = state ^ w;
            let next_state = state
                .iter()
                .zip(w.iter())
                .map(|(&l, &r)| l ^ r)
                .collect::<Vec<_>>();
            let next_presses = presses + 1;

            if next_state == wanted_state {
                return next_presses;
            }

            if !visited.contains(&next_state) {
                visited.insert(next_state.clone());
                queue.push_back((next_state, next_presses));
            }
        }
    }

    usize::MAX
}

fn find_least_button_presses_indicators(machines: &Vec<Machine>) -> Vec<usize> {
    let mut least_amount = Vec::with_capacity(machines.len());

    for machine in machines {
        least_amount.push(solve_single_machine_indicators(machine));
    }

    least_amount
}

fn solve_single_machine_joltage(machine: &Machine) -> u64 {
    let optimizer = Optimize::new();
    let zero = Int::from_u64(0);

    // Joltages
    let mut joltages = vec![Int::from_u64(0); machine.joltage.len()];

    // Create our buttons and add them as expressions to our joltages.
    let schematics = machine
        .schematics
        .iter()
        .enumerate()
        .map(|(schematics_index, schema)| {
            // A variable representhing the amount of times we're going to apply a single schematic
            let var = Int::fresh_const(&format!("schematics-{}", schematics_index));

            // which should be positive
            optimizer.assert(&var.ge(&zero));

            // Set up the consequence of the application of that variable
            for index in schema {
                joltages[*index] = &joltages[*index] + &var;
            }

            var
        })
        .collect::<Vec<_>>();

    // Add the joltage constraints to the system
    for (i, joltage) in joltages.iter().enumerate() {
        optimizer.assert(&joltage.eq(Int::from_u64(machine.joltage[i])));
    }

    // Tell the optimizer that we are optimizing on the minimum number of accumulated presses.
    let total_presses = schematics
        .iter()
        .fold(Int::from_u64(0), |acc, current| acc + current);
    optimizer.minimize(&total_presses);

    // Check if we can satisfy, and find the `total_presses`' lowest value
    match optimizer.check(&[]) {
        z3::SatResult::Sat => optimizer
            .get_model()
            .unwrap()
            .eval(&total_presses, true)
            .unwrap()
            .as_u64()
            .unwrap(),
        z3::SatResult::Unsat | z3::SatResult::Unknown => unreachable!(),
    }
}

fn find_least_button_presses_joltage(machines: &[Machine]) -> Vec<u64> {
    let mut least_amount = Vec::with_capacity(machines.len());

    for machine in machines {
        least_amount.push(solve_single_machine_joltage(machine));
    }

    least_amount
}

impl Parts for Solution {
    fn part_1(&self, input: &str) -> PartSolution {
        let machines = parse_input(input);

        let sum_of_least_amount_of_button_presses: usize =
            find_least_button_presses_indicators(&machines).iter().sum();

        sum_of_least_amount_of_button_presses.into()
    }

    fn part_2(&self, input: &str) -> PartSolution {
        let machines = parse_input(input);

        let sum_of_least_amount_of_button_presses: u64 =
            find_least_button_presses_joltage(&machines).iter().sum();

        sum_of_least_amount_of_button_presses.into()
    }
}

#[cfg(test)]
mod test {
    mod part_1 {
        use everybody_codes_2025::{test_example_part_1, test_part_1};
        use pretty_assertions::assert_eq;

        #[test]
        fn outcome() {
            test_part_1!(479_u64);
        }

        #[test]
        fn example() {
            test_example_part_1!(7_u64);
        }
    }

    mod part_2 {
        use everybody_codes_2025::{test_example_part_2, test_part_2};
        use pretty_assertions::assert_eq;

        #[test]
        fn outcome() {
            test_part_2!(19574_u64);
        }

        #[test]
        fn example() {
            test_example_part_2!(33_u64);
        }
    }
}
