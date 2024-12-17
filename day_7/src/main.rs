enum Operations {
    Addition,
    Multiplication,
    Concatenation,
}

#[derive(Debug)]
struct Line {
    target: u64,
    parts: Vec<u64>,
}

fn main() {
    let file_path = "../data/input_d7.txt";
    let input_data = utils::read_to_string(file_path).expect("Failed to read input");

    let total_sum = sum_valid_targets(&input_data);

    println!("Total valid target sum: {}", total_sum);
}

fn sum_valid_targets(input: &str) -> u64 {
    let lines: Vec<Line> = input
        .trim()
        .lines()
        .map(|x| -> Line {
            let sides = x.split(":").collect::<Vec<&str>>();

            Line {
                target: sides[0].parse::<u64>().unwrap(),
                parts: sides[1]
                    .split_whitespace()
                    .map(|x| x.parse::<u64>().unwrap())
                    .collect(),
            }
        })
        .collect();

    lines
        .into_iter()
        .filter(is_target_reachable)
        .fold(0, |acc, line| acc + line.target)
}

fn apply_operation(left_number: u64, right_number: u64, operation: &Operations) -> u64 {
    match operation {
        Operations::Addition => left_number + right_number,
        Operations::Multiplication => left_number * right_number,
        Operations::Concatenation => concat(left_number, right_number),
    }
}

fn concat(a: u64, b: u64) -> u64 {
    let concatenated = format!("{}{}", a, b);
    concatenated
        .parse::<u64>()
        .expect("Failed to parse concatenated to i64")
}

fn generate_operator_variants(code: u64, len: usize) -> Vec<Operations> {
    let mut operations: Vec<Operations> = Vec::new();
    let mut steps = code;

    for _ in 0..len {
        let remainder = steps.wrapping_rem(3);
        match remainder {
            0 => operations.push(Operations::Addition),
            1 => operations.push(Operations::Multiplication),
            2 => operations.push(Operations::Concatenation),
            _ => unreachable!(),
        }
        steps = steps.checked_div(3).unwrap();
    }

    operations
}

fn is_target_reachable(line: &Line) -> bool {
    let gaps = line.parts.len() - 1;

    'outer: for i in 0..3u32.pow(gaps as u32) {
        let operations = generate_operator_variants(i as u64, gaps);

        let mut total = line.parts[0];
        for (j, _) in operations.iter().enumerate().take(line.parts.len() - 1) {
            total = apply_operation(total, line.parts[j + 1], &operations[j]);
            if total > line.target {
                continue 'outer;
            }
        }

        if total == line.target {
            return true;
        }
    }
    false
}
