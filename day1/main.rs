use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();

    let lines = match read_lines("input.txt") {
        Ok(r) => r,
        Err(_) => {
            println!("Couldn't read input file");
            return;
        }
    };

    for line in lines.flatten() {
        let tokens: Vec<&str> = line.split_whitespace().collect();
        left.push(tokens[0].parse().unwrap());
        right.push(tokens[1].parse().unwrap());
    }

    left.sort_unstable();
    right.sort_unstable();
    
    let mut total_diff = 0;
    let mut total_simularity = 0;

    for i in 0..left.len() {
        total_diff += (left[i] - right[i]).abs();

        let mut matching_index = match right.iter().position(|&x| x == left[i]) {
            Some(r) => r,
            None => {
                continue;
            }
        };

        let mut matches = 1;
        while right[matching_index + 1] == right[matching_index] {
            matches += 1;
            matching_index += 1;
        }

        total_simularity += matches * left[i];
    }

    println!("Total difference = {}", total_diff);
    println!("Total simularity = {}", total_simularity);
}
