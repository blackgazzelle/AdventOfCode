use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn main() {
    if let Ok(file) = File::open(Path::new("./src/input.txt")) {
        let reader = BufReader::new(&file);
        let lines: Vec<String> = reader.lines().collect::<Result<_, _>>().unwrap();
        println!("Doing Challenge 1");
        let sum = chall_1(&lines);
        println!("Sum is {}", sum);
        chall_2(&lines);
    } else {
        println!("That file does not exist");
    }
}

fn chall_1(lines: &Vec<String>) -> u32 {
    let mut sum: u32 = 0;
    for line in lines {
        let nums: Vec<u32> = line.chars().filter_map(|a| a.to_digit(10)).collect();
        if nums.len() > 1 {
            sum += nums[0] * 10 + nums[nums.len() - 1];
        } else {
            sum += nums[0] * 10 + nums[0];
        }
    }
    sum
}

fn chall_2(lines: &Vec<String>) -> u32 {
    //let nums_words = [
    //    "1", "2", "3", "4", "5", "6", "7", "8", "9", "one", "two", "three", "four", "five", "six ",
    //    "seven", "eight", "nine",
    //];
    let nums_words = HashMap::from([
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);
    let mut sum: u32 = 0;
    for line in lines {
        let mut values: HashMap<usize, &str> = HashMap::new();
        for word in nums_words.keys() {
            let sub_strs: Vec<(usize, &str)> = line.match_indices(word).collect();
            for strs in sub_strs {
                values.insert(strs.0, strs.1);
            }
        }
        let min: u32 = *nums_words
            .get(values.get(values.keys().min().unwrap()).unwrap())
            .unwrap();
        let max: u32 = *nums_words
            .get(values.get(values.keys().max().unwrap()).unwrap())
            .unwrap();
        sum += min * 10 + max;
    }
    println!("Sum is {}", sum);
    return 0;
}
