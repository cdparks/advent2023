advent_of_code::solution!(13);

pub fn part_one(input: &str) -> Option<usize> {
    Some(run(input, 0))
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(run(input, 1))
}

fn run(input: &str, diff: usize) -> usize {
    input
        .split("\n\n")
        .map(Pattern::parse)
        .flat_map(|pattern| {
            pattern
                .find_reflection(diff)
                .map(|rows| rows * 100)
                .or_else(|| pattern.transposed().find_reflection(diff))
        })
        .sum()
}

#[derive(Debug)]
struct Pattern {
    grid: Vec<Vec<char>>,
}

impl Pattern {
    fn parse(block: &str) -> Self {
        let grid = block.lines().map(|line| line.chars().collect()).collect();
        Self { grid }
    }

    fn find_reflection(&self, diff: usize) -> Option<usize> {
        let n = self.grid.len() - 1;
        for i in 0..n {
            let mut same = 0;
            let mut total = 0;
            for (lhs, rhs) in (0..=i).rev().zip(i + 1..=n) {
                for (x, y) in self.grid[lhs].iter().zip(self.grid[rhs].iter()) {
                    if x == y {
                        same += 1;
                    }
                    total += 1;
                }
            }
            if same == total - diff {
                return Some(i + 1);
            }
        }
        None
    }

    fn transposed(self) -> Self {
        Self {
            grid: transpose(self.grid),
        }
    }
}

// https://stackoverflow.com/questions/64498617/how-to-transpose-a-vector-of-vectors-in-rust
fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(405));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(400));
    }
}
