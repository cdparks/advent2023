#![feature(array_windows)]
use sscanf::sscanf;
use std::iter::once;
use std::ops::{AddAssign, Mul};

advent_of_code::solution!(18);

pub fn part_one(input: &str) -> Option<i64> {
    Some(Plan::parse(Mode::Plain, input).area())
}

pub fn part_two(input: &str) -> Option<i64> {
    Some(Plan::parse(Mode::Hex, input).area())
}

#[derive(Copy, Clone)]
enum Mode {
    Plain,
    Hex,
}

struct Plan {
    steps: Vec<Step>,
}

impl Plan {
    fn parse(mode: Mode, input: &str) -> Self {
        let steps = input
            .lines()
            .flat_map(|line| Step::parse(mode, line))
            .collect();
        Self { steps }
    }

    fn area(&self) -> i64 {
        let len: i64 = self.steps.iter().map(|step| step.distance).sum();

        let vertices: Vec<_> = once(Point::zero())
            .chain(self.steps.iter().scan(Point::zero(), |point, step| {
                *point += step.heading.diff() * step.distance;
                Some(*point)
            }))
            .collect();

        // https://stackoverflow.com/questions/451426/how-do-i-calculate-the-area-of-a-2d-polygon
        let area = &vertices[..]
            .array_windows::<2>()
            .map(|[p1, p2]| p1.x * p2.y - p1.y * p2.x)
            .sum()
            / 2i64;

        area.abs() + len / 2 + 1
    }
}

struct Step {
    heading: Heading,
    distance: i64,
}

impl Step {
    fn parse(mode: Mode, line: &str) -> Option<Self> {
        let (heading, distance, color) = sscanf!(line, "{str} {i64} (#{str})").ok()?;
        let (heading, distance) = match mode {
            Mode::Plain => (Heading::parse(heading)?, distance),
            Mode::Hex => {
                assert!(color.len() == 6);
                let distance = i64::from_str_radix(&color[0..5], 16).ok()?;
                let heading = Heading::parse(&color[5..])?;
                (heading, distance)
            }
        };
        Some(Self { heading, distance })
    }
}

enum Heading {
    N,
    E,
    S,
    W,
}

impl Heading {
    fn parse(input: &str) -> Option<Self> {
        input.chars().next().and_then(|c| match c {
            'R' | '0' => Some(Heading::E),
            'D' | '1' => Some(Heading::S),
            'L' | '2' => Some(Heading::W),
            'U' | '3' => Some(Heading::N),
            _ => None,
        })
    }

    fn diff(&self) -> Point {
        match self {
            Self::N => Point::new(0, -1),
            Self::E => Point::new(1, 0),
            Self::S => Point::new(0, 1),
            Self::W => Point::new(-1, 0),
        }
    }
}

#[derive(Copy, Clone)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    fn zero() -> Self {
        Self::new(0, 0)
    }
}

impl AddAssign<Point> for Point {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl Mul<i64> for Point {
    type Output = Self;

    fn mul(self, scale: i64) -> Self {
        Self::new(self.x * scale, self.y * scale)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(62));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(952408144115));
    }
}
