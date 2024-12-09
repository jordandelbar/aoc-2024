pub fn parse_input(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let input = "...#.\n#...#.";
        let want: Vec<Vec<char>> = vec![vec!['.', '.', '.', '#', '.'], vec!['#', '.', '.', '.', '#', '.']];
        let got = parse_input(input);
        assert_eq!(want, got);
    }
}