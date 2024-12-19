use std::fs::read_to_string;
use std::time::Instant;
use eqsolver::multivariable::MultiVarNewtonFD;
use nalgebra::{vector, Vector2};
use regex::Regex;

struct Machine {
    a: (f64, f64),
    b: (f64, f64),
    z: (f64, f64),
}

fn solve_machine(m: &Machine) -> Option<u64> {
    let f = |v: Vector2<f64>| vector![
        m.a.0 as f64 * v[0] + m.b.0 as f64 * v[1] - m.z.0 as f64, 
        m.a.1 as f64 * v[0] + m.b.1 as f64 * v[1] - m.z.1 as f64,
    ];

    let solution = 
        if let Ok(sol) = MultiVarNewtonFD::new(f).solve(vector![1., 1.]) {
            sol
        }
        else {
            return None;
        };

    let solution_rounded = (
        (solution.x * 1000.).round() / 1000.,
        (solution.y * 1000.).round() / 1000.,
    );

    if solution_rounded.0.fract() != 0.0 || solution_rounded.1.fract() != 0.0 {
        return None;
    }

    println!("{} * ({} {}), {} * ({} {}) = ({} {})", 
        solution_rounded.0, m.a.0, m.a.1, 
        solution_rounded.1, m.b.0, m.b.1,
        m.z.0, m.z.1
    );

    return Some((solution_rounded.0 * 3. + solution_rounded.1) as u64);
}

fn main() {
    let now = Instant::now();
    let input = read_to_string("input.txt").unwrap();
    let machines_txt = input.split("\n\n");

    let mut machines: Vec<Machine> = Vec::new();

    let number_regex = Regex::new(r"\d+").unwrap();
    for machine in machines_txt {
        let values: Vec<u64> = number_regex.find_iter(machine)
            .map(|s| s.as_str().parse().unwrap()).collect();
        machines.push(Machine { 
            a: (values[0] as f64, values[1] as f64), 
            b: (values[2] as f64, values[3] as f64), 
            z: (values[4] as f64, values[5] as f64), 
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
