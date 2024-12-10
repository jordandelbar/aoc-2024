pub fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

pub fn find_char(map: &[Vec<char>], target: char) -> Option<(i32, i32)> {
    for (row_index, row) in map.iter().enumerate() {
        if let Some(col_index) = row.iter().position(|&c| c == target) {
            return Some((row_index as i32, col_index as i32));
        }
    }
    None
}

pub fn max_indices(map: &[Vec<char>]) -> Option<(i32, i32)> {
    if map.is_empty() {
        return None;
    }

    let max_row_index = map.len() - 1;
    let max_col_index = map[max_row_index].len() - 1;

    Some((max_row_index as i32, max_col_index as i32))
}

pub fn is_within_bounds(position: (i32, i32), max_bounds: (i32, i32)) -> bool {
    position.0 >= 0 && position.0 <= max_bounds.0 && position.1 >= 0 && position.1 <= max_bounds.1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let input = "...#.\n#...#.";
        let want: Vec<Vec<char>> = vec![
            vec!['.', '.', '.', '#', '.'],
            vec!['#', '.', '.', '.', '#', '.'],
        ];
        let got = parse_input(input);
        assert_eq!(want, got);
    }
}
