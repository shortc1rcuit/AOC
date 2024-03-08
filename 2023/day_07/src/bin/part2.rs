use day_07::*;

fn main() {
    let input = include_str!("input.txt");
    let output = part2(input);
    println!("{}", output);
}

fn part2(input: &str) -> u32 {
    calc_winnings::<JokerCard>(input)
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum JokerCard {
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Queen,
    King,
    Ace,
}

impl Card for JokerCard {
    const CARD_CHARS: &'static str = "J23456789TQKA";

    fn get_frequencies(cards: &[Self; 5]) -> [u8; 13]
    where
        Self: Sized,
    {
        let mut card_freq = [0_u8; 13];

        for card in cards {
            card_freq[*card as usize] += 1;
        }

        let joker_count = card_freq[0];
        card_freq[0] = 0;
        let max_index = card_freq
            .iter()
            .enumerate()
            .max_by_key(|(_, x)| *x)
            .unwrap()
            .0;
        card_freq[max_index] += joker_count;

        card_freq
    }
}

impl TryFrom<char> for JokerCard {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'J' => Ok(JokerCard::Joker),
            '2' => Ok(JokerCard::Two),
            '3' => Ok(JokerCard::Three),
            '4' => Ok(JokerCard::Four),
            '5' => Ok(JokerCard::Five),
            '6' => Ok(JokerCard::Six),
            '7' => Ok(JokerCard::Seven),
            '8' => Ok(JokerCard::Eight),
            '9' => Ok(JokerCard::Nine),
            'T' => Ok(JokerCard::Ten),
            'Q' => Ok(JokerCard::Queen),
            'K' => Ok(JokerCard::King),
            'A' => Ok(JokerCard::Ace),
            x => Err(format!(
                "{} is an invalid char. Valid chars are {}",
                x,
                JokerCard::CARD_CHARS
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::part2;

    #[test]
    fn test_part2() {
        let input = include_str!("test.txt");
        assert_eq!(part2(input), 5905);
    }
}
