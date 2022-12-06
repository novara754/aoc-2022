use std::collections::HashSet;

use crate::harness::PuzzleSolution;

pub struct Day6;

fn find_marker(input: &str, marker_length: usize) -> u64 {
    input
        .as_bytes()
        .windows(marker_length)
        .enumerate()
        .find_map(|(idx, window)| {
            let set: HashSet<_> = window.iter().collect();
            if set.len() == marker_length {
                Some(idx + marker_length)
            } else {
                None
            }
        })
        .unwrap() as u64
}

impl PuzzleSolution for Day6 {
    type Output = u64;

    fn part1(&self, input: &str) -> u64 {
        find_marker(input, 4)
    }

    fn part2(&self, input: &str) -> u64 {
        find_marker(input, 14)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(Day6.part1("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
        assert_eq!(Day6.part1("nppdvjthqldpwncqszvftbrmjlhg"), 6);
        assert_eq!(Day6.part1("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
        assert_eq!(Day6.part1("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);
    }

    #[test]
    fn part2() {
        assert_eq!(Day6.part2("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 19);
        assert_eq!(Day6.part2("bvwbjplbgvbhsrlpgdmjqwftvncz"), 23);
        assert_eq!(Day6.part2("nppdvjthqldpwncqszvftbrmjlhg"), 23);
        assert_eq!(Day6.part2("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 29);
        assert_eq!(Day6.part2("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 26);
    }
}
