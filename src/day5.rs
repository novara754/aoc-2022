use itertools::Itertools;
use regex::Regex;

use crate::harness::PuzzleSolution;

pub struct Day5;

// fn print_ship(ship: &[Vec<char>]) {
//     let biggest_stack_size = ship.iter().map(|stack| stack.len()).max().unwrap_or(0);
//     for row in (0..biggest_stack_size).rev() {
//         for stack in ship {
//             if row < stack.len() {
//                 print!("[{}]  ", stack[row]);
//             } else {
//                 print!("     ");
//             }
//         }
//         println!()
//     }
//     for i in 0..ship.len() {
//         print!("{: >2}   ", i + 1);
//     }
//     println!();
// }

impl PuzzleSolution for Day5 {
    type Output = String;

    fn part1(&self, input: &str) -> String {
        let input = input.trim_end();
        let row_regex = Regex::new(r"(\[[A-Z]\]|(:?   ))\s?").unwrap();
        let instruction_regex = Regex::new(r"^move (\d+) from (\d) to (\d)$").unwrap();

        let (stacks, instructions) = {
            let mut parts = input.split("\n\n");
            let stacks = parts.next().unwrap();
            let instructions = parts.next().unwrap();
            (stacks, instructions)
        };

        let mut ship: Vec<Vec<char>> = vec![];

        for row in stacks.split('\n').rev() {
            for (column, container) in row_regex
                .captures_iter(row)
                .map(|capture| capture.get(1).unwrap().as_str())
                .enumerate()
            {
                if ship.len() <= column {
                    ship.push(vec![]);
                }

                if container == "   " {
                    continue;
                }

                ship[column].push(container.chars().nth(1).unwrap());
            }
        }

        for instruction_raw in instructions.split('\n') {
            let instruction = instruction_regex.captures(instruction_raw).unwrap();
            let (amount, from, to): (usize, usize, usize) = (
                instruction.get(1).unwrap().as_str().parse().unwrap(),
                instruction.get(2).unwrap().as_str().parse().unwrap(),
                instruction.get(3).unwrap().as_str().parse().unwrap(),
            );
            for _ in 0..amount {
                let container = ship[from - 1].pop().unwrap();
                ship[to - 1].push(container);
            }
        }

        ship.iter().map(|stack| stack.last().unwrap()).join("")
    }

    fn part2(&self, input: &str) -> String {
        let input = input.trim_end();
        let row_regex = Regex::new(r"(\[[A-Z]\]|(:?   ))\s?").unwrap();
        let instruction_regex = Regex::new(r"^move (\d+) from (\d) to (\d)$").unwrap();

        let (stacks, instructions) = {
            let mut parts = input.split("\n\n");
            let stacks = parts.next().unwrap();
            let instructions = parts.next().unwrap();
            (stacks, instructions)
        };

        let mut ship: Vec<Vec<char>> = vec![];

        for row in stacks.split('\n').rev() {
            for (column, container) in row_regex
                .captures_iter(row)
                .map(|capture| capture.get(1).unwrap().as_str())
                .enumerate()
            {
                if ship.len() <= column {
                    ship.push(vec![]);
                }

                if container == "   " {
                    continue;
                }

                ship[column].push(container.chars().nth(1).unwrap());
            }
        }

        for instruction_raw in instructions.split('\n') {
            let instruction = instruction_regex.captures(instruction_raw).unwrap();
            let (amount, from, to): (usize, usize, usize) = (
                instruction.get(1).unwrap().as_str().parse().unwrap(),
                instruction.get(2).unwrap().as_str().parse().unwrap(),
                instruction.get(3).unwrap().as_str().parse().unwrap(),
            );

            let mut tmp = vec![];
            for _ in 0..amount {
                let container = ship[from - 1].pop().unwrap();
                tmp.push(container);
            }
            for _ in 0..amount {
                let container = tmp.pop().unwrap();
                ship[to - 1].push(container);
            }
        }

        ship.iter().map(|stack| stack.last().unwrap()).join("")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r#"    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2"#;

    #[test]
    fn part1() {
        assert_eq!(Day5.part1(TEST_INPUT), "CMZ");
    }

    #[test]
    fn part2() {
        assert_eq!(Day5.part2(TEST_INPUT), "MCD");
    }
}
