use std::fs::File;
use std::io::{self, BufRead, Read};

fn read_file(file_path: &str) -> io::BufReader<File> {
    let file = File::open(file_path).expect("File not found");
    io::BufReader::new(file)
}

pub fn read_to_string(file_path: &str) -> io::Result<String> {
    let mut buf_reader = read_file(file_path);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    Ok(contents)
}

pub fn read_input_d1(file_path: &str) -> (Vec<u32>, Vec<u32>) {
    let mut vector1 = Vec::new();
    let mut vector2 = Vec::new();

    let reader = read_file(&file_path);

    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        let numbers: Vec<&str> = line.split_whitespace().collect();

        if numbers.len() == 2 {
            if let (Ok(num1), Ok(num2)) = (numbers[0].parse::<u32>(), numbers[1].parse::<u32>()) {
                vector1.push(num1);
                vector2.push(num2);
            }
        }
    }

    (vector1, vector2)
}

pub fn read_input_d2(file_path: &str) -> Vec<Vec<u32>> {
    let reader = read_file(&file_path);

    let mut result = Vec::new();

    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        let numbers: Vec<u32> = line
            .split_whitespace()
            .map(|x| x.parse::<u32>().expect("Failed to parse number"))
            .collect();
        result.push(numbers)
    }

    result
}

pub fn read_input_d4(file_path: &str) -> Vec<Vec<char>> {
    let reader = read_file(&file_path);

    let mut result = Vec::new();

    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        let chars: Vec<char> = line.chars().collect();
        result.push(chars)
    }

    result
}
