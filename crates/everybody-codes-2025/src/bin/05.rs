use std::cmp::Ordering;
use std::fmt::Display;

use shared::{PartSolution, Parts};

shared::solution!(11842_usize, 8_997_499_433_555_usize, 31_656_969);

fn to_number(numbers: impl Iterator<Item = usize>) -> usize {
    numbers.fold(0_usize, |acc, val| {
        // ilog10() panics on 0
        // ilog10() returns 0 for '1-9', 1 for '10-99', so we add 1.
        let digits = val.checked_ilog10().map(|l| l + 1).unwrap_or_default();

        acc * 10_usize.pow(digits) + val
    })
}

#[derive(Eq, PartialEq)]
struct Level(Option<usize>, usize, Option<usize>);

impl Level {
    fn quality(&self) -> usize {
        let level: [Option<usize>; 3] = (self.0, Some(self.1), self.2).into();

        to_number(level.into_iter().flatten())
    }
}

impl Display for Level {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(left) = self.0 {
            write!(f, "{}-", left)?;
        } else {
            write!(f, "  ")?;
        }

        write!(f, "{}", self.1)?;

        if let Some(right) = self.2 {
            write!(f, "-{}", right)?;
        }

        Ok(())
    }
}

impl Ord for Level {
    fn cmp(&self, other: &Self) -> Ordering {
        self.quality().cmp(&other.quality())
    }
}

impl PartialOrd for Level {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Eq, PartialEq)]
struct Sword {
    identifier: usize,
    spline: Vec<Level>,
}

impl Sword {
    fn new(identifier: usize) -> Self {
        Self {
            identifier,
            spline: Vec::new(),
        }
    }

    fn quality(&self) -> usize {
        to_number(self.spline.iter().map(|sp| sp.1))
    }
}

impl Display for Sword {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "id: {}", self.identifier)?;
        writeln!(f)?;

        let mut iter = self.spline.iter();

        if let Some(level) = iter.next() {
            writeln!(f, "{}", level)?;
        }

        for level in iter {
            writeln!(f, "  |")?;
            writeln!(f, "{}", level)?;
        }

        Ok(())
    }
}

impl Ord for Sword {
    fn cmp(&self, other: &Self) -> Ordering {
        let quality = self.quality().cmp(&other.quality());

        match quality {
            Ordering::Equal => {
                // same amount of splines, no `0` in the inputs, so no risk of
                // a spline with `1` followed by a `0` becoming `10`
                // having to be compared to a spline with 1 level where the middle is `10`
                for (left, right) in self.spline.iter().zip(other.spline.iter()) {
                    match left.cmp(right) {
                        Ordering::Equal => continue,
                        other @ (Ordering::Less | Ordering::Greater) => {
                            return other;
                        },
                    }
                }

                // fall back to identifier if all else is equal
                self.identifier.cmp(&other.identifier)
            },
            other @ (Ordering::Less | Ordering::Greater) => other,
        }
    }
}

impl PartialOrd for Sword {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn build_sword(identifier: usize, segments: &str) -> Sword {
    let segments = segments
        .split(',')
        .map(|segment| segment.parse::<usize>().unwrap());

    let mut sword = Sword::new(identifier);

    for segment in segments {
        let mut handled = false;
        for &mut Level(ref mut left, middle, ref mut right) in &mut sword.spline {
            match (left.as_mut(), right.as_mut()) {
                (None, None) => {
                    if segment < middle {
                        *left = Some(segment);
                        handled = true;
                        break;
                    }

                    if segment > middle {
                        *right = Some(segment);
                        handled = true;
                        break;
                    }
                },
                (None, Some(_)) => {
                    if segment < middle {
                        *left = Some(segment);
                        handled = true;
                        break;
                    }
                },
                (Some(_), None) => {
                    if segment > middle {
                        *right = Some(segment);
                        handled = true;
                        break;
                    }
                },
                (Some(_), Some(_)) => continue,
            }
        }

        if !handled {
            sword.spline.push(Level(None, segment, None));
        }
    }

    sword
}

impl Parts for Solution {
    fn part_1(&self, input: &str) -> PartSolution {
        let (identifier, segments) = input.trim().split_once(':').unwrap();

        build_sword(identifier.parse().unwrap(), segments)
            .quality()
            .into()
    }

    fn part_2(&self, input: &str) -> PartSolution {
        let mut min = usize::MAX;
        let mut max = usize::MIN;

        for line in input.trim().lines() {
            let (identifier, segments) = line.trim().split_once(':').unwrap();

            let sword = build_sword(identifier.parse().unwrap(), segments);

            min = min.min(sword.quality());
            max = max.max(sword.quality());
        }

        (max - min).into()
    }

    fn part_3(&self, input: &str) -> PartSolution {
        let mut swords = Vec::new();

        for line in input.trim().lines() {
            let (identifier, segments) = line.trim().split_once(':').unwrap();

            swords.push(build_sword(identifier.parse().unwrap(), segments));
        }

        swords.sort();
        swords.reverse();

        // for sword in swords {
        //     println!("{}", sword);
        // }

        swords
            .iter()
            .enumerate()
            .fold(0, |acc, (position, sword)| {
                acc + ((position + 1) * sword.identifier)
            })
            .into()
    }
}

#[cfg(test)]
mod test {
    use shared::{test_example, test_solution};

    #[test]
    fn outcome_1() {
        test_solution!(1, 8_575_623_452_usize);
    }

    #[test]
    fn example_1() {
        test_example!(1, 581_078);
    }

    #[test]
    fn outcome_2() {
        test_solution!(2, 8_997_499_433_555_usize);
    }

    #[test]
    fn example_2() {
        test_example!(2, 77053_usize);
    }

    #[test]
    fn outcome_3() {
        test_solution!(3, 31_656_969);
    }

    #[test]
    fn example_3_1() {
        test_example!(3, 1, 260);
    }

    #[test]
    fn example_3_2() {
        test_example!(3, 2, 4);
    }
}
