use std::collections::{BTreeMap, BTreeSet};

use shared::{PartSolution, Parts};

shared::solution!(2695, 301, 2275);

impl Parts for Solution {
    fn part_1(&self, input: &str) -> PartSolution {
        let set: BTreeSet<usize> = input
            .trim()
            .split(',')
            .map(|i| i.parse().unwrap())
            .collect();

        set.iter().sum::<usize>().into()
    }

    fn part_2(&self, input: &str) -> PartSolution {
        let mut set: BTreeSet<usize> = BTreeSet::new();

        for v in input.trim().split(',').map(|i| i.parse().unwrap()) {
            set.insert(v);

            if set.len() > 20 {
                set.pop_last();
            }
        }

        set.iter().sum::<usize>().into()
    }

    fn part_3(&self, input: &str) -> PartSolution {
        let mut map: BTreeMap<usize, usize> = BTreeMap::new();

        for v in input.trim().split(',').map(|i| i.parse().unwrap()) {
            map.entry(v).and_modify(|v| *v += 1).or_insert(1);
        }

        map.values().max().copied().unwrap_or_default().into()
    }
}

#[cfg(test)]
mod test {
    use shared::{test_example, test_solution};

    #[test]
    fn outcome_1() {
        test_solution!(1, 2695);
    }

    #[test]
    fn example_1() {
        test_example!(1, 29);
    }

    #[test]
    fn outcome_2() {
        test_solution!(2, 301);
    }

    #[test]
    fn example_2() {
        test_example!(2, 781);
    }

    #[test]
    fn outcome_3() {
        test_solution!(3, 2275);
    }

    #[test]
    fn example_3() {
        test_example!(3, 3);
    }
}
