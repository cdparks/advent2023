use sscanf::sscanf;
use std::collections::{HashMap, VecDeque};

advent_of_code::solution!(19);

pub fn part_one(input: &str) -> Option<usize> {
    let (flows, parts) = input.split_once("\n\n")?;
    let flows = flows
        .lines()
        .flat_map(|line| {
            let flow = Flow::parse(line)?;
            Some((flow.id, flow))
        })
        .collect();
    let parts: Vec<_> = parts.lines().flat_map(Part::parse).collect();
    Some(evaluate(&parts, &flows))
}

pub fn part_two(input: &str) -> Option<usize> {
    let (flows, _) = input.split_once("\n\n")?;
    let flows = flows
        .lines()
        .flat_map(|line| {
            let flow = Flow::parse(line)?;
            Some((flow.id, flow))
        })
        .collect();
    Some(accepts(&flows))
}

fn evaluate<'a>(parts: &[Part<usize>], flows: &HashMap<&'a str, Flow<'a>>) -> usize {
    let mut result = 0;
    for part in parts.iter() {
        let mut flow = &flows["in"];
        while let Some(target) = flow.evaluate(part) {
            match target {
                Target::Accept => {
                    result += part.rating();
                    break;
                }
                Target::Reject => {
                    break;
                }
                Target::Label(id) => {
                    flow = &flows[id];
                }
            }
        }
    }
    result
}

fn accepts<'a>(flows: &HashMap<&'a str, Flow<'a>>) -> usize {
    let mut result = 0;
    let mut queue = VecDeque::from([("in".into(), 0, Part::full())]);
    while let Some((target, index, part)) = queue.pop_front() {
        match target {
            Target::Accept => {
                result += part.accepts();
            }
            Target::Reject => continue,
            Target::Label(id) => {
                queue.extend(flows[id].explore(index, part));
            }
        }
    }
    result
}

struct Flow<'a> {
    id: &'a str,
    rules: Vec<Rule<'a>>,
}

impl<'a> Flow<'a> {
    fn parse(input: &'a str) -> Option<Self> {
        let (id, body) = &input[0..input.len() - 1].split_once('{')?;
        let rules = body.split(',').flat_map(Rule::parse).collect();
        Some(Self { id, rules })
    }

    fn evaluate(&self, part: &Part<usize>) -> Option<Target<'a>> {
        self.rules.iter().find_map(|rule| rule.evaluate(part))
    }

    fn explore(&self, index: usize, part: Part<Range>) -> Vec<(Target<'a>, usize, Part<Range>)> {
        if let Some(rule) = self.rules.get(index) {
            return rule.explore(self.into(), index, part);
        }
        Vec::new()
    }
}

#[derive(Copy, Clone)]
enum Rule<'a> {
    Lt(usize, usize, Target<'a>),
    Gt(usize, usize, Target<'a>),
    Jmp(Target<'a>),
}

impl<'a> Rule<'a> {
    fn parse(input: &'a str) -> Option<Self> {
        match input.split_once(':') {
            None => Some(Self::Jmp(Target::parse(input)?)),
            Some((condition, target)) => {
                let mut chars = condition.chars();
                let category = match chars.next()? {
                    'x' => Some(0),
                    'm' => Some(1),
                    'a' => Some(2),
                    's' => Some(3),
                    _ => None,
                }?;
                let operator = chars.next()?;
                let value = str::parse(&chars.collect::<String>()).ok()?;
                let target = Target::parse(target)?;
                match operator {
                    '<' => Some(Self::Lt(category, value, target)),
                    '>' => Some(Self::Gt(category, value, target)),
                    _ => None,
                }
            }
        }
    }

    fn evaluate(&self, part: &Part<usize>) -> Option<Target<'a>> {
        match *self {
            Self::Gt(category, value, target) if part.get(category) > value => Some(target),
            Self::Lt(category, value, target) if part.get(category) < value => Some(target),
            Self::Jmp(target) => Some(target),
            _ => None,
        }
    }

    fn explore(
        &self,
        src: Target<'a>,
        index: usize,
        part: Part<Range>,
    ) -> Vec<(Target<'a>, usize, Part<Range>)> {
        match *self {
            Rule::Lt(category, value, dst) => {
                let (lo, hi) = part.get(category);
                if lo >= value {
                    vec![(src, index + 1, part)]
                } else if hi < value {
                    vec![(dst, 0, part)]
                } else {
                    let lhs = part.with(category, (lo, value - 1));
                    let rhs = part.with(category, (value, hi));
                    vec![(dst, 0, lhs), (src, index + 1, rhs)]
                }
            }
            Rule::Gt(category, value, dst) => {
                let (lo, hi) = part.get(category);
                if hi <= value {
                    vec![(src, index + 1, part)]
                } else if lo > value {
                    vec![(dst, 0, part)]
                } else {
                    let lhs = part.with(category, (lo, value));
                    let rhs = part.with(category, (value + 1, hi));
                    vec![(src, index + 1, lhs), (dst, 0, rhs)]
                }
            }
            Rule::Jmp(dst) => vec![(dst, 0, part)],
        }
    }
}

#[derive(Copy, Clone)]
enum Target<'a> {
    Accept,
    Reject,
    Label(&'a str),
}

impl<'a> Target<'a> {
    fn parse(input: &'a str) -> Option<Self> {
        input.chars().next().map(|c| match c {
            'A' => Self::Accept,
            'R' => Self::Reject,
            _ => Self::Label(input),
        })
    }
}

impl<'a> From<&Flow<'a>> for Target<'a> {
    fn from(flow: &Flow<'a>) -> Self {
        flow.id.into()
    }
}

impl<'a> From<&'a str> for Target<'a> {
    fn from(id: &'a str) -> Self {
        Self::Label(id)
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct Part<T> {
    ratings: [T; 4],
}

impl Part<usize> {
    fn parse(input: &str) -> Option<Self> {
        let (x, m, a, s) = sscanf!(input, "{{x={usize},m={usize},a={usize},s={usize}}}").ok()?;
        Some(Self {
            ratings: [x, m, a, s],
        })
    }

    fn rating(self) -> usize {
        self.ratings.iter().sum()
    }
}

impl Part<Range> {
    fn full() -> Self {
        Self {
            ratings: [(1, 4000); 4],
        }
    }

    fn accepts(&self) -> usize {
        self.ratings.iter().map(|(lo, hi)| hi - lo + 1).product()
    }
}

impl<T> Part<T> {
    fn get(&self, category: usize) -> T
    where
        T: Copy,
    {
        self.ratings[category]
    }

    fn with(&self, category: usize, value: T) -> Part<T>
    where
        T: Copy,
    {
        let mut ratings = self.ratings;
        ratings[category] = value;
        Self { ratings }
    }
}

type Range = (usize, usize);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(19114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(167409079868000));
    }
}
