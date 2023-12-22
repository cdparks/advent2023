#![feature(never_type)]
use num::integer::lcm;
use sscanf::sscanf;
use std::collections::HashMap;

advent_of_code::solution!(8);

pub fn part_one(input: &str) -> Option<u64> {
    Map::parse(input)?.count_steps(|node| node == "AAA", |node| node == "ZZZ")
}

pub fn part_two(input: &str) -> Option<u64> {
    Map::parse(input)?.count_steps(|node| node.ends_with('A'), |node| node.ends_with('Z'))
}

enum Direction {
    Left,
    Right,
}

struct Map<'a> {
    directions: Vec<Direction>,
    nodes: HashMap<&'a str, (&'a str, &'a str)>,
}

impl<'a> Map<'a> {
    fn count_steps<F, G>(&self, is_src: F, is_dst: G) -> Option<u64>
    where
        F: Fn(&str) -> bool,
        G: Fn(&str) -> bool,
    {
        self.nodes
            .keys()
            .filter(|node| is_src(node))
            .map(|mut node| {
                for (i, dir) in self.directions.iter().cycle().enumerate() {
                    let (left, right) = &self.nodes[node];
                    node = match dir {
                        Direction::Left => left,
                        Direction::Right => right,
                    };
                    if is_dst(node) {
                        return i as u64 + 1;
                    }
                }
                // compiler doesn't know about cycle()
                unreachable!("infinite loop w/ early return")
            })
            .reduce(lcm)
    }

    fn parse(input: &'a str) -> Option<Self> {
        let (first, rest) = input.split_once("\n\n")?;

        let directions = first
            .chars()
            .map(|c| match c {
                'L' => Direction::Left,
                _ => Direction::Right,
            })
            .collect();

        let nodes = rest
            .lines()
            .flat_map(|line| {
                let (node, left, right) = sscanf!(line, "{} = ({}, {})", &str, &str, &str).ok()?;
                Some((node, (left, right)))
            })
            .collect();

        Some(Map { directions, nodes })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            1, "examples", DAY,
        ));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            2, "examples", DAY,
        ));
        assert_eq!(result, Some(6));
    }
}
