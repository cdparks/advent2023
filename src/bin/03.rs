#![feature(let_chains)]
#![feature(never_type)]
advent_of_code::solution!(3);

use std::collections::HashMap;
use std::collections::HashSet;
use std::str::FromStr;

pub fn part_one(input: &str) -> Option<u32> {
    let engine: Engine = str::parse(input).ok()?;
    let total = engine
        .numbers
        .into_iter()
        .filter_map(|(x, y, number, len)| {
            if neighbors(x, y, len).any(|(x, y)| engine.symbols.contains_key(&(x, y))) {
                Some(number)
            } else {
                None
            }
        })
        .sum();
    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    let engine: Engine = str::parse(input).ok()?;
    let mut star_parts: HashMap<(isize, isize), HashSet<(isize, isize, u32)>> = HashMap::new();
    engine.numbers.into_iter().for_each(|(x, y, number, len)| {
        let part = (x, y, number);
        neighbors(x, y, len).for_each(|(x, y)| {
            if engine.symbols.get(&(x, y)) == Some(&b'*') {
                star_parts.entry((x, y)).or_default().insert(part);
            }
        })
    });
    let total: u32 = star_parts
        .values()
        .filter(|parts| parts.len() == 2)
        .map(|parts| {
            parts
                .into_iter()
                .map(|(_, _, number)| number)
                .product::<u32>()
        })
        .sum();
    Some(total)
}

fn neighbors(x0: isize, y0: isize, len: usize) -> impl Iterator<Item = (isize, isize)> {
    let lo = x0 - 1;
    let hi = x0 + len as isize;
    (lo..=hi).flat_map(move |x| [y0 - 1, y0, y0 + 1].map(|y| (x, y)))
}

struct Engine {
    numbers: Vec<(isize, isize, u32, usize)>,
    symbols: HashMap<(isize, isize), u8>,
}

impl FromStr for Engine {
    type Err = !;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut numbers = Vec::new();
        let mut symbols = HashMap::new();
        for (y, line) in input.lines().map(str::as_bytes).enumerate() {
            let mut x: usize = 0;
            while let Some(first) = line[x..].first() {
                if first.is_ascii_digit() {
                    let x0 = x;
                    x += 1;
                    let mut acc = (*first - b'0') as u32;
                    while let Some(b) = line[x..].first()
                        && b.is_ascii_digit()
                    {
                        acc = acc * 10 + ((*b - b'0') as u32);
                        x += 1;
                    }
                    numbers.push((x0 as isize, y as isize, acc, x - x0));
                } else if *first == b'.' {
                    x += 1;
                } else {
                    symbols.insert((x as isize, y as isize), *first);
                    x += 1;
                }
            }
        }
        Ok(Self { numbers, symbols })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(467835));
    }
}
