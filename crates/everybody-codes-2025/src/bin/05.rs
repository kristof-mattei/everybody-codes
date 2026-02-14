use std::collections::BTreeMap;

use everybody_codes_2025::shared::{PartSolution, Parts};

everybody_codes_2025::solution!(756_u64, 355_555_479_253_787_usize);

fn parse_input(input: &str) -> (BTreeMap<u64, u64>, Vec<u64>) {
    let (raw_fresh_ranges, raw_ingredients) = input.split_once("\n\n").unwrap();

    let mut fresh_ranges = BTreeMap::new();

    for (from, to) in raw_fresh_ranges.lines().map(|range| {
        let (from, to) = range.split_once('-').unwrap();

        let from = from.parse().unwrap();
        let to = to.parse().unwrap();

        (from, to)
    }) {
        fresh_ranges
            .entry(from)
            .and_modify(|existing_to: &mut u64| {
                *existing_to = (*existing_to).max(to);
            })
            .or_insert(to);
    }

    let ingredients: Vec<u64> = raw_ingredients
        .lines()
        .map(|ingredient| ingredient.parse().unwrap())
        .collect();

    (fresh_ranges, ingredients)
}

impl Parts for Solution {
    fn part_1(&self, input: &str) -> PartSolution {
        let mut fresh = 0;
        let (fresh_ranges, ingredients) = parse_input(input);

        for ingredient in &ingredients {
            for (&from, &to) in &fresh_ranges {
                if (from..=to).contains(ingredient) {
                    fresh += 1;
                    break;
                }
            }
        }

        PartSolution::USize(fresh)
    }

    fn part_2(&self, input: &str) -> PartSolution {
        let (fresh_ranges, _) = parse_input(input);

        // flatten the ranges
        let fresh_ranges =
            fresh_ranges
                .into_iter()
                .fold(vec![], |mut total: Vec<(u64, u64)>, current| {
                    if let Some(last) = total.last_mut() {
                        if last.1 >= current.0 {
                            // overlap, update
                            last.1 = last.1.max(current.1);
                        } else {
                            total.push(current);
                        }
                    } else {
                        total.push(current);
                    }

                    total
                });

        let count = fresh_ranges
            .iter()
            .map(|&(from, to)| (from..=to).count())
            .sum();

        PartSolution::USize(count)
    }
}

#[cfg(test)]
mod test {
    mod part_1 {
        use everybody_codes_2025::{test_example_part_1, test_part_1};
        use pretty_assertions::assert_eq;

        #[test]
        fn outcome() {
            test_part_1!(756_u64);
        }

        #[test]
        fn example() {
            test_example_part_1!(3_u64);
        }
    }

    mod part_2 {
        use everybody_codes_2025::{test_example_part_2, test_part_2};
        use pretty_assertions::assert_eq;

        #[test]
        fn outcome() {
            test_part_2!(355_555_479_253_787_usize);
        }

        #[test]
        fn example() {
            test_example_part_2!(14_usize);
        }
    }
}
