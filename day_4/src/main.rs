use utils::*;

fn main() {
    let input = read_input_d4("../data/input_d4.txt");
    println!("Part 1: {:?}", &input);
}

fn catch_xmas(_input: &Vec<Vec<char>>) -> i32 {
    return 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_catch_xmas_horizontal() {
        let input = vec![vec!['X', 'M', 'A', 'S'], vec!['X', 'M', 'B', 'D']];
        let got = catch_xmas(&input);
        let want = 1;
        assert_eq!(want, got);
    }
}