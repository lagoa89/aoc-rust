use std::fs;
use std::io::Read;
use std::{fs::File, io::BufReader};

pub fn read_buffer_to_string(filename: &str) -> Result<String, std::io::Error> {
    let file = File::open(filename)?;
    let mut buf_reader = BufReader::new(file);

    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;

    Ok(contents)
}

pub fn vec_to_string(contents: String) -> Vec<String> {
    contents
        .lines()
        .into_iter()
        .map(|line| line.trim())
        .map(|line| line.to_string())
        .collect()
}

pub fn vec_to_numbers(contents: String) -> Vec<i32> {
    contents
        .lines()
        .into_iter()
        .map(|line| line.trim())
        .map(|line| line.parse().unwrap())
        .collect()
}

pub fn read_file(filename: &str) -> Result<String, std::io::Error> {
    println!("Opening file {}", filename);
    fs::read_to_string(filename)
}

pub fn read_file_to_vec(filename: &str) -> Result<Vec<u8>, std::io::Error> {
    println!("Opening file {}", filename);
    fs::read(filename)
}

pub fn process_input_into_vec(contents: String) -> Vec<i32> {
    println!("Number of lines: {}", contents.len());

    contents
        .lines()
        .into_iter()
        .map(|line| line.trim())
        .map(|line| line.parse().unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
