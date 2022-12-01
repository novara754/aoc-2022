use harness::PuzzleSolution;

mod day1;
mod harness;

fn main() {
    let days: [&dyn PuzzleSolution; 1] = [&day1::Day1];
    for (idx, solution) in days.iter().enumerate() {
        let input = std::fs::read_to_string(format!("inputs/day{}.txt", idx + 1)).unwrap();
        println!(
            "Day {}\n\tPart 1: {}\n\tPart 2: {}",
            idx + 1,
            solution.part1(&input),
            solution.part2(&input)
        );
    }
}
