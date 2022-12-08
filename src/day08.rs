use crate::{read_file_to_string, solve, BoxResult};
use itertools::FoldWhile::{Continue, Done};
use itertools::Itertools;
use std::cmp;
fn part_one(input: &str) -> BoxResult<usize> {
    let trees: Vec<Vec<usize>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|char| char.to_digit(10).unwrap() as usize)
                .collect::<Vec<usize>>()
        })
        .collect();
    let mut count_visible_trees: usize = 0;
    trees.iter().enumerate().for_each(|(y, row)| {
        row.iter().enumerate().for_each(|(x, value)| {
            let column = trees.iter().map(|r| r[x]).collect::<Vec<usize>>();
            if x == 0 || y == 0 || x == row.len() - 1 || y == column.len() - 1 {
                count_visible_trees += 1;
            } else if value > row[..x].iter().max().unwrap()
                || value > row[x + 1..].iter().max().unwrap()
            {
                count_visible_trees += 1;
            } else if value > column[..y].iter().max().unwrap()
                || value > column[y + 1..].iter().max().unwrap()
            {
                count_visible_trees += 1;
            }
        });
    });
    Ok(count_visible_trees)
}

fn part_two(input: &str) -> BoxResult<usize> {
    let trees: Vec<Vec<usize>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|char| char.to_digit(10).unwrap() as usize)
                .collect::<Vec<usize>>()
        })
        .collect();
    let mut highest_scenic_score: usize = 0;
    trees.iter().enumerate().for_each(|(y, row)| {
        row.iter().enumerate().for_each(|(x, value)| {
            let column = trees.iter().map(|r| r[x]).collect::<Vec<usize>>();
            let left = row[..x]
                .iter()
                .rev()
                .fold_while(0usize, |acc, xi| {
                    if xi >= value {
                        Done(acc + 1)
                    } else {
                        Continue(acc + 1)
                    }
                })
                .into_inner();
            let right = row[x + 1..]
                .iter()
                .fold_while(0usize, |acc, xi| {
                    if xi >= value {
                        Done(acc + 1)
                    } else {
                        Continue(acc + 1)
                    }
                })
                .into_inner();
            let up = column[..y]
                .iter()
                .rev()
                .fold_while(0usize, |acc, yi| {
                    if yi >= value {
                        Done(acc + 1)
                    } else {
                        Continue(acc + 1)
                    }
                })
                .into_inner();
            let down = column[y + 1..]
                .iter()
                .fold_while(0usize, |acc, yi| {
                    if yi >= value {
                        Done(acc + 1)
                    } else {
                        Continue(acc + 1)
                    }
                })
                .into_inner();
            highest_scenic_score = cmp::max(highest_scenic_score, left * right * up * down)
        });
    });
    Ok(highest_scenic_score)
}

pub fn run() {
    let input = &read_file_to_string("./inputs/day08.txt");
    solve!(1, part_one, input);
    solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_part_one() {
        let input = &read_file_to_string("./examples/day08.txt");
        assert_eq!(part_one(input).ok(), Some(21));
    }

    #[test]
    fn test_part_two() {
        let input = &read_file_to_string("./examples/day08.txt");
        assert_eq!(part_two(input).ok(), Some(8));
    }
}
