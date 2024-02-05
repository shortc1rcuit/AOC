use day_06::*;

fn main() {
    let input = include_str!("input.txt");
    let output = part1(input);
    println!("{}", output);
}

fn part1(input: &str) -> u64 {
    let (times, distances) = parse(input).unwrap().1;

    times
        .into_iter()
        .zip(distances)
        .map(|(time, distance)| {
            count_solutions(time as f64, distance as f64)
        })
        .product()
}

#[cfg(test)]
mod tests {
    use crate::part1;

    #[test]
    fn test_part1() {
        let input = include_str!("test.txt");
        assert_eq!(part1(input), 288);
    }
}
