use crate::harness::PuzzleSolution;

#[derive(Debug)]
struct CpuState<'a> {
    instructions: Vec<&'a str>,
    x_reg: i64,
    pc: usize,
    current_instruction_cycle: u64,
}

impl<'a> CpuState<'a> {
    fn new(input: &'a str) -> Self {
        Self {
            instructions: input.lines().collect(),
            x_reg: 1,
            pc: 0,
            current_instruction_cycle: 0,
        }
    }

    fn run_cycle(&mut self) -> bool {
        if self.pc >= self.instructions.len() {
            return false;
        }

        let mut parts = self.instructions[self.pc].split(' ');
        let mnemonic = parts.next().unwrap();

        match (mnemonic, self.current_instruction_cycle) {
            ("addx", 0) => {
                self.current_instruction_cycle += 1;
            }
            ("addx", 1) => {
                let operand: i64 = parts.next().unwrap().parse().unwrap();
                self.x_reg += operand;

                self.pc += 1;
                self.current_instruction_cycle = 0;
            }
            ("noop", 0) => {
                self.pc += 1;
                self.current_instruction_cycle = 0;
            }
            _ => unreachable!(),
        }

        true
    }
}

pub struct Day10;

impl PuzzleSolution for Day10 {
    type Output = String;

    fn part1(&self, input: &str) -> String {
        let mut cpu = CpuState::new(input);
        let mut total_strengths = 0;

        for cycle in 1.. {
            if [20, 60, 100, 140, 180, 220].iter().any(|c| *c == cycle) {
                total_strengths += cycle * cpu.x_reg;
            }
            if !cpu.run_cycle() {
                break;
            }
        }

        total_strengths.to_string()
    }

    fn part2(&self, input: &str) -> String {
        let mut cpu = CpuState::new(input);

        let mut display_out = [
            String::new(),
            String::new(),
            String::new(),
            String::new(),
            String::new(),
            String::new(),
        ];
        let mut crt_column = 1;
        let mut crt_row = 1;

        for _ in 1.. {
            if (cpu.x_reg - (crt_column - 1)).abs() <= 1 {
                display_out[crt_row - 1].push('#');
            } else {
                display_out[crt_row - 1].push('.');
            }

            crt_column += 1;
            if crt_column == 41 {
                crt_row += 1;
                if crt_row == 7 {
                    crt_row = 1;
                }
                crt_column = 1;
            }

            if !cpu.run_cycle() {
                break;
            }
        }

        "\n".to_owned() + &display_out.join("\n")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r#"addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop"#;

    #[test]
    fn part1() {
        assert_eq!(Day10.part1(TEST_INPUT), "13140");
    }

    #[test]
    fn part2() {
        let out = Day10.part2(TEST_INPUT);
        eprintln!("{out}");
        assert_eq!(
            out,
            r#"\n##..##..##..##..##..##..##..##..##..##...
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######....."#
        );
    }
}
