use day_05::*;

fn main() {
    let input = include_str!("input.txt");
    let output = part2(input);
    println!("{}", output);
}

fn part2(input: &str) -> u32 {
    let (input, seeds) = parse_seeds_as_ranges(input).unwrap();
    let maps = parse_maps(input).unwrap().1;

    seeds
        .into_iter()
        .map(|seed| {
            maps.iter()
                .fold(vec![seed], |acc, map| {
                    acc.into_iter()
                        .flat_map(|seed| map.map_range(seed))
                        .collect()
                })
                .into_iter()
                .map(|seed| seed.start)
                .min()
                .unwrap()
        })
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use crate::part2;

    #[test]
    fn test_part2() {
        let input = include_str!("test.txt");
        assert_eq!(part2(input), 46);
    }
}
