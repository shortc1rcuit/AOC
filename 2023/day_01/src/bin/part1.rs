fn main() {
    let input = include_str!("input.txt");
    let output = part1(input);
    println!("{}", output);
}

fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let mut digits = line.chars().filter(|c| c.is_ascii_digit());
            let tens = digits
                .next()
                .expect("There should be a digit in each line")
                .to_digit(10)
                .unwrap();

            let units = digits
                .last()
                .map(|c| c.to_digit(10).unwrap())
                .unwrap_or(tens);

            (10 * tens) + units
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::part1;

    #[test]
    fn test_part1() {
        let input = "1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet";
        assert_eq!(part1(input), 142);
    }
}
