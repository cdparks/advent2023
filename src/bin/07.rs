use sscanf::sscanf;

advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<u32> {
    run(Mode::Jack, input)
}

pub fn part_two(input: &str) -> Option<u32> {
    run(Mode::Joker, input)
}

fn run(mode: Mode, input: &str) -> Option<u32> {
    let mut hands = input
        .lines()
        .map(|line| Hand::parse(mode, line))
        .collect::<Option<Vec<Hand>>>()?;
    hands.sort();
    let score = hands
        .into_iter()
        .enumerate()
        .map(|(i, hand)| (i + 1) as u32 * hand.bid)
        .sum();
    Some(score)
}

#[derive(PartialEq, PartialOrd, Eq, Ord, Clone, Copy, Debug)]
struct Hand {
    kind: Kind,
    cards: [u8; 5],
    bid: u32,
}

impl Hand {
    fn parse(mode: Mode, input: &str) -> Option<Self> {
        let (cards, bid) = sscanf!(input, "{} {}", String, u32).ok()?;
        let cards = cards
            .chars()
            .flat_map(|c| parse_card(mode, c))
            .collect::<Vec<_>>()
            .try_into()
            .ok()?;
        let kind = categorize(cards);
        Some(Self { kind, cards, bid })
    }
}

#[allow(clippy::enum_variant_names)]
#[derive(PartialEq, PartialOrd, Eq, Ord, Clone, Copy, Debug)]
enum Kind {
    HighCard,
    OnePair,
    TwoPairs,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

fn categorize(cards: [u8; 5]) -> Kind {
    let mut hist = [0; 15];
    cards.iter().copied().for_each(|card| {
        hist[card as usize] += 1;
    });

    if hist[JOKER] > 0 {
        let card = hist
            .iter()
            .enumerate()
            .filter(|(card, _)| *card != JOKER)
            .max_by_key(|(_, count)| *count)
            .map(|(card, _)| card)
            .unwrap_or(JOKER);

        if card != JOKER {
            hist[card] += hist[JOKER];
            hist[JOKER] = 0;
        }
    }

    let mut counts = hist
        .into_iter()
        .filter(|count| *count != 0)
        .collect::<Vec<_>>();
    counts.sort();

    match &counts[..] {
        [5] => Kind::FiveOfAKind,
        [1, 4] => Kind::FourOfAKind,
        [2, 3] => Kind::FullHouse,
        [1, 1, 3] => Kind::ThreeOfAKind,
        [1, 2, 2] => Kind::TwoPairs,
        [1, 1, 1, 2] => Kind::OnePair,
        _ => Kind::HighCard,
    }
}

const JOKER: usize = 0;
const JACK: usize = 11;

fn parse_card(mode: Mode, c: char) -> Option<u8> {
    let card = match c {
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        'T' => 10,
        'J' => match mode {
            Mode::Joker => JOKER as u8,
            Mode::Jack => JACK as u8,
        },
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => return None,
    };
    Some(card)
}

#[derive(Clone, Copy)]
enum Mode {
    Jack,
    Joker,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6440));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5905));
    }
}
