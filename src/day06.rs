use std::collections::HashSet;

use crate::{read_file_to_string, solve, BoxResult};

fn part_one(input: &str) -> BoxResult<usize> {
    let chars: Vec<char> = input.chars().collect();
    let mut i = 0;
    let mut f = false;
    let mut res = 0;
    while i < (chars.len() - 4) && !f {
        let set: HashSet<&char> = HashSet::from_iter(&chars[i..i + 4]);
        if set.len() == 4 {
            f = true;
            res = i + 4
        }
        i += 1;
    }
    Ok(res)
}

fn part_two(input: &str) -> BoxResult<usize> {
    let chars: Vec<char> = input.chars().collect();
    let mut i = 0;
    let mut f = false;
    let mut res = 0;
    while i < (chars.len() - 14) && !f {
        let set: HashSet<&char> = HashSet::from_iter(&chars[i..i + 14]);
        if set.len() == 14 {
            f = true;
            res = i + 14
        }
        i += 1;
    }
    Ok(res)
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
