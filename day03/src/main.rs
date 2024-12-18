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

    let full_regex = Regex::new(r"(do\(\))|(don't\(\))|mul\(\d+,\d+\)").unwrap();
    let mut sum_p2: i32 = 0;
    
    let mut doit: bool = true;
    
    for tok in full_regex.find_iter(&message) {
        println!("{}", tok.as_str());

        match tok.as_str() {
            "do()" => { doit = true; continue; },
            "don't()" => { doit = false; continue; },
            _ => {}
        }

        if !doit { continue };

        let digit_regex = Regex::new(r"\d+").unwrap();
        let digits: Vec<_> = digit_regex.find_iter(tok.as_str()).map(|s| s.as_str()).collect();
        sum_p2 += digits[0].parse::<i32>().unwrap() * digits[1].parse::<i32>().unwrap();
    }

    println!("Sum P2: {}", sum_p2);
}
