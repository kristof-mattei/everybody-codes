#![expect(clippy::string_slice, reason = "We're in ASCII only world")]

use everybody_codes_2025::shared::{PartSolution, Parts};

everybody_codes_2025::solution!(20_223_751_480_u64, 30_260_171_216_u64);

fn parse_ids(ids: &str) -> (u64, u64) {
    let (id1, id2) = ids.split_once('-').unwrap();

    (id1.parse().unwrap(), id2.parse().unwrap())
}

fn parse_input(input: &str) -> Vec<(u64, u64)> {
    input
        .trim()
        .split(',')
        .map(parse_ids)
        .collect::<Vec<(u64, u64)>>()
}

impl Parts for Solution {
    fn part_1(&self, input: &str) -> PartSolution {
        let parsed = parse_input(input);

        let mut count = 0;

        for (start, stop) in parsed {
            for value in start..=stop {
                let value_s = value.to_string();

                if value_s.len() % 2 == 0 {
                    if value_s[0..value_s.len() / 2] == value_s[value_s.len() / 2..] {
                        count += value;
                    }
                }
            }
        }

        PartSolution::U64(count)
    }

    fn part_2(&self, input: &str) -> PartSolution {
        let parsed = parse_input(input);

        let mut count = 0;

        for (start, stop) in parsed {
            for value in start..=stop {
                let value_s: Vec<_> = value.to_string().chars().collect();

                for i in 1..=(value_s.len() / 2) {
                    if value_s.len() % i == 0 {
                        let mut valid = true;

                        let chunks: Vec<_> = value_s.chunks(i).collect();

                        for w in chunks.windows(2) {
                            let w0 = w[0];
                            let w1 = w[1];

                            if w0 != w1 {
                                valid = false;
                                break;
                            }
                        }

                        if valid {
                            count += value;
                            // ensure we don't count values like `222222` multiple times (1, 2 & 3)
                            break;
                        }
                    }
                }
            }
        }

        PartSolution::U64(count)
    }
}

#[cfg(test)]
mod test {

    mod part_1 {
        use everybody_codes_2025::{test_example_part_1, test_part_1};
        use pretty_assertions::assert_eq;

        #[test]
        fn outcome() {
            test_part_1!(20_223_751_480_u64);
        }

        #[test]
        fn example() {
            test_example_part_1!(1_227_775_554);
        }
    }

    mod part_2 {
        use everybody_codes_2025::{test_example_part_2, test_part_2};
        use pretty_assertions::assert_eq;

        #[test]
        fn outcome() {
            test_part_2!(30_260_171_216_u64);
        }

        #[test]
        fn example() {
            test_example_part_2!(4_174_379_265_u64);
        }
    }
}
