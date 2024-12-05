use std::io::BufRead;
use utils::read_file;

pub fn read_input(file_path: &str) -> Vec<Vec<i32>> {
    let reader = read_file(&file_path);

    let mut result = Vec::new();

    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        let numbers: Vec<i32> = line
            .split_whitespace()
            .map(|x| x.parse::<i32>().expect("Failed to parse number"))
            .collect();
        result.push(numbers)
    }

    result
}

