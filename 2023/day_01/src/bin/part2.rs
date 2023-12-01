use day_01::line_to_num;

fn main() {
    let input = include_str!("input.txt");
    let output = part2(input);
    println!("{}", output);
}

const WORD_TO_NUM: [(&str, &str); 9] = [
    ("one", "o1e"),
    ("two", "t2o"),
    ("three", "t3e"),
    ("four", "f4r"),
    ("five", "f5e"),
    ("six", "s6x"),
    ("seven", "s7n"),
    ("eight", "e8t"),
    ("nine", "n9e"),
];

fn part2(input: &str) -> u32 {
    let mut input = input.to_string();

    for (word, num) in WORD_TO_NUM {
        input = input.replace(word, num)
    }

    input.lines().map(line_to_num).sum()
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
