use crate::harness::PuzzleSolution;

pub struct Day2;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Move {
    R,
    P,
    S,
}

impl Move {
    fn from_char(c: char) -> Self {
        match c {
            'A' => Self::R,
            'B' => Self::P,
            'C' => Self::S,
            'X' => Self::R,
            'Y' => Self::P,
            'Z' => Self::S,
            _ => panic!("invalid input `{c}`"),
        }
    }

    fn get_score(self) -> u64 {
        match self {
            Self::R => 1,
            Self::P => 2,
            Self::S => 3,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Outcome {
    L,
    D,
    W,
}

impl Outcome {
    fn from_char(c: char) -> Self {
        match c {
            'X' => Self::L,
            'Y' => Self::D,
            'Z' => Self::W,
            _ => panic!("invalid input `{c}`"),
        }
    }

    fn from_moves(opponent: Move, me: Move) -> Self {
        match (opponent, me) {
            (Move::R, Move::P) | (Move::P, Move::S) | (Move::S, Move::R) => Self::W,
            (a, b) if a == b => Self::D,
            _ => Self::L,
        }
    }

    fn get_score(self) -> u64 {
        match self {
            Self::L => 0,
            Self::D => 3,
            Self::W => 6,
        }
    }
}

impl PuzzleSolution for Day2 {
    type Output = u64;

    fn part1(&self, input: &str) -> u64 {
        fn parse_line(line: &str) -> (Move, Move) {
            let mut parts = line.chars();
            let (opponent, me) = (parts.next().unwrap(), parts.nth(1).unwrap());
            (Move::from_char(opponent), Move::from_char(me))
        }

        input
            .trim()
            .split('\n')
            .map(|line| {
                let (opponent, me) = parse_line(line);
                let outcome = Outcome::from_moves(opponent, me);
                me.get_score() + outcome.get_score()
            })
            .sum()
    }

    fn part2(&self, input: &str) -> u64 {
        fn parse_line(line: &str) -> (Move, Outcome) {
            let mut parts = line.chars();
            let (opponent, me) = (parts.next().unwrap(), parts.nth(1).unwrap());
            (Move::from_char(opponent), Outcome::from_char(me))
        }

        input
            .trim()
            .split('\n')
            .map(|line| {
                let (opponent, outcome) = parse_line(line);
                let me = match (opponent, outcome) {
                    (Move::R, Outcome::L) => Move::S,
                    (Move::P, Outcome::L) => Move::R,
                    (Move::S, Outcome::L) => Move::P,
                    (a, Outcome::D) => a,
                    (Move::R, Outcome::W) => Move::P,
                    (Move::P, Outcome::W) => Move::S,
                    (Move::S, Outcome::W) => Move::R,
                };
                me.get_score() + outcome.get_score()
            })
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "A Y\nB X\nC Z";

    #[test]
    fn part1() {
        assert_eq!(Day2.part1(TEST_INPUT), 15);
    }

    #[test]
    fn part2() {
        assert_eq!(Day2.part2(TEST_INPUT), 12);
    }
}
