use std::fs;

use nom::{branch::alt, bytes::complete::tag, character::complete, IResult};

fn main() {
    let input = fs::read_to_string("input.txt").expect("File not found");

    println!("{}", part_1(&input));
    part_2(&input);
}

struct Crt {
    clock: i32,
    register: i32,
    signal_strength: i32,
}

impl Crt {
    fn cycle(&mut self) {
        self.clock += 1;

        if (self.clock + 20) % 40 == 0 {
            self.signal_strength += self.clock * self.register;
        }
    }

    fn draw(&mut self) {
        let x = self.clock % 40;
        let dx = (x - self.register).abs();

        if dx <= 1 {
            print!("#");
        } else {
            print!(".");
        }

        self.clock += 1;

        if self.clock % 40 == 0 {
            println!();
        }
    }
}

enum Instruction {
    Noop,
    Addx(i32),
}

fn parse_noop(input: &str) -> IResult<&str, Instruction> {
    let (input, _) = tag("noop")(input)?;

    Ok((input, Instruction::Noop))
}

fn parse_addx(input: &str) -> IResult<&str, Instruction> {
    let (input, _) = tag("addx ")(input)?;
    let (input, count) = complete::i32(input)?;

    Ok((input, Instruction::Addx(count)))
}

fn parse_line(input: &str) -> IResult<&str, Instruction> {
    alt((parse_noop, parse_addx))(input)
}

fn part_1(input: &str) -> i32 {
    let instructions: Vec<Instruction> = input.lines().map(|x| parse_line(x).unwrap().1).collect();
    let mut crt = Crt {
        clock: 0,
        register: 1,
        signal_strength: 0,
    };

    for instruction in instructions {
        crt.cycle();

        if let Instruction::Addx(x) = instruction {
            crt.cycle();
            crt.register += x;
        }
    }

    crt.signal_strength
}

fn part_2(input: &str) {
    let instructions: Vec<Instruction> = input.lines().map(|x| parse_line(x).unwrap().1).collect();
    let mut crt = Crt {
        clock: 0,
        register: 1,
        signal_strength: 0,
    };

    for instruction in instructions {
        crt.draw();

        if let Instruction::Addx(x) = instruction {
            crt.draw();
            crt.register += x;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(INPUT), 13140)
    }
}
