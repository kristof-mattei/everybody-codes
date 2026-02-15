use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign};

use everybody_codes_2025::shared::{PartSolution, Parts};

everybody_codes_2025::solution!("[155819,797337]", 622, 60242);

fn parse_input(input: &str) -> Complex {
    let line = input.lines().next().unwrap();

    let (_, instructions) = line.split_once('=').unwrap();

    let (left, right) = instructions
        .strip_prefix('[')
        .unwrap()
        .strip_suffix(']')
        .unwrap()
        .split_once(',')
        .unwrap();

    Complex(left.parse().unwrap(), right.parse().unwrap())
}

#[derive(Copy, Clone)]
struct Complex(i64, i64);

impl Add for Complex {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Complex(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl AddAssign for Complex {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
    }
}

impl Mul for Complex {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let Complex(x1, y1) = self;
        let Complex(x2, y2) = rhs;

        Complex(x1 * x2 - y1 * y2, x1 * y2 + y1 * x2)
    }
}

impl MulAssign for Complex {
    fn mul_assign(&mut self, rhs: Self) {
        let y2 = rhs.1;
        let y1 = self.1;
        let x2 = rhs.0;
        let x1 = self.0;

        self.0 = x1 * x2 - y1 * y2;
        self.1 = x1 * y2 + y1 * x2;
    }
}

impl Div for Complex {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Complex(self.0 / rhs.0, self.1 / rhs.1)
    }
}

impl DivAssign for Complex {
    fn div_assign(&mut self, rhs: Self) {
        self.0 /= rhs.0;
        self.1 /= rhs.1;
    }
}

fn paint(input: &str, step: usize) -> PartSolution {
    let complex = parse_input(input);

    let mut painted = 0;

    for x in (complex.0..=(complex.0 + 1_000)).step_by(step) {
        for y in (complex.1..=(complex.1 + 1_000)).step_by(step) {
            let mut skip = false;
            let mut result = Complex(0, 0);

            for _ in 0..100 {
                result *= result;
                result /= Complex(100_000, 100_000);
                result += Complex(x, y);

                if result.0 > 1_000_000
                    || result.1 > 1_000_000
                    || result.0 < -1_000_000
                    || result.1 < -1_000_000
                {
                    skip = true;
                    break;
                }
            }

            if !skip {
                painted += 1;
            }
        }
    }

    painted.into()
}

impl Parts for Solution {
    fn part_1(&self, input: &str) -> PartSolution {
        let complex = parse_input(input);

        let mut result = Complex(0, 0);

        for _ in 0..3 {
            result *= result;
            result /= Complex(10, 10);
            result += complex;
        }

        PartSolution::String(format!("[{},{}]", result.0, result.1))
    }

    fn part_2(&self, input: &str) -> PartSolution {
        paint(input, 10)
    }

    fn part_3(&self, input: &str) -> PartSolution {
        paint(input, 1)
    }
}

#[cfg(test)]
mod test {
    use everybody_codes_2025::{test_example, test_solution};

    #[test]
    fn outcome_1() {
        test_solution!(1, "[155819,797337]");
    }

    #[test]
    fn example_1() {
        test_example!(1, "[357,862]");
    }

    #[test]
    fn outcome_2() {
        test_solution!(2, 622);
    }

    #[test]
    fn example_2() {
        test_example!(2, 4076);
    }

    #[test]
    fn outcome_3() {
        test_solution!(3, 60242);
    }

    #[test]
    fn example_3() {
        test_example!(3, 406_954);
    }
}
