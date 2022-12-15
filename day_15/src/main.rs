use std::{collections::BTreeMap, fs, ops::RangeInclusive};

use nom::{bytes::complete::tag, character::complete, sequence::preceded, IResult};
fn main() {
    let input = fs::read_to_string("input.txt").expect("File not found");

    println!("{}", part_1(&input, 2000000));
    println!("{}", part_2(&input, 4000000));
}

fn parse_coordinate(input: &str) -> IResult<&str, (i64, i64)> {
    let (input, x) = preceded(tag("x="), complete::i64)(input)?;
    let (input, y) = preceded(tag(", y="), complete::i64)(input)?;

    Ok((input, (x, y)))
}

fn parse_line(input: &str) -> IResult<&str, ((i64, i64), (i64, i64))> {
    let (input, point_1) = preceded(tag("Sensor at "), parse_coordinate)(input)?;
    let (input, point_2) = preceded(tag(": closest beacon is at "), parse_coordinate)(input)?;

    Ok((input, (point_1, point_2)))
}

fn remove_duplicates(input: Vec<(i64, i64)>) -> Vec<(i64, i64)> {
    let mut deduped = vec![];

    for x in input {
        if !deduped.contains(&x) {
            deduped.push(x);
        }
    }

    deduped
}

fn manhattan(point_1: (i64, i64), point_2: (i64, i64)) -> i64 {
    (point_1.0 - point_2.0).abs() + (point_1.1 - point_2.1).abs()
}

fn intersects(a: &RangeInclusive<i64>, b: &RangeInclusive<i64>) -> bool {
    a.contains(b.start()) || b.contains(a.start())
}

fn union(a: &RangeInclusive<i64>, b: &RangeInclusive<i64>) -> RangeInclusive<i64> {
    let start = a.start().min(b.start());
    let end = a.end().max(b.end());

    *start..=*end
}

fn part_1(input: &str, y: i64) -> u32 {
    let parsed_input = input.lines().map(|x| parse_line(x).unwrap().1);

    let beacons = parsed_input.clone().map(|(_, b)| b).collect();
    let beacons = remove_duplicates(beacons);
    let sensor_and_dist = parsed_input.map(|(a, b)| (a, manhattan(a, b)));

    let mut not_beacons = vec![];

    for (sensor, dist) in sensor_and_dist {
        let dy = (sensor.1 - y).abs();
        let dx = dist - dy;

        not_beacons.push((sensor.0 - dx)..=(sensor.0 + dx));
    }

    let mut i = 0;

    not_beacons.sort_by(|a, b| a.start().cmp(b.start()));

    while i < not_beacons.len() - 1 {
        if intersects(&not_beacons[i], &not_beacons[i + 1]) {
            let merged = union(&not_beacons[i], &not_beacons[i + 1]);
            not_beacons.remove(i + 1);
            not_beacons[i] = merged;
        } else {
            i += 1;
        }
    }

    let relevant_beacon_count = beacons
        .iter()
        .filter(|a| (a.1 == y) && not_beacons.iter().any(|x| x.contains(&a.0)))
        .count() as u32;

    not_beacons
        .iter()
        .filter(|x| !x.is_empty())
        .map(|x| (x.end() - x.start() + 1) as u32)
        .sum::<u32>()
        - relevant_beacon_count
}

fn part_2(input: &str, size: i64) -> i64 {
    let parsed_input = input.lines().map(|x| parse_line(x).unwrap().1);

    let sensor_and_dist = parsed_input.map(|(a, b)| (a, manhattan(a, b)));

    let mut no_distress: BTreeMap<i64, Vec<RangeInclusive<i64>>> = BTreeMap::new();

    let mut h_ranges = vec![];

    for (sensor, dist) in sensor_and_dist {
        for dy in (-dist)..=dist {
            h_ranges.push((
                sensor.1 + dy,
                (sensor.0 - (dist - dy.abs())).max(0)..=(sensor.0 + (dist - dy.abs())).min(size),
            ));
        }
    }

    for (y, x_range) in h_ranges {
        if y >= 0 && y <= size {
            no_distress
                .entry(y)
                .and_modify(|x| x.push(x_range.clone()))
                .or_insert_with(|| vec![x_range]);
        }
    }

    let (y, x_range) = no_distress
        .iter_mut()
        .map(|(y, ranges)| {
            let mut i = 0;

            ranges.sort_by(|a, b| a.start().cmp(b.start()));

            while i < ranges.len() - 1 {
                if intersects(&ranges[i], &ranges[i + 1]) {
                    let merged = union(&ranges[i], &ranges[i + 1]);
                    ranges.remove(i + 1);
                    ranges[i] = merged;
                } else {
                    i += 1;
                }
            }

            (y, ranges.clone())
        })
        .find(|(_, ranges)| ranges != &vec![0..=size])
        .unwrap();

    let x = x_range[0].end() + 1;

    x * size + y
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3
";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(INPUT, 10), 26);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(INPUT, 20), 291);
    }
}
