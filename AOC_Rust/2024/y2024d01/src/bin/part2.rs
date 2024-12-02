use std::collections::HashMap;

use y2024d01::parse;

fn main() {
    let input = include_str!("input.txt");
    let output = part2(input);
    println!("{}", output);
}

fn part2(input: &str) -> u32 {
    let (left, right) = parse(input);

    let mut freq: HashMap<u32, u32> = HashMap::new();

    for r in right {
        freq.entry(r).and_modify(|x| *x += 1).or_insert(1);
    }

    left.into_iter()
        .map(|l| l * freq.get(&l).unwrap_or(&0))
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::part2;

    #[test]
    fn test_part2() {
        let input = include_str!("test.txt");
        assert_eq!(part2(input), 31);
    }
}
