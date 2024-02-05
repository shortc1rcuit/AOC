use nom::{
    bytes::complete::tag,
    character::complete::{self, line_ending, space1},
    multi::separated_list1,
    sequence::{delimited, pair, preceded},
    IResult,
};

fn main() {
    let input = include_str!("input.txt");
    let output = part1(input);
    println!("{}", output);
}

fn part1(input: &str) -> u32 {
    //We're trying to solve the inequality t(s-t) > d
    //(Where t = time held, s = total time allowed and d = distance to beat)
    //This rearranges to t^2 - st + d < 0, which we can solve using the quadratic formula
    let (times, distances) = parse(input).unwrap().1;

    times
        .into_iter()
        .zip(distances)
        .map(|(time, distance)| {
            let (time, distance) = (time as f64, distance as f64);

            //The roots of the quadratic
            let root1 = (time - (time.powf(2.0) - 4.0 * distance).sqrt()) / 2.0;
            let root2 = (time + (time.powf(2.0) - 4.0 * distance).sqrt()) / 2.0;

            //The closest integers in the exclusive range root1..root2
            let lower = if root1.fract() == 0.0 {
                (root1 as u32) + 1
            } else {
                root1.ceil() as u32
            };

            let upper = if root2.fract() == 0.0 {
                (root2 as u32) - 1
            } else {
                root2.floor() as u32
            };

            upper - lower + 1
        })
        .product()
}

fn parse(input: &str) -> IResult<&str, (Vec<u32>, Vec<u32>)> {
    let (input, times) = delimited(
        pair(tag("Time:"), space1),
        separated_list1(space1, complete::u32),
        line_ending,
    )(input)?;
    let (input, distances) = preceded(
        pair(tag("Distance:"), space1),
        separated_list1(space1, complete::u32),
    )(input)?;

    Ok((input, (times, distances)))
}

#[cfg(test)]
mod tests {
    use crate::part1;

    #[test]
    fn test_part1() {
        let input = include_str!("test.txt");
        assert_eq!(part1(input), 288);
    }
}
