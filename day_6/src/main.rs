mod day_utils;
mod first_part;
mod second_part;

use day_utils::{find_char, max_indices};
use first_part::Guard;
use utils::read_to_string;

fn main() {
    let input = read_to_string("../data/input_d6.txt").unwrap();
    let map = day_utils::parse_input(&input);
    let guard_index = find_char(&map, '^').unwrap();
    let mut guard = Guard::new(guard_index, (-1, 0));
    let max_bounds = max_indices(&map).unwrap();
    while guard.move_forward(map.clone(), max_bounds) {}
    println!("The answer is: {}", guard.visited_positions.len())
}
