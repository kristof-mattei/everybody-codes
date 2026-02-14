use std::cmp::Reverse;
use std::vec::Vec;

use everybody_codes_2025::shared::{PartSolution, Parts};
use hashbrown::{HashMap, HashSet};

everybody_codes_2025::solution!(122_430_isize, 8_135_565_324_isize);

fn parse_input(input: &str) -> Vec<(isize, isize, isize)> {
    input
        .trim()
        .lines()
        .map(|line| {
            let mut iter = line.split(',');

            let x = iter.next().unwrap().parse().unwrap();
            let y = iter.next().unwrap().parse().unwrap();
            let z = iter.next().unwrap().parse().unwrap();

            (x, y, z)
        })
        .collect()
}

fn distance((x1, y1, z1): (isize, isize, isize), (x2, y2, z2): (isize, isize, isize)) -> f64 {
    #![expect(clippy::cast_precision_loss, reason = "General order only")]
    (((x2 - x1).pow(2) + (y2 - y1).pow(2) + (z2 - z1).pow(2)) as f64).sqrt()
}

fn find_closest<const L: usize>(junction_boxes: &[(isize, isize, isize)]) -> usize {
    let pairs = build_pairs_and_sort(&junction_boxes);

    let mut circuits = Vec::<HashSet<(isize, isize, isize)>>::new();

    // .into_iter().take(10)
    for ((left, right), _) in pairs.into_iter().take(L) {
        // no circuit with left, no circuit with right
        let left_index = circuits
            .iter()
            .position(|&ref circuit| circuit.contains(&left));

        let right_index = circuits
            .iter()
            .position(|&ref circuit| circuit.contains(&right));

        match (left_index, right_index) {
            (None, None) => {
                circuits.push([left, right].into_iter().collect::<HashSet<_>>());
            },
            (None, Some(right_index)) => {
                circuits[right_index].insert(left);
            },
            (Some(left_index), None) => {
                circuits[left_index].insert(right);
            },
            (Some(left_index), Some(right_index)) => {
                if left_index == right_index {
                    // already connected
                } else {
                    // so we can do swap remove
                    let (left_index, right_index) =
                        (left_index.min(right_index), left_index.max(right_index));

                    let circuit_to_merge = circuits.swap_remove(right_index);

                    for junction_box in circuit_to_merge {
                        circuits[left_index].insert(junction_box);
                    }
                }
            },
        }
    }

    circuits.sort_unstable_by_key(|&ref list| Reverse(list.len()));

    let top_3_multiplied = circuits
        .iter()
        .take(3)
        .fold(1, |acc, &ref curr| acc * curr.len());

    top_3_multiplied
}

type Pairs = Vec<(((isize, isize, isize), (isize, isize, isize)), f64)>;

fn build_pairs_and_sort(junction_boxes: &&[(isize, isize, isize)]) -> Pairs {
    let mut pairs = HashMap::with_capacity(junction_boxes.len() * (junction_boxes.len() / 2));

    for i in 0..junction_boxes.len() {
        for j in (i + 1)..junction_boxes.len() {
            let p1 = junction_boxes[i];
            let p2 = junction_boxes[j];

            pairs.insert((p1, p2), distance(p1, p2));
        }
    }

    let mut pairs = pairs.into_iter().collect::<Vec<_>>();
    pairs.sort_unstable_by(|&(_, ref d1), &(_, ref d2)| f64::total_cmp(d1, d2));

    pairs
}

fn find_closest_until(junction_boxes: &[(isize, isize, isize)]) -> isize {
    let pairs = build_pairs_and_sort(&junction_boxes);

    let mut circuits = Vec::<HashSet<(isize, isize, isize)>>::new();

    let mut last_merge_pair = None;

    for ((left, right), _) in pairs {
        // no circuit with left, no circuit with right
        let left_index = circuits.iter().position(|circuit| circuit.contains(&left));

        let right_index = circuits.iter().position(|circuit| circuit.contains(&right));

        match (left_index, right_index) {
            (None, None) => {
                circuits.push([left, right].into_iter().collect::<HashSet<_>>());
                last_merge_pair = Some((left, right));
            },
            (None, Some(right_index)) => {
                circuits[right_index].insert(left);
                last_merge_pair = Some((left, right));
            },
            (Some(left_index), None) => {
                circuits[left_index].insert(right);
                last_merge_pair = Some((left, right));
            },
            (Some(left_index), Some(right_index)) => {
                if left_index == right_index {
                    // already connected
                } else {
                    // so we can do swap remove
                    let (left_index, right_index) =
                        (left_index.min(right_index), left_index.max(right_index));

                    let circuit_to_merge = circuits.swap_remove(right_index);

                    for junction_box in circuit_to_merge {
                        circuits[left_index].insert(junction_box);
                    }

                    last_merge_pair = Some((left, right));
                }
            },
        }

        // do we have a single circuit that contains ALL the junction boxes from the input?
        if let &[ref circuit] = &*circuits
            && circuit.len() == junction_boxes.len()
        {
            break;
        }
    }

    last_merge_pair
        .map(|((lx, _, _), (rx, _, _))| lx * rx)
        .unwrap_or_default()
}

impl Parts for Solution {
    fn part_1(&self, input: &str) -> PartSolution {
        let junction_boxes = parse_input(input);

        let result = find_closest::<1000>(&junction_boxes);

        PartSolution::USize(result)
    }

    fn part_2(&self, input: &str) -> PartSolution {
        let junction_boxes = parse_input(input);

        let result = find_closest_until(&junction_boxes);

        PartSolution::ISize(result)
    }
}

#[cfg(test)]
mod tests {

    mod part_1 {
        use everybody_codes_2025::shared::solution::read_file;
        use everybody_codes_2025::test_part_1;
        use pretty_assertions::assert_eq;

        use crate::{DAY, find_closest, parse_input};

        #[test]
        fn outcome() {
            test_part_1!(122_430_u64);
        }

        #[test]
        fn example() {
            let input: &str = &read_file("examples", &DAY);
            let junction_boxes = parse_input(input);

            let result = find_closest::<10>(&junction_boxes);

            assert_eq!(40_usize, result);
        }
    }

    mod part_2 {
        use everybody_codes_2025::{test_example_part_2, test_part_2};
        use pretty_assertions::assert_eq;

        #[test]
        fn outcome() {
            test_part_2!(8_135_565_324_isize);
        }

        #[test]
        fn example() {
            test_example_part_2!(25272_isize);
        }
    }
}
