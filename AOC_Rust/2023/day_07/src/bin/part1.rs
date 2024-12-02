use day_07::*;

fn main() {
    let input = include_str!("input.txt");
    let output = part1(input);
    println!("{}", output);
}

fn part1(input: &str) -> u32 {
    calc_winnings::<NormalCard>(input)
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum NormalCard {
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

impl Card for NormalCard {
    const CARD_CHARS: &'static str = "23456789TJQKA";

    fn get_frequencies(cards: &[Self; 5]) -> [u8; 13]
    where
        Self: Sized,
    {
        let mut card_freq = [0_u8; 13];

        for card in cards {
            card_freq[*card as usize] += 1;
        }

        card_freq
    }
}

impl TryFrom<char> for NormalCard {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '2' => Ok(NormalCard::Two),
            '3' => Ok(NormalCard::Three),
            '4' => Ok(NormalCard::Four),
            '5' => Ok(NormalCard::Five),
            '6' => Ok(NormalCard::Six),
            '7' => Ok(NormalCard::Seven),
            '8' => Ok(NormalCard::Eight),
            '9' => Ok(NormalCard::Nine),
            'T' => Ok(NormalCard::Ten),
            'J' => Ok(NormalCard::Jack),
            'Q' => Ok(NormalCard::Queen),
            'K' => Ok(NormalCard::King),
            'A' => Ok(NormalCard::Ace),
            x => Err(format!(
                "{} is an invalid char. Valid chars are {}",
                x,
                NormalCard::CARD_CHARS
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::cmp::Ordering;

    use crate::{part1, Hand, NormalCard};

    #[test]
    fn test_part1() {
        let input = include_str!("test.txt");
        assert_eq!(part1(input), 6440);
    }

    #[test]
    fn test_hand_ord() {
        let hand1 = Hand::new([
            NormalCard::Seven,
            NormalCard::Seven,
            NormalCard::Seven,
            NormalCard::Eight,
            NormalCard::Eight,
        ]);
        let hand2 = Hand::new([
            NormalCard::Seven,
            NormalCard::Seven,
            NormalCard::Eight,
            NormalCard::Eight,
            NormalCard::Eight,
        ]);
        assert_eq!(hand1.cmp(&hand2), Ordering::Less)
    }
}
