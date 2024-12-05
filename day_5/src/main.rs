mod day_utils;

use utils::*;
use std::collections::{HashMap, HashSet, VecDeque};

fn main() {
    let input = read_to_string("../data/input_d5.txt").expect("Input not found..");
    let result = sum_middle_pages(&input);
    println!("first part result: {}", result);
}

fn sum_middle_pages(input: &str) -> i32 {
    let (rules, updates) = day_utils::parse_input(input);

    updates
        .iter()
        .filter(|update| is_valid_update(&rules, update))
        .map(|update| update[update.len() / 2])
        .sum()
}

fn is_valid_update(rules: &[(i32, i32)], update: &[i32]) -> bool {
    let mut graph: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut in_degree: HashMap<i32, usize> = HashMap::new();
    let update_set: HashSet<i32> = update.iter().cloned().collect();

    for &(x, y) in rules {
        if update_set.contains(&x) && update_set.contains(&y) {
            graph.entry(x).or_default().push(y);
            *in_degree.entry(y).or_default() += 1;
            in_degree.entry(x).or_default();
        }
    }

    let mut queue: VecDeque<i32> = in_degree
        .iter()
        .filter(|&(_, &deg)| deg == 0)
        .map(|(&node, _)| node)
        .collect();

    let mut sorted_order = Vec::new();

    while let Some(current) = queue.pop_front() {
        sorted_order.push(current);
        if let Some(neighbors) = graph.get(&current) {
            for &neighbor in neighbors {
                if let Some(degree) = in_degree.get_mut(&neighbor) {
                    *degree -= 1;
                    if *degree == 0 {
                        queue.push_back(neighbor);
                    }
                }
            }
        }
    }

    sorted_order == update
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_valid_update() {
        let input = "\
47|53
53|29

47,53,29";
        let result = sum_middle_pages(input);
        assert_eq!(result, 53);
    }
}

#[test]
fn test_single_invalid_update() {
    let input = "\
47|53
53|29

47,29,53";
    let result = sum_middle_pages(input);
    assert_eq!(result, 0);
}