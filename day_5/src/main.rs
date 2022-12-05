use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Failed to find file");

    println!("{}", part_1(&input, 9));
    println!("{}", part_2(&input, 9));
}

fn part_1(input: &str, columns: usize) -> String {
    let input: Vec<&str> = input.split("\n\n").collect();
    let mut setup = parse_setup(input[0], columns);

    for instruction in input[1].lines().map(parse_instruction) {
        for _ in 0..instruction.count {
            let x = setup[instruction.from].pop().unwrap();
            setup[instruction.to].push(x);
        }
    }

    setup.into_iter().map(|mut x| x.pop().unwrap()).collect()
}

fn parse_setup(input: &str, columns: usize) -> Vec<Vec<char>> {
    let mut setup = vec![vec![]; columns];

    for i in 0..columns {
        for row in input
            .lines()
            .rev()
            .skip(1)
            .map(|x| parse_setup_line(x, columns))
        {
            if row[i] != ' ' {
                setup[i].push(row[i]);
            }
        }
    }

    setup
}

fn parse_setup_line(line: &str, columns: usize) -> Vec<char> {
    let mut items = vec![];
    let line: Vec<char> = line.chars().skip(1).collect();

    for i in 0..columns {
        items.push(line[i * 4]);
    }

    items
}

struct Instruction {
    count: u32,
    from: usize,
    to: usize,
}

fn parse_instruction(line: &str) -> Instruction {
    let line: Vec<&str> = line.split(' ').collect();

    Instruction {
        count: line[1].parse().unwrap(),
        from: line[3].parse::<usize>().unwrap() - 1,
        to: line[5].parse::<usize>().unwrap() - 1,
    }
}

fn part_2(input: &str, columns: usize) -> String {
    let input: Vec<&str> = input.split("\n\n").collect();
    let mut setup = parse_setup(input[0], columns);

    for instruction in input[1].lines().map(parse_instruction) {
        let mut moving = vec![];

        for _ in 0..instruction.count {
            moving.push(setup[instruction.from].pop().unwrap());
        }

        for _ in 0..instruction.count {
            setup[instruction.to].push(moving.pop().unwrap());
        }
    }

    setup.into_iter().map(|mut x| x.pop().unwrap()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

        assert_eq!(part_1(input, 3), "CMZ");
    }

    #[test]
    fn test_part_2() {
        let input = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

        assert_eq!(part_2(input, 3), "MCD");
    }
}
