use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let file_path = "../data/input.txt";
    let (vector1, vector2) = read_input(file_path);
    let computed_distance = compute_distance(vector1, vector2);
    println!("{}", computed_distance);
}

fn read_input(file_path: &str) -> (Vec<i32>, Vec<i32>) {
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

fn compute_distance(mut a: Vec<i32>, mut b: Vec<i32>) -> i32 {
    a.sort();
    b.sort();
    a.into_iter().zip(b).map(|(x, y)| i32::abs(x - y)).sum::<i32>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_distance() {
        let a = vec![3, 4, 2, 1, 3, 3];
        let b = vec![4, 3, 5, 3, 9, 3];

        let want = 11;
        let got = compute_distance(a, b);

        assert_eq!(got, want);
    }
} 

