advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let result = input
        .lines()
        .flat_map(|line| {
            let digits: Vec<u32> = line.chars().filter_map(|c| c.to_digit(10)).collect();
            Some(digits.first()? * 10 + digits.last()?)
        })
        .sum();
    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let input = DIGITS
        .iter()
        .enumerate()
        .try_fold(input.to_string(), |input, (i, digit)| {
            let chars: Vec<char> = digit.chars().collect();
            // e.g. three => t3e, to handle overlap
            let replacement = format!("{}{}{}", chars.first()?, i, chars.last()?);
            Some(input.replace(digit, &replacement))
        })?;
    part_one(input.as_str())
}

const DIGITS: [&str; 10] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            1, "examples", DAY,
        ));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            2, "examples", DAY,
        ));
        assert_eq!(result, Some(281));
    }
}
