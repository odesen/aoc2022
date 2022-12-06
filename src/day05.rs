use regex::Regex;

use crate::{read_file_to_string, solve, BoxResult};

fn part_one(input: &str) -> BoxResult<String> {
    let (stack_str, procedures) = input.split_once("\n\n").unwrap();
    let stack_lines: Vec<_> = stack_str.lines().collect();
    let stack_count = stack_lines.last().unwrap().split_ascii_whitespace().count();
    let mut stacks: Vec<Vec<char>> = vec![vec![]; stack_count];
    stack_lines
        .iter()
        .rev()
        .flat_map(|row| {
            row.chars()
                .skip(1)
                .step_by(4)
                .enumerate()
                .filter(|(_, c)| !c.is_ascii_whitespace())
        })
        .for_each(|(i, c)| stacks[i].push(c));
    let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    procedures.lines().for_each(|line| {
        if let Some(caps) = re.captures(&line) {
            let (first, second, third) = (
                match_to_usize(caps.get(1).unwrap()),
                match_to_usize(caps.get(2).unwrap()),
                match_to_usize(caps.get(3).unwrap()),
            );
            (0..first).for_each(|_i| {
                if let Some(val) = stacks[second - 1].pop() {
                    stacks[third - 1].push(val);
                }
            });
        }
    });
    let res = stacks.iter().fold(String::from(""), |acc, arr| {
        acc + &arr.last().unwrap().to_string()
    });
    Ok(res)
}

fn match_to_usize(m: regex::Match) -> usize {
    m.as_str().parse::<usize>().unwrap()
}

fn part_two(input: &str) -> BoxResult<String> {
    let (stack_str, procedures) = input.split_once("\n\n").unwrap();
    let stack_lines: Vec<_> = stack_str.lines().collect();
    let stack_count = stack_lines.last().unwrap().split_ascii_whitespace().count();
    let mut stacks: Vec<Vec<char>> = vec![vec![]; stack_count];
    stack_lines
        .iter()
        .rev()
        .flat_map(|row| {
            row.chars()
                .skip(1)
                .step_by(4)
                .enumerate()
                .filter(|(_, c)| !c.is_ascii_whitespace())
        })
        .for_each(|(i, c)| stacks[i].push(c));
    let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    procedures.lines().for_each(|line| {
        if let Some(caps) = re.captures(&line) {
            let (first, second, third) = (
                match_to_usize(caps.get(1).unwrap()),
                match_to_usize(caps.get(2).unwrap()),
                match_to_usize(caps.get(3).unwrap()),
            );
            let range_drain = stacks[second - 1].len() - first;
            let dra: Vec<char> = stacks[second - 1].drain(range_drain..).collect();
            dra.iter().for_each(|v| stacks[third - 1].push(*v));
        }
    });
    let res = stacks.iter().fold(String::from(""), |acc, arr| {
        acc + &arr.last().unwrap().to_string()
    });
    Ok(res)
}

pub fn run() {
    let input = &read_file_to_string("./inputs/day05.txt");
    solve!(1, part_one, input);
    solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_part_one() {
        let input = &read_file_to_string("./examples/day05.txt");
        assert_eq!(part_one(input).ok(), Some(String::from("CMZ")));
    }

    #[test]
    fn test_part_two() {
        let input = &read_file_to_string("./examples/day05.txt");
        assert_eq!(part_two(input).ok(), Some(String::from("MCD")));
    }
}
