pub fn count_number_xmas(input: &[Vec<char>]) -> usize {
    let sequences = look_for_xmas(input);
    sequences
        .iter()
        .filter(|seq| matches!(seq.as_str(), "XMAS" | "SAMX"))
        .count()
}

// look_for_xmas returns a vector of maximum 4 strings that contains
// the next 3 characters in every direction (horizontal, vertical, diagonal)
// if the index goes out of bound the record is discarded
fn look_for_xmas(input: &[Vec<char>]) -> Vec<String> {
    let mut sequences = Vec::new();

    for (row_index, row) in input.iter().enumerate() {
        for col_index in 0..row.len() {
            if matches!(input[row_index][col_index], 'X' | 'S') {
                sequences.extend(
                    [
                        (1, 0),  // Vertical
                        (0, 1),  // Horizontal
                        (1, 1),  // Diagonal east
                        (1, -1), // Diagonal west
                    ]
                    .iter()
                    .filter_map(|&(row_step, col_step)| {
                        extract_sequence(input, row_index, col_index, row_step, col_step)
                    }),
                );
            }
        }
    }

    sequences
}

fn extract_sequence(
    input: &[Vec<char>],
    start_row: usize,
    start_col: usize,
    row_step: isize,
    col_step: isize,
) -> Option<String> {
    let mut sequence = String::new();
    let mut row = start_row as isize;
    let mut col = start_col as isize;

    for _ in 0..4 {
        // Check if out of bound
        if row < 0
            || col < 0
            || row >= input.len() as isize
            || col >= input[row as usize].len() as isize
        {
            return None;
        }
        sequence.push(input[row as usize][col as usize]);
        row += row_step;
        col += col_step;
    }

    Some(sequence)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_sequence_valid() {
        let input = vec![
            vec!['a', 'b', 'c', 'd'],
            vec!['e', 'f', 'g', 'h'],
            vec!['i', 'j', 'k', 'l'],
            vec!['m', 'n', 'o', 'p'],
        ];

        // Horizontal right
        assert_eq!(
            extract_sequence(&input, 0, 0, 0, 1),
            Some("abcd".to_string())
        );

        // Vertical down
        assert_eq!(
            extract_sequence(&input, 0, 0, 1, 0),
            Some("aeim".to_string())
        );

        // Diagonal down-right
        assert_eq!(
            extract_sequence(&input, 0, 0, 1, 1),
            Some("afkp".to_string())
        );

        // Diagonal down-left
        assert_eq!(
            extract_sequence(&input, 0, 3, 1, -1),
            Some("dgjm".to_string())
        );

        // Out of bounds horizontally
        assert_eq!(extract_sequence(&input, 0, 1, 0, 1), None);

        // Out of bounds vertically
        assert_eq!(extract_sequence(&input, 1, 0, 1, 0), None);
    }

    #[test]
    fn test_look_for_xmas() {
        let input = vec![
            vec!['X', 'M', 'A', 'S'],
            vec!['X', 'M', 'B', 'D'],
            vec!['S', 'A', 'M', 'X'],
        ];
        let got = look_for_xmas(&input);
        let want = vec!["XMAS", "XMBD", "SAMX"];
        assert_eq!(want, got);
    }
}
