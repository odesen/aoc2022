use std::collections::BinaryHeap;
use std::ops::Add;
use std::{cmp::Ordering, collections::BTreeMap};

use crate::{read_file_to_string, solve, AOCError, BoxResult};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    fn neighbors(self) -> Vec<Point> {
        Vec::from_iter(
            [
                Point::new(-1, 0),
                Point::new(1, 0),
                Point::new(0, -1),
                Point::new(0, 1),
            ]
            .iter()
            .map(|m| self + m.clone()),
        )
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct Node {
    point: Point,
    cost: usize,
}

impl Node {
    fn new(point: Point, cost: usize) -> Self {
        Self { point, cost }
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    point: Point,
    cost: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.point.cmp(&other.point))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// Dijkstra's shortest path algorithm.

fn shortest_path(graph: &BTreeMap<Point, Node>, start: &Point, end: &Point) -> Option<usize> {
    let mut dist: BTreeMap<Point, usize> =
        BTreeMap::from_iter(graph.keys().map(|p| (p.clone(), usize::MAX)));

    let mut heap: BinaryHeap<State> = BinaryHeap::new();

    dist.entry(start.clone()).and_modify(|c| *c = 0);
    heap.push(State {
        point: start.clone(),
        cost: 0,
    });

    while let Some(State { point, cost }) = heap.pop() {
        // println!("{:?}", dist);
        if point == *end {
            return Some(cost);
        }

        if cost > *dist.get(&point).unwrap() {
            continue;
        }
        if let Some(pos) = graph.get(&point) {
            for edge in pos.point.neighbors() {
                if let Some(neighbor) = graph.get(&edge) {
                    if pos.cost as isize - neighbor.cost as isize >= -1 {
                        let next = State {
                            point: neighbor.point,
                            cost: cost + 1,
                        };

                        if next.cost < *dist.get(&next.point).unwrap() {
                            heap.push(next);
                            dist.entry(next.point)
                                .and_modify(|c| *c = next.cost)
                                .or_insert(next.cost);
                        }
                    }
                }
            }
        }
    }

    None
}

fn build_graph(input: &str) -> (BTreeMap<Point, Node>, Point, Point) {
    let mut start: Option<Point> = None;
    let mut end: Option<Point> = None;
    let mut graph: BTreeMap<Point, Node> = BTreeMap::new();

    input.lines().enumerate().for_each(|(n_line, line)| {
        line.chars().enumerate().for_each(|(n_char, char)| {
            let p = Point::new(n_char as isize, n_line as isize);
            match char {
                'S' => {
                    start = Some(p);
                    graph.entry(p).or_insert(Node::new(p, 'a' as usize - 97));
                }
                'E' => {
                    end = Some(p);
                    graph.entry(p).or_insert(Node::new(p, 'z' as usize - 97));
                }
                c if c.is_ascii_alphabetic() => {
                    graph.entry(p).or_insert(Node::new(p, char as usize - 97));
                }
                _ => println!("{}", AOCError),
            }
        })
    });

    (graph, start.unwrap(), end.unwrap())
}

fn part_one(input: &str) -> BoxResult<usize> {
    let (graph, start, end) = build_graph(input);
    let res = shortest_path(&graph, &start, &end);
    Ok(res.unwrap())
}

fn part_two(input: &str) -> BoxResult<usize> {
    let (graph, _start, end) = build_graph(input);
    let res = graph
        .values()
        .filter(|n| n.cost == 0)
        .filter_map(|p| shortest_path(&graph, &p.point, &end))
        .min();
    Ok(res.unwrap())
}

pub fn run() {
    let input = &read_file_to_string("./inputs/day12.txt");
    solve!(1, part_one, input);
    solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_part_one() {
        let input = &read_file_to_string("./examples/day12.txt");
        assert_eq!(part_one(input).ok(), Some(31));
    }

    #[test]
    fn test_part_two() {
        let input = &read_file_to_string("./examples/day12.txt");
        assert_eq!(part_two(input).ok(), Some(29));
    }
}
