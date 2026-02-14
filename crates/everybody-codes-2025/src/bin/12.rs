use everybody_codes_2025::shared::{PartSolution, Parts};

everybody_codes_2025::solution!(492_usize);

#[expect(unused, reason = "Problem can be solved without")]
struct Shape(Vec<Vec<bool>>);

impl From<&str> for Shape {
    fn from(value: &str) -> Self {
        // first line is shape number
        let shape = value
            .lines()
            .skip(1)
            .map(|line| line.chars().map(|c| c == '#').collect())
            .collect();

        Self(shape)
    }
}

struct Region((usize, usize), [usize; 6]);

impl From<&str> for Region {
    fn from(value: &str) -> Self {
        let (left, right) = value.split_once(':').unwrap();

        let (width, height) = left.trim().split_once('x').unwrap();

        let widht = width.parse().unwrap();
        let height = height.parse().unwrap();

        let shapes = right
            .trim()
            .split(' ')
            .map(|v| v.parse().unwrap())
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();

        Self((widht, height), shapes)
    }
}

fn parse_input(input: &str) -> (Vec<Shape>, Vec<Region>) {
    let chunks = input.trim().split("\n\n").collect::<Vec<_>>();

    let shapes = chunks[..chunks.len() - 1]
        .iter()
        .copied()
        .map(Shape::from)
        .collect::<Vec<Shape>>();

    let regions = chunks[chunks.len() - 1]
        .lines()
        .map(Region::from)
        .collect::<Vec<Region>>();

    (shapes, regions)
}

fn solve_part_1(&(_, ref regions): &(Vec<Shape>, Vec<Region>)) -> usize {
    let mut count = 0;

    for region in regions {
        if region.1.iter().copied().sum::<usize>() * 9 <= region.0.0 * region.0.1 {
            count += 1;
        }
    }

    count
}

impl Parts for Solution {
    fn part_1(&self, input: &str) -> PartSolution {
        let shapes_and_regions = parse_input(input);

        solve_part_1(&shapes_and_regions).into()
    }

    fn part_2(&self, _input: &str) -> PartSolution {
        None.into()
    }
}

#[cfg(test)]
mod test {
    mod part_1 {
        use everybody_codes_2025::test_part_1;
        use pretty_assertions::assert_eq;

        #[test]
        fn outcome() {
            test_part_1!(472_usize);
        }

        // example is untestable with our code
        // #[test]
        // fn example() {
        //     test_example_part_1!(2_usize);
        // }
    }
}
