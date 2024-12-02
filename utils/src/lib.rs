use std::fs::File;
use std::io::{self, BufRead};

pub fn read_input(file_path: &str) -> (Vec<i32>, Vec<i32>) {
    let mut vector1 = Vec::new();
    let mut vector2 = Vec::new();

    let file = File::open(file_path).expect("");
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        let line = line.expect("");
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
