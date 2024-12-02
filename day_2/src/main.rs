use utils::read_input_d2;

fn main() {
    let data = read_input_d2("../data/input_d2.txt");
    let mut results = 0;

    for line in data.iter() {
        if is_monotonic(&line) && is_difference_safe(&line) {
            results += 1
        }
    }

    println!("{}", results);
}

fn is_monotonic(slice: &[i32]) -> bool {
    let increasing = slice.windows(2).all(|w| w[0] <= w[1]);
    let decreasing = slice.windows(2).all(|w| w[0] >= w[1]);

    increasing || decreasing
}

fn is_difference_safe(slice: &[i32]) -> bool {
    let differences = slice
        .windows(2)
        .all(|w| i32::abs(w[0] - w[1]) <= 3 && i32::abs(w[0] - w[1]) > 0);
    differences
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug)]
    struct TestCase {
        input: Vec<i32>,
        expected: bool,
    }

    #[test]
    fn test_is_monotonic() {
        let test_cases = vec![
            TestCase {
                input: vec![1, 2, 3, 4, 5],
                expected: true,
            },
            TestCase {
                input: vec![5, 4, 3, 2, 1],
                expected: true,
            },
            TestCase {
                input: vec![1, 2, 2, 3],
                expected: true,
            },
            TestCase {
                input: vec![3, 2, 2, 1],
                expected: true,
            },
            TestCase {
                input: vec![1, 3, 2],
                expected: false,
            },
        ];

        for case in test_cases {
            let got = is_monotonic(&case.input);
            assert_eq!(got, case.expected, "Failed for input: {:?}", case.input)
        }
    }

    #[test]
    fn test_is_difference_safe() {
        let test_cases = vec![
            TestCase {
                input: vec![1, 2, 3, 4],
                expected: true,
            },
            TestCase {
                input: vec![4, 3, 2, 1],
                expected: true,
            },
            TestCase {
                input: vec![1, 2, 3, 7],
                expected: false,
            },
            TestCase {
                input: vec![1, 2, 2, 3],
                expected: false,
            },
            TestCase {
                input: vec![2, 2, 2, 2],
                expected: false,
            },
        ];

        for case in test_cases {
            let got = is_difference_safe(&case.input);
            assert_eq!(got, case.expected, "Failed for input: {:?}", case.input)
        }
    }
}
