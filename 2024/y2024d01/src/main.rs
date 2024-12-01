fn main() {
    let input = include_str!("input.txt");

    let (mut left, mut right): (Vec<_>, Vec<_>) = input
        .lines()
        .map(|s| {
            s.split_once("   ")
                .map(|(l, r)| (l.parse::<u32>().unwrap(), r.parse::<u32>().unwrap()))
                .unwrap()
        })
        .unzip();

    left.sort();
    right.sort();

    let sum: u32 = left
        .into_iter()
        .zip(right)
        .map(|(l, r)| l.abs_diff(r))
        .sum();

    println!("{}", sum)
}
