use std::fs::File;
use std::io::{self, BufRead};
use std::isize;
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn check_character(
    text: &Vec<Vec<char>>, 
    x: usize, y: usize,
    xdir: isize, ydir: isize,
    matches: &mut u32
) {
    let target = match text[y][x] {
        'X' => 'M',
        'M' => 'A',
        'A' => 'S',
        'S' => {
            *matches += 1;
            return;
        },
        _ => { return; }
    };

    let nextx = x as isize + xdir;
    if nextx < 0 || nextx >= text[y].len() as isize { return };

    let nexty = y as isize + ydir;
    if nexty < 0 || nexty >= text.len() as isize { return };

    if text[nexty as usize][nextx as usize] == target {
        check_character(text, nextx as usize, nexty as usize, xdir, ydir, matches);
    }
}

fn check_start(
    text: &Vec<Vec<char>>, 
    x: usize, y: usize,
    matches: &mut u32
) {
    for cy in -1..=1 {
        for cx in -1..=1 {
            if cy == 0 && cx == 0 { continue; }
            check_character(text, x, y, cx, cy, matches);
        }
    }
}

fn main() {
    let lines = match read_lines("input.txt") {
        Ok(r) => r,
        Err(_) => {
            println!("Couldn't read input file");
            return;
        }
    };

    let mut chars: Vec<Vec<char>> = Vec::new();

    for line in lines.flatten() {
        let line_chars: Vec<char> = line.chars().collect();
        chars.push(line_chars);
    }

    let mut matches: u32 = 0;

    for y in 0..chars.len() {
        for x in 0..chars[y].len() {
            if chars[y][x] == 'X' {
                check_start(&chars, x, y, &mut matches);
            }
        }
    }

    println!("Matches: {}", matches);
}
