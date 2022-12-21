use std::cmp;

use regex::Regex;

use crate::{read_file_to_string, AOCError, BoxResult};

#[derive(Clone, Copy, Debug)]
struct Sensor {
    location: Point,
    closest_beacon: Point,
}

struct Zone {
    min: isize,
    max: isize,
    sensors: Vec<Sensor>,
}

impl Zone {
    fn coverage_on_y(&self, y: isize) -> Ranges {
        let mut ranges = Ranges { ranges: vec![] };
        for s in &self.sensors {
            let distance = s.location.manhattan_distance(s.closest_beacon) as isize;
            let offset = s.location.y.abs_diff(y) as isize;
            let remaining: isize = distance - offset;

            if remaining <= 0 {
                continue;
            }

            let mut min_x = cmp::max(s.location.x - remaining, self.min);
            let mut max_x = cmp::min(s.location.x + remaining, self.max);

            if s.closest_beacon.y == y && s.closest_beacon.x == min_x {
                min_x += 1;
            }
            if s.closest_beacon.y == y && s.closest_beacon.x == max_x {
                max_x -= 1;
            }

            ranges.union(Range {
                start: min_x,
                end: max_x,
            });
        }

        ranges
    }
}

#[derive(Clone, Copy, Debug)]
struct Range {
    start: isize,
    end: isize,
}

impl Range {
    fn union(self, rhs: Self) -> Option<Self> {
        let self_end = if rhs.start - 1 == self.end {
            self.end + 1
        } else {
            self.end
        };
        let rhs_end = if self.start - 1 == rhs.end {
            rhs.end + 1
        } else {
            rhs.end
        };
        if rhs.start >= self.start && rhs_end <= self_end {
            return Some(Range {
                start: self.start,
                end: self_end,
            });
        }
        if self.start >= rhs.start && self_end <= rhs_end {
            return Some(Range {
                start: rhs.start,
                end: rhs_end,
            });
        }
        if rhs.start >= self.start && rhs.start <= self_end {
            return Some(Range {
                start: self.start,
                end: rhs_end,
            });
        }
        if self.start >= rhs.start && self.start <= rhs_end {
            return Some(Range {
                start: rhs.start,
                end: self_end,
            });
        }

        None
    }
}

#[derive(Clone, Debug)]
struct Ranges {
    ranges: Vec<Range>,
}

impl Ranges {
    fn overlap(&mut self) {
        loop {
            let mut to_merge = None;

            'find_merge: for (i, a) in self.ranges.iter().enumerate() {
                for (j, b) in self.ranges.iter().enumerate() {
                    if i == j {
                        continue;
                    } else if let Some(c) = a.union(*b) {
                        to_merge = Some((i, j, c));
                        break 'find_merge;
                    }
                }
            }

            if let Some((i, j, c)) = to_merge {
                self.ranges.remove(i.max(j));
                self.ranges.remove(i.min(j));
                self.ranges.push(c);
            } else {
                break;
            }
        }
    }

    fn union(&mut self, v: Range) {
        self.ranges.push(v);
        self.overlap();
    }
}

impl From<String> for Sensor {
    fn from(input: String) -> Self {
        let re = Regex::new(
            r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)",
        )
        .unwrap();
        let res = re.captures(&input).unwrap();
        Sensor {
            location: Point {
                x: res.get(1).unwrap().as_str().parse::<isize>().unwrap(),
                y: res.get(2).unwrap().as_str().parse::<isize>().unwrap(),
            },
            closest_beacon: Point {
                x: res.get(3).unwrap().as_str().parse::<isize>().unwrap(),
                y: res.get(4).unwrap().as_str().parse::<isize>().unwrap(),
            },
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn manhattan_distance(self, other: Point) -> usize {
        other.x.abs_diff(self.x) + other.y.abs_diff(self.y)
    }
}

fn part_one(input: &str, y: isize) -> BoxResult<isize> {
    let zone = Zone {
        min: isize::MIN,
        max: isize::MAX,
        sensors: input
            .lines()
            .map(|line| Sensor::from(line.to_string()))
            .collect(),
    };
    Ok(zone
        .coverage_on_y(y)
        .ranges
        .iter()
        .fold(0, |acc, range| acc + 1 + (range.end - range.start)))
}

fn part_two(input: &str, limit: isize) -> BoxResult<isize> {
    let limit: isize = limit;
    let zone: Zone = Zone {
        min: 0,
        max: limit,
        sensors: input
            .lines()
            .map(|line| Sensor::from(line.to_string()))
            .collect(),
    };
    let mut p = None;
    for y in 0..=limit {
        let ranges = zone.coverage_on_y(y);

        if ranges.ranges.len() > 1 {
            let x = ranges
                .ranges
                .into_iter()
                .find(|r| r.start > 0)
                .unwrap()
                .start
                - 1;

            // Check if the candidate is exactly equal to a beacon
            let new_p = Point { x, y };
            if zone.sensors.iter().any(|s| new_p == s.closest_beacon) {
                continue;
            }

            // If not, we found the solution
            p = Some(new_p);
            break;
        }
    }

    match p {
        Some(Point { x, y }) => Ok(x * 4000000 + y),
        None => panic!("{}", AOCError),
    }
}

pub fn run() {
    let input = &read_file_to_string("./examples/day15.txt");
    part_one(input, 2000000).ok();
    part_two(input, 4000000).ok();
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_part_one_example() {
        let input = &read_file_to_string("./examples/day15.txt");
        assert_eq!(part_one(input, 10).ok(), Some(26));
    }

    #[test]
    fn test_part_two_example() {
        let input = &read_file_to_string("./examples/day15.txt");
        assert_eq!(part_two(input, 20).ok(), Some(56000011));
    }

    #[test]
    fn test_part_one() {
        let input = &read_file_to_string("./inputs/day15.txt");
        assert_eq!(part_one(input, 2000000).ok(), Some(5525990));
    }

    #[test]
    fn test_part_two() {
        let input = &read_file_to_string("./inputs/day15.txt");
        assert_eq!(part_two(input, 4000000).ok(), Some(11756174628223));
    }
}
