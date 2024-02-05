use nom::{
    bytes::complete::tag,
    character::complete::{self, line_ending, space1},
    multi::separated_list1,
    sequence::{delimited, pair, preceded},
    IResult,
};

pub fn parse(input: &str) -> IResult<&str, (Vec<u64>, Vec<u64>)> {
    let (input, times) = delimited(
        pair(tag("Time:"), space1),
        separated_list1(space1, complete::u64),
        line_ending,
    )(input)?;
    let (input, distances) = preceded(
        pair(tag("Distance:"), space1),
        separated_list1(space1, complete::u64),
    )(input)?;

    Ok((input, (times, distances)))
}

pub fn count_solutions(time: f64, distance: f64) -> u64 {
    //We're trying to solve the inequality t(s-t) > d
    //(Where t = time held, s = total time allowed and d = distance to beat)
    //This rearranges to t^2 - st + d < 0, which we can solve using the quadratic formula

    //The roots of the quadratic
    let root1 = (time - (time.powf(2.0) - 4.0 * distance).sqrt()) / 2.0;
    let root2 = (time + (time.powf(2.0) - 4.0 * distance).sqrt()) / 2.0;

    //The closest integers in the exclusive range root1..root2
    let lower = if root1.fract() == 0.0 {
        (root1 as u64) + 1
    } else {
        root1.ceil() as u64
    };

    let upper = if root2.fract() == 0.0 {
        (root2 as u64) - 1
    } else {
        root2.floor() as u64
    };

    upper - lower + 1
}