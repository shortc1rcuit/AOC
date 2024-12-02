use nom::{
    branch::alt,
    character::complete::{self, newline},
    multi::separated_list0,
    sequence::{delimited, separated_pair},
    IResult, Parser,
};
use std::{cmp::Ordering, fs};

fn main() {
    let input = fs::read_to_string("input.txt").expect("File not found");

    println!("{}", part_1(&input));
    println!("{}", part_2(&input));
}

fn parse_list_item(input: &str) -> IResult<&str, ListItem> {
    let (input, item) = alt((
        complete::u32.map(ListItem::Number),
        parse_nested_list.map(ListItem::List),
    ))(input)?;

    Ok((input, item))
}

fn parse_nested_list(input: &str) -> IResult<&str, NestedList> {
    let (input, items) = delimited(
        complete::char('['),
        separated_list0(complete::char(','), parse_list_item),
        complete::char(']'),
    )(input)?;

    Ok((input, NestedList { items }))
}

fn parse_input(input: &str) -> IResult<&str, (NestedList, NestedList)> {
    let (input, list) = separated_pair(parse_nested_list, newline, parse_nested_list)(input)?;

    Ok((input, list))
}

#[derive(Clone, PartialEq)]
struct NestedList {
    items: Vec<ListItem>,
}

#[derive(Clone, PartialEq)]
enum ListItem {
    Number(u32),
    List(NestedList),
}

fn in_order(list_left: &NestedList, list_right: &NestedList) -> Ordering {
    use ListItem::{List, Number};

    let min_len = list_left.items.len().min(list_right.items.len());

    for i in 0..min_len {
        let left_item = list_left.items[i].clone();
        let right_item = list_right.items[i].clone();

        if let Number(l) = left_item {
            if let Number(r) = right_item {
                match l.cmp(&r) {
                    Ordering::Less => return Ordering::Less,
                    Ordering::Greater => return Ordering::Greater,
                    Ordering::Equal => {}
                }
            } else if let List(r) = right_item {
                let list_left = NestedList {
                    items: vec![Number(l)],
                };
                let comparison = in_order(&list_left, &r);

                if comparison != Ordering::Equal {
                    return comparison;
                }
            }
        } else if let List(l) = left_item {
            if let Number(r) = right_item {
                let list_right = NestedList {
                    items: vec![Number(r)],
                };
                let comparison = in_order(&l, &list_right);

                if comparison != Ordering::Equal {
                    return comparison;
                }
            } else if let List(r) = right_item {
                let comparison = in_order(&l, &r);

                if comparison != Ordering::Equal {
                    return comparison;
                }
            }
        }
    }

    list_left.items.len().cmp(&list_right.items.len())
}

fn part_1(input: &str) -> usize {
    input
        .split("\n\n")
        .enumerate()
        .map(|(a, b)| (a + 1, parse_input(b).unwrap().1))
        .filter(|(_, (b, c))| in_order(b, c) == Ordering::Less)
        .map(|(a, _)| a)
        .sum()
}

fn parse_input_pt2(input: &str) -> IResult<&str, Vec<NestedList>> {
    let (input, list) = separated_list0(newline, parse_nested_list)(input)?;

    Ok((input, list))
}

fn part_2(input: &str) -> usize {
    use ListItem::{List, Number};

    let mut packets = parse_input_pt2(&input.replace("\n\n", "\n")).unwrap().1;

    let divider_1 = NestedList {
        items: vec![List(NestedList {
            items: vec![Number(2)],
        })],
    };
    let divider_2 = NestedList {
        items: vec![List(NestedList {
            items: vec![Number(6)],
        })],
    };

    packets.push(divider_1.clone());
    packets.push(divider_2.clone());

    packets.sort_by(in_order);

    let key_1 = packets.iter().position(|x| *x == divider_1).unwrap() + 1;
    let key_2 = packets.iter().position(|x| *x == divider_2).unwrap() + 1;

    key_1 * key_2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_in_order() {
        use ListItem::*;

        let list_left = NestedList {
            items: vec![Number(1), Number(1), Number(3), Number(1), Number(1)],
        };
        let list_right = NestedList {
            items: vec![Number(1), Number(1), Number(5), Number(1), Number(1)],
        };

        assert_eq!(in_order(&list_left, &list_right), Ordering::Less);

        let list_left = NestedList {
            items: vec![
                List(NestedList {
                    items: vec![Number(1)],
                }),
                List(NestedList {
                    items: vec![Number(2), Number(3), Number(4)],
                }),
            ],
        };
        let list_right = NestedList {
            items: vec![
                List(NestedList {
                    items: vec![Number(1)],
                }),
                Number(4),
            ],
        };

        assert_eq!(in_order(&list_left, &list_right), Ordering::Less);

        let list_left = NestedList {
            items: vec![Number(9)],
        };
        let list_right = NestedList {
            items: vec![List(NestedList {
                items: vec![Number(8), Number(7), Number(6)],
            })],
        };

        assert_eq!(in_order(&list_left, &list_right), Ordering::Greater);
    }

    const INPUT: &str = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]
";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(INPUT), 13)
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(INPUT), 140)
    }
}
