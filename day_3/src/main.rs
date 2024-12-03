use utils::read_to_string;
use regex::Regex;

fn main() {
    let input = read_to_string("../data/input_d3.txt").unwrap();
    let result = find_pattern(&input);
    println!("{}", result);
}

fn find_pattern(input: &str) -> i32 {
    let mut result = 0;
    let mut active = 1;
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|(don\'t)|(do)").expect("Failed to compile regex");
    for capture in re.captures_iter(input) {
        let capture_1 = capture.get(1).map_or(0, |m| m.as_str().parse::<i32>().unwrap());
        let capture_2 = capture.get(2).map_or(0, |m| m.as_str().parse::<i32>().unwrap());
        let dont_value = capture.get(3).map_or("", |m| m.as_str());
        let do_value = capture.get(4).map_or("", |m| m.as_str());
        if !do_value.is_empty() && do_value == "do" {
            active = 1;
        }
        if !dont_value.is_empty() && dont_value == "don't" {
            active = 0;
        }
        if active == 1 {
            result += capture_1 * capture_2;
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