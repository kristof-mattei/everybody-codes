use std::vec::Vec;

use everybody_codes_2025::shared::{PartSolution, Parts};

everybody_codes_2025::solution!(4_776_100_539_usize, 1_476_550_548_usize);

fn parse_input(input: &str) -> Vec<(isize, isize)> {
    input
        .trim()
        .lines()
        .map(|line| {
            let mut iter = line.split(',');

            let x = iter.next().unwrap().parse().unwrap();
            let y = iter.next().unwrap().parse().unwrap();

            (x, y)
        })
        .collect()
}

fn surface((x1, y1): (isize, isize), (x2, y2): (isize, isize)) -> usize {
    (x1.abs_diff(x2) + 1) * (y1.abs_diff(y2) + 1)
}

fn find_largest_tile(tiles: &[(isize, isize)]) -> usize {
    let mut max = 0;
    for i in 0..tiles.len() {
        for j in (i + 1)..tiles.len() {
            max = max.max(surface(tiles[i], tiles[j]));
        }
    }

    max
}

struct Edge {
    x1: isize,
    y1: isize,
    x2: isize,
    y2: isize,
}

fn manhattan((x1, y1): (isize, isize), (x2, y2): (isize, isize)) -> usize {
    x1.abs_diff(x2) + y1.abs_diff(y2)
}

fn sort(a: isize, b: isize) -> (isize, isize) {
    (a.min(b), a.max(b))
}

fn rectangle_area(x1: isize, y1: isize, x2: isize, y2: isize) -> usize {
    let w = x1.abs_diff(x2) + 1;
    let h = y1.abs_diff(y2) + 1;

    w * h
}

fn intersects(min_x: isize, min_y: isize, max_x: isize, max_y: isize, edges: &Vec<Edge>) -> bool {
    for edge in edges {
        let (i_min_x, i_max_x) = sort(edge.x1, edge.x2);
        let (i_min_y, i_max_y) = sort(edge.y1, edge.y2);

        if min_x < i_max_x && max_x > i_min_x && min_y < i_max_y && max_y > i_min_y {
            return true;
        }
    }

    false
}

fn find_largest_tile_within_red_green(tiles: &[(isize, isize)]) -> usize {
    let mut edges: Vec<Edge> = Vec::with_capacity(tiles.len());

    for window in tiles.windows(2) {
        let (x1, y1) = window[0];
        let (x2, y2) = window[1];

        edges.push(Edge { x1, y1, x2, y2 });
    }

    {
        // last & first connect
        let (x1, y1) = tiles[0];
        let (x2, y2) = tiles[tiles.len() - 1];

        edges.push(Edge { x1, y1, x2, y2 });
    }

    let mut max_area: usize = 0;

    for i in 0..tiles.len() {
        for j in (i + 1)..tiles.len() {
            let from = tiles[i];
            let to = tiles[j];

            let (min_x, max_x) = sort(from.0, to.0);
            let (min_y, max_y) = sort(from.1, to.1);

            let distance = manhattan(from, to);

            if distance * distance > max_area {
                if !intersects(min_x, min_y, max_x, max_y, &edges) {
                    let area = rectangle_area(from.0, from.1, to.0, to.1);

                    max_area = max_area.max(area);
                }
            }
        }
    }

    max_area
}

impl Parts for Solution {
    fn part_1(&self, input: &str) -> PartSolution {
        let red_tiles = parse_input(input);

        let largest_tile = find_largest_tile(&red_tiles);

        PartSolution::USize(largest_tile)
    }

    fn part_2(&self, input: &str) -> PartSolution {
        let red_tiles = parse_input(input);

        let largest_tile = find_largest_tile_within_red_green(&red_tiles);

        PartSolution::USize(largest_tile)
    }
}

#[cfg(test)]
mod test {
    mod part_1 {
        use everybody_codes_2025::{test_example_part_1, test_part_1};
        use pretty_assertions::assert_eq;

        #[test]
        fn outcome() {
            test_part_1!(4_776_100_539_usize);
        }

        #[test]
        fn example() {
            test_example_part_1!(50_usize);
        }
    }

    mod part_2 {
        use everybody_codes_2025::{test_example_part_2, test_part_2};
        use pretty_assertions::assert_eq;

        #[test]
        fn outcome() {
            test_part_2!(1_476_550_548_usize);
        }

        #[test]
        fn example() {
            test_example_part_2!(24_isize);
        }
    }
}
