use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
    process::exit,
};

fn main() {
    if let Ok(file) = File::open(Path::new("./src/input.txt")) {
        let reader = BufReader::new(&file);
        let lines: Vec<String> = reader.lines().collect::<Result<_, _>>().unwrap();
        println!("Doing Challenge 1");
        let sum = chall_1(&lines);
        println!("Sum is {}", sum);
        println!("Doing Challnege 2");
        let sum = chall_2(&lines);
        println!("SUm is {}", sum);
    } else {
        println!("That file does not exist");
    }
}

fn chall_1(lines: &Vec<String>) -> i32 {
    let red = 12;
    let green = 13;
    let blue = 14;
    let mut sum = 0;
    for line in lines {
        // Split string into game runs and game stats
        let first_split = line.split(": ").collect::<Vec<&str>>();
        let game_num: i32 = first_split[0].split(" ").collect::<Vec<&str>>()[1]
            .parse()
            .unwrap();
        let mut valid_game = true;

        // Get game stats and see if it is a valid game if so add the game num to the sum
        let rounds = first_split[1].split("; ").collect::<Vec<&str>>();
        for round in rounds {
            let blocks = round.split(", ").collect::<Vec<&str>>();
            for block in blocks {
                let block_pair = block.split(" ").collect::<Vec<&str>>();
                let block_count: i32 = block_pair[0].parse().unwrap();
                match block_pair[1] {
                    "red" => {
                        if block_count > red {
                            valid_game = false;
                            break;
                        }
                    }
                    "green" => {
                        if block_count > green {
                            valid_game = false;
                            break;
                        }
                    }
                    "blue" => {
                        if block_count > blue {
                            valid_game = false;
                            break;
                        }
                    }
                    _ => {
                        println!("{} is not a valid color", block_pair[1]);
                        exit(-1)
                    }
                }
            }
            if !valid_game {
                break;
            }
        }
        if valid_game {
            sum += game_num
        }
    }
    sum
}

fn chall_2(lines: &Vec<String>) -> i32 {
    let mut sum = 0;
    for line in lines {
        // Split string into game runs and game stats
        let first_split = line.split(": ").collect::<Vec<&str>>();
        // Get game stats and see if it is a valid game if so add the game num to the sum
        let mut max_red = 1;
        let mut max_green = 1;
        let mut max_blue = 1;

        let rounds = first_split[1].split("; ").collect::<Vec<&str>>();
        for round in rounds {
            let blocks = round.split(", ").collect::<Vec<&str>>();
            for block in blocks {
                let block_pair = block.split(" ").collect::<Vec<&str>>();
                let block_count: i32 = block_pair[0].parse().unwrap();
                match block_pair[1] {
                    "red" => {
                        if block_count > max_red {
                            max_red = block_count
                        }
                    }
                    "green" => {
                        if block_count > max_green {
                            max_green = block_count
                        }
                    }
                    "blue" => {
                        if block_count > max_blue {
                            max_blue = block_count
                        }
                    }
                    _ => {
                        println!("{} is not a valid color", block_pair[1]);
                        exit(-1)
                    }
                }
            }
        }
        sum += max_red * max_green * max_blue;
    }
    sum
}
