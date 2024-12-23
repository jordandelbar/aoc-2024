use utils::{read_file_to_string, string_to_grid};

fn main() {
    let input = read_file_to_string("../data/input_d8.txt").unwrap();
    let test: Vec<Vec<char>> = string_to_grid(&input, false);
    println!("Test part 1: {:?}", test);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_to_grid() {
        let input = ".....\n.....";

        let got: Vec<Vec<char>> = string_to_grid(&input, false);
        let want = vec![vec!['.', '.', '.', '.', '.'],vec!['.', '.', '.', '.', '.']];

        assert_eq!(want, got);
    }
}
