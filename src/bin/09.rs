advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<i64> {
    let report = Report::parse(input);
    let result = report
        .histories
        .into_iter()
        .map(|history| extrapolate(history.into_iter()))
        .sum();
    Some(result)
}

pub fn part_two(input: &str) -> Option<i64> {
    let report = Report::parse(input);
    let result = report
        .histories
        .into_iter()
        .map(|history| extrapolate(history.into_iter().rev()))
        .sum();
    Some(result)
}

fn extrapolate<I>(history: I) -> i64
where
    I: Iterator<Item = i64>,
{
    let mut diffs: Vec<i64> = history.collect();
    let mut result = diffs[diffs.len() - 1];
    loop {
        let mut done = true;
        let mut diff = 0;
        diffs = diffs
            .windows(2)
            .map(|pair| {
                diff = pair[1] - pair[0];
                done = done && diff == 0;
                diff
            })
            .collect();
        if done {
            break;
        }
        result += diff;
    }
    result
}

#[derive(Debug)]
struct Report {
    histories: Vec<Vec<i64>>,
}

impl Report {
    fn parse(input: &str) -> Self {
        let histories = input
            .lines()
            .map(|line| {
                line.split_whitespace()
                    .flat_map(str::parse)
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        Self { histories }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }
}
