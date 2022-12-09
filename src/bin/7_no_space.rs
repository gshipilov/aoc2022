use aoc2022::read_input;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

fn main() {
    let input = read_input("7_no_space");
    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}

fn part_1(input: &str) -> usize {
    let fs = parse_fs(input);

    fs.values()
        .map(|d| d.size(&fs))
        .filter(|&size| size <= 100_000)
        .sum()
}

fn part_2(input: &str) -> usize {
    let max_space = 70_000_000;
    let need_space = 30_000_000;

    let fs = parse_fs(input);

    let used_space = fs.get(&Vec::new()).unwrap().size(&fs);

    let min_delete = need_space - (max_space - used_space);
    fs.values()
        .map(|d| d.size(&fs))
        .sorted()
        .filter(|&s| s >= min_delete)
        .min()
        .unwrap()
}

fn parse_fs(input: &str) -> FileSystem {
    let mut path = Vec::new();
    let mut fs: FileSystem = HashMap::new();

    fs.insert(Vec::new(), Directory::new(Vec::new()));

    input
        .lines()
        .map(|line| line.split(' ').collect::<Vec<_>>())
        .for_each(|tokens| {
            if tokens[0] == "$" {
                match tokens[1] {
                    "ls" => (),
                    "cd" => match tokens[2] {
                        "/" => {
                            path = Vec::new();
                        }
                        ".." => {
                            path.pop();
                        }
                        v => {
                            path.push(v.to_string());
                        }
                    },
                    _ => unreachable!(),
                }
            } else {
                match tokens[0] {
                    "dir" => {
                        let mut child_path = path.clone();
                        child_path.push(tokens[1].to_string());

                        fs.entry(child_path.clone())
                            .or_insert_with(|| Directory::new(child_path));

                        fs.get_mut(&path)
                            .unwrap()
                            .children
                            .insert(tokens[1].to_string());
                    }
                    num_str => {
                        let size = num_str.parse::<usize>().unwrap();
                        let file = File {
                            name: tokens[1].to_string(),
                            size,
                        };

                        let cwd = fs.get_mut(&path).unwrap();
                        cwd.files.insert(file);
                    }
                }
            }
        });

    fs
}

type FileSystem = HashMap<Vec<String>, Directory>;

#[derive(Debug, Clone)]
struct Directory {
    path: Vec<String>,
    children: HashSet<String>,
    files: HashSet<File>,
}

impl Directory {
    fn new(path: Vec<String>) -> Self {
        Directory {
            path,
            children: HashSet::default(),
            files: HashSet::default(),
        }
    }

    fn size(&self, fs: &FileSystem) -> usize {
        let files: usize = self.files.iter().map(|f| f.size).sum();
        let children: usize = self.children(fs).iter().map(|d| d.size(fs)).sum();

        files + children
    }

    fn children(&self, fs: &FileSystem) -> Vec<Directory> {
        self.children
            .iter()
            .map(|child| {
                let mut path = self.path.clone();
                path.push(child.clone());

                path
            })
            .map(|path| fs.get(&path).cloned().unwrap())
            .collect()
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct File {
    name: String,
    size: usize,
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const TEST_INPUT: &str = indoc! {r#"
    $ cd /
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
    7214296 k
    "#};

    #[test]
    fn test_part_1() {
        assert_eq!(95437, part_1(TEST_INPUT));
    }

    #[test]
    fn test_part_2() {
        assert_eq!(24933642, part_2(TEST_INPUT));
    }
}
