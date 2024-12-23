use std::fs::File;
use std::io::{self, BufReader, Read};

pub fn read_file_to_string(file_path: &str) -> io::Result<String> {
    let file = File::open(file_path)?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    Ok(contents)
}

pub fn string_to_grid<T>(input: &str, whitespace_separator: bool) -> Vec<Vec<T>>
where
    T: std::str::FromStr,
    T::Err: std::fmt::Debug,
{
    input
        .lines()
        .map(|line| {
            if whitespace_separator {
                line.split_whitespace()
                    .map(|s| s.parse::<T>().expect("Failed to parse item"))
                    .collect()
            } else {
                line.chars()
                    .map(|c| c.to_string().parse::<T>().expect("Failed to parse item"))
                    .collect()
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_to_grid_with_chars() {
        let input = "a b c\nd e f\ng h i";
        let expected = vec![
            vec!['a', 'b', 'c'],
            vec!['d', 'e', 'f'],
            vec!['g', 'h', 'i'],
        ];
        let result = string_to_grid::<char>(input, true);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_string_to_grid_with_integers() {
        let input = "1 2 3\n4 5 6\n7 8 9";
        let expected = vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
            vec![7, 8, 9],
        ];
        let result = string_to_grid::<i32>(input, true);
        assert_eq!(result, expected);
    }

    #[test]
    #[should_panic(expected = "Failed to parse item")]
    fn test_string_to_grid_with_invalid_integers() {
        let input = "1 2 x\n4 5 6\n7 8 9";
        string_to_grid::<i32>(input, true);
    }

    #[test]
    fn test_thing() {
        let input = "...#.\n#...#.";
        let want: Vec<Vec<char>> = vec![
            vec!['.', '.', '.', '#', '.'],
            vec!['#', '.', '.', '.', '#', '.'],
        ];
        let got: Vec<Vec<char>> = string_to_grid(input, false);
        assert_eq!(want, got);
    }
}
