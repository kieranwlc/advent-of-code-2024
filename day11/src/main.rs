use std::time::Instant;
use std::{collections::HashMap, fs::read_to_string};

fn blink(
    stone: String, 
    remaining_iterations: u64, 
    map: &mut HashMap<(String, u64), u64>,
) -> u64 {
    if remaining_iterations == 0 {
        return 1;
    }

    if let Some(stones) = map.get(&(stone.clone(), remaining_iterations)) { 
        return *stones;
    }

    let stone_count = 
        if stone == "0" {
            blink("1".to_string(), remaining_iterations - 1, map)
        } 
        else if stone.len() % 2 == 0 {
            let (left, right) = stone.split_at(stone.len()/2);
            blink(left.parse::<u64>().unwrap().to_string(), remaining_iterations - 1, map) +
                blink(right.parse::<u64>().unwrap().to_string(), remaining_iterations - 1, map)

        } 
        else {
            blink((stone.parse::<u64>().unwrap() * 2024).to_string(), 
                remaining_iterations - 1, map)
        };

    map.insert((stone, remaining_iterations), stone_count);
    return stone_count;
}

fn main() {
    let now = Instant::now();
    let input = read_to_string("input.txt").unwrap();
    let initial_stones: Vec<String> = input.split_whitespace().map(|s| s.to_string()).collect();

    let mut stone_map = HashMap::new();

    let mut stone_count = 0;
    for stone in initial_stones {
        stone_count += blink(stone, 75, &mut stone_map);
    }

    println!("Stones {}", stone_count);
    println!("{}ms", now.elapsed().as_millis());
}
