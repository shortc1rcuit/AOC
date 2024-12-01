pub fn parse(input: &str) -> (Vec<u32>, Vec<u32>) {
    input
        .lines()
        .map(|s| {
            s.split_once("   ")
                .map(|(l, r)| (l.parse::<u32>().unwrap(), r.parse::<u32>().unwrap()))
                .unwrap()
        })
        .unzip()
}
