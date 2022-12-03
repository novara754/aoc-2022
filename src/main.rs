use std::time::Instant;

use harness::PuzzleSolution;

mod day1;
mod day2;
mod day3;
mod harness;

fn main() {
    let days: [&dyn PuzzleSolution; 3] = [&day1::Day1, &day2::Day2, &day3::Day3];
    for (idx, solution) in days.iter().enumerate() {
        let input = std::fs::read_to_string(format!("inputs/day{}.txt", idx + 1)).unwrap();

        let (part1, part1_time) = {
            let start = Instant::now();
            let part1 = solution.part1(&input);
            (part1, start.elapsed())
        };
        let (part2, part2_time) = {
            let start = Instant::now();
            let part2 = solution.part2(&input);
            (part2, start.elapsed())
        };

        println!(
            "Day {}\n\tPart 1: {}\t({} ms)\n\tPart 2: {}\t({} ms)",
            idx + 1,
            part1,
            part1_time.as_micros() as f64 / 1000.0,
            part2,
            part2_time.as_micros() as f64 / 1000.0
        );
    }
}
