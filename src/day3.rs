use std::collections::HashSet;

use itertools::Itertools;

use crate::harness::PuzzleSolution;

pub struct Day3;

fn get_priority(item: &u8) -> u64 {
    let priority = if item.is_ascii_lowercase() {
        item - b'a' + 1
    } else {
        item - b'A' + 27
    };
    priority as u64
}

impl PuzzleSolution for Day3 {
    fn part1(&self, input: &str) -> u64 {
        input
            .trim()
            .split('\n')
            .map(|rucksack| {
                let (first_half, second_half) = rucksack.split_at(rucksack.len() / 2);

                let first_half: HashSet<_> = first_half.bytes().collect();
                let second_half: HashSet<_> = second_half.bytes().collect();

                let priority = first_half
                    .intersection(&second_half)
                    .map(get_priority)
                    .next()
                    .unwrap();

                priority
            })
            .sum()
    }

    fn part2(&self, input: &str) -> u64 {
        input
            .trim()
            .split('\n')
            .chunks(3)
            .into_iter()
            .map(|group| {
                let mut rucksacks = group.map(|rucksack| rucksack.bytes().collect::<HashSet<_>>());

                let first = rucksacks.next().unwrap();
                let second = rucksacks.next().unwrap();
                let third = rucksacks.next().unwrap();
                let intersection = &(&first & &second) & &third;

                intersection.iter().map(get_priority).next().unwrap()
            })
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r#"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"#;

    #[test]
    fn part1() {
        assert_eq!(Day3.part1(TEST_INPUT), 157);
    }

    #[test]
    fn part2() {
        assert_eq!(Day3.part2(TEST_INPUT), 70);
    }
}
