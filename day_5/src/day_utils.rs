use std::io::BufRead;
use utils::read_file;

pub fn read_input(file_path: &str) -> (Vec<(i32, i32)>, Vec<Vec<i32>>) {
    let input = read_file(file_path);
    let mut lines = input.lines();

    let mut pairs = Vec::new();
    while let Some(Ok(line)) = lines.next() {
        if line.contains('|') {
            let nums: Vec<i32> = line.split('|').map(|n| n.parse().unwrap()).collect();
            pairs.push((nums[0], nums[1]));
        } else if line.is_empty() {
            break;
        }
    }

    let mut vectors = Vec::new();
    for line in lines {
        if let Ok(line) = line {
            let vec_nums: Vec<i32> = line.split(',').map(|n| n.parse().unwrap()).collect();
            vectors.push(vec_nums);
        }
    }

    (pairs, vectors)
}
