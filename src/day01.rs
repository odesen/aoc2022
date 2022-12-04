use std::{cmp::Reverse, collections::BinaryHeap};

use crate::{read_file_to_string, solve, BoxResult};

fn part_one(input: &str) -> BoxResult<usize> {
    let mut max_calories: usize = 0;
    let mut caloriers_per_person: usize = 0;
    input.lines().for_each(|line| match line.parse::<usize>() {
        Ok(n) => caloriers_per_person += n,
        Err(_) => {
            if max_calories < caloriers_per_person {
                max_calories = caloriers_per_person
            }
            caloriers_per_person = 0
        }
    });
    Ok(max_calories)
}

fn part_two(input: &str) -> BoxResult<usize> {
    let mut three_max_calories = BinaryHeap::with_capacity(4);
    let mut caloriers_per_person: usize = 0;
    input.lines().for_each(|line| match line.parse::<usize>() {
        Ok(n) => caloriers_per_person += n,
        Err(_) => {
            three_max_calories.push(Reverse(caloriers_per_person));
            if three_max_calories.len() > 3 {
                three_max_calories.pop();
            }
            caloriers_per_person = 0
        }
    });
    Ok(three_max_calories.into_iter().map(|rev| rev.0).sum())
}

pub fn run() {
    let input = &read_file_to_string("./inputs/day1.txt");
    solve!(1, part_one, input);
    solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = &read_file_to_string("./examples/day1.txt");
        assert_eq!(part_one(input).ok(), Some(24000));
    }

    #[test]
    fn test_part_two() {
        let input = &read_file_to_string("./examples/day1.txt");
        assert_eq!(part_two(input).ok(), Some(45000));
    }
}
