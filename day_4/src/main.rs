use utils::read_input_d4;

fn main() {
    let input = read_input_d4("../data/input_d4.txt");
    let v = look_for_xmas(&input);
    let mut count = 0;
    for catch in v {
        if is_xmas(&catch) {
            count +=1;
        }
    }
    println!("{}", count)
}

// look_for_xmas returns a vector of maximum 8 strings that contains
// the next 3 characters in every direction (horizontal, vertical, diagonal and left and right)
// if the index goes out of bound the record is discarded
fn look_for_xmas(input: &Vec<Vec<char>>) -> Vec<String> {
    let mut sequence = Vec::new();

    for row_index in 0..input.len() {
        for col_index in 0..input[row_index].len() {
            if input[row_index][col_index] == 'X' || input[row_index][col_index] == 'S' {
                // horizontal
                if col_index < input[row_index].len() - 3 {
                    let mut string_to_check = String::new();
                    string_to_check.push(input[row_index][col_index]);
                    string_to_check.push(input[row_index][col_index + 1]);
                    string_to_check.push(input[row_index][col_index + 2]);
                    string_to_check.push(input[row_index][col_index + 3]);
                    sequence.push(string_to_check);
                }
                // diagonal east
                if row_index < input.len() - 3 && col_index < input[row_index].len() - 3 {
                    let mut string_to_check = String::new();
                    string_to_check.push(input[row_index][col_index]);
                    string_to_check.push(input[row_index + 1][col_index + 1]);
                    string_to_check.push(input[row_index + 2][col_index + 2]);
                    string_to_check.push(input[row_index + 3][col_index + 3]);
                    sequence.push(string_to_check);
                }
                // diagonal west
                if row_index < input.len() - 3 && col_index >= 3 {
                    let mut string_to_check = String::new();
                    string_to_check.push(input[row_index][col_index]);
                    string_to_check.push(input[row_index + 1][col_index - 1]);
                    string_to_check.push(input[row_index + 2][col_index - 2]);
                    string_to_check.push(input[row_index + 3][col_index - 3]);
                    sequence.push(string_to_check);
                }
                // vertical
                if row_index < input.len() - 3 {
                    let mut string_to_check = String::new();
                    string_to_check.push(input[row_index][col_index]);
                    string_to_check.push(input[row_index + 1][col_index]);
                    string_to_check.push(input[row_index + 2][col_index]);
                    string_to_check.push(input[row_index + 3][col_index]);
                    sequence.push(string_to_check);
                }
            }
        }
    }

    sequence
}

fn is_xmas(word: &str) -> bool {
    match word {
        "XMAS" => true,
        "SAMX" => true,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_look_for_xmas() {
        let input = vec![vec!['X', 'M', 'A', 'S'], vec!['X', 'M', 'B', 'D'], vec!['S', 'A', 'M', 'X']];
        let got = look_for_xmas(&input);
        let want = vec!["XMAS".to_string(), "XMBD".to_string(), "SAMX".to_string()];
        assert_eq!(want, got);
    }
}