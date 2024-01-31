use nom::{
    bytes::complete::tag,
    character::complete::{self, digit1, space0, space1},
    multi::separated_list1,
    sequence::tuple,
    IResult,
};

fn main() {
    let input = include_str!("input.txt");
    let output = part1(input);
    println!("{}", output);
}

fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let (winning, our) = parse_line(line).unwrap().1;

            (1 << our.iter().filter(|x| winning.contains(x)).count()) >> 1
        })
        .sum()
}

fn parse_line(line: &str) -> IResult<&str, (Vec<u8>, Vec<u8>)> {
    let (line, _) = tuple((tag("Card "), space0, digit1, tag(":"), space1))(line)?;

    let (line, winning) = separated_list1(space1, complete::u8)(line)?;
    let (line, _) = tuple((tag(" |"), space1))(line)?;
    let (line, our) = separated_list1(space1, complete::u8)(line)?;

    Ok((line, (winning, our)))
}

#[cfg(test)]
mod tests {
    use crate::part1;

    #[test]
    fn test_part1() {
        let input = include_str!("test.txt");
        assert_eq!(part1(input), 13);
    }
}