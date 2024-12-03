use regex::Regex;

fn main() {
    println!("Hello, world!");
}

fn find_pattern(input: &str) -> usize {
    let re = Regex::new(r"mul\(\d{1,3},\d{1,3}\)").expect("Failed to compile regex");
    re.find_iter(input).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_pattern() {
        let input = String::from("wermul(3,2)warawpermul(2,4)");

        let got = find_pattern(&input);
        let want: usize = 2;
        assert_eq!(got, want);
    }
}