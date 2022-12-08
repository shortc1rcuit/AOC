use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("File not found");

    println!("{}", part_1(&input));
    println!("{}", part_2(&input));
}

fn parse_line(input: &str) -> Vec<u32> {
    input.chars().map(|x| x.to_digit(10).unwrap()).collect()
}

fn part_1(input: &str) -> u32 {
    let grid: Vec<Vec<u32>> = input.lines().map(parse_line).collect();
    let mut total_visable = 0;

    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            let cur_height = grid[y][x];
            let mut visable = true;

            //Going up
            for i in 0..y {
                visable &= cur_height > grid[i][x];
            }

            if visable {
                total_visable += 1;
                continue;
            }

            visable = true;

            //Going left
            for i in 0..x {
                visable &= cur_height > grid[y][i];
            }

            if visable {
                total_visable += 1;
                continue;
            }

            visable = true;

            //Going down
            for i in (y + 1)..grid.len() {
                visable &= cur_height > grid[i][x];
            }

            if visable {
                total_visable += 1;
                continue;
            }

            visable = true;

            //Going right
            for i in (x + 1)..grid[y].len() {
                visable &= cur_height > grid[y][i];
            }

            if visable {
                total_visable += 1;
                continue;
            }
        }
    }

    total_visable
}

fn part_2(input: &str) -> u32 {
    let grid: Vec<Vec<u32>> = input.lines().map(parse_line).collect();
    let mut largest = 0;

    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            let cur_height = grid[y][x];
            let mut scenic_score = vec![0; 4];

            //Going up
            for i in (0..y).rev() {
                scenic_score[0] += 1;

                if cur_height <= grid[i][x] {
                    break;
                }
            }

            //Going left
            for i in (0..x).rev() {
                scenic_score[1] += 1;

                if cur_height <= grid[y][i] {
                    break;
                }
            }

            //Going down
            for i in (y + 1)..grid.len() {
                scenic_score[2] += 1;

                if cur_height <= grid[i][x] {
                    break;
                }
            }

            //Going right
            for i in (x + 1)..grid[y].len() {
                scenic_score[3] += 1;

                if cur_height <= grid[y][i] {
                    break;
                }
            }

            let scenic_score: u32 = scenic_score.iter().product();
            largest = scenic_score.max(largest);
        }
    }

    largest
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_test() {
        let input = "30373
25512
65332
33549
35390";

        assert_eq!(part_1(input), 21);
    }

    #[test]
    fn part_2_test() {
        let input = "30373
25512
65332
33549
35390";

        assert_eq!(part_2(input), 8);
    }
}
