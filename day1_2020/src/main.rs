use colored::*;
use utils::*;

const TOTAL: i32 = 2020;
const FILENAME: &str = "./day1_2020/input.txt";

fn main() {
    match read_file(FILENAME) {
        Ok(contents) => {
            let numbers = process_input_into_vec(contents);
            find_two_that_add_to_total(numbers)
        }
        Err(err) => {
            println!("{} {}", "There was an error reading".red(), FILENAME.red());
            println!("{} {}", "Error:".red(), err);
        }
    }
}

fn find_two_that_add_to_total(numbers: Vec<i32>) {
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

// fn read(filename: &str) -> Result<String, std::io::Error> {
//     println!("Opening file {}", FILENAME);
//     fs::read_to_string(filename)
// }

// fn process_input_into_vec(contents: String) -> Vec<i32> {
//     println!("Number of lines: {}", contents.len());

//     contents
//         .lines()
//         .into_iter()
//         .map(|line| line.trim())
//         .map(|line| line.parse().unwrap())
//         .collect()
// }

#[test]
fn test_basic() {
    assert_eq!(1, 1)
}
