use utils::read_input_d1;

fn main() {
    let file_path = "../data/input_d1.txt";
    let (vector1, vector2) = read_input_d1(file_path);
    let computed_distance = compute_distance(vector1.clone(), vector2.clone());
    println!("computed distance: {}", computed_distance);

    let similarity_score = compute_similarity_score(&vector1, &vector2);
    println!("similarity score: {}", similarity_score);
}

fn compute_distance(mut a: Vec<u32>, mut b: Vec<u32>) -> u32 {
    a.sort();
    b.sort();
    a.into_iter()
        .zip(b)
        .map(|(x, y)| u32::abs(x - y))
        .sum::<u32>()
}

fn compute_similarity_score(a: &[u32], b: &[u32]) -> u32 {
    a.iter()
        .map(|x| x * b.iter().filter(|&y| y == x).count() as u32)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const A: [u32; 6] = [3, 4, 2, 1, 3, 3];
    const B: [u32; 6] = [4, 3, 5, 3, 9, 3];

    #[test]
    fn test_compute_distance() {
        let a = A.to_vec();
        let b = B.to_vec();

        let want = 11;
        let got = compute_distance(a, b);

        assert_eq!(got, want);
    }

    #[test]
    fn test_compute_similarity_score() {
        let a = A.to_vec();
        let b = B.to_vec();

        let want = 31;
        let got = compute_similarity_score(&a, &b);

        assert_eq!(got, want);
    }
}
