use utils::read_to_string;
use regex::Regex;

fn main() {
    let input = read_to_string("../data/input_d3.txt").unwrap();
    let result = find_pattern(&input);
    println!("{}", result);
}

fn find_pattern(input: &str) -> i32 {
    let mut result = 0;
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").expect("Failed to compile regex");
    for capture in re.captures_iter(input) {
        result += capture[1].parse::<i32>().unwrap() * capture[2].parse::<i32>().unwrap()
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
}