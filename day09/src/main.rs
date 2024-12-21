use std::fs::read_to_string;
use std::time::Instant;
use std::u64;

#[derive(Clone, Copy)]
struct Region {
    id: u64,
    size: u64,
    empty: u64
}

fn defrag(regions: &mut Vec<Region>) {
    let mut focus_region_i = regions.len() - 1;
    'regions: while focus_region_i > 0 {
        println!("Focus {} {} {}", 
            regions[focus_region_i].id,
            regions[focus_region_i].size,
            regions[focus_region_i].empty
        );

        for check_i in 0..focus_region_i {
            if regions[check_i].empty >= regions[focus_region_i].size {
                println!("MV file {} to FS after file {}", 
                    regions[focus_region_i].id,
                    check_i);

                let new_space = regions[check_i].empty - regions[focus_region_i].size;

                regions[focus_region_i - 1].empty += regions[focus_region_i].size + regions[focus_region_i].empty;
                regions[check_i].empty = 0;
                let mut swap = regions.remove(focus_region_i);
                swap.empty = new_space;
                regions.insert(check_i + 1, swap);
                continue 'regions;
            }
        }

        println!("No space");

        focus_region_i -= 1;
    }
}

fn checksum(regions: &Vec<Region>) -> u64 {
    let mut sum = 0;
    let mut block_count = 0;

    for region in regions {
        for _ in 0..region.size {
            sum += block_count * region.id;
            block_count += 1;
        }

        for _ in 0..region.empty {
            block_count += 1;
        }
    }

    return sum; 
}

fn main() {
    let now = Instant::now();

    let input = read_to_string("input.txt").unwrap();
    let conv = input.chars().map(|c| c.to_digit(10));
    let mut digits: Vec<u64> = Vec::new();

    for o in conv {
        if let Some(d) = o {
            digits.push(d as u64);
        }
    }

    let mut regions_p1: Vec<Region> = Vec::new();
    let mut regions_p2: Vec<Region> = Vec::new();

    for i in (0..digits.len()).step_by(2) {
        regions_p1.push(Region { id: i as u64 / 2, size: digits[i], empty: *digits.get(i + 1).unwrap_or(&0) });
        regions_p2.push(Region { id: i as u64 / 2, size: digits[i], empty: *digits.get(i + 1).unwrap_or(&0) });
    }

    defrag(&mut regions_p2);
    println!("P2 Checksum: {}", checksum(&regions_p2));

    println!("{}ms", now.elapsed().as_millis());
}
