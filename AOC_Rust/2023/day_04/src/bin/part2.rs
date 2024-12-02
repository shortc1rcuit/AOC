use day_04::parse_line;

fn main() {
    let input = include_str!("input.txt");
    let output = part2(input);
    println!("{}", output);
}

fn part2(input: &str) -> u32 {
    let mut counts = vec![1; input.lines().count()];

    for (i, (winning, our)) in input
        .lines()
        .map(|line| parse_line(line).unwrap().1)
        .enumerate()
    {
        let matching = our.iter().filter(|x| winning.contains(x)).count();

        for index in i + 1..=i + matching {
            counts[index] += counts[i]
        }
    }

    counts.iter().sum()
}

#[cfg(test)]
mod tests {
    use crate::part2;

    #[test]
    fn test_part1() {
        let input = include_str!("test.txt");
        assert_eq!(part2(input), 30);
    }
}
