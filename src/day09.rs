use itertools::Itertools;
use std::collections::HashSet;
use std::ops;

use crate::{read_file_to_string, solve, AOCError, BoxResult};

#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
struct Point {
    x: isize,
    y: isize,
}

impl ops::Sub<Point> for Point {
    type Output = Point;

    fn sub(self, rhs: Point) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

struct RopeV1 {
    head: Point,
    tail: Point,
}

impl RopeV1 {
    fn default() -> RopeV1 {
        RopeV1 {
            head: Point { x: 0, y: 0 },
            tail: Point { x: 0, y: 0 },
        }
    }

    fn move_tail(&mut self) {
        let p_diff: Point = self.head - self.tail;

        if p_diff.x.abs() > 1 && p_diff.y == 0 {
            self.tail.x += p_diff.x.signum();
        } else if p_diff.y.abs() > 1 && p_diff.x == 0 {
            self.tail.y += p_diff.y.signum();
        } else if p_diff.x.abs() + p_diff.y.abs() > 2 {
            self.tail.x += p_diff.x.signum();
            self.tail.y += p_diff.y.signum();
        }
    }

    fn up(&mut self) {
        self.head.y += 1;
        self.move_tail();
    }
    fn left(&mut self) {
        self.head.x -= 1;
        self.move_tail();
    }
    fn right(&mut self) {
        self.head.x += 1;
        self.move_tail();
    }
    fn down(&mut self) {
        self.head.y -= 1;
        self.move_tail();
    }
}

struct RopeV2 {
    head: Point,
    tails: [Point; 9],
}

impl RopeV2 {
    fn default() -> RopeV2 {
        RopeV2 {
            head: Point { x: 0, y: 0 },
            tails: [Point { x: 0, y: 0 }; 9],
        }
    }

    fn lachenille(&mut self) {
        for i in 0..9 {
            let p_before: Point = if i == 0 { self.head } else { self.tails[i - 1] };
            let p_diff: Point = p_before - self.tails[i];

            if p_diff.x.abs() > 1 && p_diff.y == 0 {
                self.tails[i].x += p_diff.x.signum();
            } else if p_diff.y.abs() > 1 && p_diff.x == 0 {
                self.tails[i].y += p_diff.y.signum();
            } else if p_diff.x.abs() + p_diff.y.abs() > 2 {
                self.tails[i].x += p_diff.x.signum();
                self.tails[i].y += p_diff.y.signum();
            }
        }
    }

    fn up(&mut self) {
        self.head.y += 1;
        self.lachenille();
    }

    fn left(&mut self) {
        self.head.x -= 1;
        self.lachenille();
    }

    fn right(&mut self) {
        self.head.x += 1;
        self.lachenille();
    }

    fn down(&mut self) {
        self.head.y -= 1;
        self.lachenille();
    }
}

fn part_one(input: &str) -> BoxResult<usize> {
    let mut res: HashSet<Point> = HashSet::new();
    let mut rope = RopeV1::default();
    input.lines().for_each(|line| {
        if let Some((direction, moves)) = line.split_ascii_whitespace().collect_tuple() {
            let n_moves = moves.parse::<isize>().unwrap();
            match direction {
                "R" => {
                    for _ in 0..n_moves {
                        rope.right();
                        res.insert(rope.tail);
                    }
                }
                "U" => {
                    for _ in 0..n_moves {
                        rope.up();
                        res.insert(rope.tail);
                    }
                }
                "L" => {
                    for _ in 0..n_moves {
                        rope.left();
                        res.insert(rope.tail);
                    }
                }
                "D" => {
                    for _ in 0..n_moves {
                        rope.down();
                        res.insert(rope.tail);
                    }
                }
                _ => println!("{}", AOCError),
            }
        }
    });
    Ok(res.len())
}

fn part_two(input: &str) -> BoxResult<usize> {
    let mut res: HashSet<Point> = HashSet::new();
    let mut rope = RopeV2::default();
    input.lines().for_each(|line| {
        if let Some((direction, moves)) = line.split_ascii_whitespace().collect_tuple() {
            let n_moves = moves.parse::<isize>().unwrap();
            match direction {
                "R" => {
                    for _ in 0..n_moves {
                        rope.right();
                        res.insert(rope.tails[8]);
                    }
                }
                "U" => {
                    for _ in 0..n_moves {
                        rope.up();
                        res.insert(rope.tails[8]);
                    }
                }
                "L" => {
                    for _ in 0..n_moves {
                        rope.left();
                        res.insert(rope.tails[8]);
                    }
                }
                "D" => {
                    for _ in 0..n_moves {
                        rope.down();
                        res.insert(rope.tails[8]);
                    }
                }
                _ => println!("{}", AOCError),
            }
        }
    });
    Ok(res.len())
}

pub fn run() {
    let input = &read_file_to_string("./inputs/day09.txt");
    solve!(1, part_one, input);
    solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_part_one() {
        let input = &read_file_to_string("./examples/day09.txt");
        assert_eq!(part_one(input).ok(), Some(13));
    }

    #[test]
    fn test_part_two() {
        let input = &read_file_to_string("./examples/day09bis.txt");
        assert_eq!(part_two(input).ok(), Some(36));
    }
}
