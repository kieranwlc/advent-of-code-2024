use std::fs::read_to_string;
use std::time::Instant;
use std::{u32, u64};

fn mv_frag(digits: &mut Vec<u32>) -> u64 {
    let mut checksum: u64 = 0;

    let mut blocks_added: u64 = 0;
    let mut index: usize = 0;
    let mut start_id: u64 = 0;
    let mut mv_index: usize = match digits.len() % 2 {
        1 => {digits.len() - 1}
        _ => {digits.len() - 2}
    };
    let mut mv_id: u64 = match digits.len() % 2 {
        0 => {(digits.len() as u64) / 2}
        _ => {(digits.len() as u64 - 1) / 2}
    };

    while index < digits.len() {
        if digits[index] == 0 {
            index += 1;
            continue;
        }

        match index % 2 {
            0 => {
                checksum += start_id * blocks_added;
                digits[index] -= 1;
                if digits[index] == 0 {
                    start_id += 1;
                }
            },
            _ => {
                if mv_index <= index {
                    index += 1;
                    continue;
                }

                checksum += mv_id * blocks_added;
                digits[index] -= 1;
                digits[mv_index] -= 1;

                if digits[mv_index] == 0 {
                    mv_id -= 1;
                }

                while digits[mv_index] == 0 && mv_index > index {
                    mv_index -= 2;
                }
            }
        };

        blocks_added += 1;
    }

    return checksum;
}

fn mv_unfrag(digits: &mut Vec<u32>) -> u64 {
    let mut checksum = 0;

    let mut digit_index = 0;
    let mut file_id = 0;
    while digit_index < digits.len() {
        digits[digit_index] += file_id * 10;
        println!("{}", digits[digit_index]);
        file_id += 1;
        digit_index += 2;
    }

    let mut mv_index: usize = match digits.len() % 2 {
        1 => {digits.len() - 1}
        _ => {digits.len() - 2}
    };

    while mv_index >= 2 {
        let mut space_index = 1;
        let digit_val = digits[mv_index] % 10;

        while space_index < mv_index {
            if digits[space_index] >= digit_val {
                if (mv_index + 1) < digits.len() {
                    digits[mv_index - 1] += digits[mv_index + 1];
                    digits.remove(mv_index + 1);
                }
                digits[mv_index - 1] += digit_val;
                digits.remove(mv_index);
                digits.insert(space_index + 1, 0);
                digits.insert(space_index + 2, digits[mv_index]);

                println!("Val {} fits in {}", digit_val, digits[space_index]);
                break;
            }

            space_index += 2;
        }

        mv_index -= 2;
    }

    let mut blocks_added: u64 = 0;

    let mut digit_index = 0;
    while digit_index < digits.len() {
        print!("{} ", digits[digit_index]);
        digit_index += 2
    }
    print!("{}", "\n");

    let mut digit_index = 0;
    while digit_index < digits.len() {
        if digits[digit_index] % 10 == 0 {
            digit_index += 2;
            continue;
        }

        let file_id: u64 = digits[digit_index] as u64 / 10;

        checksum += file_id * blocks_added;
        blocks_added += 1;
        digits[digit_index] -= 1;

        if digits[digit_index] % 10 == 0 {
            digit_index += 2
        }
    }

    return checksum;
}

fn main() {
    let now = Instant::now();

    let input = read_to_string("input.txt").unwrap();
    let input_chars_as_digits = input.chars().map(|c| c.to_digit(10));
    let mut digits: Vec<u32> = Vec::new();

    for charopt in input_chars_as_digits {
        if let None = charopt {
            continue;
        }

        digits.push(charopt.unwrap());
    }

    println!("Fragmented {}", mv_frag(&mut digits.clone()));
    println!("Defragmented {}", mv_unfrag(&mut digits.clone()));
    println!("{}ms", now.elapsed().as_millis());
}
