mod day_utils;
use std::collections::HashMap;

fn main() {
    let (rules, pages) = day_utils::read_input("../data/input_d5.txt");
    let my_dict = create_rule_dict(rules);
    let following_pages = get_following_pages(0, &pages[0]);
    println!("{:?}", following_pages);
}

fn create_rule_dict(pairs: Vec<(i32, i32)>) -> HashMap<i32, Vec<i32>> {
    let mut dict: HashMap<i32, Vec<i32>> = HashMap::new();

    for (key, value) in pairs {
        dict.entry(key).or_insert_with(Vec::new).push(value);
    }

    dict
}

fn get_following_pages(index: u32, input: &Vec<i32>) -> Vec<i32> {
    let mut output = Vec::new();
    let index = index + 1;
    for i in index as usize..input.len() {
        output.push(input[i]);
    }

    output
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
    fn test_get_following_pages() {
        let input = vec![5, 4, 3, 2, 1];
        let want = vec![3, 2, 1];
        let got = get_following_pages(1, &input);
        assert_eq!(want, got);
    }
}
