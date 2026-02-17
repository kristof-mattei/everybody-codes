use shared::{PartSolution, Parts};

shared::solution!();

impl Parts for Solution {
    fn part_1(&self, _input: &str) -> PartSolution {
        PartSolution::None
    }

    fn part_2(&self, _input: &str) -> PartSolution {
        PartSolution::None
    }

    fn part_3(&self, _input: &str) -> PartSolution {
        PartSolution::None
    }
}

#[cfg(test)]
mod test {
    use shared::{test_example, test_solution};

    #[test]
    fn outcome_1() {
        test_solution!(1, PartSolution::None);
    }

    #[test]
    fn example_1() {
        test_example!(1, PartSolution::None);
    }

    #[test]
    fn outcome_2() {
        test_solution!(2, PartSolution::None);
    }

    #[test]
    fn example_2() {
        test_example!(2, PartSolution::None);
    }

    #[test]
    fn outcome_3() {
        test_solution!(3, PartSolution::None);
    }

    #[test]
    fn example_3() {
        test_example!(3, PartSolution::None);
    }
}
