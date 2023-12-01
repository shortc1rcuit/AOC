use std::{fs::File, io::Read, path::Path};

fn main() {
    let path = Path::new("input.txt");
    part_1(path);
    part_2(path);
}

fn part_1(path: &Path) {
    let mut file = File::open(&path).expect("Problem finding file");
    let mut content = String::new();

    file.read_to_string(&mut content)
        .expect("Problem finding file");

    let content = content.split("\n").collect::<Vec<&str>>();

    let mut max = 0;
    let mut total = 0;

    for line in content {
        if line != "" {
            total += line.parse::<u32>().unwrap();
        } else {
            if total > max {
                max = total;
            }

            total = 0;
        }
    }

    println!("{}", max);
}

fn part_2(path: &Path) {
    let mut file = File::open(&path).expect("Problem finding file");
    let mut content = String::new();

    file.read_to_string(&mut content)
        .expect("Problem finding file");

    let content = content.split("\n").collect::<Vec<&str>>();

    let mut totals = Vec::new();
    let mut total = 0;

    for line in content {
        if line != "" {
            total += line.parse::<u32>().unwrap();
        } else {
            totals.push(total);
            total = 0;
        }
    }

    totals.sort();
    totals.reverse();
    let result = totals[0] + totals[1] + totals[2];

    println!("{}", result);
}
