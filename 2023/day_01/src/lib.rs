pub fn line_to_num(line: &str) -> u32 {
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
}
