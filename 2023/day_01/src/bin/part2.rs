use regex::Regex;
use std::collections::HashMap;

fn main() {
    let input = include_str!("input.txt");
    let output = part2(input);
    println!("{}", output);
}

const WORD_TO_NUM: [(&str, u32); 9] = [
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
];

const REV_WORD_TO_NUM: [(&str, u32); 9] = [
    ("eno", 1),
    ("owt", 2),
    ("eerht", 3),
    ("ruof", 4),
    ("evif", 5),
    ("xis", 6),
    ("neves", 7),
    ("thgie", 8),
    ("enin", 9),
];

fn part2(input: &str) -> u32 {
    let input = input.to_string();

    //TODO: Think of a better solution
    //The issue is that Regex won't do overlapping searches
    //This means that for the specific case where two word-numbers overlap at the end of a line
    //The regex will interpret the first of the two as the last digit rather than the second
    //So to correctly get the final digit, we reverse the string and then search.
    let word_to_num = HashMap::from(WORD_TO_NUM);
    let rev_word_to_num = HashMap::from(REV_WORD_TO_NUM);
    let re = Regex::new(r"(one|two|three|four|five|six|seven|eight|nine|\d)").unwrap();
    let rev_re = Regex::new(r"(eno|owt|eerht|ruof|evif|xis|neves|thgie|enin|\d)").unwrap();

    input
        .lines()
        .map(|line| {
            let tens = re
                .find(line)
                .expect("There should be some form of a digit in each line")
                .as_str();
            let tens = if let Ok(tens) = tens.parse::<u32>() {
                tens
            } else {
                *word_to_num.get(tens).unwrap()
            };

            let rev_line = line.chars().rev().collect::<String>();
            let units = rev_re
                .find(&rev_line)
                .expect("There should be some form of a digit in each line")
                .as_str();
            let units = if let Ok(units) = units.parse::<u32>() {
                units
            } else {
                *rev_word_to_num.get(units).unwrap()
            };

            (10 * tens) + units
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::part2;

    #[test]
    fn test_part2() {
        let input = "two1nine\neightwothree\nabcone2threexyz\nxtwone3four\n4nineeightseven2\nzoneight234\n7pqrstsixteen";
        assert_eq!(part2(input), 281);
    }
}
