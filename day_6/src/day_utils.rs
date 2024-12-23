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
