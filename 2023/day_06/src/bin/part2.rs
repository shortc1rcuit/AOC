use day_06::*;

fn main() {
    let input = include_str!("input.txt");
    let output = part2(input);
    println!("{}", output);
}

fn part2(input: &str) -> u64 {
    //We're trying to solve the inequality t(s-t) > d
    //(Where t = time held, s = total time allowed and d = distance to beat)
    //This rearranges to t^2 - st + d < 0, which we can solve using the quadratic formula
    let (time, distance) = parse_bad_kerning(input).unwrap().1;

    count_solutions(time as f64, distance as f64)
}

#[cfg(test)]
mod tests {
    use crate::part2;

    #[test]
    fn test_part2() {
        let input = include_str!("test.txt");
        assert_eq!(part2(input), 71503);
    }
}
