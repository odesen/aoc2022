use crate::{read_file_to_string, solve, AOCError, BoxResult};

#[derive(Clone, Debug, Ord, Eq, PartialEq)]
enum Signal {
    Value(usize),
    List(Vec<Signal>),
}

impl From<String> for Signal {
    fn from(input: String) -> Self {
        let mut chars = input.chars().fuse();
        let mut packet: Vec<Signal> = vec![];
        let mut current_number = String::new();

        while let Some(c) = chars.next() {
            match c {
                '[' => packet.push(Signal::List(Vec::new())),
                ']' => {
                    if !current_number.is_empty() {
                        let value = current_number.parse::<usize>().unwrap();

                        current_number.clear();

                        match packet.last_mut().unwrap() {
                            Signal::List(v) => {
                                v.push(Signal::Value(value));
                            }
                            _ => println!("{}", AOCError),
                        }
                    }
                    if packet.len() > 1 {
                        let sig = packet.pop().unwrap();
                        match packet.last_mut().unwrap() {
                            Signal::List(v) => v.push(sig),
                            _ => println!("{}", AOCError),
                        }
                    }
                }
                ',' => {
                    if !current_number.is_empty() {
                        let value = current_number.parse::<usize>().unwrap();

                        current_number.clear();

                        match packet.last_mut().unwrap() {
                            Signal::List(v) => {
                                v.push(Signal::Value(value));
                            }
                            _ => println!("{}", AOCError),
                        }
                    }
                }
                x if x.is_digit(10) => {
                    current_number.push(x);
                }
                _ => println!("{}", AOCError),
            }
        }

        packet.pop().unwrap()
    }
}

impl PartialOrd for Signal {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        use Signal::*;
        match (self, other) {
            (Value(a), Value(b)) => a.partial_cmp(b),
            (List(a), List(b)) => a.partial_cmp(b),
            (Value(a), List(..)) => List(vec![Value(*a)]).partial_cmp(other),
            (List(..), Value(b)) => self.partial_cmp(&List(vec![Value(*b)])),
        }
    }
}

fn part_one(input: &str) -> BoxResult<usize> {
    let res = input
        .split("\n\n")
        .enumerate()
        .map(|(i, packets)| {
            if let Some((packet_1, packet_2)) = packets.split_once("\n") {
                let packet_1 = Signal::from(packet_1.to_string());
                let packet_2 = Signal::from(packet_2.to_string());
                if packet_1 < packet_2 {
                    i + 1
                } else {
                    0
                }
            } else {
                0
            }
        })
        .sum();
    Ok(res)
}

fn part_two(input: &str) -> BoxResult<usize> {
    let dividers = vec![
        Signal::from(String::from("[[2]]")),
        Signal::from(String::from("[[6]]")),
    ];

    let mut signals = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| Signal::from(line.to_string()))
        .collect::<Vec<Signal>>();

    signals.push(dividers[0].clone());
    signals.push(dividers[1].clone());
    signals.sort();

    Ok(dividers
        .iter()
        .map(|divider| 1 + signals.iter().position(|signal| signal == divider).unwrap())
        .product::<usize>())
}

pub fn run() {
    let input = &read_file_to_string("./inputs/day13.txt");
    solve!(1, part_one, input);
    solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_part_one() {
        let input = &read_file_to_string("./examples/day13.txt");
        assert_eq!(part_one(input).ok(), Some(13));
    }

    #[test]
    fn test_part_two() {
        let input = &read_file_to_string("./examples/day13.txt");
        assert_eq!(part_two(input).ok(), Some(140));
    }
}

//6076
