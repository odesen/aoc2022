use std::collections::HashMap;

use itertools::Itertools;

use crate::{read_file_to_string, solve, AOCError, BoxResult};

#[derive(Debug, Clone)]
struct Monkey {
    items: Vec<usize>,
    op: String,
    op_value: usize,
    div_by: usize,
    monkey_if_true: usize,
    monkey_if_false: usize,
    n_checked_items: usize,
}

impl Monkey {
    fn new() -> Monkey {
        Monkey {
            items: vec![],
            op: "".to_string(),
            op_value: 0,
            div_by: 0,
            monkey_if_true: 0,
            monkey_if_false: 0,
            n_checked_items: 0,
        }
    }
}

fn part_one(input: &str) -> BoxResult<usize> {
    let s_monkeys = input.split("\n\n");
    let mut monkeys: HashMap<usize, Monkey> = HashMap::new();

    s_monkeys.enumerate().for_each(|(i, s_monkey)| {
        let mut monkey = Monkey::new();
        s_monkey.lines().skip(1).for_each(|line| match line {
            items if line.trim().starts_with("Starting items:") => {
                monkey.items = items
                    .split(":")
                    .last()
                    .unwrap()
                    .split(",")
                    .map(|value| value.trim().parse::<usize>().unwrap())
                    .collect::<Vec<usize>>();
            }
            operation if line.trim().starts_with("Operation:") => {
                let (value, op) = operation
                    .split_ascii_whitespace()
                    .rev()
                    .take(2)
                    .collect_tuple()
                    .unwrap();
                monkey.op = op.to_string();
                monkey.op_value = if value == "old" {
                    0
                } else {
                    value.parse::<usize>().unwrap()
                };
            }
            divisible if line.trim().starts_with("Test:") => {
                monkey.div_by = divisible
                    .split_ascii_whitespace()
                    .last()
                    .unwrap()
                    .parse::<usize>()
                    .unwrap();
            }
            cond_true if line.trim().starts_with("If true:") => {
                monkey.monkey_if_true = cond_true
                    .split_ascii_whitespace()
                    .last()
                    .unwrap()
                    .parse::<usize>()
                    .unwrap();
            }
            cond_false if line.trim().starts_with("If false:") => {
                monkey.monkey_if_false = cond_false
                    .split_ascii_whitespace()
                    .last()
                    .unwrap()
                    .parse::<usize>()
                    .unwrap();
            }
            _ => println!("{}", AOCError),
        });
        monkeys.insert(i, monkey);
    });

    for _ in 0..20 {
        for i in 0..monkeys.len() {
            monkeys[&i].clone().items.iter().for_each(|v| {
                let res = match monkeys[&i].op.as_str() {
                    "+" => {
                        v + (if monkeys[&i].op_value == 0 {
                            v
                        } else {
                            &monkeys[&i].op_value
                        })
                    }
                    "*" => {
                        v * (if monkeys[&i].op_value == 0 {
                            v
                        } else {
                            &monkeys[&i].op_value
                        })
                    }
                    _ => 0,
                };
                let it = res / 3;
                if it % monkeys[&i].div_by == 0 {
                    monkeys
                        .entry(monkeys[&i].monkey_if_true)
                        .and_modify(|m| m.items.push(it));
                } else {
                    monkeys
                        .entry(monkeys[&i].monkey_if_false)
                        .and_modify(|m| m.items.push(it));
                };
            });
            monkeys.entry(i).and_modify(|m| {
                m.n_checked_items += m.items.len();
                m.items.clear();
            });
        }
    }
    let mut total_items_checked = monkeys
        .iter()
        .map(|(_, monkey)| monkey.n_checked_items)
        .collect::<Vec<usize>>();
    total_items_checked.sort();
    total_items_checked.reverse();
    Ok(total_items_checked.iter().take(2).product())
}

fn part_two(input: &str) -> BoxResult<usize> {
    let s_monkeys = input.split("\n\n");
    let mut monkeys: HashMap<usize, Monkey> = HashMap::new();

    s_monkeys.enumerate().for_each(|(i, s_monkey)| {
        let mut monkey = Monkey::new();
        s_monkey.lines().skip(1).for_each(|line| match line {
            items if line.trim().starts_with("Starting items:") => {
                monkey.items = items
                    .split(":")
                    .last()
                    .unwrap()
                    .split(",")
                    .map(|value| value.trim().parse::<usize>().unwrap())
                    .collect::<Vec<usize>>();
            }
            operation if line.trim().starts_with("Operation:") => {
                let (value, op) = operation
                    .split_ascii_whitespace()
                    .rev()
                    .take(2)
                    .collect_tuple()
                    .unwrap();
                monkey.op = op.to_string();
                monkey.op_value = if value == "old" {
                    0
                } else {
                    value.parse::<usize>().unwrap()
                };
            }
            divisible if line.trim().starts_with("Test:") => {
                monkey.div_by = divisible
                    .split_ascii_whitespace()
                    .last()
                    .unwrap()
                    .parse::<usize>()
                    .unwrap();
            }
            cond_true if line.trim().starts_with("If true:") => {
                monkey.monkey_if_true = cond_true
                    .split_ascii_whitespace()
                    .last()
                    .unwrap()
                    .parse::<usize>()
                    .unwrap();
            }
            cond_false if line.trim().starts_with("If false:") => {
                monkey.monkey_if_false = cond_false
                    .split_ascii_whitespace()
                    .last()
                    .unwrap()
                    .parse::<usize>()
                    .unwrap();
            }
            _ => println!("{}", AOCError),
        });
        monkeys.insert(i, monkey);
    });

    let pgcd: usize = monkeys.values().map(|m| m.div_by).product();

    for _ in 0..10000 {
        for i in 0..monkeys.len() {
            monkeys[&i].clone().items.iter().for_each(|v| {
                let res = match monkeys[&i].op.as_str() {
                    "+" => {
                        v + (if monkeys[&i].op_value == 0 {
                            v
                        } else {
                            &monkeys[&i].op_value
                        })
                    }
                    "*" => {
                        v * (if monkeys[&i].op_value == 0 {
                            v
                        } else {
                            &monkeys[&i].op_value
                        })
                    }
                    _ => 0,
                };
                let it = res % pgcd;
                if it % monkeys[&i].div_by == 0 {
                    monkeys
                        .entry(monkeys[&i].monkey_if_true)
                        .and_modify(|m| m.items.push(it));
                } else {
                    monkeys
                        .entry(monkeys[&i].monkey_if_false)
                        .and_modify(|m| m.items.push(it));
                };
            });
            monkeys.entry(i).and_modify(|m| {
                m.n_checked_items += m.items.len();
                m.items.clear();
            });
        }
    }
    let mut total_items_checked = monkeys
        .iter()
        .map(|(_, monkey)| monkey.n_checked_items)
        .collect::<Vec<usize>>();
    total_items_checked.sort();
    total_items_checked.reverse();
    Ok(total_items_checked.iter().take(2).product())
}

pub fn run() {
    let input = &read_file_to_string("./inputs/day11.txt");
    solve!(1, part_one, input);
    solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_part_one() {
        let input = &read_file_to_string("./examples/day11.txt");
        assert_eq!(part_one(input).ok(), Some(10605));
    }

    #[test]
    fn test_part_two() {
        let input = &read_file_to_string("./examples/day11.txt");
        assert_eq!(part_two(input).ok(), Some(2713310158));
    }
}
