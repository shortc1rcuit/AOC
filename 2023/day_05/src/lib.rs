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

pub fn parse_seeds_as_ranges(input: &str) -> IResult<&str, Vec<Range>> {
    let (input, seeds) = preceded(tag("seeds: "), separated_list1(tag(" "), parse_range))(input)?;
    let input = input.trim();

    Ok((input, seeds))
}

fn parse_range(input: &str) -> IResult<&str, Range> {
    let (input, (start, size)) = separated_pair(complete::u32, tag(" "), complete::u32)(input)?;

    Ok((input, Range::new(start, size)))
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

#[derive(Debug)]
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

    pub fn map_range(&self, mut input: Range) -> Vec<Range> {
        let mut ranges = Vec::new();

        for (i_range, o_start) in self.map_parts.iter() {
            match i_range.start.cmp(&input.start) {
                std::cmp::Ordering::Less => {
                    if i_range.contains(input.start) {
                        ranges.push(Range::new(
                            o_start + (input.start - i_range.start),
                            input.size.min(i_range.size - (input.start - i_range.start)),
                        ));

                        input.size -= input.size.min(i_range.size - (input.start - i_range.start));

                        if input.size == 0 {
                            return ranges;
                        }

                        input.start = i_range.start + i_range.size;
                    }
                }
                std::cmp::Ordering::Equal => {
                    if input.size > i_range.size {
                        ranges.push(Range::new(*o_start, i_range.size));

                        input.start += i_range.size;
                        input.size -= i_range.size;
                    } else {
                        ranges.push(Range::new(*o_start, input.size));

                        return ranges;
                    }
                }
                std::cmp::Ordering::Greater => {
                    if input.contains(i_range.start) {
                        ranges.push(Range::new(input.start, i_range.start - input.start));
                        input.start = i_range.start;
                        input.size -= i_range.start - input.start;

                        if input.size > i_range.size {
                            ranges.push(Range::new(*o_start, i_range.size));

                            input.start += i_range.size;
                            input.size -= i_range.size;
                        } else {
                            ranges.push(Range::new(*o_start, input.size));

                            return ranges;
                        }
                    } else {
                        break;
                    }
                }
            }
        }

        if input.size != 0 {
            ranges.push(input);
        }

        ranges
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Range {
    pub start: u32,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        let input = Range::new(79, 14);
        let map = dbg!(Map::new(vec![
            (Range::new(98, 2), 50),
            (Range::new(50, 48), 52)
        ]));
        assert_eq!(map.map_range(input), vec![Range::new(81, 14)]);
    }
}
