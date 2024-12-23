mod first_part;
mod second_part;

use utils::{read_file_to_string, string_to_grid};
use first_part::count_number_xmas;
use second_part::count_number_xmas_cross;

fn main() {
    let input_string = read_file_to_string("../data/input_d4.txt").unwrap();
    let input = string_to_grid(&input_string, false);
    println!("first part result: {}", count_number_xmas(&input));
    println!("second part result: {}", count_number_xmas_cross(&input));
}
