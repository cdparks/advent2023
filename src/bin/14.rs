use std::collections::HashMap;
use std::iter::once;

advent_of_code::solution!(14);

pub fn part_one(input: &str) -> Option<usize> {
    let mut platform = Platform::parse(input)?;
    platform.collapse();
    Some(platform.score())
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut platform = Platform::parse(input)?;
    let mut seen = HashMap::from([(platform.clone(), 0)]);
    loop {
        platform.collapse();
        platform.rotate(); // N -> W

        platform.collapse();
        platform.rotate(); // W -> S

        platform.collapse();
        platform.rotate(); // S -> E

        platform.collapse();
        platform.rotate(); // E -> N

        if let Some(start) = seen.get(&platform) {
            let end = seen.len();
            let cycle_len = end - start;
            let final_step = ((1_000_000_000 - start) % cycle_len) + start;
            return seen
                .into_iter()
                .find(|(_, step)| *step == final_step)
                .map(|(platform, _)| platform.score());
        }

        seen.insert(platform.clone(), seen.len());
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Platform {
    columns: Vec<Vec<Tile>>,
}

impl Platform {
    fn parse(input: &str) -> Option<Self> {
        let mut lines = input.lines();
        let first = lines.next()?;
        let mut columns: Vec<Vec<Tile>> = (0..first.len()).map(|_| Vec::new()).collect();
        for line in once(first).chain(lines) {
            for (i, c) in line.chars().enumerate() {
                columns[i].push(Tile::parse(c));
            }
        }
        Some(Self { columns })
    }

    fn collapse(&mut self) {
        for column in self.columns.iter_mut() {
            for mut i in 1..column.len() {
                'inner: while i > 0 {
                    match (&column[i], &column[i - 1]) {
                        (Tile::Round, Tile::Empty) => column.swap(i, i - 1),
                        _ => break 'inner,
                    }
                    i -= 1;
                }
            }
        }
    }

    fn score(&self) -> usize {
        self.columns
            .iter()
            .flat_map(|column| {
                column.iter().rev().enumerate().map(|(i, tile)| match tile {
                    Tile::Round => i + 1,
                    _ => 0,
                })
            })
            .sum()
    }

    fn rotate(&mut self) {
        // square input, use in-place transpose
        let n = self.columns.len();
        assert!(n > 0);
        let m = self.columns[0].len();
        assert!(n == m);
        for i in 0..n - 1 {
            for j in i + 1..n {
                (self.columns[i][j], self.columns[j][i]) = (self.columns[j][i], self.columns[i][j]);
            }
        }

        self.columns.reverse();
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
enum Tile {
    Empty,
    Round,
    Cube,
}

impl Tile {
    fn parse(input: char) -> Self {
        match input {
            'O' => Self::Round,
            '#' => Self::Cube,
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
        assert_eq!(result, Some(136));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(64));
    }
}
