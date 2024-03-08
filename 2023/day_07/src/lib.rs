use nom::{
    bytes::complete::tag,
    character::complete::{self, one_of},
    combinator::map,
    multi::count,
    sequence::separated_pair,
    IResult,
};

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl HandType {
    pub fn new<C: Card>(cards: &[C; 5]) -> HandType {
        let card_freq = C::get_frequencies(cards);

        if card_freq.contains(&5) {
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
        }
    }
}

pub trait Card {
    const CARD_CHARS: &'static str;

    fn get_frequencies(cards: &[Self; 5]) -> [u8; 13]
    where
        Self: Sized;
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Hand<C: Card> {
    hand_type: HandType,
    cards: [C; 5],
}

impl<C: Card> Hand<C> {
    pub fn new(cards: [C; 5]) -> Self {
        Hand {
            hand_type: HandType::new(&cards),
            cards,
        }
    }
}

pub fn parse<C>(input: &str) -> IResult<&str, (Hand<C>, u32)>
where
    C: Card + TryFrom<char, Error = String>,
{
    separated_pair(parse_hand, tag(" "), complete::u32)(input)
}

pub fn parse_hand<C: Card>(input: &str) -> IResult<&str, Hand<C>>
where
    C: Card + TryFrom<char, Error = String>,
{
    map(count(one_of(C::CARD_CHARS), 5), |v| {
        Hand::new([
            v[0].try_into().unwrap(),
            v[1].try_into().unwrap(),
            v[2].try_into().unwrap(),
            v[3].try_into().unwrap(),
            v[4].try_into().unwrap(),
        ])
    })(input)
}

pub fn calc_winnings<C>(input: &str) -> u32
where
    C: Card + TryFrom<char, Error = String> + Ord + Clone,
{
    let mut hands = input
        .lines()
        .map(|l| parse::<C>(l).unwrap().1)
        .collect::<Vec<_>>();
    hands.sort_by_key(|h| h.0.clone());
    hands
        .into_iter()
        .enumerate()
        .map(|(rank, (_, bid))| (rank as u32 + 1) * bid)
        .sum()
}
