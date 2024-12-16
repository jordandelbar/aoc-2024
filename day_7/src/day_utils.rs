use std::io::{self, BufRead};
use utils::read_file;

pub fn parse_input(file_path: &str) -> io::Result<Vec<(i64, Vec<i64>)>> {
    let reader = read_file(file_path);

    let mut equations = Vec::new();
    for line in reader.lines() {
        let line = line?;
        if let Some((target, rest)) = line.split_once(':') {
            let target: i64 = target.trim().parse().expect("Failed to parse target");
            let numbers: Vec<i64> = rest
                .split_whitespace()
                .map(|n| n.parse().expect("Failed to parse number"))
                .collect();
            equations.push((target, numbers));
        }
    }

    Ok(equations)
}
