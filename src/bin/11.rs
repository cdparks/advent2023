#![feature(array_windows)]
use itertools::Itertools;

advent_of_code::solution!(11);

pub fn part_one(input: &str) -> Option<usize> {
    run(&parse(input), 1)
}

pub fn part_two(input: &str) -> Option<usize> {
    run(&parse(input), 1_000_000 - 1)
}

fn parse(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn run(grid: &[Vec<char>], diff: i64) -> Option<usize> {
    let galaxies = &expand(grid, diff);
    let result = galaxies
        .into_iter()
        .combinations(2)
        .map(|pair| manhattan_distance(&pair[0], &pair[1]))
        .sum();
    Some(result)
}

type Point = (i64, i64);

fn expand(grid: &[Vec<char>], diff: i64) -> Vec<Point> {
    let mut row_diff = 0i64;
    let row_diffs: Vec<_> = grid
        .iter()
        .map(|row| {
            if row.iter().all(|c| *c == '.') {
                row_diff += diff;
            }
            row_diff
        })
        .collect();

    let mut col_diff = 0i64;
    let col_diffs: Vec<_> = (0..grid[0].len())
        .map(|col| {
            if (0..grid.len()).all(|row| grid[row][col] == '.') {
                col_diff += diff;
            }
            col_diff
        })
        .collect();

    let row_diffs = &row_diffs;
    let col_diffs = &col_diffs;
    grid.into_iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.into_iter()
                .enumerate()
                .filter(|(_, c)| **c == '#')
                .map(move |(x, _)| {
                    let dx = col_diffs[x];
                    let dy = row_diffs[y];
                    (x as i64 + dx, y as i64 + dy)
                })
        })
        .unique()
        .collect()
}

fn manhattan_distance(src: &Point, dst: &Point) -> usize {
    ((dst.0 - src.0).abs() + (dst.1 - src.1).abs()) as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(374));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(82000210));
    }
}
