use nom::{
    bytes::complete::tag,
    character::complete::{self, digit1, space0, space1},
    multi::separated_list1,
    sequence::tuple,
    IResult,
};

pub fn parse_line(line: &str) -> IResult<&str, (Vec<u8>, Vec<u8>)> {
    let (line, _) = tuple((tag("Card "), space0, digit1, tag(":"), space1))(line)?;

    let (line, winning) = separated_list1(space1, complete::u8)(line)?;
    let (line, _) = tuple((tag(" |"), space1))(line)?;
    let (line, our) = separated_list1(space1, complete::u8)(line)?;

    Ok((line, (winning, our)))
}
