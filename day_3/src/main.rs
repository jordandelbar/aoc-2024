use utils::read_to_string;
use regex::Regex;

fn main() {
    let input = read_to_string("../data/input_d3.txt").unwrap();
    let result = find_pattern(&input);
    println!("{}", result);
}

fn find_pattern(input: &str) -> i32 {
    let mut result = 0;
    let mut is_active = true;
    let regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|(don\'t)|(do)").expect("Failed to compile regex");

    for captures in regex.captures_iter(input) {
        if captures.get(4).is_some() {
            is_active = true;
        } else if captures.get(3).is_some() {
            is_active = false;
        } else if let (Some(m1), Some(m2)) = (captures.get(1), captures.get(2)) {
            if is_active {
                let num1: i32 = m1.as_str().parse().unwrap();
                let num2: i32 = m2.as_str().parse().unwrap();
                result += num1 * num2;
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_pattern() {
        let input = String::from("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))");

        let got = find_pattern(&input);
        let want = 161;
        assert_eq!(got, want);
    }

    #[test]
    fn test_find_pattern_with_activation() {
        let input = String::from("xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))");
        let got = find_pattern(&input);
        let want = 48;
        assert_eq!(got, want);
    }
}