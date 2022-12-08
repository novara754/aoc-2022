use std::collections::HashSet;

use crate::harness::PuzzleSolution;

pub struct Day8;

impl PuzzleSolution for Day8 {
    type Output = u64;

    fn part1(&self, input: &str) -> u64 {
        let grid: Vec<Vec<_>> = input
            .lines()
            .map(|line| line.as_bytes().iter().map(|b| (*b - b'0') as i16).collect())
            .collect();

        let mut transposed_grid = vec![vec![0; grid.len()]; grid[0].len()];

        let mut visible = HashSet::<(usize, usize)>::new();

        for (y, row) in grid.iter().enumerate() {
            {
                let mut tallest = -1;
                for (x, &tree) in row.iter().enumerate() {
                    transposed_grid[x][y] = tree;
                    if tallest < tree {
                        tallest = tree;
                        visible.insert((x, y));
                    }
                }
            }
            {
                let mut tallest = -1;
                for (x, &tree) in row.iter().enumerate().rev() {
                    if tallest < tree {
                        tallest = tree;
                        visible.insert((x, y));
                    }
                }
            }
        }

        for (x, col) in transposed_grid.iter().enumerate() {
            {
                let mut tallest = -1;
                for (y, &tree) in col.iter().enumerate() {
                    if tallest < tree {
                        tallest = tree;
                        visible.insert((x, y));
                    }
                }
            }
            {
                let mut tallest = -1;
                for (y, &tree) in col.iter().enumerate().rev() {
                    if tallest < tree {
                        tallest = tree;
                        visible.insert((x, y));
                    }
                }
            }
        }

        visible.len().try_into().unwrap()
    }

    fn part2(&self, input: &str) -> u64 {
        let grid: Vec<Vec<_>> = input
            .lines()
            .map(|line| line.as_bytes().iter().map(|b| (*b - b'0') as i16).collect())
            .collect();

        let mut heighest_score = 0;
        for y in 0..grid.len() {
            for x in 0..grid[0].len() {
                let own_height = grid[y][x];

                let mut right_score = 0;
                {
                    for dx in 1..grid[0].len() {
                        let x = x + dx;
                        if x >= grid[0].len() {
                            break;
                        }
                        right_score += 1;
                        if grid[y][x] >= own_height {
                            break;
                        }
                    }
                }

                let mut left_score = 0;
                {
                    for dx in 1..grid[0].len() {
                        let x = x as isize - dx as isize;
                        if x < 0 {
                            break;
                        }
                        left_score += 1;
                        if grid[y][x as usize] >= own_height {
                            break;
                        }
                    }
                }

                let mut up_score = 0;
                {
                    for dy in 1..grid.len() {
                        let y = y + dy;
                        if y >= grid.len() {
                            break;
                        }
                        up_score += 1;
                        if grid[y][x] >= own_height {
                            break;
                        }
                    }
                }

                let mut down_score = 0;
                {
                    for dy in 1..grid.len() {
                        let y = y as isize - dy as isize;
                        if y < 0 {
                            break;
                        }
                        down_score += 1;
                        if grid[y as usize][x] >= own_height {
                            break;
                        }
                    }
                }

                heighest_score =
                    heighest_score.max(right_score * left_score * up_score * down_score);
            }
        }

        heighest_score
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r#"30373
25512
65332
33549
35390"#;

    #[test]
    fn part1() {
        assert_eq!(Day8.part1(TEST_INPUT), 21);
    }

    #[test]
    fn part2() {
        assert_eq!(Day8.part2(TEST_INPUT), 8);
    }
}
