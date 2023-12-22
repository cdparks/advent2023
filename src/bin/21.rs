#![feature(let_chains)]
use std::{collections::{HashMap, HashSet, VecDeque}, array};

advent_of_code::solution!(21);

pub fn part_one(input: &str) -> Option<usize> {
    let max_steps = if cfg!(test) { 6 } else { 64 };
    Some(Map::parse(input).count_reachable_plots(max_steps))
}

pub fn part_two(_input: &str) -> Option<usize> {
    None
}

type Point = (i64, i64);

struct Map {
    grid: HashMap<Point, Tile>,
}

impl Map {
    fn parse(input: &str) -> Self {
        let grid = input
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars().enumerate().flat_map(move |(x, c)| {
                    let tile = Tile::parse(c)?;
                    Some(((x as i64, y as i64), tile))
                })
            })
            .collect();
        Self { grid }
    }

    fn start(&self) -> Point {
        self.grid.iter().find_map(|(point, tile)| {
            if let Tile::Start = tile {
                Some(*point)
            } else {
                None
            }
        }).expect("No start point?")
    }

    fn count_reachable_plots(&self, max_steps: usize) -> usize {
        let start = self.start();
        let mut seen: [_; 2] = array::from_fn(|_| HashSet::<Point>::new());
        let mut queue = VecDeque::from([(start, 0)]);
        while let Some((point @ (x, y), step)) = queue.pop_front()
            && step <= max_steps
        {
            let Some(tile) = self.grid.get(&point) else {
                continue;
            };
            if *tile == Tile::Rock {
                continue;
            };
            let points = &mut seen[step % 2];
            if points.contains(&point) {
                continue;
            }
            points.insert(point);
            for (dx, dy) in [(0, -1), (1, 0), (0, 1), (-1, 0)] {
                queue.push_back(((x + dx, y + dy), step + 1));
            }
        }
        seen[max_steps % 2].len()
    }
}

#[derive(PartialEq)]
enum Tile {
    Start,
    Plot,
    Rock,
}

impl Tile {
    fn parse(c: char) -> Option<Self> {
        match c {
            'S' => Some(Self::Start),
            '.' => Some(Self::Plot),
            '#' => Some(Self::Rock),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(16));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
