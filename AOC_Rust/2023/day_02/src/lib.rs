use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete,
    multi::separated_list1,
    sequence::{separated_pair, tuple},
    IResult,
};

pub fn parse_game(game: &str) -> IResult<&str, (u16, Vec<Set>)> {
    let (game, (_, game_num, _)) = tuple((tag("Game "), complete::u16, tag(": ")))(game)?;
    let (game, sets) = separated_list1(tag("; "), parse_set)(game)?;

    Ok((game, (game_num, sets)))
}

fn parse_set(set_str: &str) -> IResult<&str, Set> {
    let mut set = Set::default();

    let (set_str, cubes) = separated_list1(tag(", "), parse_cube)(set_str)?;

    for (count, colour) in cubes {
        if colour == "red" {
            set.red = count;
        } else if colour == "green" {
            set.green = count;
        } else if colour == "blue" {
            set.blue = count;
        }
    }

    Ok((set_str, set))
}

fn parse_cube(cube_str: &str) -> IResult<&str, (u32, &str)> {
    separated_pair(
        complete::u32,
        complete::char(' '),
        alt((tag("red"), tag("green"), tag("blue"))),
    )(cube_str)
}

#[derive(Default)]
pub struct Set {
    pub red: u32,
    pub green: u32,
    pub blue: u32,
}
