use nom::{
    bytes::complete::{is_a, tag},
    character::complete,
    sequence::separated_pair,
    IResult,
};
use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("File not found");

    println!("{}", part_1(&input));
    println!("{}", part_2(&input));
}

#[derive(Clone)]
enum Direction {
    Up,
    Left,
    Down,
    Right,
}

fn parse_line(input: &str) -> IResult<&str, Vec<Direction>> {
    let (input, (direction, count)) = separated_pair(is_a("ULDR"), tag(" "), complete::u32)(input)?;
    let instruction = match direction {
        "U" => Some(Direction::Up),
        "L" => Some(Direction::Left),
        "D" => Some(Direction::Down),
        "R" => Some(Direction::Right),
        _ => None,
    };

    Ok((input, vec![instruction.unwrap(); count as usize]))
}

fn part_1(input: &str) -> usize {
    let instructions = input
        .lines()
        .map(|x| parse_line(x).unwrap().1)
        .fold(vec![], |acc, x| [acc, x].concat());
    let mut head = (0, 0);
    let mut tail = (0, 0);
    let mut past_positions = vec![(0, 0)];

    for instruction in instructions {
        match instruction {
            Direction::Up => head.1 += 1,
            Direction::Left => head.0 -= 1,
            Direction::Down => head.1 -= 1,
            Direction::Right => head.0 += 1,
        }

        let dx: i32 = head.0 - tail.0;
        let dy: i32 = head.1 - tail.1;

        if dx * dx + dy * dy > 2 {
            tail.0 += dx.signum();
            tail.1 += dy.signum();
        }

        if !past_positions.contains(&tail) {
            past_positions.push(tail);
        }
    }

    past_positions.len()
}

fn part_2(input: &str) -> usize {
    let instructions = input
        .lines()
        .map(|x| parse_line(x).unwrap().1)
        .fold(vec![], |acc, x| [acc, x].concat());
    let mut knots = vec![(0, 0); 10];
    let mut past_positions = vec![(0, 0)];

    for instruction in instructions {
        match instruction {
            Direction::Up => knots[0].1 += 1,
            Direction::Left => knots[0].0 -= 1,
            Direction::Down => knots[0].1 -= 1,
            Direction::Right => knots[0].0 += 1,
        }

        for knot_num in 1..knots.len() {
            let dx: i32 = knots[knot_num - 1].0 - knots[knot_num].0;
            let dy: i32 = knots[knot_num - 1].1 - knots[knot_num].1;

            if dx * dx + dy * dy > 2 {
                knots[knot_num].0 += dx.signum();
                knots[knot_num].1 += dy.signum();
            }
        }

        if !past_positions.contains(&knots[9]) {
            past_positions.push(knots[9]);
        }
    }

    past_positions.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_test() {
        let input = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

        assert_eq!(part_1(input), 13);
    }

    #[test]
    fn part_2_test() {
        let input = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";

        assert_eq!(part_2(input), 36);
    }
}
