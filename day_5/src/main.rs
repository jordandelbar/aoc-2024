mod day_utils;
use std::collections::HashMap;

fn main() {
    let (rules, updates) = day_utils::read_input("../data/input_d5.txt");
    let rules_dict = create_rules_dict(rules);
    let count = compute_middle_page_number(rules_dict, updates);
    println!("first part result: {}", count);
}

fn compute_middle_page_number(rules_dict: HashMap<i32, Vec<i32>>, updates: Vec<Vec<i32>>) -> i32 {
    let mut count = 0;
    for update in updates {
        println!("update: {:?}", &update);
        for (idx, &page_number) in update.iter().enumerate() {
            let preceding_pages = get_preceding_pages(idx, &update);
            println!("preceding_pages: {:?}", &preceding_pages);

            if let Some(rules_values) = rules_dict.get(&page_number) {
                if preceding_pages.iter().any(|x| rules_values.contains(x)) {
                    println!("has been triggered because: {:?}", rules_values);
                    count += find_middle_number(&update);
                    break
                }
            }
        }
    }

    count
}

fn find_middle_number(vector: &Vec<i32>) -> i32 {
    vector[vector.len() / 2]
}

fn create_rules_dict(pairs: Vec<(i32, i32)>) -> HashMap<i32, Vec<i32>> {
    let mut dict: HashMap<i32, Vec<i32>> = HashMap::new();

    for (key, value) in pairs {
        dict.entry(key).or_insert_with(Vec::new).push(value);
    }

    dict
}

fn get_preceding_pages(index: usize, input: &[i32]) -> Vec<i32> {
    input[..index].iter().rev().cloned().collect()
}

fn generate_preceding_pages(input: &[i32]) -> Vec<Vec<i32>> {
    (0..input.len())
        .map(|index| get_preceding_pages(index, input))
        .collect::<Vec<Vec<i32>>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_rule_dictionary() {
        let input = vec![(87, 5), (54, 3), (54, 4), (87, 2)];
        let want: HashMap<i32, Vec<i32>> = [(87, vec![5, 2]), (54, vec![3, 4])]
            .iter()
            .cloned()
            .collect();
        let got = create_rule_dict(input);
        assert_eq!(want, got);
    }

    #[test]
    fn test_get_preceding_pages() {
        let pages = vec![1, 2, 3, 4, 5];
        let result = get_preceding_pages(3, &pages);
        assert_eq!(result, vec![3, 2, 1]);
    }

    #[test]
    fn test_generate_preceding_pages() {
        let pages = vec![1, 2, 3, 4, 5];
        let result = generate_preceding_pages(&pages);
        assert_eq!(
            result,
            vec![vec![], vec![1], vec![2, 1], vec![3, 2, 1], vec![4, 3, 2, 1]]
        );
    }
}
