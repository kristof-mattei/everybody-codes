use shared::{PartSolution, Parts};

shared::solution!(11842_usize, 2_938_775_510_205_usize, 109_152_510_616_usize);

impl Parts for Solution {
    fn part_1(&self, input: &str) -> PartSolution {
        let mut iter = input.trim().lines();

        let first: usize = iter.next().unwrap().parse().unwrap();
        let last: usize = iter.last().unwrap().parse().unwrap();

        let rotations = (2025 * first) / last;

        rotations.into()
    }

    fn part_2(&self, input: &str) -> PartSolution {
        let mut iter = input.trim().lines();

        let first: f64 = iter.next().unwrap().parse().unwrap();
        let last: f64 = iter.last().unwrap().parse().unwrap();

        let rotations = (10_000_000_000_000_f64 * last) / first;

        #[expect(
            clippy::cast_possible_truncation,
            clippy::cast_sign_loss,
            reason = "Full rotations requested"
        )]
        ((rotations + 1_f64) as usize).into()
    }

    fn part_3(&self, input: &str) -> PartSolution {
        let (driven_product, drivers_product) = input
            .trim()
            .lines()
            .fold(Option::<(f64, f64)>::None, |acc, line| {
                if let Some((input_product, output_product)) = acc {
                    if let Some((left_str, right_str)) = line.split_once('|') {
                        Some((
                            input_product * left_str.parse::<f64>().unwrap(),
                            output_product * right_str.parse::<f64>().unwrap(),
                        ))
                    } else {
                        // last idler only affects driven product
                        let idler: f64 = line.parse().unwrap();
                        Some((input_product * idler, output_product))
                    }
                } else {
                    // first idler only affects driver product
                    let first: f64 = line.parse().unwrap();
                    Some((1_f64, first))
                }
            })
            .unwrap();

        #[expect(
            clippy::cast_possible_truncation,
            clippy::cast_sign_loss,
            reason = "Full rotations requested"
        )]
        (((100_f64 * drivers_product) / driven_product) as usize).into()
    }
}

#[cfg(test)]
mod test {
    use shared::{test_example, test_solution};

    #[test]
    fn outcome_1() {
        test_solution!(1, 11842_usize);
    }

    #[test]
    fn example_1() {
        test_example!(1, 15888);
    }

    #[test]
    fn outcome_2() {
        test_solution!(2, 2_938_775_510_205_usize);
    }

    #[test]
    fn example_2() {
        test_example!(2, 1_274_509_803_922_usize);
    }

    #[test]
    fn outcome_3() {
        test_solution!(3, 109_152_510_616_usize);
    }

    #[test]
    fn example_3() {
        test_example!(3, 6818);
    }
}
