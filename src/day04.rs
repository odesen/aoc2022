use regex::Regex;

use crate::{read_file_to_string, solve, BoxResult};

fn part_one(input: &str) -> BoxResult<usize> {
    let re = Regex::new(r"(\d+)-(\d+),(\d+)-(\d+)").unwrap();
    let res = input.lines().fold(0usize, |sum, line| {
        let caps = re.captures(&line).unwrap();
        let (first, second, third, last) = (
            match_to_usize(caps.get(1).unwrap()),
            match_to_usize(caps.get(2).unwrap()),
            match_to_usize(caps.get(3).unwrap()),
            match_to_usize(caps.get(4).unwrap()),
        );
        if (first >= third && second <= last) || (first <= third && second >= last) {
            sum + 1
        } else {
            sum
        }
    });
    Ok(res)
}

fn match_to_usize(m: regex::Match) -> usize {
    m.as_str().parse::<usize>().unwrap()
}

fn part_two(input: &str) -> BoxResult<usize> {
    let re = Regex::new(r"(\d+)-(\d+),(\d+)-(\d+)").unwrap();
    let res = input.lines().fold(0usize, |sum, line| {
        let caps = re.captures(&line).unwrap();
        let (first, second, third, last) = (
            match_to_usize(caps.get(1).unwrap()),
            match_to_usize(caps.get(2).unwrap()),
            match_to_usize(caps.get(3).unwrap()),
            match_to_usize(caps.get(4).unwrap()),
        );
        if first <= last && third <= second {
            sum + 1
        } else {
            sum
        }
    });
    Ok(res)
}

pub fn run() {
    let input = &read_file_to_string("./inputs/day4.txt");
    solve!(1, part_one, input);
    solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_part_one() {
        let input = &read_file_to_string("./examples/day4.txt");
        assert_eq!(part_one(input).ok(), Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = &read_file_to_string("./examples/day4.txt");
        assert_eq!(part_two(input).ok(), Some(4));
    }
}
