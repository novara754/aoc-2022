use crate::harness::PuzzleSolution;

pub struct Day1;

fn get_elf_calories(input: &str) -> impl Iterator<Item = u64> + '_ {
    input.split("\n\n").map(|elf| {
        elf.split_whitespace()
            .map(|line| line.parse::<u64>().unwrap())
            .sum::<u64>()
    })
}

impl PuzzleSolution for Day1 {
    fn part1(&self, input: &str) -> u64 {
        get_elf_calories(input)
            .enumerate()
            .max_by_key(|(_idx, calories)| *calories)
            .unwrap()
            .1
    }

    fn part2(&self, input: &str) -> u64 {
        let mut elf_calories: Vec<_> = get_elf_calories(input).collect();
        elf_calories.sort_by(|a, b| b.cmp(a));
        elf_calories[0..3].iter().sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r#"
1000
2000
3000

4000

5000
6000

7000
8000
9000

10000"#;

    #[test]
    fn part1() {
        assert_eq!(Day1.part1(TEST_INPUT), 24000);
    }

    #[test]
    fn part2() {
        assert_eq!(Day1.part2(TEST_INPUT), 45000);
    }
}
