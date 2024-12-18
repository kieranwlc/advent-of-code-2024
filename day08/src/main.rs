use std::path::Path;
use std::fs::File;
use std::io::{self, BufRead};
use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::time::Instant;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn one_antinode(
    antinode_positions: &mut Vec<(usize, usize)>, 
    last_pos: &(usize, usize), 
    distance: &(isize, isize),
    bounds: (usize, usize),
) {
    
    let s_antinode_pos: (isize, isize) = (
        last_pos.0 as isize + distance.0,
        last_pos.1 as isize + distance.1,
    );

    if s_antinode_pos.1 < 0 || s_antinode_pos.1 >= bounds.1 as isize {
        return;
    }

    if s_antinode_pos.0 < 0 || s_antinode_pos.0 >= bounds.0 as isize {
        return;
    }

    let antinode_pos: (usize, usize) = (s_antinode_pos.0 as usize, s_antinode_pos.1 as usize);

    if let None = antinode_positions.iter().find(|&p| p == &antinode_pos) {
        antinode_positions.push(antinode_pos);
    }
}

fn repeat_antinodes(
    antinode_positions: &mut Vec<(usize, usize)>, 
    last_pos: &(usize, usize), 
    distance: &(isize, isize),
    bounds: (usize, usize),
    ) {

    if let None = antinode_positions.iter().find(|p| p == &last_pos) {
        antinode_positions.push(*last_pos);
    }
    
    let s_antinode_pos: (isize, isize) = (
        last_pos.0 as isize + distance.0,
        last_pos.1 as isize + distance.1,
    );

    if s_antinode_pos.1 < 0 || s_antinode_pos.1 >= bounds.1 as isize {
        return;
    }

    if s_antinode_pos.0 < 0 || s_antinode_pos.0 >= bounds.0 as isize {
        return;
    }

    let antinode_pos: (usize, usize) = (s_antinode_pos.0 as usize, s_antinode_pos.1 as usize);

    repeat_antinodes(antinode_positions, &antinode_pos, distance, bounds);
}

fn main() {
    let now = Instant::now();

    let mut chars: Vec<Vec<char>> = Vec::new();
    let lines = read_lines("input.txt").unwrap();
    for line in lines.flatten() {
        let line_chars: Vec<char> = line.chars().collect();
        chars.push(line_chars);
    }

    let mut frequencies = HashMap::new();

    for y in 0..chars.len() {
        for x in 0..chars[y].len() {
            if chars[y][x] == '.' || chars[y][x] == '#' {
                continue;
            }

            match frequencies.entry(chars[y][x]) {
                Entry::Vacant(e) => {e.insert(vec![(x, y)]);},
                Entry::Occupied(mut e) => {e.get_mut().push((x, y));},
            };
        }
    }

    let bounds: (usize, usize) = (chars[0].len(), chars.len());

    let mut antinode_positions_single: Vec<(usize, usize)> = Vec::new();
    for frequency in frequencies.keys() {
        for pos0 in frequencies.get(frequency).unwrap() {
            for pos1 in frequencies.get(frequency).unwrap() {
                if pos0 == pos1 {
                    continue;
                }

                let distance: (isize, isize) = (
                    pos0.0 as isize - pos1.0 as isize, 
                    pos0.1 as isize - pos1.1 as isize
                );

                one_antinode(&mut antinode_positions_single, pos0, &distance, bounds);
            }
        }
    }

    println!("Number of antinodes {}", antinode_positions_single.len());
    println!("{}ms", now.elapsed().as_millis());

    let mut antinode_positions: Vec<(usize, usize)> = Vec::new();

    for frequency in frequencies.keys() {
        for pos0 in frequencies.get(frequency).unwrap() {
            for pos1 in frequencies.get(frequency).unwrap() {
                if pos0 == pos1 {
                    continue;
                }

                let distance: (isize, isize) = (
                    pos0.0 as isize - pos1.0 as isize, 
                    pos0.1 as isize - pos1.1 as isize
                );

                repeat_antinodes(&mut antinode_positions, pos0, &distance, bounds);
            }
        }
    }

    println!("Number of antinodes repeated {}", antinode_positions.len());
    println!("{}ms", now.elapsed().as_millis());
}
