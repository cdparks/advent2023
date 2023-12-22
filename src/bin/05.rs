#![feature(never_type)]
use sscanf::sscanf;
use std::str::FromStr;

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u64> {
    let almanac: Almanac = str::parse(input).unwrap();
    almanac
        .seeds
        .into_iter()
        .map(|seed| {
            almanac.mappings.iter().fold(seed, |acc, mapping| {
                mapping
                    .iter()
                    .copied()
                    .find_map(|(dst, lo, len)| {
                        let hi = lo + len - 1;
                        if lo <= acc && acc <= hi {
                            Some(acc - lo + dst)
                        } else {
                            None
                        }
                    })
                    .unwrap_or(acc)
            })
        })
        .min()
}

pub fn part_two(input: &str) -> Option<u64> {
    let almanac: Almanac = str::parse(input).unwrap();
    let mut ranges: Vec<(u64, u64)> = almanac
        .seeds
        .chunks(2)
        .flat_map(TryInto::<[u64; 2]>::try_into)
        .map(|[lo, len]| (lo, lo + len - 1))
        .collect();

    for mapping in almanac.mappings.iter() {
        ranges = ranges
            .iter()
            .copied()
            .flat_map(|(lo, hi)| {
                // suppose we have an interval like this:
                //
                // |------------------------------|
                //
                // with mappings like this:
                //      |=====|
                //                  |========|
                //
                // we'll gradually move the head of the interval forward:
                //
                // |----|
                // ^    |=====|
                //            |-----|
                // ---------> ^     |========|
                //                           |----|
                //            -------------> ^
                // until we've exhausted the mappings or the head of the
                // interval crosses its upper bound, remapping the
                // parts of it that overlapped the mappings.
                //
                // |----|=====|-----|========|----|
                //                           ^
                let mut head = lo;
                let mut next = Vec::new();
                for (dst, src, len) in mapping.iter().copied() {
                    let end = src + len - 1;

                    // mapping does not intersect this interval, but a
                    // later one could
                    if head > end {
                        continue;
                    }

                    // no latter mapping can intersect this interval;
                    // src will always be higher
                    if src > hi {
                        break;
                    }

                    // exhausted interval, the head has crossed the
                    // upper bound
                    if head > hi {
                        break;
                    }

                    // slice off the prefix of the interval before the
                    // lower bound of this mapping
                    if src > head {
                        next.push((head, src - 1));
                    }

                    // remap the overlapping section
                    let mapped_lo = head.max(src) - src + dst;
                    let mapped_hi = hi.min(end) - src + dst;
                    next.push((mapped_lo, mapped_hi));

                    // then move the head to point after it
                    head = hi.min(end) + 1;
                }

                // remainder of the interval after the upper bound of
                // any mapping
                if head < hi {
                    next.push((head, hi));
                }
                next
            })
            .collect();
    }

    ranges.into_iter().map(|(lo, _)| lo).min()
}

struct Almanac {
    seeds: Vec<u64>,
    mappings: Vec<Vec<(u64, u64, u64)>>,
}

impl FromStr for Almanac {
    type Err = !;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (seeds_str, rest) = s.split_once("\n\n").unwrap();

        let seeds = sscanf!(seeds_str, "seeds: {}", String)
            .unwrap()
            .split_whitespace()
            .flat_map(str::parse)
            .collect();

        let mappings: Vec<Vec<(u64, u64, u64)>> = rest
            .split("\n\n")
            .map(|block| {
                let mut mapping: Vec<(u64, u64, u64)> = block
                    .lines()
                    .skip(1)
                    .flat_map(|line| sscanf!(line, "{} {} {}", u64, u64, u64))
                    .collect();
                mapping.sort_by_key(|&(_, src, _)| src);
                mapping
            })
            .collect();

        Ok(Almanac { seeds, mappings })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }
}
