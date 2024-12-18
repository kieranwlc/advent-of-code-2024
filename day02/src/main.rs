use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn report_is_valid(report: &Vec<i32>, tolerance: i32) -> bool {
    let diff0 = report[1] - report[0];

    for i in 0..report.len() - 1 {
        let diff = report[i + 1] - report[i];
        if (diff0 * diff) > 0 && diff.abs() <= 3 {
            continue;
        }

        if tolerance <= 0 {
            return false;
        }

        for i in 0..report.len() { 
            let mut retry_report = report.clone();
            retry_report.remove(i);
            if report_is_valid(&retry_report, tolerance - 1) {
                return true;
            }
        }

        return false;
    }

    return true;
}

fn main() {
    let lines = match read_lines("input.txt") {
        Ok(r) => r,
        Err(_) => {
            println!("Couldn't read input file");
            return;
        }
    };

    let mut safe_reports = 0;
    let mut safe_reports_with_tolerance = 0;

    for line in lines.flatten() {
        let report: Vec<i32> = line.split_whitespace().flat_map(|x| x.parse()).collect();

        if report_is_valid(&report, 1) {
            safe_reports_with_tolerance += 1;
        }

        if report_is_valid(&report, 0) {
            safe_reports += 1;
        }
    }

    println!("Safe Reports {}", safe_reports);
    println!("Safe Reports with tolerance {}", safe_reports_with_tolerance);
}
