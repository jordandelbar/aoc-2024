use utils::*;

fn main() {
    let input = read_input_d4("../data/input_d4.txt");
    println!("Part 1: {:?}", &input);
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_catch_xmas() {
        let input = "XMAS";
        let got = catch_xmas(input);
        let want = 1;
        assert_eq!(want, got);
    }
}