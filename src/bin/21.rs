#![feature(let_chains)]
use std::collections::{HashMap, HashSet, VecDeque};

advent_of_code::solution!(21);

pub fn part_one(input: &str) -> Option<usize> {
    let max_steps = if cfg!(test) { 6 } else { 64 };
    let map = Map::parse(input);
    Some(map.count_reachable_plots(max_steps))
}

pub fn part_two(input: &str) -> Option<usize> {
    let map = Map::parse(input);
    let max_steps = 26501365;
    let extra = max_steps % map.size;

    let mut ys = [0; 3];
    for n in 0..3 {
        let steps = n * map.size + extra;
        ys[n as usize] = map.count_reachable_plots(steps as usize) as i64;
    }

    let x = max_steps / map.size;
    Some(solve(ys, x) as usize)
}


// Find quadratic coefficients given a 3-element sequence and substitute in x
fn solve(ys: [i64; 3], x: i64) -> i64 {
    // First difference
    let dy1 = ys[1] - ys[0];
    let dy2 = ys[2] - ys[1];

    // Second difference
    let ddy = dy2 - dy1;

    // 2a = second difference
    let a = ddy / 2;

    // 3a + b = ys₂ - ys₁
    let b = dy2 - 3 * a;

    // first term of sequence
    let c = ys[0];

    return a * x.pow(2) + b * x + c;
}

type Point = (i64, i64);

struct Map {
    grid: HashMap<Point, Tile>,
    size: i64,
}

impl Map {
    fn parse(input: &str) -> Self {
        let grid: HashMap<Point, Tile> = input
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars().enumerate().flat_map(move |(x, c)| {
                    let tile = Tile::parse(c)?;
                    Some(((x as i64, y as i64), tile))
                })
            })
            .collect();
        let max_x = grid.keys().map(|(x, _)| *x).max().unwrap_or_default();
        let max_y = grid.keys().map(|(_, y)| *y).max().unwrap_or_default();
        assert!(max_x == max_y);
        Self {
            grid,
            size: max_x + 1,
        }
    }

    fn start(&self) -> Point {
        self.grid
            .iter()
            .find_map(|(point, tile)| {
                if let Tile::Start = tile {
                    Some(*point)
                } else {
                    None
                }
            })
            .expect("No start point?")
    }

    fn count_reachable_plots(&self, max_steps: usize) -> usize {
        let start = self.start();
        let parity = max_steps % 2;
        let mut seen = HashSet::new();
        let mut queue = VecDeque::from([(start, 0)]);
        let mut count = 0;

        while let Some((point @ (x, y), step)) = queue.pop_front()
            && step <= max_steps
        {
            let grid_point = (x.rem_euclid(self.size), y.rem_euclid(self.size));
            let Some(tile) = self.grid.get(&grid_point) else {
                continue;
            };

            if *tile == Tile::Rock {
                continue;
            };

            if seen.contains(&point) {
                continue;
            }
            seen.insert(point);

            if step % 2 == parity {
                count += 1;
            }

            for (dx, dy) in [(0, -1), (1, 0), (0, 1), (-1, 0)] {
                queue.push_back(((x + dx, y + dy), step + 1));
            }
        }
        count
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
