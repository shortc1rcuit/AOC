use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("File not found");

    println!("{}", part_1(&input));
    println!("{}", part_2(&input));
}

#[derive(Clone, Copy, Debug)]
struct Cell {
    height: u8,
    steps: Option<u32>,
}

fn find_path_length(
    start: (usize, usize),
    end: (usize, usize),
    mut map: Vec<Vec<Cell>>,
) -> Option<u32> {
    let mut queue = vec![start];
    map[start.1][start.0].steps = Some(0);

    let height = map.len();
    let width = map[0].len();

    while !queue.is_empty() {
        let (cur_x, cur_y) = queue.remove(0);
        let current = map[cur_y][cur_x];

        if end == (cur_x, cur_y) {
            return current.steps;
        }

        if cur_x > 0 {
            let checking = &mut map[cur_y][cur_x - 1];

            if checking.steps.is_none() && checking.height <= 1 + current.height {
                queue.push((cur_x - 1, cur_y));
                checking.steps = Some(current.steps.unwrap() + 1);
            }
        }

        if cur_y > 0 {
            let checking = &mut map[cur_y - 1][cur_x];

            if checking.steps.is_none() && checking.height <= 1 + current.height {
                queue.push((cur_x, cur_y - 1));
                checking.steps = Some(current.steps.unwrap() + 1);
            }
        }

        if cur_x < width - 1 {
            let checking = &mut map[cur_y][cur_x + 1];

            if checking.steps.is_none() && checking.height <= 1 + current.height {
                queue.push((cur_x + 1, cur_y));
                checking.steps = Some(current.steps.unwrap() + 1);
            }
        }

        if cur_y < height - 1 {
            let checking = &mut map[cur_y + 1][cur_x];

            if checking.steps.is_none() && checking.height <= 1 + current.height {
                queue.push((cur_x, cur_y + 1));
                checking.steps = Some(current.steps.unwrap() + 1);
            }
        }
    }

    None
}

fn part_1(input: &str) -> u32 {
    let input: Vec<Vec<char>> = input.lines().map(|x| x.chars().collect()).collect();
    let mut end = (0, 0);
    let mut start = (0, 0);
    let mut map = vec![];

    for y in 0..input.len() {
        map.push(vec![]);

        for x in 0..input[y].len() {
            if input[y][x] == 'S' {
                start = (x, y);
                map[y].push(Cell {
                    height: 1,
                    steps: None,
                });
            } else if input[y][x] == 'E' {
                end = (x, y);
                map[y].push(Cell {
                    height: 26,
                    steps: None,
                });
            } else {
                map[y].push(Cell {
                    height: input[y][x] as u8 - 96,
                    steps: None,
                })
            }
        }
    }

    find_path_length(start, end, map).unwrap()
}

fn part_2(input: &str) -> u32 {
    let input: Vec<Vec<char>> = input.lines().map(|x| x.chars().collect()).collect();
    let mut end = (0, 0);
    let mut starts = vec![];
    let mut map = vec![];

    for y in 0..input.len() {
        map.push(vec![]);

        for x in 0..input[y].len() {
            if input[y][x] == 'S' || input[y][x] == 'a' {
                starts.push((x, y));
                map[y].push(Cell {
                    height: 1,
                    steps: None,
                });
            } else if input[y][x] == 'E' {
                end = (x, y);
                map[y].push(Cell {
                    height: 26,
                    steps: None,
                });
            } else {
                map[y].push(Cell {
                    height: input[y][x] as u8 - 96,
                    steps: None,
                })
            }
        }
    }

    starts
        .iter()
        .filter_map(|start| find_path_length(*start, end, map.clone()))
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi
";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(INPUT), 31);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(INPUT), 29);
    }
}
