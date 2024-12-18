use std::fs::read_to_string;

fn blink(stone: &str, max_iterations: u32, iteration: u32, stone_count: &mut u32) {
    if iteration == max_iterations {
        return;
    }

    if stone == "0" {
        blink("1", max_iterations, iteration + 1, stone_count);
        return;
    }

    if stone.len() % 2 == 0 {
        *stone_count += 1;

        let (left, right) = stone.split_at(stone.len()/2);
        blink(left, max_iterations, iteration + 1, stone_count);

        let right = &right.parse::<u64>().unwrap().to_string();
        blink(right, max_iterations, iteration + 1, stone_count);
        return;
    }

    let new_stone = &(stone.parse::<u64>().unwrap() * 2024).to_string();
    blink(new_stone, max_iterations, iteration + 1, stone_count);
}

fn main() {
    let input = read_to_string("input.txt").unwrap();
    let stones: Vec<&str> = input.split_whitespace().collect();

    let mut stone_count = stones.len() as u32;
    for stone in stones {
        if stone.len() == 0 {
            continue;
        }

        blink(stone, 25, 0, &mut stone_count);
    }

    println!("Stones {}", stone_count);
}
