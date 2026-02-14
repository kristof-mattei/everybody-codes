use std::mem::swap;

use everybody_codes_2025::shared::{PartSolution, Parts};

everybody_codes_2025::solution!(17229_u64, 170_520_923_035_051_u64);

fn parse_input(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|line| line.as_bytes().iter().map(|b| b - b'0').collect::<Vec<_>>())
        .collect::<Vec<Vec<u8>>>()
}

fn find_top_batteries<const N: usize>(battery_packs: &[Vec<u8>]) -> u64 {
    let mut sum: u64 = 0;

    for battery_pack in battery_packs {
        let top_n = find_top_batteries_in_pack::<N>(battery_pack);

        for (index, b) in top_n.iter().rev().enumerate() {
            let multiplier = 10_u64.pow(u32::try_from(index).unwrap());
            let b = u64::from(*b);

            sum += b * multiplier;
        }
    }

    sum
}

fn find_top_batteries_in_pack<const N: usize>(battery_pack: &[u8]) -> Vec<u8> {
    let mut result = battery_pack[(battery_pack.len().saturating_sub(N))..].to_vec();

    for b in battery_pack.iter().rev().skip(N) {
        insert_b(*b, &mut result);
    }

    result
}

fn insert_b(mut new_b: u8, new_battery_pack: &mut [u8]) {
    for b in new_battery_pack.iter_mut() {
        if new_b >= *b {
            swap(b, &mut new_b);
        } else {
            break;
        }
    }
}

impl Parts for Solution {
    fn part_1(&self, input: &str) -> PartSolution {
        let parsed = parse_input(input);

        let sum = find_top_batteries::<2>(&parsed);

        PartSolution::U64(sum)
    }

    fn part_2(&self, input: &str) -> PartSolution {
        let parsed = parse_input(input);

        let sum = find_top_batteries::<12>(&parsed);

        PartSolution::U64(sum)
    }
}

#[cfg(test)]
mod test {

    mod part_1 {
        use everybody_codes_2025::{test_example_part_1, test_part_1};
        use pretty_assertions::assert_eq;

        #[test]
        fn outcome() {
            test_part_1!(17229_u64);
        }

        #[test]
        fn example() {
            test_example_part_1!(357_u64);
        }
    }

    mod part_2 {
        use everybody_codes_2025::{test_example_part_2, test_part_2};
        use pretty_assertions::assert_eq;

        #[test]
        fn outcome() {
            test_part_2!(170_520_923_035_051_u64);
        }

        #[test]
        fn example() {
            test_example_part_2!(3_121_910_778_619_u64);
        }
    }
}
