#![feature(let_chains)]
advent_of_code::solution!(17);

use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::ops::Add;

pub fn part_one(input: &str) -> Option<usize> {
    Some(Graph::parse(input).heat_loss(1, 3))
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(Graph::parse(input).heat_loss(4, 10))
}

struct Graph {
    graph: HashMap<Point, usize>,
}

impl Graph {
    pub fn parse(input: &str) -> Self {
        let graph = input
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.bytes()
                    .enumerate()
                    .map(move |(x, byte)| (Point::new(x as i64, y as i64), (byte - b'0') as usize))
            })
            .collect();
        Self { graph }
    }

    pub fn target(&self) -> Point {
        let x = self
            .graph
            .keys()
            .map(|&point| point.x)
            .max()
            .unwrap_or_default();
        let y = self
            .graph
            .keys()
            .map(|&point| point.y)
            .max()
            .unwrap_or_default();
        Point::new(x, y)
    }

    pub fn heat_loss(&self, min: usize, max: usize) -> usize {
        let target = self.target();

        let mut costs: HashMap<Entry, usize> = HashMap::new();
        let mut queue: BinaryHeap<(Reverse<usize>, Entry)> = BinaryHeap::new();

        for heading in [Heading::E, Heading::S] {
            let entry = Entry::new(Point::zero(), heading, 0);
            queue.push((Reverse(0), entry));
            costs.insert(entry, 0);
        }

        while let Some((Reverse(cost), entry)) = queue.pop() {
            if entry.point == target && entry.steps >= min {
                return cost;
            }

            for heading in [entry.heading, entry.heading.left(), entry.heading.right()] {
                let successor = entry.step(heading);
                if entry.heading != successor.heading && entry.steps < min {
                    continue;
                }

                if successor.steps > max {
                    continue;
                }

                if let Some(successor_cost) = self.graph.get(&successor.point) {
                    let cost = successor_cost + cost;

                    if costs.get(&successor).unwrap_or(&usize::MAX) <= &cost {
                        continue;
                    }

                    queue.push((Reverse(cost), successor));
                    costs.insert(successor, cost);
                }
            }
        }
        usize::MAX
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
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

    fn step(self, heading: Heading) -> Point {
        self + heading.diff()
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self::new(self.x + other.x, self.y + other.y)
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Heading {
    N,
    E,
    S,
    W,
}

impl Heading {
    fn diff(&self) -> Point {
        match self {
            Self::N => Point::new(0, -1),
            Self::E => Point::new(1, 0),
            Self::S => Point::new(0, 1),
            Self::W => Point::new(-1, 0),
        }
    }

    fn left(&self) -> Heading {
        match self {
            Self::N => Self::W,
            Self::E => Self::N,
            Self::S => Self::E,
            Self::W => Self::S,
        }
    }

    fn right(&self) -> Heading {
        match self {
            Self::N => Self::E,
            Self::E => Self::S,
            Self::S => Self::W,
            Self::W => Self::N,
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Entry {
    point: Point,
    heading: Heading,
    steps: usize,
}

impl Entry {
    fn new(point: Point, heading: Heading, steps: usize) -> Self {
        Self {
            point,
            heading,
            steps,
        }
    }

    fn step(&self, heading: Heading) -> Self {
        Self::new(
            self.point.step(heading),
            heading,
            if heading == self.heading {
                self.steps + 1
            } else {
                1
            },
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(102));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(94));
    }
}
