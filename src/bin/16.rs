#![feature(let_chains)]

advent_of_code::solution!(16);

use itertools::Itertools;
use rayon::prelude::*;
use std::collections::vec_deque::VecDeque;
use std::collections::{HashMap, HashSet};

//use itertools::Itertools;

pub fn part_one(input: &str) -> Option<usize> {
    Some(Grid::parse(input).count_energized((0, 0), Heading::E))
}

pub fn part_two(input: &str) -> Option<usize> {
    let grid = Grid::parse(input);
    grid.starts()
        .into_par_iter()
        .map(|(point, heading)| grid.count_energized(point, heading))
        .max()
}

type Point = (i32, i32);

struct Grid {
    grid: HashMap<Point, Tile>,
    max_x: i32,
    max_y: i32,
}

impl Grid {
    fn parse(input: &str) -> Self {
        let grid: HashMap<Point, Tile> = input
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(move |(x, c)| ((x as i32, y as i32), Tile::parse(c)))
            })
            .collect();
        let max_x = grid.keys().map(|(x, _)| *x).max().unwrap_or_default();
        let max_y = grid.keys().map(|(_, y)| *y).max().unwrap_or_default();
        Self { grid, max_x, max_y }
    }

    fn count_energized(&self, start: Point, heading: Heading) -> usize {
        let mut seen = HashSet::new();
        let mut queue = VecDeque::from([(start, heading)]);
        while let Some((point, heading)) = queue.pop_front() {
            if let Some(tile) = self.grid.get(&point)
                && !seen.contains(&(point, heading))
            {
                seen.insert((point, heading));
                match tile {
                    Tile::Empty => {
                        queue.push_back((heading.step(point), heading));
                    }
                    Tile::Vertical => {
                        let north = (Heading::N.step(point), Heading::N);
                        let south = (Heading::S.step(point), Heading::S);
                        match heading {
                            Heading::N => queue.push_back(north),
                            Heading::S => queue.push_back(south),
                            Heading::E | Heading::W => queue.extend([north, south]),
                        };
                    }
                    Tile::Horizontal => {
                        let east = (Heading::E.step(point), Heading::E);
                        let west = (Heading::W.step(point), Heading::W);
                        match heading {
                            Heading::E => queue.push_back(east),
                            Heading::W => queue.push_back(west),
                            Heading::N | Heading::S => queue.extend([east, west]),
                        };
                    }
                    Tile::UpLeft => {
                        match heading {
                            Heading::N => queue.push_back((Heading::W.step(point), Heading::W)),
                            Heading::E => queue.push_back((Heading::S.step(point), Heading::S)),
                            Heading::S => queue.push_back((Heading::E.step(point), Heading::E)),
                            Heading::W => queue.push_back((Heading::N.step(point), Heading::N)),
                        };
                    }
                    Tile::UpRight => {
                        match heading {
                            Heading::N => queue.push_back((Heading::E.step(point), Heading::E)),
                            Heading::E => queue.push_back((Heading::N.step(point), Heading::N)),
                            Heading::S => queue.push_back((Heading::W.step(point), Heading::W)),
                            Heading::W => queue.push_back((Heading::S.step(point), Heading::S)),
                        };
                    }
                }
            }
        }
        seen.iter().map(|(point, _)| point).unique().count()
    }

    fn starts(&self) -> Vec<(Point, Heading)> {
        let mut starts = Vec::new();
        for x in 0..self.max_x {
            starts.push(((x, 0), Heading::S));
            starts.push(((x, self.max_y), Heading::N));
        }

        for y in 0..self.max_y {
            starts.push(((0, y), Heading::W));
            starts.push(((self.max_x, y), Heading::E));
        }
        starts
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
enum Heading {
    N,
    E,
    S,
    W,
}

impl Heading {
    fn step(&self, point: Point) -> Point {
        let (x, y) = point;
        match self {
            Self::N => (x, y - 1),
            Self::E => (x + 1, y),
            Self::S => (x, y + 1),
            Self::W => (x - 1, y),
        }
    }
}

enum Tile {
    Empty,
    Vertical,
    Horizontal,
    UpLeft,
    UpRight,
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Empty => ".",
                Self::Vertical => "|",
                Self::Horizontal => "-",
                Self::UpLeft => "\\",
                Self::UpRight => "/",
            }
        )
    }
}

impl Tile {
    fn parse(c: char) -> Self {
        match c {
            '|' => Self::Vertical,
            '-' => Self::Horizontal,
            '\\' => Self::UpLeft,
            '/' => Self::UpRight,
            _ => Self::Empty,
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(51));
    }
}
