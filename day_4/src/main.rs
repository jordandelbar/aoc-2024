mod first_part;
mod second_part;

use first_part::*;
use utils::read_input_d4;

fn main() {
    let input = read_input_d4("../data/input_d4.txt");
    println!("first part result: {}", count_number_xmas(&input));
}
