use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Failed to find file");
    //let input = "A Y\nB X\nC Z".to_string();

    println!("{}", part_1(&input));
    println!("{}", part_2(&input));
}

//a = opponent's move
//b = my move
//1 = rock; 2 = paper; 3 = sissors
//0 = loss; 1 = draw; 2 = win

fn part_1(input: &str) -> i32 {
    input
        .lines()
        .map(|x| x.chars())
        .map(|mut x| (x.next().unwrap(), x.nth(1).unwrap()))
        .map(|(a, b)| ((a as i32) - 64, (b as i32) - 87))
        .map(|(a, b)| ((b - a + 1).rem_euclid(3)) * 3 + b)
        .sum()
}

//r = game result

//r = (b-a+1)%3
//r%3 = (b-a+1)%3
//(r-1)%3 = (b-a)%3
//b = (r+a-1)%3

//But that maps scissors to 0
//So we add 2, mod 3 and add 1

//b = (r+a-1+2)%3+1
//b = (r+a+1)%3+1

fn part_2(input: &str) -> i32 {
    input
        .lines()
        .map(|x| x.chars())
        .map(|mut x| (x.next().unwrap(), x.nth(1).unwrap()))
        .map(|(a, r)| ((a as i32) - 64, (r as i32) - 88))
        .map(|(a, r)| (r + a + 1).rem_euclid(3) + 1 + r * 3)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_test() {
        let input = "A Y\nB X\nC Z";
        assert_eq!(part_1(input), 15);
    }

    #[test]
    fn part_2_test() {
        let input = "A Y\nB X\nC Z";
        assert_eq!(part_2(input), 12);
    }
}