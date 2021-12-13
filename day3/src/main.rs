use utils::*;

const FILENAME: &str = "./day3/input.txt";

fn main() {
    let contents = read_buffer_to_string(FILENAME).unwrap();
    let numbers = vec_to_string(contents);

    let result = part1(&numbers);
    println!("{}", result)
}

fn part1(input: &Vec<String>) -> u32 {
    let inputs = convert_to_vecs(input);
    let width = inputs[0].len();

    let mut bit_accs: Vec<i32> = (0..width).map(|_| 0).collect();

    for item in inputs {
        item.clone().into_iter().enumerate().for_each(|(i, bit)| {
            bit_accs[i] = bit_accs[i] + calculate(item[i]);
        });
    }

    let gamma_string: String = bit_accs
        .clone()
        .into_iter()
        .map(|bit_acc| if bit_acc > 0 { "1" } else { "0" })
        .collect::<String>();

    let epsilon_string: String = bit_accs
        .clone()
        .into_iter()
        .map(|bit_acc| if bit_acc > 0 { "0" } else { "1" })
        .collect::<String>();

    println!("{} {}", gamma_string, epsilon_string);

    let gamma: u32 = u32::from_str_radix(&gamma_string, 2).unwrap();
    let epsilon: u32 = u32::from_str_radix(&epsilon_string, 2).unwrap();

    gamma * epsilon
}

fn calculate(input: u32) -> i32 {
    if input == 1 {
        1
    } else {
        -1
    }
}

fn convert_to_vecs(input: &Vec<String>) -> Vec<Vec<u32>> {
    let f = input
        .into_iter()
        .map(|item| {
            item.chars()
                .into_iter()
                .map(|bit| bit.to_digit(10).unwrap())
                .collect()
        })
        .collect();
    f
}

#[cfg(test)]
mod tests {
    use crate::{convert_to_vecs, part1};

    #[test]
    fn can_convert() {
        let input = vec![
            "00100".to_string(),
            "11110".to_string(),
            "10110".to_string(),
            "10111".to_string(),
            "10101".to_string(),
            "01111".to_string(),
            "00111".to_string(),
            "11100".to_string(),
            "10000".to_string(),
            "11001".to_string(),
            "00010".to_string(),
            "01010".to_string(),
        ];

        let result = convert_to_vecs(&input);
        assert_eq!(1, 1)
    }

    #[test]
    fn it_works() {
        let input = vec![
            "00100".to_string(),
            "11110".to_string(),
            "10110".to_string(),
            "10111".to_string(),
            "10101".to_string(),
            "01111".to_string(),
            "00111".to_string(),
            "11100".to_string(),
            "10000".to_string(),
            "11001".to_string(),
            "00010".to_string(),
            "01010".to_string(),
        ];

        let result = part1(&input);
        // let gamma = "10110";
        // let epsilon = "01001";
        // assert_eq!(result.0, gamma);
        // assert_eq!(result.1, epsilon);

        assert_eq!(result, 198)
    }
}
