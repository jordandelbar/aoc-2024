use utils::read_input_d4;

fn main() {
    let input = read_input_d4("../data/input_d4.txt");
    let sequences = look_for_xmas(&input);
    let first_part_count = sequences
        .iter()
        .filter(|seq| matches!(seq.as_str(), "XMAS" | "SAMX"))
        .count();
    println!("{}", first_part_count);
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
