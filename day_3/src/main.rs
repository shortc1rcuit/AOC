use itertools::Itertools;
use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Failed to find file");

    println!("{}", part_1(&input));
    println!("{}", part_2(&input));
}

fn part_1(input: &str) -> u32 {
    input
        .lines()
        .map(|x| x.split_at(x.len() / 2))
        .map(|(a, b)| common_char(&mut a.to_string(), b))
        .map(priority)
        .sum::<u32>()
}

fn common_char(string_1: &mut String, string_2: &str) -> char {
    loop {
        let first = string_1.chars().next().unwrap();

        if string_2.contains(first) {
            return first;
        } else {
            *string_1 = string_1.replace(first, "");
        }
    }
}

fn priority(item: char) -> u32 {
    if item.is_uppercase() {
        item as u32 - 38
    } else {
        item as u32 - 96
    }
}

fn part_2(input: &str) -> u32 {
    input
        .lines()
        .tuples()
        .map(|(a, b, c)| (common_chars(&mut a.to_string(), b), c))
        .map(|(mut a, b)| common_char(&mut a, b))
        .map(priority)
        .sum::<u32>()
}

fn common_chars(string_1: &mut String, string_2: &str) -> String {
    let mut common = "".to_string();

    while !string_1.is_empty() {
        let first = string_1.chars().next().unwrap();

        if string_2.contains(first) {
            common = format!("{}{}", common, first);
        }

        *string_1 = string_1.replace(first, "");
    }

    common
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"
            .to_string();

        assert_eq!(part_1(&input), 157);
    }

    #[test]
    fn test_part_2() {
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"
            .to_string();

        assert_eq!(part_2(&input), 70);
    }
}
