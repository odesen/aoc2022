use crate::{read_file_to_string, solve, AOCError, BoxResult};
use itertools::{self, Itertools};

fn get_char_value(byte: u8) -> Result<usize, AOCError> {
    match byte {
        b'a'..=b'z' => Ok((1 + (byte - b'a')) as usize),
        b'A'..=b'Z' => Ok((27 + (byte - b'A')) as usize),
        _ => Err(AOCError),
    }
}

fn part_one(input: &str) -> BoxResult<usize> {
    input
        .lines()
        .map(|line| {
            let (first, last) = line.split_at(line.len() / 2);
            let duplicate = first
                .bytes()
                .find(|x| last.as_bytes().contains(x))
                .ok_or(AOCError)?;
            Ok(get_char_value(duplicate)?)
        })
        .sum()
}

fn part_two(input: &str) -> BoxResult<usize> {
    input
        .lines()
        .tuples()
        .map(|(line1, line2, line3)| {
            let duplicate = line1
                .bytes()
                .filter(|x| line2.as_bytes().contains(x))
                .find(|x| line3.as_bytes().contains(x))
                .ok_or(AOCError)?;
            Ok(get_char_value(duplicate)?)
        })
        .sum()
}

pub fn run() {
    let input = &read_file_to_string("./inputs/day03.txt");
    solve!(1, part_one, input);
    solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_part_one() {
        let input = &read_file_to_string("./examples/day03.txt");
        assert_eq!(part_one(input).ok(), Some(157));
    }

    #[test]
    fn test_part_two() {
        let input = &read_file_to_string("./examples/day03.txt");
        assert_eq!(part_two(input).ok(), Some(70));
    }
}
