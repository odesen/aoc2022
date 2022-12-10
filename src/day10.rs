use crate::{read_file_to_string, solve, AOCError, BoxResult};

struct CPU {
    cycle: usize,
    register: isize,
    signal: usize,
}

impl CPU {
    fn init() -> CPU {
        CPU {
            cycle: 0,
            register: 1,
            signal: 0,
        }
    }

    fn inst_noop(&mut self) {
        self.inc_cycle();
    }

    fn inst_addx(&mut self, value: isize) {
        self.inc_cycle();
        self.inc_cycle();
        self.register += value;
    }

    fn inc_cycle(&mut self) {
        self.cycle += 1;
        if [20, 60, 100, 140, 180, 220].contains(&self.cycle) {
            self.signal += self.cycle * self.register as usize
        }
    }
}

struct CRT {
    cycle: usize,
    register: isize,
    pixels: Vec<Vec<char>>,
}

impl CRT {
    fn display(self) -> String {
        self.pixels
            .iter()
            .fold(String::from(""), |acc, chars| {
                [acc, chars.iter().collect::<String>(), "\n".to_string()].join("")
            })
            .trim_end()
            .to_string()
    }

    fn inc_cycle(&mut self) {
        self.cycle += 1;
        let sprite_range = self.register - 1..=self.register + 1;
        let row = (self.cycle - 1) / 40;
        let x = (self.cycle - 1) % 40;
        self.pixels[row].insert(
            x,
            if sprite_range.contains(&(x as isize)) {
                '#'
            } else {
                '.'
            },
        );
    }

    fn init() -> CRT {
        CRT {
            cycle: 0,
            register: 1,
            pixels: vec![vec![]; 6],
        }
    }

    fn inst_addx(&mut self, value: isize) {
        self.inc_cycle();
        self.inc_cycle();
        self.register += value;
    }

    fn inst_noop(&mut self) {
        self.inc_cycle();
    }
}

fn part_one(input: &str) -> BoxResult<usize> {
    let mut cpu: CPU = CPU::init();
    input.lines().for_each(|line| match line {
        "noop" => cpu.inst_noop(),
        add if line.starts_with("addx") => {
            if let Some(value) = add.split_ascii_whitespace().last() {
                cpu.inst_addx(value.parse::<isize>().unwrap());
            }
        }
        _ => println!("{}", AOCError),
    });
    Ok(cpu.signal)
}

fn part_two(input: &str) -> BoxResult<String> {
    let mut crt: CRT = CRT::init();
    input.lines().for_each(|line| match line {
        "noop" => crt.inst_noop(),
        add if line.starts_with("addx") => {
            if let Some(value) = add.split_ascii_whitespace().last() {
                crt.inst_addx(value.parse::<isize>().unwrap());
            }
        }
        _ => println!("{}", AOCError),
    });
    Ok(crt.display())
}

pub fn run() {
    let input = &read_file_to_string("./inputs/day10.txt");
    solve!(1, part_one, input);
    solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_part_one() {
        let input = &read_file_to_string("./examples/day10.txt");
        assert_eq!(part_one(input).ok(), Some(13140));
    }

    #[test]
    fn test_part_two() {
        let input = &read_file_to_string("./examples/day10.txt");
        assert_eq!(
            part_two(input).ok(),
            Some(String::from(
                "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######....."
            ))
        );
    }
}
