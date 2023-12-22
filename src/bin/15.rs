use std::array;

advent_of_code::solution!(15);

pub fn part_one(input: &str) -> Option<usize> {
    Some(input.split(',').map(hash).sum())
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut table = Table::new();
    input
        .split(',')
        .flat_map(Command::parse)
        .for_each(|command| match command {
            Command::Add { label, value } => table.add(label, value),
            Command::Remove { label } => table.remove(label),
        });
    Some(table.power())
}

struct Table<'a> {
    boxes: [Vec<Entry<'a>>; 256],
}

impl<'a> Table<'a> {
    fn new() -> Self {
        Self {
            boxes: array::from_fn(|_| Vec::new()),
        }
    }

    fn add(&mut self, label: &'a str, value: u8) {
        let entry = Entry { label, value };
        let entries = &mut self.boxes[hash(label)];
        match entries.iter().position(|entry| entry.label == label) {
            Some(pos) => entries[pos] = entry,
            None => entries.push(entry),
        }
    }

    fn remove(&mut self, label: &str) {
        let entries = &mut self.boxes[hash(label)];
        if let Some(pos) = entries.iter().position(|entry| entry.label == label) {
            entries.remove(pos);
        }
    }

    fn power(&self) -> usize {
        self.boxes
            .iter()
            .enumerate()
            .flat_map(|(i, entries)| {
                entries
                    .iter()
                    .enumerate()
                    .map(move |(j, entry)| (i + 1) * (j + 1) * entry.value as usize)
            })
            .sum()
    }
}

struct Entry<'a> {
    label: &'a str,
    value: u8,
}

enum Command<'a> {
    Add { label: &'a str, value: u8 },
    Remove { label: &'a str },
}

impl<'a> Command<'a> {
    fn parse(input: &'a str) -> Option<Self> {
        input
            .split_once('=')
            .and_then(|(label, value)| {
                let value = str::parse(value).ok()?;
                Some(Self::Add { label, value })
            })
            .or_else(|| {
                let (label, _) = input.split_once('-')?;
                Some(Self::Remove { label })
            })
    }
}

fn hash(input: &str) -> usize {
    input.bytes().fold(0, |mut acc, b| {
        acc += b as usize;
        acc *= 17;
        acc % 256
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1320));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(145));
    }
}
