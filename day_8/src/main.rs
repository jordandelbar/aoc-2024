use std::collections::HashSet;
use utils::{read_file_to_string, string_to_grid};

fn main() {
    let input = read_file_to_string("../data/input_d8.txt").unwrap();
    let test: Vec<Vec<char>> = string_to_grid(&input, false);
    let result = find_all_frequencies(test);
    println!("{:?}", result);
}

fn find_all_frequencies(grid: Vec<Vec<char>>) -> HashSet<char> {
    grid.iter()
        .flat_map(|line| line.iter())
        .filter(|&&c| c != '.')
        .copied()
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_find_all_frequencies() {
        let input = "..3..\n..A..";

        let grid: Vec<Vec<char>> = string_to_grid(&input, false);
        let got = find_all_frequencies(grid);
        let want = HashSet::from(['3', 'A']);

        assert_eq!(want, got);
    }
}
