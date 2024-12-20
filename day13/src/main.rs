use std::fs::read_to_string;
use std::time::Instant;
use regex::Regex;

struct Machine {
    a: (i64, i64),
    b: (i64, i64),
    z: (i64, i64),
}

fn solve_machine(m: &Machine) -> Option<i64> {
    let d = m.a.0 * m.b.1 - m.a.1 * m.b.0;
    let dx = m.z.0 * m.b.1 - m.z.1 * m.b.0;
    let dy = m.a.0 * m.z.1 - m.a.1 * m.z.0;

    if dx % d != 0 || dy % d != 0 {
        return None
    }

    let x = dx / d;
    let y = dy / d;

    return Some((x * 3 + y) as i64);
}

fn main() {
    let now = Instant::now();
    let input = read_to_string("input.txt").unwrap();
    let machines_txt = input.split("\n\n");

    let mut machines: Vec<Machine> = Vec::new();
    let mut machines_2: Vec<Machine> = Vec::new();

    let number_regex = Regex::new(r"\d+").unwrap();
    for machine in machines_txt {
        let values: Vec<i64> = number_regex.find_iter(machine)
            .map(|s| s.as_str().parse().unwrap()).collect();
        machines.push(Machine { 
            a: (values[0], values[1]), 
            b: (values[2], values[3]), 
            z: (values[4], values[5]), 
        });

        machines_2.push(Machine { 
            a: (values[0], values[1]), 
            b: (values[2], values[3]), 
            z: (values[4] + 10000000000000, values[5] + 10000000000000), 
        });
    }

    let mut total_price = 0;
    for machine in machines {
        if let Some(price) = solve_machine(&machine) {
            total_price += price;
        }
    }
    println!("Total price = {}", total_price);

    let mut total_price = 0;
    for machine in machines_2 {
        if let Some(price) = solve_machine(&machine) {
            total_price += price;
        }
    }
    println!("Total price = {}", total_price);

    println!("{}ms", now.elapsed().as_millis());
}
