use nom::{
    bytes::complete::tag,
    character::complete::{self, alpha1, line_ending},
    multi::{count, separated_list1},
    sequence::{preceded, separated_pair, tuple},
    IResult,
};

pub fn parse_seeds_as_starts(input: &str) -> IResult<&str, Vec<u32>> {
    let (input, seeds) = preceded(tag("seeds: "), separated_list1(tag(" "), complete::u32))(input)?;
    let input = input.trim();

    Ok((input, seeds))
}

pub fn parse_maps(input: &str) -> IResult<&str, Vec<Map>> {
    let (input, maps) = separated_list1(count(line_ending, 2), parse_map)(input)?;

    Ok((input, maps))
}

fn parse_map(input: &str) -> IResult<&str, Map> {
    let (input, _) = tuple((alpha1, tag("-to-"), alpha1, tag(" map:"), line_ending))(input)?;

    let (input, map_parts) = separated_list1(line_ending, parse_line)(input)?;

    Ok((input, Map::new(map_parts)))
}

fn parse_line(input: &str) -> IResult<&str, (Range, u32)> {
    let (input, (num1, (num2, num3))) = separated_pair(
        complete::u32,
        tag(" "),
        separated_pair(complete::u32, tag(" "), complete::u32),
    )(input)?;

    Ok((input, (Range::new(num2, num3), num1)))
}

pub struct Map {
    map_parts: Vec<(Range, u32)>,
}

impl Map {
    pub fn new(mut map_parts: Vec<(Range, u32)>) -> Map {
        map_parts.sort_by_key(|k| k.0.start);
        Map { map_parts }
    }

    pub fn map(&self, input: u32) -> u32 {
        for (i_range, o_start) in self.map_parts.iter() {
            if i_range.contains(input) {
                return o_start + (input - i_range.start);
            }
        }

        input
    }
}

pub struct Range {
    start: u32,
    size: u32,
}

impl Range {
    fn new(start: u32, size: u32) -> Range {
        Range { start, size }
    }

    fn contains(&self, x: u32) -> bool {
        (x >= self.start) && (x - self.start < self.size)
    }
}
