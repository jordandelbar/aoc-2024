use utils::read_input;

fn main() {
    let file_path = "../data/input.txt";
    let (vector1, vector2) = read_input(file_path);
    let computed_distance = compute_distance(vector1.clone(), vector2.clone());
    println!("computed distance: {}", computed_distance);

    let similarity_score = compute_similarity_score(&vector1, &vector2);
    println!("similarity score: {}", similarity_score);
}

fn compute_distance(mut a: Vec<i32>, mut b: Vec<i32>) -> i32 {
    a.sort();
    b.sort();
    a.into_iter()
        .zip(b)
        .map(|(x, y)| i32::abs(x - y))
        .sum::<i32>()
}

fn compute_similarity_score(a: &[i32], b: &[i32]) -> i32 {
    let mut results = vec![];

    for value in a.iter() {
        let result = b.iter().filter(|&n| *n == *value).count() as i32;
        results.push(result * value);
    }

    results.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_distance() {
        let a = vec![3, 4, 2, 1, 3, 3];
        let b = vec![4, 3, 5, 3, 9, 3];

        let want = 11;
        let got = compute_distance(a, b);

        assert_eq!(got, want);
    }

    #[test]
    fn test_compute_similarity_score() {
        let a = vec![3, 4, 2, 1, 3, 3];
        let b = vec![4, 3, 5, 3, 9, 3];

        let want = 31;
        let got = compute_similarity_score(&a, &b);

        assert_eq!(got, want);
    }
}
