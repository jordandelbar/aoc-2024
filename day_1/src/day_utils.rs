use utils::read_file_to_string;

pub fn read_input(file_path: &str) -> (Vec<i32>, Vec<i32>) {
    let mut vector1 = Vec::new();
    let mut vector2 = Vec::new();

    let reader = read_file_to_string(file_path).unwrap();

    for line in reader.lines() {
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
