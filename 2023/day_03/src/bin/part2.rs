use day_03::*;

use std::collections::HashMap;

fn main() {
    let input = include_str!("input.txt");
    let output = part2(input);
    println!("{}", output);
}

fn part2(input: &str) -> u32 {
    let mut numbers = HashMap::new();
    let mut gears: Vec<(i16, i16)> = Vec::new();

    let mut state = ParseState::Else;
    let mut id = 0;

    for (y, line) in input
        .lines()
        .enumerate()
        .map(|(y, line)| (y.try_into().unwrap(), line))
    {
        let mut x = 0;

        for c in line.chars() {
            state = if let Some(digit) = c.to_digit(10) {
                match state {
                    ParseState::Number(start, number) => {
                        ParseState::Number(start, 10 * number + digit)
                    }
                    ParseState::Else => ParseState::Number(x, digit),
                }
            } else {
                if let ParseState::Number(start, number) = state {
                    for x in start..x {
                        numbers.insert((x, y), (number, id));
                    }

                    id += 1;
                }

                if c == '*' {
                    gears.push((x, y));
                }

                ParseState::Else
            };

            x += 1;
        }

        if let ParseState::Number(start, number) = state {
            for x in start..x {
                numbers.insert((x, y), (number, id));
            }

            id += 1;

            state = ParseState::Else;
        }
    }

    let mut sum = 0;

    for (gear_x, gear_y) in gears {
        let mut neighbours = Vec::new();

        for neighbour in NEIGHBOURS
            .iter()
            .filter_map(|neighbour| numbers.get(&(gear_x + neighbour.0, gear_y + neighbour.1)))
        {
            if !neighbours.contains(neighbour) {
                neighbours.push(*neighbour)
            }
        }

        if neighbours.len() == 2 {
            sum += neighbours[0].0 * neighbours[1].0
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use crate::part2;

    #[test]
    fn test_part2() {
        let input = include_str!("test.txt");
        assert_eq!(part2(input), 467835);
    }
}
