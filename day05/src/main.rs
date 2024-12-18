use std::collections::hash_map::Entry;
use std::fs::read_to_string;
use std::collections::HashMap;

fn main() {
    let input = read_to_string("input.txt").unwrap();
    let mut sections = input.split("\n\n");
    let rules_text = sections.next().unwrap();
    let updates = sections.next().unwrap();

    let mut rules = HashMap::new();

    for rule in rules_text.split("\n") {
        let mut values = rule.split("|");
        let key = values.next().unwrap();
        let val = values.next().unwrap();
        match rules.entry(key) {
            Entry::Vacant(e) => {e.insert(vec![val]);},
            Entry::Occupied(mut e) => {e.get_mut().push(val);},
        };
    }

    let comes_before = |l: &str, r: &str| -> bool {
        if let Some(afters) = rules.get(l) { 
            for after in afters {
                if after == &r {
                    return true;
                }
            }
        }

        return false;
    };

    let mut sum_clean = 0;
    let mut sum_fixed = 0;

    for update in updates.split("\n") {
        let mut values: Vec<_> = update.split(",").collect();
        let mut clean = true;
        let mut finished = false;

        if update.len() == 0 {
            continue;
        }

        while !finished {
            finished = true;

            for p0 in 0..values.len() {
                let mut pivot = p0;
                for i in (p0 + 1)..values.len() {
                    if comes_before(values[i], values[pivot]) {
                        finished = false;
                        clean = false;
                        let mv = values.remove(i);
                        values.insert(pivot, mv);
                        pivot += 1;
                    }
                }
            }
        }

        match clean {
            true => {
                sum_clean += values.get(values.len()/2).unwrap().parse::<i32>().unwrap();
            }
            false => {
                sum_fixed += values.get(values.len()/2).unwrap().parse::<i32>().unwrap();
            }
        }
    }

    println!("Clean Sum {}", sum_clean);
    println!("Fixed Sum {}", sum_fixed);
}
