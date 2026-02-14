use everybody_codes_2025::shared::{PartSolution, Parts};

everybody_codes_2025::solution!();

impl Parts for Solution {
    fn part_1(&self, _input: &str) -> PartSolution {
        None.into()
    }

    fn part_2(&self, _input: &str) -> PartSolution {
        None.into()
    }
}

#[cfg(test)]
mod test {
    mod part_1 {
        use everybody_codes_2025::shared::solution::read_file;
        use everybody_codes_2025::shared::{PartSolution, Parts as _};
        use pretty_assertions::assert_eq;

        use crate::{DAY, Solution};

        #[test]
        fn outcome() {
            assert_eq!(
                PartSolution::None,
                (Solution {}).part_1(&read_file("inputs", &DAY))
            );
        }

        #[test]
        fn example() {
            assert_eq!(
                PartSolution::None,
                (Solution {}).part_1(&read_file("examples", &DAY))
            );
        }
    }

    mod part_2 {
        use everybody_codes_2025::shared::solution::read_file;
        use everybody_codes_2025::shared::{PartSolution, Parts as _};
        use pretty_assertions::assert_eq;

        use crate::{DAY, Solution};

        #[test]
        fn outcome() {
            assert_eq!(
                PartSolution::None,
                (Solution {}).part_2(&read_file("inputs", &DAY))
            );
        }

        #[test]
        fn example() {
            assert_eq!(
                PartSolution::None,
                (Solution {}).part_2(&read_file("examples", &DAY))
            );
        }
    }
}
