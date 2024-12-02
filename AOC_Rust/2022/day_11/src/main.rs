use std::fs;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, digit1, newline},
    multi::separated_list1,
    sequence::tuple,
    IResult,
};

fn main() {
    let input = fs::read_to_string("input.txt").expect("File not found");

    println!("{}", part_1(&input));
    println!("{}", part_2(&input));
}

#[derive(Debug, Clone, Copy)]
enum Operation {
    Add(u64),
    Multiply(u64),
    Square,
}

struct Monkey {
    items: Vec<u64>,
    operation: Operation,
    divisable: u64,
    throw_true: usize,
    throw_false: usize,
    interactions: u64,
}

impl Monkey {
    fn interact_low_worry(&mut self) -> (u64, usize) {
        self.interactions += 1;

        let mut item = self.items[0];

        self.items = self.items[1..].to_vec();

        match self.operation {
            Operation::Add(x) => item += x,
            Operation::Multiply(x) => item *= x,
            Operation::Square => item *= item,
        }

        item /= 3;

        if item % self.divisable == 0 {
            (item, self.throw_true)
        } else {
            (item, self.throw_false)
        }
    }

    fn interact_high_worry(&mut self, moduli: u64) -> (u64, usize) {
        self.interactions += 1;

        let mut item = self.items[0];

        self.items = self.items[1..].to_vec();

        match self.operation {
            Operation::Add(x) => item += x,
            Operation::Multiply(x) => item *= x,
            Operation::Square => item *= item,
        }

        item %= moduli;

        if item % self.divisable == 0 {
            (item, self.throw_true)
        } else {
            (item, self.throw_false)
        }
    }
}

fn parse_add(input: &str) -> IResult<&str, Operation> {
    let (input, (_, num)) = tuple((tag("+ "), complete::u64))(input)?;

    Ok((input, Operation::Add(num)))
}

fn parse_multiply(input: &str) -> IResult<&str, Operation> {
    let (input, (_, num)) = tuple((tag("* "), complete::u64))(input)?;

    Ok((input, Operation::Multiply(num)))
}

fn parse_square(input: &str) -> IResult<&str, Operation> {
    let (input, _) = tag("* old")(input)?;

    Ok((input, Operation::Square))
}

fn parse_monkey(input: &str) -> IResult<&str, Monkey> {
    let (input, _) = tuple((tag("Monkey "), digit1, tag(":\n")))(input)?;

    let (input, (_, items, _)) = tuple((
        tag("  Starting items: "),
        separated_list1(tag(", "), complete::u64),
        newline,
    ))(input)?;

    let (input, (_, operation, _)) = tuple((
        tag("  Operation: new = old "),
        alt((parse_add, parse_multiply, parse_square)),
        newline,
    ))(input)?;

    let (input, (_, divisable, _)) =
        tuple((tag("  Test: divisible by "), complete::u64, newline))(input)?;

    let (input, (_, throw_true, _)) =
        tuple((tag("    If true: throw to monkey "), complete::u64, newline))(input)?;

    let (input, (_, throw_false)) =
        tuple((tag("    If false: throw to monkey "), complete::u32))(input)?;

    Ok((
        input,
        Monkey {
            items,
            operation,
            divisable,
            throw_true: throw_true as usize,
            throw_false: throw_false as usize,
            interactions: 0,
        },
    ))
}

fn part_1(input: &str) -> u64 {
    let mut monkeys: Vec<Monkey> = input
        .split("\n\n")
        .map(|x| parse_monkey(x).unwrap().1)
        .collect();

    for _ in 0..20 {
        for monkey in 0..monkeys.len() {
            while !monkeys[monkey].items.is_empty() {
                let (item, throw_to) = monkeys[monkey].interact_low_worry();

                monkeys[throw_to].items.push(item);
            }
        }
    }

    let mut monkeys = monkeys.iter().map(|x| x.interactions).collect::<Vec<u64>>();
    monkeys.sort_by(|a, b| b.partial_cmp(a).unwrap());

    monkeys[0] * monkeys[1]
}

fn part_2(input: &str) -> u64 {
    let mut monkeys: Vec<Monkey> = input
        .split("\n\n")
        .map(|x| parse_monkey(x).unwrap().1)
        .collect();

    let moduli = monkeys.iter().map(|x| x.divisable).product();

    for _ in 0..10000 {
        for monkey in 0..monkeys.len() {
            while !monkeys[monkey].items.is_empty() {
                let (item, throw_to) = monkeys[monkey].interact_high_worry(moduli);

                monkeys[throw_to].items.push(item);
            }
        }
    }

    let mut monkeys = monkeys.iter().map(|x| x.interactions).collect::<Vec<u64>>();
    monkeys.sort_by(|a, b| b.partial_cmp(a).unwrap());

    monkeys[0] * monkeys[1]
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1
";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(INPUT), 10605);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(INPUT), 2713310158);
    }
}
