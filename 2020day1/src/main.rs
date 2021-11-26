use colored::*;
use std::fs;

const TOTAL: i32 = 2020;
const FILENAME: &str = "./2020day1/input.txt";

fn main() {
    // let contents = fs::read_to_string(FILENAME)
    //     .map_err(|_| "Something went wrong ")
    //     .unwrap(); //.expect("Something went wrong reading the file");

    match read(FILENAME) {
        Ok(contents) => {
            process_input(contents);
        }
        Err(err) => {
            println!("{} {}", "There was an error reading".red(), FILENAME.red());
            println!("{} {}", "Error:".red(), err);
        }
    }
}

fn read(filename: &str) -> Result<String, std::io::Error> {
    println!("Opening file {}", FILENAME);
    fs::read_to_string(filename)
}

fn process_input(contents: String) {
    println!("Number of lines: {}", contents.len());

    let numbers: Vec<i32> = contents
        .lines()
        .into_iter()
        .map(|line| line.trim())
        .map(|line| line.parse().unwrap())
        .collect();

    let mut found = false;
    for number1 in numbers.clone() {
        for number2 in numbers.clone() {
            if number1 + number2 == TOTAL {
                found = true;

                println!(
                    "Found them {} {}",
                    number1.to_string().green(),
                    number2.to_string().green()
                );
                println!(
                    "Multiplied together {}",
                    (number1 * number2).to_string().green()
                );

                break;
            }
        }
        if found {
            break;
        };
    }
}

#[test]
fn test_basic() {
    assert_eq!(1, 1)
}
