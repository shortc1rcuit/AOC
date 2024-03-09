use std::collections::HashMap;

use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, line_ending, one_of},
    combinator,
    multi::{count, many1, separated_list1},
    sequence::{delimited, separated_pair, terminated},
    IResult,
};

fn main() {
    let input = include_str!("test1.txt");
    let output = part1(input);
    println!("{}", output);
}

fn part1(input: &str) -> u32 {
    let (instructions, network) = parse(input).unwrap().1;

    let mut position = "AAA";
    let mut step_count = 0;

    for instruction in instructions.into_iter().cycle() {
        step_count += 1;

        let (left, right) = network.get(position).unwrap();

        position = match instruction {
            Instruction::Left => left,
            Instruction::Right => right,
        };

        if position == "ZZZ" {
            break;
        }
    }

    step_count
}

type ParseOutput<'a> = IResult<&'a str, (Vec<Instruction>, HashMap<&'a str, (&'a str, &'a str)>)>;

fn parse(input: &str) -> ParseOutput {
    let (input, instructions): (_, Vec<Instruction>) = parse_instructions(input)?;

    let (input, network) =
        combinator::map(separated_list1(line_ending, parse_link), |v: Vec<_>| {
            v.into_iter().collect::<HashMap<_, _>>()
        })(input)?;

    Ok((input, (instructions, network)))
}

fn parse_instructions(input: &str) -> IResult<&str, Vec<Instruction>> {
    terminated(
        many1(combinator::map(one_of("LR"), |c| c.try_into().unwrap())),
        count(line_ending, 2),
    )(input)
}

fn parse_link(input: &str) -> IResult<&str, (&str, (&str, &str))> {
    separated_pair(
        alpha1,
        tag(" = "),
        delimited(
            tag("("),
            separated_pair(alpha1, tag(", "), alpha1),
            tag(")"),
        ),
    )(input)
}

#[derive(Clone, Copy)]
enum Instruction {
    Left,
    Right,
}

impl TryFrom<char> for Instruction {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'L' => Ok(Instruction::Left),
            'R' => Ok(Instruction::Right),
            c => Err(format!(
                "{} is not a valid Instruction. Valid instructions are \"L\" and \"R\".",
                c
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::part1;

    #[test]
    fn test_part1_1() {
        let input = include_str!("test1.txt");
        assert_eq!(part1(input), 2);
    }

    #[test]
    fn test_part1_2() {
        let input = include_str!("test2.txt");
        assert_eq!(part1(input), 6);
    }
}