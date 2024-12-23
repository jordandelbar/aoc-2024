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

fn count_number_antennas(grid: Vec<Vec<char>>) -> u32 {
    grid.iter()
    .flat_map(|line| line.iter())
    .filter(|&&c| c == '#')
    .count() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_all_frequencies() {
        let input = "..3..\n..A..";

        let grid: Vec<Vec<char>> = string_to_grid(&input, false);
        let got = find_all_frequencies(grid);
        let want = HashSet::from(['3', 'A']);

        assert_eq!(want, got);
    }

    #[test]
    fn test_count_number_antennas() {
        let input = "..##.......\n";
        let grid: Vec<Vec<char>> = string_to_grid(&input, false);

        let got = count_number_antennas(grid);
        let want = 2;

        assert_eq!(want, got);
    }
}
