use utils::*;

const FILENAME: &str = "./day2/input.txt";

#[derive(Debug, PartialEq, Clone)]
enum Direction {
    FORWARD,
    DOWN,
    UP,
}

#[derive(Clone)]
struct Movement {
    direction: Direction,
    amount: i32,
}

fn main() {
    let contents = read_buffer_to_string(FILENAME).unwrap();

    let lines: Vec<&str> = contents
        .lines()
        .into_iter()
        .map(|line| line.trim())
        .collect();

    println!("Lines, {}", lines.len());

    let result = extract_into_movements(lines);

    part1(result.clone());
    part2(result);
}

fn part1(result: Vec<Movement>) {
    let horizontal_movements = result
        .clone()
        .into_iter()
        .filter(|result| result.direction == Direction::FORWARD)
        .fold(0, |acc, movement| acc + movement.amount);

    let down_movements = result
        .clone()
        .into_iter()
        .filter(|result| result.direction == Direction::DOWN)
        .fold(0, |acc, movement| acc + movement.amount);

    let up_movements = result
        .into_iter()
        .filter(|result| result.direction == Direction::UP)
        .fold(0, |acc, movement| acc + movement.amount);

    let depth = down_movements - up_movements;
    println!(
        "part 1 simp - depth {}, forward {}, multiplied {}",
        depth,
        horizontal_movements,
        depth * horizontal_movements
    );
}

fn part2(movements: Vec<Movement>) -> u128 {
    let mut aim: u128 = 0;
    let mut depth: u128 = 0;
    let mut forward: u128 = 0;
    for movement in movements {
        if movement.direction == Direction::FORWARD {
            forward = forward + u128::try_from(movement.amount).unwrap();
            depth = depth + u128::try_from(movement.amount).unwrap() * aim;
        }
        if movement.direction == Direction::DOWN {
            aim = aim + u128::try_from(movement.amount).unwrap();
        }
        if movement.direction == Direction::UP {
            aim = aim - u128::try_from(movement.amount).unwrap();
        }
    }

    println!(
        "part 2 - depth {}, forward {}, multiplied {}",
        depth,
        forward,
        depth * forward
    );

    depth * forward
}

fn extract_into_movements(lines: Vec<&str>) -> Vec<Movement> {
    let result: Vec<Movement> = lines
        .into_iter()
        .map(|line| {
            let mut result_split = line.split(" ");
            let direction_string = result_split.next().unwrap();
            let amount = result_split.next().unwrap();
            let direction = match direction_string {
                "forward" => Direction::FORWARD,
                "down" => Direction::DOWN,
                "up" => Direction::UP,
                _ => panic!("unrecognised direction"),
            };

            Movement {
                direction: direction,
                amount: amount.parse().unwrap(),
            }
        })
        .collect();
    result
}

#[cfg(test)]
mod tests {
    use crate::{part2, Direction, Movement};

    #[test]
    fn it_works() {
        let movements = vec![
            Movement {
                direction: Direction::FORWARD,
                amount: 5,
            },
            Movement {
                direction: Direction::DOWN,
                amount: 5,
            },
            Movement {
                direction: Direction::FORWARD,
                amount: 8,
            },
            Movement {
                direction: Direction::UP,
                amount: 3,
            },
            Movement {
                direction: Direction::DOWN,
                amount: 8,
            },
            Movement {
                direction: Direction::FORWARD,
                amount: 2,
            },
        ];

        let result = part2(movements);
        assert_eq!(result, 900);
    }
}
