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
    let input = include_str!("input.txt");
    let output = part1(input);
    println!("{}", output);
}

fn part1(input: &str) -> u32 {
    let ParseOutput {
        starts,
        instructions,
        network,
    } = parse(input, |n| n == "AAA", |n| n == "ZZZ").unwrap().1;

    let mut position = starts[0];
    let mut step_count = 0;

    for instruction in instructions.into_iter().cycle() {
        step_count += 1;

        let (left, right) = network[position].1;

        position = match instruction {
            Instruction::Left => left,
            Instruction::Right => right,
        };

        if network[position].0 {
            break;
        }
    }

    step_count
}

struct ParseOutput {
    starts: Vec<usize>,
    instructions: Vec<Instruction>,
    network: Vec<(bool, (usize, usize))>,
}

fn parse<F, G>(input: &str, mut is_start: F, mut is_end: G) -> IResult<&str, ParseOutput>
where
    F: FnMut(&str) -> bool,
    G: FnMut(&str) -> bool,
{
    let (input, instructions): (_, Vec<Instruction>) = parse_instructions(input)?;
    let (input, name_network) = separated_list1(line_ending, parse_link)(input)?;

    let mut name_map = HashMap::new();
    let mut starts = Vec::new();

    for (i, (name, (_, _))) in name_network.iter().enumerate() {
        name_map.insert(*name, i);

        if is_start(name) {
            starts.push(i)
        }
    }

    let network = name_network
        .into_iter()
        .map(|(name, (left, right))| {
            (
                is_end(name),
                (
                    *name_map.get(&left).unwrap(),
                    *name_map.get(&right).unwrap(),
                ),
            )
        })
        .collect();

    Ok((
        input,
        ParseOutput {
            starts,
            instructions,
            network,
        },
    ))
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
