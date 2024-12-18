use std::path::Path;
use std::fs::File;
use std::io::{self, BufRead};
use std::time::Instant;
use std::u32;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn find_summits_distinct(map: &Vec<Vec<u32>>, position: &(usize, usize), summits: &mut Vec<(usize, usize)>) {
    if map[position.1][position.0] == 9 {
        summits.push(*position);
        return;
    }

    if position.1 + 1 < map.len() {
        if map[position.1 + 1][position.0] == map[position.1][position.0] + 1 {
            find_summits_distinct(map, &(position.0, position.1 + 1), summits);
        }
    }

    if position.1 > 0 {
        if map[position.1 - 1][position.0] == map[position.1][position.0] + 1 {
            find_summits_distinct(map, &(position.0, position.1 - 1), summits);
        }
    }

    if position.0 + 1 < map[position.1].len() {
        if map[position.1][position.0 + 1] == map[position.1][position.0] + 1 {
            find_summits_distinct(map, &(position.0 + 1, position.1), summits);
        }
    }

    if position.0 > 0 {
        if map[position.1][position.0 - 1] == map[position.1][position.0] + 1 {
            find_summits_distinct(map, &(position.0 - 1, position.1), summits);
        }
    }
}

fn find_summits(map: &Vec<Vec<u32>>, position: &(usize, usize), summits: &mut Vec<(usize, usize)>) {
    println!("{} {}", position.0, position.1);
    if map[position.1][position.0] == 9 {
        if let None = summits.into_iter().find(|p| p == &position) {
            summits.push(*position);
        }
        return;
    }

    if position.1 + 1 < map.len() {
        if map[position.1 + 1][position.0] == map[position.1][position.0] + 1 {
            find_summits(map, &(position.0, position.1 + 1), summits);
        }
    }

    if position.1 > 0 {
        if map[position.1 - 1][position.0] == map[position.1][position.0] + 1 {
            find_summits(map, &(position.0, position.1 - 1), summits);
        }
    }

    if position.0 + 1 < map[position.1].len() {
        if map[position.1][position.0 + 1] == map[position.1][position.0] + 1 {
            find_summits(map, &(position.0 + 1, position.1), summits);
        }
    }

    if position.0 > 0 {
        if map[position.1][position.0 - 1] == map[position.1][position.0] + 1 {
            find_summits(map, &(position.0 - 1, position.1), summits);
        }
    }
}

fn main() {
    let now = Instant::now();

    let mut tiles: Vec<Vec<u32>> = Vec::new();
    let mut trailheads: Vec<(usize, usize)> = Vec::new();

    let rows = read_lines("input.txt").unwrap();
    let mut row_i = 0;
    for row in rows.flatten() {
        let row_tiles: Vec<u32> = row.chars().map(|c| c.to_digit(10).unwrap()).collect();
        
        let mut col = 0;
        for tile in &row_tiles {
            if *tile == 0 {
                trailheads.push((col, row_i));
            }

            col += 1;
        }

        tiles.push(row_tiles);
        row_i += 1;
    }

    let mut total_score = 0;
    let mut total_score_distinct = 0;
    for trailhead in trailheads {
        let mut summits: Vec<(usize, usize)> = Vec::new();
        let mut summits_distinct: Vec<(usize, usize)> = Vec::new();
        find_summits(&tiles, &trailhead, &mut summits);
        find_summits_distinct(&tiles, &trailhead, &mut summits_distinct);
        total_score += summits.len();
        total_score_distinct += summits_distinct.len();
    }

    println!("Total summit score: {}", total_score);
    println!("Total distinct summit score: {}", total_score_distinct);
    println!("{}ms", now.elapsed().as_millis());
}
