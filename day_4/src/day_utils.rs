use std::io::BufRead;
use utils::read_file;

pub fn read_input(file_path: &str) -> Vec<Vec<char>> {
    let reader = read_file(&file_path);

    let mut result = Vec::new();

    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        let chars: Vec<char> = line.chars().collect();
        result.push(chars)
    }

    result
}
