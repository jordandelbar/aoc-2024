use std::fs::File;
use std::io::{self, BufRead};

fn read_file(file_path: &str) -> io::BufReader<File> {
    let file = File::open(file_path).expect("File not found");
    io::BufReader::new(file)
}

pub fn read_input_d1(file_path: &str) -> (Vec<i32>, Vec<i32>) {
    let mut vector1 = Vec::new();
    let mut vector2 = Vec::new();

    let reader = read_file(&file_path);

    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        let numbers: Vec<&str> = line.split_whitespace().collect();

        if numbers.len() == 2 {
            if let (Ok(num1), Ok(num2)) = (numbers[0].parse::<i32>(), numbers[1].parse::<i32>()) {
                vector1.push(num1);
                vector2.push(num2);
            }
        }
    }

    (vector1, vector2)
}

pub fn read_input_d2(file_path: &str) -> Vec<Vec<i32>> {
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
