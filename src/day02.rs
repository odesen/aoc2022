use crate::{read_file_to_string, solve, BoxResult};

const ROCK: usize = 1;
const PAPER: usize = 2;
const SCISSORS: usize = 3;
const WIN: usize = 6;
const DRAW: usize = 3;
const LOSE: usize = 0;

fn part_one(input: &str) -> BoxResult<usize> {
    let res = input.lines().fold(0usize, |sum, line| {
        sum + match line {
            "A X" => ROCK + DRAW,
            "A Y" => PAPER + WIN,
            "A Z" => SCISSORS + LOSE,
            "B X" => ROCK + LOSE,
            "B Y" => PAPER + DRAW,
            "B Z" => SCISSORS + WIN,
            "C X" => ROCK + WIN,
            "C Y" => PAPER + LOSE,
            "C Z" => SCISSORS + DRAW,
            _ => 0,
        }
    });
    Ok(res)
}

fn part_two(input: &str) -> BoxResult<usize> {
    let res = input.lines().fold(0usize, |sum, line| {
        sum + match line {
            "A X" => SCISSORS + LOSE,
            "A Y" => ROCK + DRAW,
            "A Z" => PAPER + WIN,
            "B X" => ROCK + LOSE,
            "B Y" => PAPER + DRAW,
            "B Z" => SCISSORS + WIN,
            "C X" => PAPER + LOSE,
            "C Y" => SCISSORS + DRAW,
            "C Z" => ROCK + WIN,
            _ => 0,
        }
    });
    Ok(res)
}

pub fn run() {
    let input = &read_file_to_string("./inputs/day2.txt");
    solve!(1, part_one, input);
    solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_part_one() {
        let input = &read_file_to_string("./examples/day2.txt");
        assert_eq!(part_one(input).ok(), Some(15));
    }

    #[test]
    fn test_part_two() {
        let input = &read_file_to_string("./examples/day2.txt");
        assert_eq!(part_two(input).ok(), Some(12));
    }
}
