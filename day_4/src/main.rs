use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Failed to find file");

    println!("{}", part_1(&input));
    println!("{}", part_2(&input));
}

fn part_1(input: &str) -> usize {
    input.lines().filter(|x| contained(x)).count()
}

fn contained(bounds: &str) -> bool {
    let bounds: Vec<u32> = bounds
        .split(|x: char| !x.is_numeric())
        .map(|x| x.parse::<u32>().unwrap())
        .collect();

    ((bounds[0] <= bounds[2]) & (bounds[1] >= bounds[3]))
        || ((bounds[0] >= bounds[2]) & (bounds[1] <= bounds[3]))
}

fn part_2(input: &str) -> usize {
    input.lines().filter(|x| overlapping(x)).count()
}

fn overlapping(bounds: &str) -> bool {
    let bounds: Vec<u32> = bounds
        .split(|x: char| !x.is_numeric())
        .map(|x| x.parse::<u32>().unwrap())
        .collect();

    (bounds[0] <= bounds[3]) & (bounds[1] >= bounds[2])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_test() {
        let input = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

        assert_eq!(part_1(input), 2);
    }

    #[test]
    fn part_2_test() {
        let input = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

        assert_eq!(part_2(input), 4);
    }
}
