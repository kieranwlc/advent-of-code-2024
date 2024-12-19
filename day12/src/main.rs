use std::path::Path;
use std::fs::File;
use std::io::{self, BufRead};
use std::time::Instant;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[derive(Clone)]
struct Tile {
    traversed: bool,
    plant: char,
}

struct Geometry {
    area: u32,
    perimeter: u32,
}

struct SideGeometry {
    area: u32,
    sides: u32,
}

fn contains(tiles: &Vec<Vec<Tile>>, pos: &(isize, isize), target: char) -> bool {
    if pos.1 >= tiles.len() as isize || pos.1 < 0 {
        return false;
    }

    if pos.0 >= tiles.len() as isize || pos.0 < 0 {
        return false;
    }

    if tiles[pos.1 as usize][pos.0 as usize].plant == target {
        return true;
    }

    return false;
}

fn calculate_region_sides (
    tiles: &mut Vec<Vec<Tile>>, 
    pos: &(usize, usize)) -> SideGeometry {
    tiles[pos.1][pos.0].traversed = true;

    let mut total_geometry = SideGeometry { area: 1, sides: 0 };

    tiles[pos.1][pos.0].traversed = true;
    let region_plant = tiles[pos.1][pos.0].plant;

    let n = contains(tiles, &(pos.0 as isize, pos.1 as isize - 1), region_plant);
    let ne = contains(tiles, &(pos.0 as isize + 1, pos.1 as isize - 1), region_plant);
    let e = contains(tiles, &(pos.0 as isize + 1, pos.1 as isize), region_plant);
    let se = contains(tiles, &(pos.0 as isize + 1, pos.1 as isize + 1), region_plant);
    let s = contains(tiles, &(pos.0 as isize, pos.1 as isize + 1), region_plant);
    let sw = contains(tiles, &(pos.0 as isize - 1, pos.1 as isize + 1), region_plant);
    let w = contains(tiles, &(pos.0 as isize - 1, pos.1 as isize), region_plant);
    let nw = contains(tiles, &(pos.0 as isize - 1, pos.1 as isize - 1), region_plant);
    
    if (n == e) && (!ne || n == false) {
        total_geometry.sides += 1;
    }

    if (s == e) && (!se || s == false) {
        total_geometry.sides += 1;
    }

    if (s == w) && (!sw || s == false) {
        total_geometry.sides += 1;
    }

    if (n == w) && (!nw || n == false) {
        total_geometry.sides += 1;
    }

    if pos.1 + 1 < tiles.len() {
        if tiles[pos.1 + 1][pos.0].plant == region_plant { 
            if !tiles[pos.1 + 1][pos.0].traversed {
                let add_geo = calculate_region_sides(tiles, &(pos.0, pos.1 + 1));
                total_geometry.sides += add_geo.sides;
                total_geometry.area += add_geo.area;
            }
        }
    }

    if pos.1 > 0 {
        if tiles[pos.1 - 1][pos.0].plant == region_plant {
            if !tiles[pos.1 - 1][pos.0].traversed {
                let add_geo = calculate_region_sides(tiles, &(pos.0, pos.1 - 1));
                total_geometry.sides += add_geo.sides;
                total_geometry.area += add_geo.area;
            }
        }
    }

    if pos.0 + 1 < tiles[pos.1].len() {
        if tiles[pos.1][pos.0 + 1].plant == region_plant { 
            if !tiles[pos.1][pos.0 + 1].traversed {
                let add_geo = calculate_region_sides(tiles, &(pos.0 + 1, pos.1));
                total_geometry.sides += add_geo.sides;
                total_geometry.area += add_geo.area;
            }
        }
    }

    if pos.0 > 0 {
        if tiles[pos.1][pos.0 - 1].plant == region_plant { 
            if !tiles[pos.1][pos.0 - 1].traversed {
                let add_geo = calculate_region_sides(tiles, &(pos.0 - 1, pos.1));
                total_geometry.sides += add_geo.sides;
                total_geometry.area += add_geo.area;
            }
        }
    }

    return total_geometry;
}

fn calculate_region_fence(
    tiles: &mut Vec<Vec<Tile>>, 
    pos: &(usize, usize)) -> Geometry {

    let mut total_geometry = Geometry { area: 1, perimeter: 4 };

    tiles[pos.1][pos.0].traversed = true;
    let region_plant = tiles[pos.1][pos.0].plant;

    if pos.1 + 1 < tiles.len() {
        if tiles[pos.1 + 1][pos.0].plant == region_plant { 
            total_geometry.perimeter -= 1;
            if !tiles[pos.1 + 1][pos.0].traversed {
                let add_geo = calculate_region_fence(tiles, &(pos.0, pos.1 + 1));
                total_geometry.perimeter += add_geo.perimeter;
                total_geometry.area += add_geo.area;
            }
        }
    }

    if pos.1 > 0 {
        if tiles[pos.1 - 1][pos.0].plant == region_plant {
            total_geometry.perimeter -= 1;
            if !tiles[pos.1 - 1][pos.0].traversed {
                let add_geo = calculate_region_fence(tiles, &(pos.0, pos.1 - 1));
                total_geometry.perimeter += add_geo.perimeter;
                total_geometry.area += add_geo.area;
            }
        }
    }

    if pos.0 + 1 < tiles[pos.1].len() {
        if tiles[pos.1][pos.0 + 1].plant == region_plant { 
            total_geometry.perimeter -= 1;
            if !tiles[pos.1][pos.0 + 1].traversed {
                let add_geo = calculate_region_fence(tiles, &(pos.0 + 1, pos.1));
                total_geometry.perimeter += add_geo.perimeter;
                total_geometry.area += add_geo.area;
            }
        }
    }

    if pos.0 > 0 {
        if tiles[pos.1][pos.0 - 1].plant == region_plant { 
            total_geometry.perimeter -= 1;
            if !tiles[pos.1][pos.0 - 1].traversed {
                let add_geo = calculate_region_fence(tiles, &(pos.0 - 1, pos.1));
                total_geometry.perimeter += add_geo.perimeter;
                total_geometry.area += add_geo.area;
            }
        }
    }

    return total_geometry;
}

fn main() {
    let now = Instant::now();

    let mut tiles: Vec<Vec<Tile>> = Vec::new();

    let rows = read_lines("input.txt").unwrap();
    for row in rows.flatten() {
        let row_tiles: Vec<Tile> = row.chars().map(|c| 
            Tile { traversed: false, plant: c }).collect();
        tiles.push(row_tiles);
    }

    let mut tiles_sides = tiles.clone();

    let mut price: u32 = 0;
    for y in 0..tiles.len() {
        for x in 0..tiles[y].len() {
            if !tiles[y][x].traversed {
                let region_geometry = calculate_region_fence(&mut tiles, &(x, y));
                price += region_geometry.area * region_geometry.perimeter;
            }

        }
    }
    println!("Total Price {}", price);
    println!("{}ms", now.elapsed().as_millis());

    let mut price: u32 = 0;
    for y in 0..tiles_sides.len() {
        for x in 0..tiles_sides[y].len() {
            if !tiles_sides[y][x].traversed {
                let region_geometry = calculate_region_sides(&mut tiles_sides, &(x, y));
                price += region_geometry.area * region_geometry.sides;
            }
        }
    }

    println!("Total Price Sides {}", price);
    println!("{}ms", now.elapsed().as_millis());
}
