use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines};
use std::path::Path;

fn main() {
    if let Ok(lines) = read_lines("./src/input.txt") {
        // Consumes the iterator, returns an (Optional) String
        println!("Doing Challenge 1");
        let sum = chall_1(lines);
        println!("Sum is {}", sum);
    } else {
        println!("That path does not exist");
    }
}

fn chall_1(lines: Lines<BufReader<File>>) -> u32 {
    let mut sum: u32 = 0;
    for line in lines {
        if let Ok(ip) = line {
            let nums: Vec<u32> = ip.chars().filter_map(|a| a.to_digit(10)).collect();
            if nums.len() > 1 {
                sum += nums[0] * 10 + nums[nums.len() - 1];
            } else {
                sum += nums[0] * 10 + nums[0];
            }
        }
    }
    sum
}

fn chall_2(lines: Lines<BufReader<File>>) -> u32 {}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
