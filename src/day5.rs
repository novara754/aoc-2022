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

struct Ship {
    stacks: Vec<Vec<char>>,
}

struct Instruction {
    amount: usize,
    to: usize,
    from: usize,
}

fn rearrange_crates(input: &str, transfer_fn: impl Fn(&mut Ship, Instruction)) -> String {
    let input = input.trim_end();
    let instruction_regex =
        Regex::new(r"^move (?P<amount>\d+) from (?P<from>\d) to (?P<to>\d)$").unwrap();

    let (stacks, instructions) = {
        let mut parts = input.split("\n\n");
        let stacks = parts.next().unwrap();
        let instructions = parts.next().unwrap();
        (stacks, instructions)
    };

    let mut ship = Ship { stacks: vec![] };

    for row in stacks.split('\n').rev() {
        for (idx, ch) in row.chars().enumerate() {
            if ch.is_alphabetic() {
                let column = (idx - 1) / 4;

                if ship.stacks.len() <= column {
                    ship.stacks.resize(column + 1, vec![]);
                }

                ship.stacks[column].push(ch);
            }
        }
    }

    for instruction_raw in instructions.split('\n') {
        let captures = instruction_regex.captures(instruction_raw).unwrap();

        let get_val = |name| captures.name(name).unwrap().as_str().parse().unwrap();
        let instruction = Instruction {
            amount: get_val("amount"),
            from: get_val("from"),
            to: get_val("to"),
        };

        transfer_fn(&mut ship, instruction);
    }

    ship.stacks
        .iter()
        .map(|stack| stack.last().unwrap())
        .join("")
}

impl PuzzleSolution for Day5 {
    type Output = String;

    fn part1(&self, input: &str) -> String {
        rearrange_crates(input, |ship, instruction| {
            for _ in 0..instruction.amount {
                let container = ship.stacks[instruction.from - 1].pop().unwrap();
                ship.stacks[instruction.to - 1].push(container);
            }
        })
    }

    fn part2(&self, input: &str) -> String {
        rearrange_crates(input, |ship, instruction| {
            let mut tmp = vec![];
            for _ in 0..instruction.amount {
                let container = ship.stacks[instruction.from - 1].pop().unwrap();
                tmp.push(container);
            }
            for _ in 0..instruction.amount {
                let container = tmp.pop().unwrap();
                ship.stacks[instruction.to - 1].push(container);
            }
        })
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
