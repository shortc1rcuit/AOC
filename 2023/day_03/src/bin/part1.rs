use std::collections::HashSet;

fn main() {
    let input = include_str!("input.txt");
    let output = part1(input);
    println!("{}", output);
}

const NEIGHBOURS: [(i16, i16); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
    (1, 0),
    (1, -1),
    (0, -1),
];

fn part1(input: &str) -> u32 {
    let mut symbols: HashSet<(i16, i16)> = HashSet::new();
    let mut numbers = Vec::new();

    let mut state = ParseState::Else;

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
                    numbers.push(((start..x, y), number));
                }

                if c != '.' {
                    symbols.insert((x, y));
                }

                ParseState::Else
            };

            x += 1;
        }

        if let ParseState::Number(start, number) = state {
            numbers.push(((start..x, y), number));

            state = ParseState::Else;
        }
    }

    let mut sum = 0;

    //Filter only gives a shared reference to x_range, but we need to mutate it so we can
    //iterate over it. This is why we use a for loop.
    for ((mut x_range, y), number) in numbers {
        if x_range.any(|x| {
            NEIGHBOURS
                .iter()
                .any(|neighbor| symbols.contains(&(x + neighbor.0, y + neighbor.1)))
        }) {
            sum += number;
        }
    }

    sum
}

enum ParseState {
    Number(i16, u32),
    Else,
}

#[cfg(test)]
mod tests {
    use crate::part1;

    #[test]
    fn test_part1() {
        let input = "467..114..\n...*......\n..35..633.\n......#...\n617*......\n.....+.58.\n..592.....\n......755.\n...$.*....\n.664.598..";
        assert_eq!(part1(input), 4361);
    }
}
