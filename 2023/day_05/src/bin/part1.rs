use nom::{
    bytes::complete::tag,
    character::complete::{self, alpha1, line_ending},
    multi::{count, separated_list1},
    sequence::{preceded, separated_pair, tuple},
    IResult,
};

fn main() {
    let input = include_str!("input.txt");
    let output = part1(input);
    println!("{}", output);
}

fn part1(input: &str) -> u32 {
    let (seeds, maps) = parse(input).unwrap().1;

    seeds
        .into_iter()
        .map(|seed| maps.iter().fold(seed, |input, map| map.map(input)))
        .min()
        .unwrap()
}

fn parse(input: &str) -> IResult<&str, (Vec<u32>, Vec<Map>)> {
    let (input, seeds) = preceded(tag("seeds: "), separated_list1(tag(" "), complete::u32))(input)?;
    let input = input.trim();

    let (input, maps) = separated_list1(count(line_ending, 2), parse_map)(input)?;

    Ok((input, (seeds, maps)))
}

fn parse_map(input: &str) -> IResult<&str, Map> {
    let (input, _) = tuple((alpha1, tag("-to-"), alpha1, tag(" map:"), line_ending))(input)?;

    let (input, maps) = separated_list1(line_ending, parse_line)(input)?;

    Ok((input, Map { maps }))
}

fn parse_line(input: &str) -> IResult<&str, (u32, u32, u32)> {
    let (input, (num1, (num2, num3))) = separated_pair(
        complete::u32,
        tag(" "),
        separated_pair(complete::u32, tag(" "), complete::u32),
    )(input)?;

    Ok((input, (num1, num2, num3)))
}

struct Map {
    maps: Vec<(u32, u32, u32)>,
}

impl Map {
    fn map(&self, input: u32) -> u32 {
        for (o_start, i_start, diff) in self.maps.iter() {
            if (input >= *i_start) && (input - i_start < *diff) {
                return o_start + (input - i_start);
            }
        }

        input
    }
}

#[cfg(test)]
mod tests {
    use crate::part1;

    #[test]
    fn test_part1() {
        let input = include_str!("test.txt");
        assert_eq!(part1(input), 35);
    }
}
