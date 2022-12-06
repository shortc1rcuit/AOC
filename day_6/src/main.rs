use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("File not found");

    println!("{}", part_1(&input).unwrap());
    println!("{}", part_2(&input).unwrap());
}

fn part_1(input: &str) -> Option<usize> {
    let input = input.chars().collect::<Vec<char>>();
    let windows = input.windows(4).enumerate();

    for (i, chunk) in windows {
        if all_different(chunk) {
            return Some(i + 4);
        }
    }

    None
}

fn part_2(input: &str) -> Option<usize> {
    let input = input.chars().collect::<Vec<char>>();
    let windows = input.windows(14).enumerate();

    for (i, chunk) in windows {
        if all_different(chunk) {
            return Some(i + 14);
        }
    }

    None
}

fn all_different(chunk: &[char]) -> bool {
    for i in 0..chunk.len() {
        for j in i + 1..chunk.len() {
            if chunk[i] == chunk[j] {
                return false;
            }
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_test() {
        assert_eq!(part_1("bvwbjplbgvbhsrlpgdmjqwftvncz"), Some(5));
        assert_eq!(part_1("nppdvjthqldpwncqszvftbrmjlhg"), Some(6));
        assert_eq!(part_1("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), Some(10));
        assert_eq!(part_1("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), Some(11));
    }

    #[test]
    fn part_2_test() {
        assert_eq!(part_2("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), Some(19));
        assert_eq!(part_2("bvwbjplbgvbhsrlpgdmjqwftvncz"), Some(23));
        assert_eq!(part_2("nppdvjthqldpwncqszvftbrmjlhg"), Some(23));
        assert_eq!(part_2("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), Some(29));
        assert_eq!(part_2("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), Some(26));
    }
}
