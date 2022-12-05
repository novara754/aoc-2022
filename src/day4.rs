use crate::harness::PuzzleSolution;

pub struct Day4;

fn parse_elf(elf: &str) -> (u64, u64) {
    let mut parts = elf.split('-').map(|num| num.parse().unwrap());
    (parts.next().unwrap(), parts.next().unwrap())
}

fn parse_pairs(pair: &str) -> ((u64, u64), (u64, u64)) {
    let mut parts = pair.split(',').map(parse_elf);
    (parts.next().unwrap(), parts.next().unwrap())
}

fn fully_overlap(a: (u64, u64), b: (u64, u64)) -> bool {
    a.0 <= b.0 && a.1 >= b.1
}

fn kinda_overlap(a: (u64, u64), b: (u64, u64)) -> bool {
    a.0 <= b.0 && a.1 >= b.0 || a.0 <= b.1 && a.1 >= b.1
}

impl PuzzleSolution for Day4 {
    type Output = u64;

    fn part1(&self, input: &str) -> u64 {
        input
            .trim()
            .split('\n')
            .filter(|pair| {
                let (first, second) = parse_pairs(pair);
                fully_overlap(first, second) || fully_overlap(second, first)
            })
            .count()
            .try_into()
            .unwrap()
    }

    fn part2(&self, input: &str) -> u64 {
        input
            .trim()
            .split('\n')
            .filter(|pair| {
                let (first, second) = parse_pairs(pair);
                kinda_overlap(first, second) || kinda_overlap(second, first)
            })
            .count()
            .try_into()
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r#"2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8"#;

    #[test]
    fn part1() {
        assert_eq!(Day4.part1(TEST_INPUT), 2);
    }

    #[test]
    fn part2() {
        assert_eq!(Day4.part2(TEST_INPUT), 4);
    }
}
