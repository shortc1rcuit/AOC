use std::{collections::HashMap, fs};

use nom::{
    branch::alt,
    bytes::complete::{is_a, tag},
    character::complete::{self, alpha1, newline},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

fn main() {
    let input = fs::read_to_string("input.txt").expect("File not found");

    println!("{}", part_1(&input));
    println!("{}", part_2(&input));
}

#[derive(Debug)]
enum LineType<'a> {
    Navigate(&'a str),
    GoBack,
    ListFiles,
    File(u32, &'a str),
    Directory(&'a str),
}

fn parse_navigate(input: &str) -> IResult<&str, LineType> {
    let (input, _) = tag("cd ")(input)?;
    let (input, location) = alt((tag(".."), alpha1))(input)?;

    if location == ".." {
        Ok((input, LineType::GoBack))
    } else {
        Ok((input, LineType::Navigate(location)))
    }
}

fn parse_list(input: &str) -> IResult<&str, LineType> {
    let (input, _) = tag("ls")(input)?;

    Ok((input, LineType::ListFiles))
}

fn parse_command(input: &str) -> IResult<&str, LineType> {
    let (input, _) = tag("$ ")(input)?;
    let (input, command) = alt((parse_navigate, parse_list))(input)?;

    Ok((input, command))
}

fn parse_file(input: &str) -> IResult<&str, LineType> {
    let (input, (size, name)) =
        separated_pair(complete::u32, tag(" "), is_a("qwertyuiopasdfghjklzxcvbnm."))(input)?;

    Ok((input, LineType::File(size, name)))
}

fn parse_directory(input: &str) -> IResult<&str, LineType> {
    let (input, _) = tag("dir ")(input)?;
    let (input, name) = alpha1(input)?;

    Ok((input, LineType::Directory(name)))
}

fn parse_line(input: &str) -> IResult<&str, LineType> {
    let (input, line) = alt((parse_command, parse_directory, parse_file))(input)?;

    Ok((input, line))
}

fn parse_input(input: &str) -> IResult<&str, Vec<LineType>> {
    let (input, _) = tag("$ cd /\n")(input)?;
    let (input, instructions) = separated_list1(newline, parse_line)(input)?;

    Ok((input, instructions))
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum FileType {
    Single,
    Folder,
}

#[derive(Debug)]
struct FileItem {
    file_type: FileType,
    size: u32,
}

fn run_instructions(input: &str) -> HashMap<Vec<&str>, FileItem> {
    let (_, instructions) = parse_input(input).unwrap();
    let mut file_layout = HashMap::new();
    let mut current_path = vec!["/"];

    file_layout.insert(
        vec!["/"],
        FileItem {
            file_type: FileType::Folder,
            size: 0,
        },
    );

    for instruction in instructions {
        match instruction {
            LineType::File(size, name) => {
                file_layout.insert(
                    [current_path.clone(), vec![name]].concat(),
                    FileItem {
                        file_type: FileType::Single,
                        size,
                    },
                );

                file_layout
                    .entry(current_path.clone())
                    .and_modify(|x| x.size += size);
            }
            LineType::Directory(name) => {
                file_layout.insert(
                    [current_path.clone(), vec![name]].concat(),
                    FileItem {
                        file_type: FileType::Folder,
                        size: 0,
                    },
                );
            }
            LineType::Navigate(name) => {
                current_path.push(name);
            }
            LineType::GoBack => {
                let current = file_layout.get(&current_path).unwrap();
                let size = current.size;

                current_path.pop();

                file_layout
                    .entry(current_path.clone())
                    .and_modify(|x| x.size += size);
            }
            LineType::ListFiles => {}
        };
    }

    while current_path != vec!["/"] {
        let current = file_layout.get(&current_path).unwrap();
        let size = current.size;

        current_path.pop();

        file_layout
            .entry(current_path.clone())
            .and_modify(|x| x.size += size);
    }

    file_layout
}

fn part_1(input: &str) -> u32 {
    let file_layout = run_instructions(input);

    file_layout
        .iter()
        .map(|(_, x)| (x.file_type, x.size))
        .filter(|(a, b)| (*b <= 100000) & (a == &FileType::Folder))
        .map(|(_, b)| b)
        .sum::<u32>()
}

fn part_2(input: &str) -> u32 {
    let file_layout = run_instructions(input);
    let space_to_delete = file_layout.get(&vec!["/"]).unwrap().size - 40000000;

    file_layout
        .iter()
        .map(|(_, x)| (x.file_type, x.size))
        .filter(|(a, b)| (*b >= space_to_delete) & (a == &FileType::Folder))
        .map(|(_, b)| b)
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_test() {
        let input = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

        assert_eq!(part_1(input), 95437);
    }

    #[test]
    fn part_2_test() {
        let input = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

        assert_eq!(part_2(input), 24933642);
    }
}
