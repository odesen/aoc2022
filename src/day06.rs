use std::collections::HashSet;

use crate::{read_file_to_string, solve, AOCError, BoxResult};

fn part_one(input: &str) -> BoxResult<usize> {
    let chars: Vec<char> = input.chars().collect();
    for i in 0..chars.len() {
        let sl: HashSet<&char> = HashSet::from_iter(&chars[i..i + 4]);
        if sl.len() == 4 {
            return Ok(i + 4);
        }
    }
    Err(Box::new(AOCError))
}

fn part_two(input: &str) -> BoxResult<usize> {
    let chars: Vec<char> = input.chars().collect();
    for i in 0..chars.len() {
        let sl: HashSet<&char> = HashSet::from_iter(&chars[i..i + 14]);
        if sl.len() == 14 {
            return Ok(i + 14);
        }
    }
    Err(Box::new(AOCError))
}

pub fn run() {
    let input = &read_file_to_string("./inputs/day06.txt");
    solve!(1, part_one, input);
    solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_part_one() {
        let input = &read_file_to_string("./examples/day06.txt");
        assert_eq!(part_one(input).ok(), Some(5));
    }

    #[test]
    fn test_part_two() {
        let input = &read_file_to_string("./examples/day06.txt");
        assert_eq!(part_two(input).ok(), Some(23));
    }
}
