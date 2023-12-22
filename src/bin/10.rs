#![feature(let_chains)]
#![feature(array_windows)]

use lazy_static::lazy_static;
use std::collections::HashMap;

advent_of_code::solution!(10);

pub fn part_one(input: &str) -> Option<i64> {
    Map::parse(input)
        .loop_path()
        .map(|path| path.len() as i64 / 2)
}

pub fn part_two(input: &str) -> Option<i64> {
    let map = Map::parse(input);
    let path = map.loop_path()?;

    // https://stackoverflow.com/questions/451426/how-do-i-calculate-the-area-of-a-2d-polygon
    let area = &path[..]
        .array_windows::<2>()
        .map(|[(x0, y0), (x1, y1)]| x0 * y1 - y0 * x1)
        .sum()
        / 2i64;

    Some(area.abs() - (path.len() as i64 / 2 - 1))
}

type Point = (i64, i64);

struct Map {
    start: Point,
    points: HashMap<Point, Tile>,
}

impl Map {
    fn parse(input: &str) -> Self {
        let mut points = HashMap::new();
        let mut start = (0, 0);
        for (y, line) in input.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                let point = (x as i64, y as i64);
                match c {
                    'S' => {
                        start = point;
                        points.insert(point, Tile::Start);
                    }
                    '.' => {
                        continue;
                    }
                    _ => {
                        points.insert(point, Tile::Tile(c));
                    }
                }
            }
        }

        Self { start, points }
    }

    fn neighbors(&self, point: &Point) -> Option<[Point; 2]> {
        let (x, y) = *point;
        self.points.get(point).and_then(|other| match other {
            Tile::Start => {
                let mut neighbors = Vec::new();
                for (dx, dy) in [(0, -1), (1, 0), (0, 1), (-1, 0)] {
                    let neighbor = (x + dx, y + dy);
                    if let Some(others) = self.neighbors(&neighbor)
                        && others.contains(&(x, y))
                    {
                        neighbors.push(neighbor);
                    }
                }
                neighbors.try_into().ok()
            }
            Tile::Tile(c) => Some(DIFFS[c].map(|(dx, dy)| (x + dx, y + dy))),
        })
    }

    // loop path includes start point on both ends
    fn loop_path(&self) -> Option<Vec<Point>> {
        let mut prev = self.start;
        let mut curr = self.start;
        let mut path = vec![curr];
        while let Some(neighbors) = self.neighbors(&curr) {
            if let Some(neighbor) = neighbors.iter().find(|point| **point != prev) {
                (curr, prev) = (*neighbor, curr);
                path.push(curr);
            } else {
                return None;
            }
            if curr == self.start {
                break;
            }
        }
        Some(path)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
enum Tile {
    Start,
    Tile(char),
}

lazy_static! {
    static ref DIFFS: HashMap<char, [Point; 2]> = HashMap::from([
        ('|', [(0, -1), (0, 1)]),
        ('-', [(-1, 0), (1, 0)]),
        ('L', [(0, -1), (1, 0)]),
        ('J', [(0, -1), (-1, 0)]),
        ('7', [(0, 1), (-1, 0)]),
        ('F', [(0, 1), (1, 0)])
    ]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            1, "examples", DAY,
        ));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            2, "examples", DAY,
        ));
        assert_eq!(result, Some(10));
    }
}
