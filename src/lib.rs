pub mod day01 {
    use std::{
        fs::File,
        io::{self, BufRead},
        path::Path,
    };

    pub fn run() {
        let mut max: i32 = 0;
        let mut max3 = [0, 0, 0];
        if let Ok(lines) = read_lines("./data/day1.txt") {
            let mut sum = 0;
            for line in lines {
                if let Ok(calorie) = line {
                    if let Ok(calorie_i32) = calorie.parse::<i32>() {
                        sum += calorie_i32
                    } else {
                        for (i, x) in max3.into_iter().enumerate() {
                            if sum > x {
                                max3[i] = sum;
                                max3.sort();
                                break;
                            }
                        }
                        if sum > max {
                            max = sum
                        }
                        sum = 0
                    }
                }
            }
        }
        println!("Most calories carried by one elf {}", max);
        println!("Most calories carried by three elf {:?}", max3);
        println!(
            "Sum of the most calories carried by three elf {}",
            max3.iter().sum::<i32>()
        );
    }

    fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where
        P: AsRef<Path>,
    {
        let file = File::open(filename)?;
        Ok(io::BufReader::new(file).lines())
    }
}
