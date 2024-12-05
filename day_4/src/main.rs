mod first_part;
mod second_part;
mod day_utils;

use first_part::*;
use second_part::*;

fn main() {
    let input = day_utils::read_input("../data/input_d4.txt");
    println!("first part result: {}", count_number_xmas(&input));
    println!("second part result: {}", count_number_xmas_cross(&input));
}
