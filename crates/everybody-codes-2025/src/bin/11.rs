use everybody_codes_2025::shared::{PartSolution, Parts};
use hashbrown::{HashMap, HashSet};

everybody_codes_2025::solution!(472_u64, 526_811_953_334_940_u64);

fn parse_input(input: &str) -> HashMap<&str, HashSet<&str>> {
    input
        .trim()
        .lines()
        .map(|line| {
            let (name, raw_outputs) = line.split_once(':').unwrap();

            let outputs = raw_outputs.trim().split(' ').map(str::trim).collect::<_>();

            (name, outputs)
        })
        .collect::<_>()
}

fn find_all_paths_from_svr_to_out(rack: &HashMap<&str, HashSet<&str>>) -> u64 {
    const SERVER: &str = "svr";
    const FFT: &str = "fft";
    const DAC: &str = "dac";
    const OUT: &str = "out";

    let server_fft = find_paths_between(rack, SERVER, FFT);
    let fft_dac = find_paths_between(rack, FFT, DAC);
    let dac_out = find_paths_between(rack, DAC, OUT);

    let server_dac = find_paths_between(rack, SERVER, DAC);
    let dac_fft = find_paths_between(rack, DAC, FFT);
    let fft_out = find_paths_between(rack, FFT, OUT);

    (server_fft * fft_dac * dac_out) + (server_dac * dac_fft * fft_out)
}

fn find_paths_between<'r>(
    rack: &HashMap<&'r str, HashSet<&'r str>>,
    current: &'r str,
    target: &str,
) -> u64 {
    fn find_paths_between_<'r>(
        rack: &HashMap<&'r str, HashSet<&'r str>>,
        current: &'r str,
        target: &str,
        cache: &mut HashMap<&'r str, u64>,
    ) -> u64 {
        if let Some(cached) = cache.get(current) {
            return *cached;
        }

        if current == target {
            return 1;
        }

        let total = rack
            .get(current)
            .into_iter()
            .flatten()
            .map(|next| find_paths_between_(rack, next, target, cache))
            .sum();

        cache.insert(current, total);

        total
    }

    find_paths_between_(rack, current, target, &mut HashMap::new())
}

impl Parts for Solution {
    fn part_1(&self, input: &str) -> PartSolution {
        let rack = parse_input(input);

        find_paths_between(&rack, "you", "out").into()
    }

    fn part_2(&self, input: &str) -> PartSolution {
        let rack = parse_input(input);

        find_all_paths_from_svr_to_out(&rack).into()
    }
}

#[cfg(test)]
mod test {
    mod part_1 {
        use everybody_codes_2025::{test_example_part_1, test_part_1};
        use pretty_assertions::assert_eq;

        #[test]
        fn outcome() {
            test_part_1!(472_u64);
        }

        #[test]
        fn example() {
            test_example_part_1!(5_u64, 1);
        }
    }

    mod part_2 {
        use everybody_codes_2025::{test_example_part_2, test_part_2};
        use pretty_assertions::assert_eq;

        #[test]
        fn outcome() {
            test_part_2!(526_811_953_334_940_u64);
        }

        #[test]
        fn example() {
            test_example_part_2!(2_u64, 2);
        }
    }
}
