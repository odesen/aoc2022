use std::error::Error;

use aoc2022::{
    day01, day02, day03, day04, day05, day06, day07, day08, day09, day10, day11, day12, day13,
    day14,
};

fn main() -> Result<(), Box<dyn Error>> {
    println!("--Day 01--");
    day01::run();
    println!("--Day 02--");
    day02::run();
    println!("--Day 03--");
    day03::run();
    println!("--Day 04--");
    day04::run();
    println!("--Day 05--");
    day05::run();
    println!("--Day 06--");
    day06::run();
    // println!("--Day 07--");
    // day07::run();
    println!("--Day 08--");
    day08::run();
    println!("--Day 09--");
    day09::run();
    println!("--Day 10--");
    day10::run();
    println!("--Day 11--");
    day11::run();
    println!("--Day 12--");
    day12::run();
    println!("--Day 13--");
    day13::run();
    println!("--Day 14--");
    day14::run();
    Ok(())
}
