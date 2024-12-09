mod day_utils;
use utils::read_to_string;

fn main() {
    let input = read_to_string("../data/input_d6.txt").unwrap();
    let map = day_utils::parse_input(&input);
}
