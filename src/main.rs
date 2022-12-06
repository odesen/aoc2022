use std::error::Error;

use aoc2022::{day01, day02, day03, day04, day05, day06};

fn main() -> Result<(), Box<dyn Error>> {
    day01::run();
    day02::run();
    day03::run();
    day04::run();
    day05::run();
    day06::run();
    Ok(())
}
