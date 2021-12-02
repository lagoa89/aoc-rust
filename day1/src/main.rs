use utils::*;

const FILENAME: &str = "./day1/input.txt";

fn main() {
    let numbers = read_buffer_vec(FILENAME).unwrap();
    calculate(numbers.clone());
    calculate_sliding(numbers);
}

fn calculate_sliding(numbers: Vec<i32>) {
    let sums: Vec<i32> = (0..(numbers.len() - 2))
        .map(|i| numbers[i] + numbers[i + 1] + numbers[i + 2])
        .collect();

    calculate(sums)
}

// How many measurements are larger than the previous measurement?
fn calculate(numbers: Vec<i32>) {
    // let result = numbers.into_iter().reduce(|a, b| a + b);
    let mut previous = numbers[0];
    let mut result: i32 = 0;
    for number in numbers {
        // println!("number {}, previous {}", number, previous);
        if number > previous {
            // println!("bigger");
            result = result + 1;
        }
        previous = number
    }

    // 1713
    println!("Number larger than previous measurement {}", result)
}
