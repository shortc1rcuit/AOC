use std::fs;

use nom::{
    bytes::complete::tag, character::complete, multi::separated_list1, sequence::separated_pair,
    IResult,
};

fn main() {
    let input = fs::read_to_string("input.txt").expect("File not found");

    println!("{}", part_1(&input));
    println!("{}", part_2(&input));
}

fn coordinate(input: &str) -> IResult<&str, (usize, usize)> {
    let (input, (x, y)) = separated_pair(complete::u32, tag(","), complete::u32)(input)?;

    Ok((input, (x as usize, y as usize)))
}

fn line(input: &str) -> IResult<&str, Vec<(usize, usize)>> {
    separated_list1(tag(" -> "), coordinate)(input)
}

#[derive(Clone, Debug)]
enum Direction {
    Up(usize),
    Left(usize),
    Down(usize),
    Right(usize),
}

#[derive(Clone, Debug)]
struct Instruction {
    start: (usize, usize),
    steps: Vec<Direction>,
}

fn pair_to_direction(pair: &[(usize, usize)]) -> Direction {
    let (point_a, point_b) = (pair[0], pair[1]);

    if point_a.0 == point_b.0 {
        if point_a.1 > point_b.1 {
            Direction::Up(point_a.1 - point_b.1)
        } else {
            Direction::Down(point_b.1 - point_a.1)
        }
    } else if point_a.0 > point_b.0 {
        Direction::Left(point_a.0 - point_b.0)
    } else {
        Direction::Right(point_b.0 - point_a.0)
    }
}

fn to_instructions(coordinates: Vec<(usize, usize)>) -> Instruction {
    let start = coordinates[0];
    let steps = coordinates.windows(2).map(pair_to_direction).collect();

    Instruction { start, steps }
}

#[derive(Clone, Debug, PartialEq)]
enum CellState {
    Empty,
    Filled,
}

fn draw_lines(
    instructions: Vec<Instruction>,
    min_x: usize,
    max_x: usize,
    max_y: usize,
) -> Vec<Vec<CellState>> {
    let mut grid = vec![vec![CellState::Empty; max_x - min_x + 3]; max_y + 1];

    for instruction in instructions {
        let (mut x, mut y) = (instruction.start.0 - min_x + 1, instruction.start.1);
        grid[y][x] = CellState::Filled;

        for step in instruction.steps {
            match step {
                Direction::Up(len) => {
                    for _ in 0..len {
                        y -= 1;
                        grid[y][x] = CellState::Filled;
                    }
                }
                Direction::Left(len) => {
                    for _ in 0..len {
                        x -= 1;
                        grid[y][x] = CellState::Filled;
                    }
                }
                Direction::Down(len) => {
                    for _ in 0..len {
                        y += 1;
                        grid[y][x] = CellState::Filled;
                    }
                }
                Direction::Right(len) => {
                    for _ in 0..len {
                        x += 1;
                        grid[y][x] = CellState::Filled;
                    }
                }
            }
        }
    }

    grid
}

fn part_1(input: &str) -> u32 {
    let coordinates = input.lines().map(|x| line(x).unwrap().1);

    let (min_x, max_x, max_y) = coordinates.clone().flatten().fold(
        (usize::MAX, usize::MIN, usize::MIN),
        |(a_x_min, a_x_max, a_y_max), (b_x, b_y)| {
            (a_x_min.min(b_x), a_x_max.max(b_x), a_y_max.max(b_y))
        },
    );

    let instructions: Vec<Instruction> = coordinates.map(to_instructions).collect();

    let mut grid = draw_lines(instructions, min_x, max_x, max_y);
    let mut sand_count = 0;

    'sim_sand: loop {
        let mut sand = (501 - min_x, 0);

        loop {
            if grid[sand.1 + 1][sand.0] == CellState::Empty {
                sand.1 += 1;
            } else if grid[sand.1 + 1][sand.0 - 1] == CellState::Empty {
                sand.0 -= 1;
                sand.1 += 1;
            } else if grid[sand.1 + 1][sand.0 + 1] == CellState::Empty {
                sand.0 += 1;
                sand.1 += 1;
            } else {
                grid[sand.1][sand.0] = CellState::Filled;
                break;
            }

            if sand.1 == max_y {
                break 'sim_sand;
            }
        }

        sand_count += 1;
    }

    sand_count
}

fn part_2(input: &str) -> u32 {
    let coordinates = input.lines().map(|x| line(x).unwrap().1);

    let (min_x, max_x, max_y) = coordinates.clone().flatten().fold(
        (usize::MAX, usize::MIN, usize::MIN),
        |(a_x_min, a_x_max, a_y_max), (b_x, b_y)| {
            (a_x_min.min(b_x), a_x_max.max(b_x), a_y_max.max(b_y))
        },
    );

    let instructions: Vec<Instruction> = coordinates.map(to_instructions).collect();

    //Can't be be bothered to properly figure out how much room to give the array on either side
    //so I'm doing this instead
    //If the program crashes increase the buffer
    const BUFFER: usize = 400;

    let mut grid = draw_lines(instructions, min_x - BUFFER, max_x + BUFFER, max_y + 2);
    let mut sand_count = 0;

    loop {
        let mut sand = (501 - min_x + BUFFER, 0);

        loop {
            if sand.1 == max_y + 1 {
                grid[sand.1][sand.0] = CellState::Filled;
                break;
            } else if grid[sand.1 + 1][sand.0] == CellState::Empty {
                sand.1 += 1;
            } else if grid[sand.1 + 1][sand.0 - 1] == CellState::Empty {
                sand.0 -= 1;
                sand.1 += 1;
            } else if grid[sand.1 + 1][sand.0 + 1] == CellState::Empty {
                sand.0 += 1;
                sand.1 += 1;
            } else {
                grid[sand.1][sand.0] = CellState::Filled;
                break;
            }
        }

        sand_count += 1;

        if sand == (501 - min_x + BUFFER, 0) {
            break;
        }
    }

    sand_count
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9
";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(INPUT), 24)
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(INPUT), 93)
    }
}
