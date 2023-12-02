use day_02::*;

fn main() {
    let input = include_str!("input.txt");
    let output = part1(input);
    println!("{}", output);
}

fn part1(input: &str) -> u16 {
    input
        .lines()
        .map(|game| parse_game(game).unwrap().1)
        .filter(|(_, game)| {
            game.iter()
                .all(|set| set.red <= 12 && set.green <= 13 && set.blue <= 14)
        })
        .map(|(x, _)| x)
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::part1;

    #[test]
    fn test_part1() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green\nGame 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue\nGame 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red\nGame 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red\nGame 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!(part1(input), 8);
    }
}
