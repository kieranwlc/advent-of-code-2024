use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::io::{self, BufRead};
use std::path::Path;
use std::fs::File;
use std::{isize, usize};

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn countup(chars: &Vec<Vec<char>>) -> i32 {
    let mut sum = 0;
    for y in 0..chars.len() {
        for x in 0..chars[y].len() {
            if chars[y][x] == 'X' || chars[y][x] == '^' {
                sum += 1;
            }
        }
    };
    return sum;
}

fn get_right_turn_dir(guard_dir: &(isize, isize)) -> (isize, isize) {
    match guard_dir {
        (0, -1) => return (1, 0),
        (1, 0) => return (0, 1),
        (0, 1) => return (-1, 0),
        (-1, 0) => return (0, -1),
        _ => panic!("Invalid direction"),
    }    
}

fn move_guard(
    chars: &mut Vec<Vec<char>>, 
    guard_position: &(usize, usize), 
    guard_dir: &(isize, isize), 
) {
    if chars[guard_position.1][guard_position.0] != '^' {
        chars[guard_position.1][guard_position.0] = 'X';
    }

    let next_pos: (isize, isize) = (guard_position.0 as isize - guard_dir.0,
        guard_position.1 as isize - guard_dir.1);

    if next_pos.0 < 0 || next_pos.0 >= chars[guard_position.1].len() as isize {
        return;
    }

    if next_pos.1 < 0 || next_pos.1 >= chars.len() as isize {
        return;
    }

    if chars[next_pos.1 as usize][next_pos.0 as usize] == '#' {
        let new_dir = get_right_turn_dir(&guard_dir);
        move_guard(chars, guard_position, &new_dir);
        return;
    }

    move_guard(chars, &(next_pos.0 as usize, next_pos.1 as usize), guard_dir);
}

fn scan_next(
    mut hit_obstacles: &mut HashMap<(isize, isize), Vec<(isize, isize)>>,
    chars: &Vec<Vec<char>>, 
    guard_position: &(usize, usize), 
    guard_dir: &(isize, isize), 
) -> bool {
    let next_pos: (isize, isize) = (guard_position.0 as isize - guard_dir.0,
        guard_position.1 as isize - guard_dir.1);

    if next_pos.0 < 0 || next_pos.0 >= chars[guard_position.1].len() as isize {
        return false;
    }

    if next_pos.1 < 0 || next_pos.1 >= chars.len() as isize {
        return false;
    }

    if chars[next_pos.1 as usize][next_pos.0 as usize] == '#' {
        let new_dir = get_right_turn_dir(&guard_dir);

        match hit_obstacles.entry(next_pos) {
            Entry::Vacant(e) => {e.insert(vec![*guard_dir]);},
            Entry::Occupied(mut e) => {
                if let Some(_) = e.get().into_iter().find(|d| *d == guard_dir) {
                    return true;
                }
                e.get_mut().push(*guard_dir);
            },
        };

        return scan_next(&mut hit_obstacles, chars, guard_position, &new_dir);
    }

    return scan_next(&mut hit_obstacles, chars, &(next_pos.0 as usize, next_pos.1 as usize), guard_dir);
}

fn scan_loops (
    chars: &Vec<Vec<char>>, 
    guard_position: &(usize, usize), 
    guard_dir: &(isize, isize), 
) -> bool {
    let next_pos: (isize, isize) = (guard_position.0 as isize - guard_dir.0,
        guard_position.1 as isize - guard_dir.1);

    if next_pos.0 < 0 || next_pos.0 >= chars[guard_position.1].len() as isize {
        return false;
    }

    if next_pos.1 < 0 || next_pos.1 >= chars.len() as isize {
        return false;
    }

    let mut hit_obstacles = HashMap::new();
    if scan_next(&mut hit_obstacles, &chars, guard_position, guard_dir) {
        return true;
    }

    return false;
}

fn main() {
    let mut chars: Vec<Vec<char>> = Vec::new();

    let lines = read_lines("input.txt").unwrap();
    for line in lines.flatten() {
        let line_chars: Vec<char> = line.chars().collect();
        chars.push(line_chars);
    }

    let guard_position = {
        let mut pos: Option<(usize, usize)> = Option::None;
        for y in 0..chars.len() {
            for x in 0..chars[y].len() {
                if chars[y][x] == '^' {
                    pos = Option::Some((x, y));
                }
            }
        };

        pos.unwrap()
    };

    let guard_dir: (isize, isize) = (0, 1);
    let mut loop_subs: i32 = 0;
    move_guard(&mut chars, &guard_position, &guard_dir);

    for y in 0..chars.len() {
        for x in 0..chars[y].len() {
            if chars[y][x] == 'X' {
                let mut altered_chars = chars.clone();
                altered_chars[y][x] = '#';
                if scan_loops(&altered_chars, &guard_position, &guard_dir) {
                    loop_subs += 1;
                }
            }
        }
    };

    println!("Squares Traversed {}", countup(&chars));
    println!("Possible Loop Placements {}", loop_subs);
}
