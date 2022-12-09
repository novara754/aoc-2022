use std::collections::HashSet;

use crate::harness::PuzzleSolution;

pub struct Day9;

fn _print_rope(head: (i64, i64), tails: &[(i64, i64)]) {
    let mut min_x = (-15).min(head.0);
    let mut max_x = 15.max(head.0);
    let mut min_y = (-15).min(head.1);
    let mut max_y = 15.max(head.1);

    for (x, y) in tails {
        min_x = min_x.min(*x);
        max_x = max_x.max(*x);
        min_y = min_y.min(*y);
        max_y = max_y.max(*y);
    }

    for y in min_y..max_y {
        for x in min_x..max_x {
            if (x, y) == head {
                eprint!("H")
            } else if let Some(idx) =
                tails
                    .iter()
                    .enumerate()
                    .find_map(|(idx, tail)| if *tail == (x, y) { Some(idx) } else { None })
            {
                eprint!("{}", idx + 1);
            } else if (x, y) == (0, 0) {
                eprint!("s");
            } else {
                eprint!(".");
            }
        }
        eprintln!();
    }
}

impl PuzzleSolution for Day9 {
    type Output = u64;

    fn part1(&self, input: &str) -> u64 {
        let mut head_position: (i64, i64) = (0, 0);
        let mut tail_position: (i64, i64) = (0, 0);

        let mut points_visited = HashSet::<(i64, i64)>::new();
        points_visited.insert(tail_position);

        for movement in input.trim().lines() {
            let mut parts = movement.split(' ');
            let direction = parts.next().unwrap();
            let distance = parts.next().unwrap().parse().unwrap();

            for _ in 0..distance {
                match direction {
                    "R" => head_position.0 += 1,
                    "L" => head_position.0 -= 1,
                    "U" => head_position.1 += 1,
                    "D" => head_position.1 -= 1,
                    _ => unreachable!(),
                }

                let mut dx = head_position.0 - tail_position.0;
                let mut dy = head_position.1 - tail_position.1;
                if dx.abs() == 1 && dy.abs() == 2 {
                    dx = 0;
                    dy = dy.signum();
                } else if dx.abs() == 2 && dy.abs() == 1 {
                    dx = dx.signum();
                    dy = 0;
                } else if dx.abs() == 2 && dy == 0 {
                    dx = dx.signum();
                } else if dx == 0 && dy.abs() == 2 {
                    dy = dy.signum();
                }
                tail_position.0 = head_position.0 - dx;
                tail_position.1 = head_position.1 - dy;

                points_visited.insert(tail_position);
            }
        }

        points_visited.len().try_into().unwrap()
    }

    fn part2(&self, input: &str) -> u64 {
        let mut head_position: (i64, i64) = (0, 0);
        let mut tail_positions: [(i64, i64); 9] = [(0, 0); 9];

        let mut points_visited = HashSet::<(i64, i64)>::new();
        points_visited.insert(tail_positions[8]);

        for movement in input.trim().lines() {
            let mut parts = movement.split(' ');
            let direction = parts.next().unwrap();
            let distance = parts.next().unwrap().parse().unwrap();

            for _ in 0..distance {
                match direction {
                    "R" => head_position.0 += 1,
                    "L" => head_position.0 -= 1,
                    "U" => head_position.1 += 1,
                    "D" => head_position.1 -= 1,
                    _ => unreachable!(),
                }

                for i in 0..tail_positions.len() {
                    let pred_position = if i == 0 {
                        head_position
                    } else {
                        tail_positions[i - 1]
                    };

                    let mut dx = pred_position.0 - tail_positions[i].0;
                    let mut dy = pred_position.1 - tail_positions[i].1;

                    if dx.abs() >= 2 && dy.abs() >= 2 {
                        dx = dx.signum();
                        dy = dy.signum();
                    } else if dx.abs() == 1 && dy.abs() == 2 {
                        dx = 0;
                        dy = dy.signum();
                    } else if dx.abs() == 2 && dy.abs() == 1 {
                        dx = dx.signum();
                        dy = 0;
                    } else if dx.abs() == 2 && dy == 0 {
                        dx = dx.signum();
                    } else if dx == 0 && dy.abs() == 2 {
                        dy = dy.signum();
                    }

                    tail_positions[i].0 = pred_position.0 - dx;
                    tail_positions[i].1 = pred_position.1 - dy;
                }
                points_visited.insert(tail_positions[8]);
            }
        }

        points_visited.len().try_into().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let test_input = r#"R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2"#;
        assert_eq!(Day9.part1(test_input), 13);
    }

    #[test]
    fn part2() {
        let test_input = r#"R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20"#;
        assert_eq!(Day9.part2(test_input), 36);
    }
}
