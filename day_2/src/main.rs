use utils::read_input_d2;

fn main() {
    let data = read_input_d2("../data/input_d2.txt");
    let safe_count = count_safe_lines(data.clone(), is_safe);
    let dampened_safe_count = count_safe_lines(data.clone(), is_safe_dampener);

    println!("safe reports: {}", safe_count);
    println!("safe reports with dampener: {}", dampened_safe_count);
}

fn count_safe_lines<F>(data: Vec<Vec<i32>>, is_safe_fn: F) -> i32
where
    F: Fn(&[i32]) -> bool,
{
    data.iter().filter(|line| is_safe_fn(line)).count() as i32
}

fn is_safe(slice: &[i32]) -> bool {
    is_monotonic(slice) && is_difference_safe(slice)
}

fn is_safe_dampener(slice: &[i32]) -> bool {
    if is_safe(slice) {
        return true;
    } else {
        for skip_index in 0..slice.len() {
            let filtered_slide: Vec<i32> = slice
                .iter()
                .enumerate()
                .filter(|&(i, _)| i != skip_index)
                .map(|(_, &x)| x)
                .collect();

            if is_safe(&filtered_slide) {
                return true;
            }
        }
    }

    false
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

    const EXAMPLE_DATA: &[&[i32]] = &[
        &[7, 6, 4, 2, 1],
        &[1, 2, 7, 8, 9],
        &[9, 7, 6, 2, 1],
        &[1, 3, 2, 4, 5],
        &[8, 6, 4, 4, 1],
        &[1, 3, 6, 7, 9],
    ];

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

    #[test]
    fn test_count_safe_lines() {
        let got = count_safe_lines(EXAMPLE_DATA.iter().map(|v| v.to_vec()).collect(), is_safe);
        let want = 2;
        assert_eq!(got, want);
    }

    #[test]
    fn test_count_safe_lines_with_dampener() {
        let got = count_safe_lines(
            EXAMPLE_DATA.iter().map(|v| v.to_vec()).collect(),
            is_safe_dampener,
        );
        let want = 4;
        assert_eq!(got, want);
    }
}
