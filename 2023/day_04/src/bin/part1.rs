use day_04::*;

fn main() {
    let input = include_str!("input.txt");
    let output = part1(input);
    println!("{}", output);
}

fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let (winning, our) = parse_line(line).unwrap().1;

            (1 << our.iter().filter(|x| winning.contains(x)).count()) >> 1
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::part1;

    #[test]
    fn test_part1() {
        let input = include_str!("test.txt");
        assert_eq!(part1(input), 13);
    }
}
