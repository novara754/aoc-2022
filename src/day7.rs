use std::collections::HashMap;

use crate::harness::PuzzleSolution;

pub struct Day7;

enum Entry {
    File(String, u64),
    Directory(String),
}

struct FileSystem {
    current_directory: Vec<String>,
    contents: HashMap<Vec<String>, Vec<Entry>>,
}

impl FileSystem {
    fn new() -> Self {
        Self {
            current_directory: vec![],
            contents: HashMap::new(),
        }
    }

    fn parse_from(input: &str) -> Self {
        let mut filesystem = FileSystem::new();
        let mut ls_output = false;

        for line in input.lines() {
            let mut parts = line.split(' ');
            let first = parts.next().unwrap();

            if first == "$" {
                ls_output = false;
                match parts.next().unwrap() {
                    "cd" => {
                        let dir_name = parts.next().unwrap();
                        filesystem.change_directory(dir_name);
                    }
                    "ls" => {
                        ls_output = true;
                    }
                    _ => unreachable!(),
                }
            } else if ls_output {
                if first == "dir" {
                    let dir_name = parts.next().unwrap();
                    filesystem.add_directory(dir_name);
                } else {
                    let file_size: u64 = first.parse().unwrap();
                    let file_name = parts.next().unwrap();
                    filesystem.add_file(file_name, file_size);
                }
            }
        }

        filesystem
    }

    fn change_directory(&mut self, dir_name: &str) {
        match dir_name {
            "/" => {
                self.current_directory.clear();
            }
            ".." => {
                self.current_directory.pop();
            }
            _ => {
                self.current_directory.push(dir_name.to_owned());
            }
        }
    }

    fn add_file(&mut self, file_name: &str, file_size: u64) {
        self.contents
            .entry(self.current_directory.clone())
            .or_default()
            .push(Entry::File(file_name.to_owned(), file_size));
    }

    fn add_directory(&mut self, dir_name: &str) {
        self.contents
            .entry(self.current_directory.clone())
            .or_default()
            .push(Entry::Directory(dir_name.to_owned()));
    }

    fn get_size_of(&self, dir_path: &[String]) -> u64 {
        let mut total_size = 0;
        for entry in self.contents.get(dir_path).unwrap() {
            total_size += match entry {
                Entry::File(_, size) => *size,
                Entry::Directory(dir_name) => {
                    let mut dir_path = dir_path.to_vec();
                    dir_path.push(dir_name.clone());
                    self.get_size_of(&dir_path)
                }
            };
        }
        total_size
    }
}

impl PuzzleSolution for Day7 {
    type Output = u64;

    fn part1(&self, input: &str) -> u64 {
        let filesystem = FileSystem::parse_from(input);
        filesystem
            .contents
            .keys()
            .map(|dir| filesystem.get_size_of(dir))
            .filter(|size| *size <= 100_000)
            .sum()
    }

    fn part2(&self, input: &str) -> u64 {
        let filesystem = FileSystem::parse_from(input);
        let total_size_used = filesystem.get_size_of(&[]);
        let total_size_free = 70_000_000 - total_size_used;
        let total_size_free_needed = 30_000_000;
        filesystem
            .contents
            .keys()
            .map(|dir| filesystem.get_size_of(dir))
            .filter(|size| *size >= (total_size_free_needed - total_size_free))
            .min()
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r#"$ cd /
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
7214296 k"#;

    #[test]
    fn part1() {
        assert_eq!(Day7.part1(TEST_INPUT), 95437);
    }

    #[test]
    fn part2() {
        assert_eq!(Day7.part2(TEST_INPUT), 24933642);
    }
}
