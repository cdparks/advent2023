advent_of_code::solution!(12);

use std::collections::HashMap;

pub fn part_one(input: &str) -> Option<usize> {
    run(input, 1)
}

pub fn part_two(input: &str) -> Option<usize> {
    run(input, 5)
}

fn run(input: &str, replicate: usize) -> Option<usize> {
    Some(
        parse(input, replicate)
            .map(|(springs, counts)| count_assignments(&springs, &counts))
            .sum(),
    )
}

fn parse(input: &str, replicate: usize) -> impl Iterator<Item = (Vec<Spring>, Vec<u32>)> + '_ {
    input.lines().flat_map(move |line| {
        let (head, tail) = line.split_once(' ')?;
        let head = vec![head; replicate].join("?");
        let tail = vec![tail; replicate].join(",");
        let springs: Vec<_> = head.chars().flat_map(Spring::parse).collect();
        let counts: Vec<_> = tail.split(',').flat_map(str::parse).collect();
        Some((springs, counts))
    })
}

type Key = (usize, usize, u32);

fn count_assignments(springs: &[Spring], counts: &[u32]) -> usize {
    let mut cache = HashMap::new();
    go(&mut cache, springs, counts, None)
}

fn go(
    cache: &mut HashMap<Key, usize>,
    springs: &[Spring],
    counts: &[u32],
    run: Option<u32>,
) -> usize {
    if springs.is_empty() {
        return match (run, counts) {
            (None, []) => return 1,
            (Some(x), [y]) if x == *y => 1,
            _ => 0,
        };
    }

    if run.is_some() && counts.is_empty() {
        return 0;
    }

    let key = (springs.len(), counts.len(), run.unwrap_or_default());
    if let Some(&result) = cache.get(&key) {
        return result;
    }

    let result = match (&springs[0], run) {
        (Spring::Working, Some(x)) => {
            if x == counts[0] {
                go(cache, &springs[1..], &counts[1..], None)
            } else {
                0
            }
        }
        (Spring::Working, None) => go(cache, &springs[1..], counts, None),
        (Spring::Damaged, Some(x)) => go(cache, &springs[1..], counts, Some(x + 1)),
        (Spring::Damaged, None) => go(cache, &springs[1..], counts, Some(1)),
        (Spring::Unknown, Some(x)) => {
            let mut result = go(cache, &springs[1..], counts, Some(x + 1));
            if counts[0] == x {
                result += go(cache, &springs[1..], &counts[1..], None);
            }
            result
        }
        (Spring::Unknown, None) => {
            let start = go(cache, &springs[1..], counts, Some(1));
            let wait = go(cache, &springs[1..], counts, None);
            start + wait
        }
    };
    cache.insert(key, result);
    result
}

enum Spring {
    Unknown,
    Working,
    Damaged,
}

impl Spring {
    fn parse(input: char) -> Option<Spring> {
        match input {
            '?' => Some(Self::Unknown),
            '.' => Some(Self::Working),
            '#' => Some(Self::Damaged),
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
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(525152));
    }
}
