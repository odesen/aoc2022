use crate::{read_file_to_string, solve, BoxResult};

type Indexed = (usize, isize);

fn decode(input: &str, key: isize, rounds: usize) -> isize {
    let mut data: Vec<Indexed> = input
        .lines()
        .map(|s| s.parse::<isize>().unwrap() * key)
        .enumerate()
        .collect();

    for _ in 0..rounds {
        for original_index in 0..data.len() {
            let index = data.iter().position(|x| x.0 == original_index).unwrap();
            let value = data[index].1;
            let new_index = index as isize + value;
            let new_index = new_index.rem_euclid(data.len() as isize - 1);
            let el = data.remove(index);
            data.insert(new_index as usize, el);
        }
    }
    let zero_index = data.iter().position(|x| x.1 == 0).unwrap();
    [1000, 2000, 3000]
        .iter()
        .map(|v| data[(zero_index + v) % data.len()].1)
        .sum()
}

fn part_one(input: &str) -> BoxResult<isize> {
    Ok(decode(input, 1, 1))
}

fn part_two(input: &str) -> BoxResult<isize> {
    Ok(decode(input, 811589153, 10))
}

pub fn run() {
    let input = &read_file_to_string("./inputs/day20.txt");
    solve!(1, part_one, input);
    solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_part_one() {
        let input = &read_file_to_string("./examples/day20.txt");
        assert_eq!(part_one(input).ok(), Some(3));
    }

    #[test]
    fn test_part_two() {
        let input = &read_file_to_string("./examples/day20.txt");
        assert_eq!(part_two(input).ok(), Some(1623178306));
    }
}
