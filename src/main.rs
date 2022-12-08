use std::{fmt::Display, time::Instant};

use harness::PuzzleSolution;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod harness;

fn print_solution(day_num: usize, solution: impl PuzzleSolution<Output = impl Display>) {
    let input = std::fs::read_to_string(format!("inputs/day{}.txt", day_num)).unwrap();

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
        day_num,
        part1,
        part1_time.as_micros() as f64 / 1000.0,
        part2,
        part2_time.as_micros() as f64 / 1000.0
    );
}

macro_rules! print_solutions {
    (impl $day:expr => $solution:expr) => {{
        print_solution($day, $solution);
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
    if let Some(day) = std::env::args().nth(1) {
        let day = day.parse().unwrap();
        match day {
            1 => print_solution(day, day1::Day1),
            2 => print_solution(day, day2::Day2),
            3 => print_solution(day, day3::Day3),
            4 => print_solution(day, day4::Day4),
            5 => print_solution(day, day5::Day5),
            6 => print_solution(day, day6::Day6),
            7 => print_solution(day, day7::Day7),
            8 => print_solution(day, day8::Day8),
            _ => panic!("invalid day"),
        }
    } else {
        print_solutions!(
            1 => day1::Day1;
            2 => day2::Day2;
            3 => day3::Day3;
            4 => day4::Day4;
            5 => day5::Day5;
            6 => day6::Day6;
            7 => day7::Day7;
            8 => day8::Day8;
        )
    }
}
