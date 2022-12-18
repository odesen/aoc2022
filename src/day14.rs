use std::{
    collections::HashSet,
    ops::{Add, Sub},
};

use crate::{read_file_to_string, solve, AOCError, BoxResult};

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn points_between(self, other: Point) -> Vec<Point> {
        let delta = (other - self).signum();
        let res = (0..self.manhattan_distance(other) + 1)
            .map(|i| {
                let p = self + delta.mul(i as isize);
                p
            })
            .collect::<Vec<Point>>();
        res
    }

    fn manhattan_distance(self, other: Point) -> usize {
        other.x.abs_diff(self.x) + other.y.abs_diff(self.y)
    }

    fn signum(self) -> Point {
        Point {
            x: self.x.signum(),
            y: self.y.signum(),
        }
    }

    fn mul(self, x: isize) -> Point {
        Point {
            x: self.x * x,
            y: self.y * x,
        }
    }
}

impl From<String> for Point {
    fn from(input: String) -> Self {
        if let Some((x, y)) = input.split_once(",") {
            Point {
                x: x.parse::<isize>().unwrap(),
                y: y.parse::<isize>().unwrap(),
            }
        } else {
            panic!("{}", AOCError)
        }
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

fn part_one(input: &str) -> BoxResult<usize> {
    let source_sand: Point = Point { x: 500, y: 0 };
    let rocks: HashSet<Point> = input
        .lines()
        .flat_map(|line| {
            line.split("->")
                .zip(line.split("->").skip(1))
                .map(|(p1, p2)| {
                    Point::from(p1.trim().to_string())
                        .points_between(Point::from(p2.trim().to_string()))
                })
        })
        .into_iter()
        .flatten()
        .collect::<HashSet<Point>>();
    let mut sands: HashSet<Point> = HashSet::new();
    let x_max = 550;
    let y_max = 200;

    'outer: loop {
        let mut unit: Point = source_sand.clone();
        '_inner: loop {
            if unit.x == x_max || unit.y == y_max {
                break 'outer;
            }
            let bottom = unit + (Point { x: 0, y: 1 });
            let bottom_left = unit + (Point { x: -1, y: 1 });
            let bottom_right = unit + (Point { x: 1, y: 1 });

            if !rocks.contains(&bottom) && !sands.contains(&bottom) {
                unit = bottom;
                continue;
            } else if !rocks.contains(&bottom_left) && !sands.contains(&bottom_left) {
                unit = bottom_left;
                continue;
            } else if !rocks.contains(&bottom_right) && !sands.contains(&bottom_right) {
                unit = bottom_right;
                continue;
            } else {
                sands.insert(unit);
                break;
            }
        }
    }
    Ok(sands.len())
}

fn part_two(input: &str) -> BoxResult<usize> {
    let source_sand: Point = Point { x: 500, y: 0 };
    let rocks: HashSet<Point> = input
        .lines()
        .flat_map(|line| {
            line.split("->")
                .zip(line.split("->").skip(1))
                .map(|(p1, p2)| {
                    Point::from(p1.trim().to_string())
                        .points_between(Point::from(p2.trim().to_string()))
                })
        })
        .into_iter()
        .flatten()
        .collect::<HashSet<Point>>();
    let mut sands: HashSet<Point> = HashSet::new();
    let (x_max, y_max) = rocks.iter().fold((0isize, 0isize), |(x_max, y_max), &val| {
        (val.x.max(x_max), val.y.max(y_max))
    });
    let floor: HashSet<Point> =
        HashSet::from_iter((Point { x: 0, y: y_max + 2 }).points_between(Point {
            x: x_max * 3,
            y: y_max + 2,
        }));

    'outer: loop {
        let mut unit: Point = source_sand.clone();
        '_inner: loop {
            if sands.contains(&source_sand) {
                break 'outer;
            }
            let bottom = unit + (Point { x: 0, y: 1 });
            let bottom_left = unit + (Point { x: -1, y: 1 });
            let bottom_right = unit + (Point { x: 1, y: 1 });

            if !rocks.contains(&bottom) && !sands.contains(&bottom) && !floor.contains(&bottom) {
                unit = bottom;
                continue;
            } else if !rocks.contains(&bottom_left)
                && !sands.contains(&bottom_left)
                && !floor.contains(&bottom_left)
            {
                unit = bottom_left;
                continue;
            } else if !rocks.contains(&bottom_right)
                && !sands.contains(&bottom_right)
                && !floor.contains(&bottom_right)
            {
                unit = bottom_right;
                continue;
            } else {
                sands.insert(unit);
                break;
            }
        }
    }
    Ok(sands.len())
}

pub fn run() {
    let input = &read_file_to_string("./inputs/day14.txt");
    solve!(1, part_one, input);
    solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_part_one() {
        let input = &read_file_to_string("./examples/day14.txt");
        assert_eq!(part_one(input).ok(), Some(24));
    }

    #[test]
    fn test_part_two() {
        let input = &read_file_to_string("./examples/day14.txt");
        assert_eq!(part_two(input).ok(), Some(93));
    }
}

//6076
