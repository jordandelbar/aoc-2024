mod day_utils;

fn main() {
    let file_path = "../data/input_d7.txt";
    let input = day_utils::parse_input(file_path).unwrap();
    let mut total_calibration_result = 0;

    for (target, numbers) in &input {
        if can_match_target(*target, numbers) {
            println!("Target {} can be matched!", target);
            total_calibration_result += target;
        } else {
            println!("Target {} cannot be matched.", target);
        }
    }

    println!("Total calibration result: {}", total_calibration_result);
}

fn can_match_target(target: i64, numbers: &[i64]) -> bool {
    fn backtrack(current_value: i64, index: usize, target: i64, numbers: &[i64]) -> bool {
        if index == numbers.len() {
            return current_value == target;
        }

        let next_num = numbers[index];

        let add_result = backtrack(current_value + next_num, index + 1, target, numbers);
        let multiply_result = backtrack(current_value * next_num, index + 1, target, numbers);

        add_result || multiply_result
    }

    backtrack(numbers[0], 1, target, numbers)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug)]
    struct TestCase {
        target: i64,
        input: Vec<i64>,
        expected: bool,
    }

    #[test]
    fn test_can_match_target() {
        let test_cases = vec! [
            TestCase {
                target: 20,
                input: vec![4, 5],
                expected: true
            },
            TestCase {
                target: 20,
                input: vec![4, 3, 2],
                expected: false
            },
        ];

        for case in test_cases {
            let got = can_match_target(case.target, &case.input);
            assert_eq!(case.expected, got);
        }
    }
}