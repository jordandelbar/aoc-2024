mod day_utils;

use std::collections::{HashMap, HashSet, VecDeque};
use utils::{read_file_to_string};

fn main() {
    let input = read_file_to_string("../data/input_d5.txt").expect("Input not found..");
    let first_part_result = sum_middle_pages(&input);
    let second_part_result = sum_middle_pages_part_two(&input);
    println!("first part result: {}", first_part_result);
    println!("second part result: {}", second_part_result);
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

fn reorder_pages(rules: &[(u32, u32)], update: &[u32]) -> Vec<u32> {
    let mut in_degree = HashMap::new();
    let mut graph = HashMap::new();

    for &(x, y) in rules {
        graph.entry(x).or_insert_with(HashSet::new).insert(y);
        in_degree.entry(x).or_insert(0);
        *in_degree.entry(y).or_insert(0) += 1;
    }

    let update_set: HashSet<u32> = update.iter().cloned().collect();
    let mut filtered_graph = HashMap::new();
    let mut filtered_in_degree = HashMap::new();

    for &page in &update_set {
        if let Some(neighbors) = graph.get(&page) {
            for &neighbor in neighbors {
                if update_set.contains(&neighbor) {
                    filtered_graph
                        .entry(page)
                        .or_insert_with(HashSet::new)
                        .insert(neighbor);
                    *filtered_in_degree.entry(neighbor).or_insert(0) += 1;
                }
            }
        }
        filtered_in_degree.entry(page).or_insert(0);
    }

    let mut sorted_pages = Vec::new();
    let mut queue: VecDeque<u32> = filtered_in_degree
        .iter()
        .filter(|&(_, &count)| count == 0)
        .map(|(&page, _)| page)
        .collect();

    while let Some(page) = queue.pop_front() {
        sorted_pages.push(page);
        if let Some(neighbors) = filtered_graph.remove(&page) {
            for neighbor in neighbors {
                let entry = filtered_in_degree.get_mut(&neighbor).unwrap();
                *entry -= 1;
                if *entry == 0 {
                    queue.push_back(neighbor);
                }
            }
        }
    }

    sorted_pages
}

fn sum_middle_pages_part_two(input: &str) -> u32 {
    let mut sections = input.split("\n\n");
    let rules_section = sections.next().unwrap();
    let updates_section = sections.next().unwrap();

    let rules: Vec<(u32, u32)> = rules_section
        .lines()
        .map(|line| {
            let parts: Vec<u32> = line.split('|').map(|x| x.parse().unwrap()).collect();
            (parts[0], parts[1])
        })
        .collect();

    let updates: Vec<Vec<u32>> = updates_section
        .lines()
        .map(|line| {
            line.split(',')
                .map(|x| x.parse().unwrap())
                .collect::<Vec<u32>>()
        })
        .collect();

    let mut sum_middle_pages = 0;

    for update in &updates {
        let mut is_valid = true;
        for &(x, y) in &rules {
            let pos_x = update.iter().position(|&p| p == x);
            let pos_y = update.iter().position(|&p| p == y);
            if let (Some(px), Some(py)) = (pos_x, pos_y) {
                if px > py {
                    is_valid = false;
                    break;
                }
            }
        }

        if !is_valid {
            let reordered = reorder_pages(&rules, update);
            let middle = reordered[reordered.len() / 2];
            sum_middle_pages += middle;
        }
    }

    sum_middle_pages
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
