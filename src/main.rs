use std::time::Instant;

use harness::PuzzleSolution;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod harness;

macro_rules! print_solutions {
    (impl $day:expr => $solution:expr) => {{
        let input = std::fs::read_to_string(format!("inputs/day{}.txt", $day)).unwrap();

        let (part1, part1_time) = {
            let start = Instant::now();
            let part1 = $solution.part1(&input);
            (part1, start.elapsed())
        };
        let (part2, part2_time) = {
            let start = Instant::now();
            let part2 = $solution.part2(&input);
            (part2, start.elapsed())
        };

        println!(
            "Day {}\n\tPart 1: {}\t({} ms)\n\tPart 2: {}\t({} ms)",
            $day,
            part1,
            part1_time.as_micros() as f64 / 1000.0,
            part2,
            part2_time.as_micros() as f64 / 1000.0
        );
    }};

    ($day:expr => $solution:expr;) => {{
        print_solutions!(impl $day => $solution);
    }};

    ($day:expr => $solution:expr; $($rest:tt)+) => {{
        print_solutions!(impl $day => $solution);
        print_solutions!($($rest)*);
    }};
}

fn main() {
    print_solutions!(
        1 => day1::Day1;
        2 => day2::Day2;
        3 => day3::Day3;
        4 => day4::Day4;
        5 => day5::Day5;
    )
}
