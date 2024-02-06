use nom::{
    bytes::complete::tag,
    character::complete::{self, one_of},
    combinator::map,
    multi::count,
    sequence::separated_pair,
    IResult,
};

fn main() {
    let input = include_str!("input.txt");
    let output = part1(input);
    println!("{}", output);
}

fn part1(input: &str) -> u32 {
    let mut hands = input
        .lines()
        .map(|l| parse(l).unwrap().1)
        .collect::<Vec<_>>();
    hands.sort_by_key(|h| h.0);
    hands
        .into_iter()
        .enumerate()
        .map(|(rank, (_, bid))| (rank as u32 + 1) * bid)
        .sum()
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl TryFrom<char> for Card {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '2' => Ok(Card::Two),
            '3' => Ok(Card::Three),
            '4' => Ok(Card::Four),
            '5' => Ok(Card::Five),
            '6' => Ok(Card::Six),
            '7' => Ok(Card::Seven),
            '8' => Ok(Card::Eight),
            '9' => Ok(Card::Nine),
            'T' => Ok(Card::Ten),
            'J' => Ok(Card::Jack),
            'Q' => Ok(Card::Queen),
            'K' => Ok(Card::King),
            'A' => Ok(Card::Ace),
            _ => Err("Invalid char. Valid chars are A23456789TJQK"),
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct Hand {
    hand_type: HandType,
    cards: [Card; 5],
}

impl Hand {
    fn new(cards: [Card; 5]) -> Self {
        let mut card_freq = [0_u8; 13];

        for card in &cards {
            card_freq[*card as usize] += 1;
        }

        let hand_type = if card_freq.contains(&5) {
            HandType::FiveOfAKind
        } else if card_freq.contains(&4) {
            HandType::FourOfAKind
        } else if card_freq.contains(&3) {
            if card_freq.contains(&2) {
                HandType::FullHouse
            } else {
                HandType::ThreeOfAKind
            }
        } else {
            let two_count = card_freq.iter().filter(|c| c == &&2).count();

            if two_count == 2 {
                HandType::TwoPair
            } else if two_count == 1 {
                HandType::OnePair
            } else {
                HandType::HighCard
            }
        };

        Hand { hand_type, cards }
    }
}

fn parse(input: &str) -> IResult<&str, (Hand, u32)> {
    separated_pair(parse_hand, tag(" "), complete::u32)(input)
}

fn parse_hand(input: &str) -> IResult<&str, Hand> {
    map(count(one_of("A23456789TJQK"), 5), |v| {
        Hand::new([
            v[0].try_into().unwrap(),
            v[1].try_into().unwrap(),
            v[2].try_into().unwrap(),
            v[3].try_into().unwrap(),
            v[4].try_into().unwrap(),
        ])
    })(input)
}

#[cfg(test)]
mod tests {
    use std::cmp::Ordering;

    use crate::{part1, Card, Hand};

    #[test]
    fn test_part1() {
        let input = include_str!("test.txt");
        assert_eq!(part1(input), 6440);
    }

    #[test]
    fn test_hand_ord() {
        let hand1 = Hand::new([
            Card::Seven,
            Card::Seven,
            Card::Seven,
            Card::Eight,
            Card::Eight,
        ]);
        let hand2 = Hand::new([
            Card::Seven,
            Card::Seven,
            Card::Eight,
            Card::Eight,
            Card::Eight,
        ]);
        assert_eq!(hand1.cmp(&hand2), Ordering::Less)
    }
}