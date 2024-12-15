use std::collections::VecDeque;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn calc_p1(lhs: i64, rhs: &VecDeque<i64>, op: i64, target: i64) -> i64 {
    if rhs.len() == 0 {
        if lhs == target {
            return lhs;
        }

        return 0;
    }
    
    let mut new_rhs = rhs.clone();
    let next_term = new_rhs.pop_front().unwrap();
    let mut new_lhs = lhs;

    match op {
        0 => { new_lhs += next_term; },
        _ => { new_lhs *= next_term; },
    }

    let add_res = calc_p1(new_lhs, &new_rhs, 0, target);
    if add_res != 0 {
        return add_res;
    }

    let mul_res = calc_p1(new_lhs, &new_rhs, 1, target);
    if mul_res != 0 {
        return mul_res;
    }

    return 0;
}

fn calc_p2(lhs: i64, rhs: &VecDeque<i64>, op: i64, target: i64) -> i64 {
    if rhs.len() == 0 {
        if lhs == target {
            return lhs;
        }

        return 0;
    }
    
    let mut new_rhs = rhs.clone();
    let next_term = new_rhs.pop_front().unwrap();
    let mut new_lhs = lhs;

    match op {
        0 => { new_lhs += next_term; },
        1 => { new_lhs *= next_term; },
        _ => { new_lhs = (new_lhs.to_string() + &next_term.to_string()).parse().unwrap();},
    }

    let add_res = calc_p2(new_lhs, &new_rhs, 0, target);
    if add_res != 0 {
        return add_res;
    }

    let mul_res = calc_p2(new_lhs, &new_rhs, 1, target);
    if mul_res != 0 {
        return mul_res;
    }

    let cat_res = calc_p2(new_lhs, &new_rhs, 2, target);
    if cat_res != 0 {
        return cat_res;
    }

    return 0;
}

fn main() {
    let lines = read_lines("input.txt").unwrap();

    let mut p1_sum = 0;
    let mut p2_sum = 0;

    for line in lines {
        let line_str = line.unwrap();

        println!("{}", line_str);

        let mut sides = line_str.split(": ");

        let answer: i64 = sides.next().unwrap().parse().unwrap();
        let rhs = sides.next().unwrap();

        let mut equation: VecDeque<i64> = VecDeque::new();
        for num in rhs.split(" ") {
            equation.push_back(num.parse().unwrap());
        }

        let lhs = equation.pop_front().unwrap();

        let add_res = calc_p1(lhs, &equation, 0, answer);
        if add_res != 0 {
            p1_sum += add_res;
        }
        else {
            let mul_res = calc_p1(lhs, &equation, 1, answer);
            if mul_res != 0 {
                p1_sum += mul_res;
            }
        }

        let add_res = calc_p2(lhs, &equation, 0, answer);
        if add_res != 0 {
            p2_sum += add_res;
            continue;
        }

        let mul_res = calc_p2(lhs, &equation, 1, answer);
        if mul_res != 0 {
            p2_sum += mul_res;
            continue;
        }

        let cat_res = calc_p2(lhs, &equation, 2, answer);
        if cat_res != 0 {
            p2_sum += cat_res;
            continue;
        }
    }

    println!("Total of calibratd = {}", p1_sum);
    println!("Total of calibrated concat = {}", p2_sum);
}
