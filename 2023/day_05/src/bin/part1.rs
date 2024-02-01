use day_05::*;

fn main() {
    let input = include_str!("input.txt");
    let output = part1(input);
    println!("{}", output);
}

fn part1(input: &str) -> u32 {
    let (input, seeds) = parse_seeds_as_starts(input).unwrap();
    let maps = parse_maps(input).unwrap().1;

    seeds
        .into_iter()
        .map(|seed| maps.iter().fold(seed, |input, map| map.map(input)))
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use crate::part1;

    #[test]
    fn test_part1() {
        let input = include_str!("test.txt");
        assert_eq!(part1(input), 35);
    }
}
