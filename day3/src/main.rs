use std::fs::read_to_string;
use regex::Regex;

fn main() {
    let message: String = read_to_string("input.txt").unwrap();
    let mul_regex = Regex::new(r"mul\(\d*,\d*\)").unwrap();

    let mut sum: i32 = 0;

    for mul in mul_regex.find_iter(&message) {
        let digit_regex = Regex::new(r"\d+").unwrap();
        let digits: Vec<_> = digit_regex.find_iter(mul.as_str()).map(|s| s.as_str()).collect();
        sum += digits[0].parse::<i32>().unwrap() * digits[1].parse::<i32>().unwrap();
    }

    println!("Sum: {}", sum);

    let input_reversed = message.chars().rev().collect::<String>();
    let dodont_regex = Regex::new(r"(\)\(od)|(\)\(t'nod)").unwrap();
    let mut sum_p2: i32 = 0;
    
    for mul in mul_regex.find_iter(&message) {
        let digit_regex = Regex::new(r"\d+").unwrap();
        let digits: Vec<_> = digit_regex.find_iter(mul.as_str()).map(|s| s.as_str()).collect();

        let mul_offset = message.len() - mul.start();
        let last_instruction = dodont_regex.find_at(&input_reversed, mul_offset);

        if let Some(m) = last_instruction {
            if m.as_str() == ")(t'nod" {
                continue;
            }
        }

        sum_p2 += digits[0].parse::<i32>().unwrap() * digits[1].parse::<i32>().unwrap();
    }

    println!("Sum P2: {}", sum_p2);
}
