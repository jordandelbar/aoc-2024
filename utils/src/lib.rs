use std::fs::File;
use std::io::{self, Read};

pub fn read_file(file_path: &str) -> io::BufReader<File> {
    let file = File::open(file_path).expect("File not found");
    io::BufReader::new(file)
}

pub fn read_to_string(file_path: &str) -> io::Result<String> {
    let mut buf_reader = read_file(file_path);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    Ok(contents)
}

