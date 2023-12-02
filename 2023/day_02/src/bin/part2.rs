use day_02::*;

fn main() {
    let input = include_str!("input.txt");
    let output = part2(input);
    println!("{}", output);
}

fn part2(input: &str) -> u32 {
    input
        .lines()
        .map(|game| {
            parse_game(game)
                .unwrap()
                .1
                .1
                .into_iter()
                .reduce(|acc, e| Set {
                    red: acc.red.max(e.red),
                    green: acc.green.max(e.green),
                    blue: acc.blue.max(e.blue),
                })
                .unwrap()
        })
        .map(|set| set.red * set.green * set.blue)
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::part2;

    #[test]
    fn test_part1() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green\nGame 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue\nGame 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red\nGame 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red\nGame 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!(part2(input), 2286);
    }
}