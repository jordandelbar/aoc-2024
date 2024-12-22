use utils::read_to_string;
fn main() {
    let input = read_to_string("../data/input_d8.txt").unwrap();
    let test = string_to_grid(&input);
    println!("Test part 1: {:?}", test);
}

fn string_to_grid(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_to_grid() {
        let input = ".....\n.....";

        let got = string_to_grid(&input);
        let want = vec![vec!['.', '.', '.', '.', '.'],vec!['.', '.', '.', '.', '.']];

        assert_eq!(want, got);
    }
}
