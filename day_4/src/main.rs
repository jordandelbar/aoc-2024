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

fn is_xmas(word: &str) -> bool {
    match word {
        "XMAS" => true,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_catch_xmas_horizontal() {
        let input = vec![vec!['X', 'M', 'A', 'S'], vec!['X', 'M', 'B', 'D']];
        let got = catch_xmas(&input);
        let want = 1;
        assert_eq!(want, got);
    }
}