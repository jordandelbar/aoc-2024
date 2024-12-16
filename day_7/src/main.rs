mod day_utils;

fn main() {
    let file_path = "../data/input_d7.txt";
    let input = day_utils::parse_input(file_path).unwrap();
    println!("{:?}", input);
}
