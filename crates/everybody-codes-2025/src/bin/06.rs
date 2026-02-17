use shared::{PartSolution, Parts};

shared::solution!(147, 3636_usize);

impl Parts for Solution {
    fn part_1(&self, input: &str) -> PartSolution {
        let mut mentors = 0_usize;
        let mut combos = 0_usize;

        for c in input.trim().chars() {
            match c {
                'A' => {
                    mentors += 1;
                },
                'a' => {
                    combos += mentors;
                },
                _ => {
                    // ignore for now
                },
            }
        }

        combos.into()
    }

    fn part_2(&self, input: &str) -> PartSolution {
        let mut mentors = [0_usize; 3];
        let mut combos = [0_usize; 3];

        for b in input.trim().as_bytes() {
            match *b {
                m @ (b'A' | b'B' | b'C') => {
                    let index = usize::from(m - b'A');

                    mentors[index] += 1;
                },
                s @ (b'a' | b'b' | b'c') => {
                    let index = usize::from(s - b'a');

                    combos[index] += mentors[index];
                },
                _ => {
                    // ignore for now
                },
            }
        }

        combos.iter().sum::<usize>().into()
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
        test_solution!(1, 147);
    }

    #[test]
    fn example_1() {
        test_example!(1, 5);
    }

    #[test]
    fn outcome_2() {
        test_solution!(2, 3636_usize);
    }

    #[test]
    fn example_2() {
        test_example!(2, 11);
    }

    #[test]
    fn outcome_3() {
        test_solution!(3, PartSolution::None);
    }

    #[test]
    fn example_3_1() {
        test_example!(3, 1, PartSolution::None);
    }

    #[test]
    fn example_3_2() {
        test_example!(3, 2, PartSolution::None);
    }
}
