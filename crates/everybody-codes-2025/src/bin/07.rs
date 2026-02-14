use std::vec::Vec;

use everybody_codes_2025::shared::grids::GridIter as _;
use everybody_codes_2025::shared::grids::grid::Grid;
use everybody_codes_2025::shared::{PartSolution, Parts};
use hashbrown::{HashMap, HashSet};

everybody_codes_2025::solution!(1626_u64, 48_989_920_237_096_u64);

enum Part {
    Start,
    Taychon,
    Empty,
}

impl TryFrom<char> for Part {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        let part = match value {
            'S' => Part::Start,
            '^' => Part::Taychon,
            '.' => Part::Empty,
            _ => return Err("Invalid part"),
        };

        Ok(part)
    }
}

fn parse_input(input: &str) -> Grid<Part> {
    let lines = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| Part::try_from(c).expect("Bad input"))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    Grid::new(lines)
}

fn split_beam(grid: &Grid<Part>) -> usize {
    let s = grid
        .row_column_index_value_iter()
        .find(|p| matches!(p, Part::Start))
        .expect("Start is guaranteed");

    let mut beams = vec![s];
    let mut processed = HashSet::new();

    while let Some(beam) = beams.pop() {
        if beam.0 == grid.get_row_length() - 1 {
            // end of the line for this beam
            continue;
        }

        let one_down = (beam.0 + 1, beam.1);

        match grid[one_down.0][beam.1] {
            Part::Start => panic!("Bad input"),
            Part::Taychon => {
                if !processed.insert((one_down.0, one_down.1)) {
                    continue;
                }

                beams.push((one_down.0, one_down.1 - 1));
                beams.push((one_down.0, one_down.1 + 1));
            },
            Part::Empty => {
                beams.push((one_down.0, one_down.1));
            },
        }
    }

    processed.len()
}

fn split_beam_all_times(grid: &mut Grid<Part>) -> usize {
    fn split_beam_all_times_r(
        grid: &Grid<Part>,
        beam: (usize, usize),
        cache: &mut HashMap<(usize, usize), usize>,
    ) -> usize {
        if let Some(v) = cache.get(&beam) {
            return *v;
        }

        if beam.0 == grid.get_row_length() - 1 {
            // end of the line for this beam
            return 1;
        }

        let one_down = (beam.0 + 1, beam.1);

        let v = match grid[one_down.0][beam.1] {
            Part::Start => panic!("Bad input"),
            Part::Taychon => {
                split_beam_all_times_r(grid, (one_down.0, beam.1 - 1), cache)
                    + split_beam_all_times_r(grid, (one_down.0, beam.1 + 1), cache)
            },
            Part::Empty => split_beam_all_times_r(grid, (one_down.0, one_down.1), cache),
        };

        cache.insert(beam, v);

        v
    }

    let s = grid
        .row_column_index_value_iter()
        .find(|p| matches!(p, Part::Start))
        .expect("Start is guaranteed");

    split_beam_all_times_r(grid, s, &mut HashMap::new())
}

impl Parts for Solution {
    fn part_1(&self, input: &str) -> PartSolution {
        let grid = parse_input(input);

        let split = split_beam(&grid);

        PartSolution::USize(split)
    }

    fn part_2(&self, input: &str) -> PartSolution {
        let mut grid = parse_input(input);

        let split = split_beam_all_times(&mut grid);

        PartSolution::USize(split)
    }
}

#[cfg(test)]
mod test {
    mod part_1 {
        use everybody_codes_2025::{test_example_part_1, test_part_1};
        use pretty_assertions::assert_eq;

        #[test]
        fn outcome() {
            test_part_1!(1626_u64);
        }

        #[test]
        fn example() {
            test_example_part_1!(21_u64);
        }
    }

    mod part_2 {
        use everybody_codes_2025::{test_example_part_2, test_part_2};
        use pretty_assertions::assert_eq;

        #[test]
        fn outcome() {
            test_part_2!(48_989_920_237_096_u64);
        }

        #[test]
        fn example() {
            test_example_part_2!(40_u64);
        }
    }
}
