use utils::*;

fn main() {
    let input = read_input_d4("../data/input_d4.txt");
    catch_xmas(&input);
}

fn catch_xmas(input: &Vec<Vec<char>>) -> i32 {
    let mut count = 0;
    for row in input {
        for index in 0..row.len() - 3 {
            let mut string_to_check = String::new();
            string_to_check.push(row[index]);
            string_to_check.push(row[index + 1]);
            string_to_check.push(row[index + 2]);
            string_to_check.push(row[index + 3]);

            if is_xmas(&string_to_check) {
                count += 1;
            }
        }
    }

    count
}

// look_for_xmas returns a vector of maximum 8 strings that contains
// the next 3 characters in every direction (horizontal, vertical, diagonal and left and right)
// if the index goes out of bound the record is discarded
fn look_for_xmas(input: &Vec<Vec<char>>) -> Vec<String> {
    let mut vector = Vec::new();
    for row_index in 0..input.len() {
        for col_index in 0..input[row_index].len() - 3 {
            let mut string_to_check = String::new();
            string_to_check.push(input[row_index][col_index]);
            string_to_check.push(input[row_index][col_index + 1]);
            string_to_check.push(input[row_index][col_index + 2]);
            string_to_check.push(input[row_index][col_index + 3]);
            vector.push(string_to_check)
        }
    }

    vector
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
    fn test_catch_xmas_horizontal() {
        let input = vec![vec!['X', 'M', 'A', 'S'], vec!['X', 'M', 'B', 'D'], vec!['S', 'A', 'M', 'X']];
        let got = catch_xmas(&input);
        let want = 2;
        assert_eq!(want, got);
    }

    #[test]
    fn test_look_for_xmas() {
        let input = vec![vec!['X', 'M', 'A', 'S'], vec!['X', 'M', 'B', 'D'], vec!['S', 'A', 'M', 'X']];
        let got = look_for_xmas(&input);
        let want = vec!["XMAS".to_string(), "XMBD".to_string(), "SAMX".to_string()];
        assert_eq!(want, got);
    }
}