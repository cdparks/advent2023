#![feature(never_type)]
use sscanf::sscanf;
use std::str::FromStr;

advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    let result = input
        .lines()
        .flat_map(str::parse)
        .filter_map(|game: Game| {
            if game.valid(12, 13, 14) {
                Some(game.id)
            } else {
                None
            }
        })
        .sum();
    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let result = input
        .lines()
        .flat_map(str::parse)
        .map(|game: Game| game.power())
        .sum();
    Some(result)
}

// Only need to store max number of red, green, and blue cubes
struct Game {
    id: u32,
    red: u32,
    green: u32,
    blue: u32,
}

impl Game {
    fn empty(id: u32) -> Self {
        Self {
            id,
            red: 0,
            green: 0,
            blue: 0,
        }
    }

    fn valid(&self, red: u32, green: u32, blue: u32) -> bool {
        self.red <= red && self.green <= green && self.blue <= blue
    }

    fn power(&self) -> u32 {
        self.red * self.green * self.blue
    }
}

impl FromStr for Game {
    type Err = !;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (id, rest) = sscanf!(s, "Game {}: {}", u32, String).expect("Game N: ...");
        let mut game = Game::empty(id);
        for round in rest.split("; ") {
            for part in round.split(", ") {
                let (n, color) = sscanf!(part, "{} {}", u32, String).expect("N red|green|blue");
                match color.chars().next() {
                    Some('r') => game.red = game.red.max(n),
                    Some('g') => game.green = game.green.max(n),
                    Some('b') => game.blue = game.blue.max(n),
                    _ => panic!("unrecognized color {}", color),
                }
            }
        }
        Ok(game)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}
