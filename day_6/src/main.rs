mod day_utils;
mod first_part;
mod second_part;

use day_utils::{find_char, max_indices};
use utils::read_to_string;

fn main() {
    let input = read_to_string("../data/input_d6.txt").unwrap();
    let map = day_utils::parse_input(&input);
    let guard_index = find_char(&map, '^').unwrap();
    let mut guard = first_part::Guard::new(guard_index, (-1, 0));
    let max_bounds = max_indices(&map).unwrap();

    // Part 1
    while guard.move_forward(&map, max_bounds) {}
    println!(
        "The first part answer is: {}",
        guard.visited_positions.len()
    );

    // Part 2
    let second_part = second_part::process_map(map.clone());
    println!("The second part answer is: {}", second_part);
}
