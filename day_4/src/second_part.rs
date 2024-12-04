pub fn look_for_xmas_cross(input: &Vec<Vec<char>>) -> u32 {
    let mut count = 0;

    for row_index in 1..input.len() - 1 {
        for col_index in 1..input[row_index].len() - 1 {
            if input[row_index][col_index] == 'A' {
                let diag_north_west = input[row_index - 1][col_index - 1];
                let diag_south_east = input[row_index + 1][col_index + 1];
                let diag_north_east = input[row_index - 1][col_index + 1];
                let diag_south_west = input[row_index + 1][col_index - 1];

                let diag_1 = format!("{}A{}", diag_north_west, diag_south_east);
                let diag_2 = format!("{}A{}", diag_north_east, diag_south_west);

                if check_mas(&diag_1, &diag_2) {
                    count += 1;
                }
            }
        }
    }

    count
}

fn check_mas(diag_1: &str, diag_2: &str) -> bool {
    if matches!(diag_1, "MAS" | "SAM") && matches!(diag_2, "MAS" | "SAM") {
        true
    } else {
        false
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_thing() {
        let matrix: Vec<Vec<char>> = vec![
            vec!['M', 'M', 'M', 'S', 'X', 'X', 'M', 'A', 'S', 'M'],
            vec!['M', 'S', 'A', 'M', 'X', 'M', 'S', 'M', 'S', 'A'],
            vec!['A', 'M', 'X', 'S', 'X', 'M', 'A', 'A', 'M', 'M'],
            vec!['M', 'S', 'A', 'M', 'A', 'S', 'M', 'S', 'M', 'X'],
            vec!['X', 'M', 'A', 'S', 'A', 'M', 'X', 'A', 'M', 'M'],
            vec!['X', 'X', 'A', 'M', 'M', 'X', 'X', 'A', 'M', 'A'],
            vec!['S', 'M', 'S', 'M', 'S', 'A', 'S', 'X', 'S', 'S'],
            vec!['S', 'A', 'X', 'A', 'M', 'A', 'S', 'A', 'A', 'A'],
            vec!['M', 'A', 'M', 'M', 'M', 'X', 'M', 'M', 'M', 'M'],
            vec!['M', 'X', 'M', 'X', 'A', 'X', 'M', 'A', 'S', 'X'],
        ];

        let got = look_for_xmas_cross(matrix);
        let want = 9;
        assert_eq!(got, want)
    }
}
