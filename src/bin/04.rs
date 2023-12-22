#![feature(never_type)]

use std::collections::HashSet;
use std::str::FromStr;

advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u64> {
    let cards: Vec<Card> = input.lines().flat_map(str::parse).collect();
    let total = cards
        .into_iter()
        .map(|card| {
            let wins = card.wins();
            if wins == 0 {
                0
            } else {
                (2u64).pow(wins as u32 - 1)
            }
        })
        .sum();
    Some(total)
}

pub fn part_two(input: &str) -> Option<u64> {
    let cards: Vec<Card> = input.lines().flat_map(str::parse).collect();
    let mut counts = vec![1; cards.len()];
    for (i, card) in cards.into_iter().enumerate() {
        for j in 0..card.wins() {
            counts[i + j + 1] += counts[i];
        }
    }
    Some(counts.into_iter().sum())
}

struct Card {
    winners: HashSet<u32>,
    given: Vec<u32>,
}

impl Card {
    fn wins(&self) -> usize {
        self.given
            .iter()
            .filter(|x| self.winners.contains(x))
            .count()
    }
}

impl FromStr for Card {
    type Err = !;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let (_, rest) = input.split_once(":").unwrap();
        let (before, after) = rest.split_once("|").unwrap();
        let winners = before.split_whitespace().flat_map(str::parse).collect();
        let given = after.split_whitespace().flat_map(str::parse).collect();
        Ok(Card { winners, given })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(30));
    }
}
