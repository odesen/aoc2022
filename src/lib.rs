pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;

use std::error::Error;
use std::fs::File;
use std::io::{self};
use std::path::Path;
use std::{fmt, fs};

pub type BoxResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug, Clone)]
pub struct AOCError;

impl fmt::Display for AOCError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "unexpected error")
    }
}

impl Error for AOCError {}

#[macro_export]
macro_rules! solve {
    ($part:expr, $solver:ident, $input:expr) => {{
        use std::fmt::Display;
        use std::time::Instant;

        fn print_result<T: Display>(func: impl FnOnce(&str) -> BoxResult<T>, input: &str) {
            let timer = Instant::now();
            let result = func(input);
            let elapsed = timer.elapsed();
            match result {
                Ok(result) => {
                    println!("{} (elapsed: {:.2?})", result, elapsed)
                }
                Err(_) => {
                    println!("not solved.")
                }
            }
        }

        println!("🎄 Part {} 🎄", $part);
        print_result($solver, $input);
    }};
}

pub fn read_file_to_buffer<P>(path: P) -> io::Result<io::BufReader<File>>
where
    P: AsRef<Path>,
{
    let file = File::open(path)?;
    Ok(io::BufReader::new(file))
}

pub fn read_file_to_string<P>(path: P) -> String
where
    P: AsRef<Path>,
{
    let f = fs::read_to_string(path);
    f.expect("coul not open input file")
}
