#![feature(never_type)]

advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u64> {
    let races = parse(Mode::Split, input)?;
    let result = races
        .into_iter()
        .map(|(time, dist)| calculate_hold(time, dist))
        .product();
    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let races = parse(Mode::Join, input)?;
    let result = races
        .into_iter()
        .map(|(time, dist)| calculate_hold(time, dist))
        .product();
    Some(result)
}

fn calculate_hold(time: u64, dist: u64) -> u64 {
    // We know:
    //
    //   dist = hold * (time - hold)
    //
    // or
    //
    //   -hold² + time * hold - dist = 0
    //
    // We can plug this into the quadratic formula; the negative signs
    // cancel:
    //
    //        time ± √(time² - 4dist)
    // hold = -----------------------
    //                 2
    //
    // We only care about integer roots, so round the low root down and
    // the high root up.
    let t = time as f64;
    let d = dist as f64;
    let root = (t * t - 4.0 * d).sqrt();
    let lo = ((t - root) / 2.0).floor();
    let hi = ((t + root) / 2.0).ceil();
    (hi - lo) as u64 - 1
}

fn parse(mode: Mode, input: &str) -> Option<Vec<(u64, u64)>> {
    let [times, dists]: [&str; 2] = input.lines().collect::<Vec<&str>>().try_into().ok()?;
    let times = parse_prefixed(mode, "Time:", times)?;
    let dists = parse_prefixed(mode, "Distance:", dists)?;
    Some(times.into_iter().zip(dists).collect())
}

fn parse_prefixed(mode: Mode, prefix: &str, input: &str) -> Option<Vec<u64>> {
    let split = input.strip_prefix(prefix)?.split_whitespace();
    Some(match mode {
        Mode::Split => split.flat_map(str::parse).collect(),
        Mode::Join => vec![split.collect::<Vec<_>>().join("").parse().ok()?],
    })
}

#[derive(Copy, Clone)]
enum Mode {
    Split,
    Join,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(71503));
    }
}
